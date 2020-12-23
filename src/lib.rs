//! Clock Control Module (CCM) driver for i.MX RT systems
//!
//! `imxrt-ccm` let you configure clocks, associate peripherals with clock gates, and control
//! peripheral clock gates. It is a lower-level driver, targeted for
//! HAL implementers. We encourage you to re-export `imxrt-ccm` APIs in your larger libraries.
//!
//! The rest of this documentation is for HAL implementers, or users who want to create higher-level
//! abstractions which require CCM configuration. HAL users who are using re-exported `imxrt-ccm` APIs
//! should consult their HAL documentation for more information on clock configuration.
//!
//! # Usage
//!
//! Implement [`Instance`](trait.Instance.html) for your peripheral instances. It's your decision on what
//! qualifies as a "peripheral instance." It could be a type that could represent the register block. Or,
//! it could represent your peripheral driver.
//!
//! This CCM driver supports clock gating for a variety of peripherals; look for "peripheral instance"
//! in documentation. You should encapsulate any chip-specific details in the `Instance` implementation.
//!
//! Then, create a safe wrapper around [`CCM::new`](struct.CCM.html#method.new) so that your users
//! can safely acquire the CCM peripheral. Your API must ensure that there is only one CCM instance.
//! The types wrapped by your CCM clocks should reflect your `Instance` implementations.
//!
//! Here's an example of how to implement an I2C `Instance` for compatibility with the I2C clock. The example
//! shows how you might include support for the two extra I2C peripherals that are available on a 1060
//! chip family.
//!
//! ```no_run
//! use imxrt_ccm as ccm;
//!
//! /// Our I2C instance
//! struct I2C {
//!     instance_id: usize,
//!     // Other members...
//! }
//!
//! unsafe impl ccm::Instance for I2C {
//!     type Inst = ccm::I2C;
//!     fn instance(&self) -> Self::Inst {
//!         match self.instance_id {
//!             1 => ccm::I2C::I2C1,
//!             2 => ccm::I2C::I2C2,
//!             #[cfg(feature = "imxrt1060")]
//!             3 => ccm::I2C::I2C3,
//!             #[cfg(feature = "imxrt1060")]
//!             4 => ccm::I2C::I2C4,
//!             _ => unreachable!()
//!         }
//!     }
//!     fn is_valid(inst: Self::Inst) -> bool {
//!         #[allow(unreachable_patterns)]
//!         match inst {
//!             ccm::I2C::I2C1 | ccm::I2C::I2C2 => true,
//!             #[cfg(feature = "imxrt1060")]
//!             ccm::I2C::I2C3 | ccm::I2C::I2C4 => true,
//!             _ => false,
//!         }
//!     }
//! }
//!
//! type CCM = ccm::CCM<
//!     # (), (), (), (),
//!     // Other clock types...
//!     I2C
//! >;
//!
//! fn take_ccm() -> Option<CCM> {
//!   // TODO safety check that ensures
//!   // CCM only taken once!
//!   Some(unsafe { CCM::new() })
//! }
//!
//! let CCM{ mut handle, i2c_clock, .. } = take_ccm().unwrap();
//! // Enable the clock, which disables all clock gates
//! let mut i2c_clock = i2c_clock.enable(&mut handle);
//! ```
//!
//! We recommend that you create driver initialization APIs that require clocks. By requiring an immutable
//! clock, you guarantee that a user has enabled the peripheral clock in their code.
//!
//! ```no_run
//! # use imxrt_ccm as ccm;
//! #
//! # /// Our I2C instance
//! # struct I2C {
//! #     instance_id: usize,
//! #     // Other members...
//! # }
//! #
//! # unsafe impl ccm::Instance for I2C {
//! #     type Inst = ccm::I2C;
//! #     fn instance(&self) -> Self::Inst {
//! #         match self.instance_id {
//! #             1 => ccm::I2C::I2C1,
//! #             2 => ccm::I2C::I2C2,
//! #             #[cfg(feature = "imxrt1060")]
//! #             3 => ccm::I2C::I2C3,
//! #             #[cfg(feature = "imxrt1060")]
//! #             4 => ccm::I2C::I2C4,
//! #             _ => unreachable!()
//! #         }
//! #     }
//! #     fn is_valid(inst: Self::Inst) -> bool {
//! #         #[allow(unreachable_patterns)]
//! #         match inst {
//! #             ccm::I2C::I2C1 | ccm::I2C::I2C2 => true,
//! #             #[cfg(feature = "imxrt1060")]
//! #             ccm::I2C::I2C3 | ccm::I2C::I2C4 => true,
//! #             _ => false,
//! #         }
//! #     }
//! # }
//! struct I2CDriver {
//!     inst: I2C,
//!     // ...
//! }
//!
//! impl I2CDriver {
//!     pub fn new(inst: I2C, clock: &ccm::I2CClock<I2C>) -> I2CDriver {
//!         // ...
//!         I2CDriver {
//!             inst,
//!             // ...
//!         }
//!     }
//! }
//!
//! let mut i2c3 = // Get I2C3 instance...
//!     # I2C { instance_id: 3 };
//! # let mut i2c_clock = unsafe { ccm::I2CClock::<I2C>::assume_enabled() };
//! // Enable I2C3 clock gate
//! i2c_clock.set_clock_gate(&mut i2c3, ccm::ClockGate::On);
//! // Create the higher-level driver, requires the I2C clock
//! let i2c = I2CDriver::new(i2c3, &i2c_clock);
//! ```
//!
//! # `imxrt-ral` support
//!
//! `imxrt-ccm` provides support for `imxrt-ral`. The feature includes `Instance` implementations on
//! all supported `imxrt-ral` peripheral instances. It also include helper functions and types,
//! which are exported in the `ral` module. Use the `imxrt-ral` support if your HAL already depends on
//! the `imxrt-ral` crate.
//!
//! To use the `imxrt-ral` support, enable the `imxrt-ral` feature. You must ensure that something else
//! in your dependency graph enables the correct `imxrt-ral` feature for your processor. See the
//! `imxrt-ral` documentation for more information.
//!
//! # Chip support
//!
//! `imxrt-ccm` does not require you to select a chip. If you do not select a chip, the crate provides
//! the most conservative implementation to support all i.MX RT vairants. However, `imxrt-ccm` has i.MX RT chip
//! features to specialize the CCM driver for your system. You *should* enable one of these features in your
//! final program, but it's not required.
//!
//! The table below describes `imxrt-ccm` chip support.
//!
//! | Feature       | Description                                                       |
//! | ------------- | ----------------------------------------------------------------- |
//! | `"imxrt1010"` | Support for i.MX RT 1010 processors, like iMXRT1011               |
//! | `"imxrt1060"` | Support for i.MX RT 1060 processors, like iMXRT1061 and iMXRT1062 |
//!
//! If you enable the `imxrt-ral` feature, you **must** enable one of these features.

#![cfg_attr(not(test), no_std)]

#[cfg(test)]
macro_rules! assert_send {
    ($type:ty) => {
        ::static_assertions::assert_impl_all!($type: Send);
    };
}
#[cfg(test)]
macro_rules! assert_not_sync {
    ($type:ty) => {
        ::static_assertions::assert_not_impl_any!($type: Sync);
    };
}

mod arm;
mod gate;
mod i2c;
mod perclock;
mod register;
mod spi;
mod uart;

#[cfg(feature = "imxrt-ral")]
pub mod ral;

pub use arm::{frequency as frequency_arm, set_frequency as set_frequency_arm, ARMClock, IPGClock};
pub use i2c::{configure as configure_i2c, frequency as frequency_i2c, I2C};
pub use perclock::{configure as configure_perclock, frequency as frequency_perclk, GPT, PIT};
pub use spi::{configure as configure_spi, frequency as frequency_spi, SPI};
pub use uart::{configure as configure_uart, frequency as frequency_uart, UART};

use core::marker::PhantomData;

/// Describes the location of a clock gate field
#[derive(Clone, Copy)]
pub struct ClockGateLocation {
    /// CCGR register offset
    ///
    /// `3` in `CCM_CCGR3[CG7]`
    offset: usize,
    /// Clock gate fields
    ///
    /// `&[7]` in `CCM_CCGR3[CG7]`
    gates: &'static [usize],
}

/// A type that can locate a clock gate
///
/// `ClockGateLocator` is implemented on all structs and enums
/// that describe peripheral instances.
pub trait ClockGateLocator: Copy + PartialEq + private::Sealed {
    /// Returns the location of a clock gate
    fn location(&self) -> ClockGateLocation;
}

mod private {
    pub trait Sealed {}
    impl Sealed for super::ADC {}
    impl Sealed for super::DCDC {}
    impl Sealed for super::DMA {}
    impl Sealed for super::GPT {}
    impl Sealed for super::I2C {}
    impl Sealed for super::PIT {}
    impl Sealed for super::PWM {}
    impl Sealed for super::SPI {}
    impl Sealed for super::UART {}
}

/// A peripheral instance that has a clock gate
///
/// `Instance` lets you associate a peripheral with its clock gate. This lets you control a peripheral's
/// clock gate by supplying the peripheral itself, rather than modifying an arbitrary field in a CCM
/// register.
///
/// Implementers must hold the invariant: the return of `instance`, passed into `is_valid`, must be
/// `true`. If `instance` never returns a variant, `is_valid` must return `false`.
///
/// Suppose that an i.MX RT processor only supports one GPT instance. Here's an example of a valid `Instance`
/// implementation. It's assumed that there is only one `MyGPT` object, and that it represents the GPT1
/// timer.
///
/// ```
/// # use imxrt_ccm::{Instance, GPT};
/// struct MyGPT;
/// unsafe impl Instance for MyGPT {
///     type Inst = GPT;
///     fn instance(&self) -> Self::Inst {
///         GPT::GPT1
///     }
///     fn is_valid(inst: GPT) -> bool {
///         inst == GPT::GPT1
///     }
/// }
///
/// let my_gpt = MyGPT;
/// assert!(MyGPT::is_valid(my_gpt.instance()));
/// ```
///
/// If `is_valid` returned `false` when called with `GPT::GPT1`, the implementation is invalid.
///
/// ```should_panic
/// # use imxrt_ccm::{Instance, GPT};
/// # struct MyGPT;
/// unsafe impl Instance for MyGPT {
///     type Inst = GPT;
///     fn instance(&self) -> Self::Inst {
///         GPT::GPT1
///     }
///     fn is_valid(inst: GPT) -> bool {
///         false // Invalid implementation!
///     }
/// }
/// # let my_gpt = MyGPT;
/// # assert!(MyGPT::is_valid(my_gpt.instance()));
/// ```
///
/// # Safety
///
/// You should only implement `Instance` on a true i.MX RT peripheral instance.
/// `Instance`s are only used when you have both a mutable reference to the instance,
/// and a mutable reference to the CCM [`Handle`](struct.Handle.html). An incorrect
/// implementation will let you control global, mutable state that should not be
/// associated with the object.
pub unsafe trait Instance {
    /// An identifier that describes the instance
    type Inst: ClockGateLocator;
    /// Returns the peripheral instance identifier
    fn instance(&self) -> Self::Inst;
    /// Returns `true` if this instance is valid for a particular
    /// implementation
    fn is_valid(inst: Self::Inst) -> bool;
}

/// Returns `Some(inst)` if `inst` is valid for this peripheral, or
/// `None` if `inst` is not valid.
#[inline(always)]
fn check_instance<I: Instance>(inst: I::Inst) -> Option<I::Inst> {
    Some(inst).filter(|inst| I::is_valid(*inst))
}

/// Peripheral instance identifier for DCDC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DCDC;

impl ClockGateLocator for DCDC {
    fn location(&self) -> ClockGateLocation {
        ClockGateLocation {
            offset: 6,
            gates: &[3],
        }
    }
}

/// Peripheral instance identifier for DMA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DMA;

impl ClockGateLocator for DMA {
    #[inline(always)]
    fn location(&self) -> ClockGateLocation {
        ClockGateLocation {
            offset: 5,
            gates: &[3],
        }
    }
}

/// Set the clock gate for a peripheral instance
///
/// `set_clock_gate` does nothing if the instance is invalid.
///
/// # Safety
///
/// This modifies global, mutable memory that's owned by the `CCM`. Calling this
/// function will let you change a clock gate setting for any peripheral instance.
#[inline(always)]
pub unsafe fn set_clock_gate<I: Instance>(inst: I::Inst, gate: ClockGate) {
    if let Some(inst) = check_instance::<I>(inst) {
        gate::set(&inst.location(), gate as u8)
    }
}

/// Returns the clock gate setting for a peripheral instance
///
/// `get_clock_gate` returns `None` if the instance is invalid.
#[inline(always)]
pub fn get_clock_gate<I: Instance>(inst: I::Inst) -> Option<ClockGate> {
    check_instance::<I>(inst).map(|inst| {
        let raw = gate::get(&inst.location());
        ClockGate::from_u8(raw)
    })
}

/// Peripheral instance identifier for ADCs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ADC {
    ADC1,
    ADC2,
}

impl ClockGateLocator for ADC {
    #[inline(always)]
    fn location(&self) -> ClockGateLocation {
        let gates = match self {
            ADC::ADC1 => &[8],
            ADC::ADC2 => &[4],
        };
        ClockGateLocation { offset: 1, gates }
    }
}

/// Peripheral instance identifier for PWM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PWM {
    PWM1,
    PWM2,
    PWM3,
    PWM4,
}

impl ClockGateLocator for PWM {
    #[inline(always)]
    fn location(&self) -> ClockGateLocation {
        let gates = match self {
            PWM::PWM1 => &[8],
            PWM::PWM2 => &[9],
            PWM::PWM3 => &[10],
            PWM::PWM4 => &[11],
        };
        ClockGateLocation { offset: 4, gates }
    }
}

/// Handle to the CCM register block
///
/// `Handle` also supports clock gating for peripherals that
/// don't have an obvious clock root, like DMA.
pub struct Handle(PhantomData<*const ()>);

unsafe impl Send for Handle {}

impl Handle {
    /// Create an instance to the CCM Handle
    ///
    /// # Safety
    ///
    /// The returned `Handle` may mutably alias another `Handle`.
    /// Users should use a safer interface to acquire a [`CCM`],
    /// which contains the `Handle` you should use.
    pub const unsafe fn new() -> Self {
        Handle(PhantomData)
    }

    /// Returns the clock gate setting for the DCDC buck converter
    #[inline(always)]
    pub fn clock_gate_dcdc<D>(&self, dcdc: &D) -> ClockGate
    where
        D: Instance<Inst = DCDC>,
    {
        // Unwrap OK: we have the instance, or the `Instance`
        // implementation is incorrect.
        get_clock_gate::<D>(dcdc.instance()).unwrap()
    }

    /// Set the clock gate for the DCDC buck converter
    #[inline(always)]
    pub fn set_clock_gate_dcdc<D>(&mut self, dcdc: &mut D, gate: ClockGate)
    where
        D: Instance<Inst = DCDC>,
    {
        unsafe { set_clock_gate::<D>(dcdc.instance(), gate) };
    }

    /// Returns the clock gate setting for the DMA controller
    #[inline(always)]
    pub fn clock_gate_dma<D>(&self, dma: &D) -> ClockGate
    where
        D: Instance<Inst = DMA>,
    {
        // Unwrap OK: we have the instance, or the `Instance`
        // implementation is incorrect.
        get_clock_gate::<D>(dma.instance()).unwrap()
    }

    /// Set the clock gate for the DMA controller
    #[inline(always)]
    pub fn set_clock_gate_dma<D>(&mut self, dma: &mut D, gate: ClockGate)
    where
        D: Instance<Inst = DMA>,
    {
        unsafe { set_clock_gate::<D>(dma.instance(), gate) };
    }

    /// Returns the clock gate setting for the ADC
    #[inline(always)]
    pub fn clock_gate_adc<A>(&self, adc: &A) -> ClockGate
    where
        A: Instance<Inst = ADC>,
    {
        // Unwrap OK: we have the instance, or the `Instance`
        // implementation is incorrect.
        get_clock_gate::<A>(adc.instance()).unwrap()
    }

    /// Set the clock gate for the ADC peripheral
    #[inline(always)]
    pub fn set_clock_gate_adc<A>(&mut self, adc: &mut A, gate: ClockGate)
    where
        A: Instance<Inst = ADC>,
    {
        unsafe { set_clock_gate::<A>(adc.instance(), gate) }
    }

    /// Returns the clock gate setting for the ADC
    #[inline(always)]
    pub fn clock_gate_pwm<P>(&self, pwm: &P) -> ClockGate
    where
        P: Instance<Inst = PWM>,
    {
        // Unwrap OK: we have the instance, or the `Instance`
        // implementation is incorrect.
        get_clock_gate::<P>(pwm.instance()).unwrap()
    }

    /// Set the clock gate for the PWM peripheral
    #[inline(always)]
    pub fn set_clock_gate_pwm<P>(&mut self, pwm: &mut P, gate: ClockGate)
    where
        P: Instance<Inst = PWM>,
    {
        unsafe { set_clock_gate::<P>(pwm.instance(), gate) }
    }

    /// Set the ARM clock frequency, returning the new ARM and IPG clock frequency
    //
    /// Changing this at runtime will affect anything that's using the ARM or IPG clocks
    /// as inputs. Keep this in mind when changing the core clock frequency throughout
    /// your programs.
    #[inline(always)]
    pub fn set_frequency_arm(hz: u32) -> (ARMClock, IPGClock) {
        // Safety: we own the CCM peripheral memory
        unsafe { arm::set_frequency(hz) }
    }

    /// Returns the ARM and IPG clock frequencies
    #[inline(always)]
    pub fn frequency_arm() -> (ARMClock, IPGClock) {
        // Safety: we own the CCM peripheral memory
        unsafe { arm::frequency() }
    }
}

/// The root clocks and CCM handle
///
/// Most root clocks are disabled. Call `enable`, and supply the
/// `handle`, to enable them.
#[non_exhaustive]
pub struct CCM<P, G, U, S, I> {
    /// The handle to the CCM register block
    ///
    /// `Handle` is used throughout the HAL
    pub handle: Handle,
    /// The periodic clock handle
    ///
    /// `perclock` is used for timers, including GPT and PIT timers
    pub perclock: Disabled<PerClock<P, G>>,
    /// The UART clock
    ///
    /// `uart_clock` is for UART peripherals.
    pub uart_clock: Disabled<UARTClock<U>>,
    /// The SPI clock
    ///
    /// `spi_clock` is for SPI peripherals.
    pub spi_clock: Disabled<SPIClock<S>>,
    /// The I2C clock
    ///
    /// `i2c_clock` is for I2C peripherals.
    pub i2c_clock: Disabled<I2CClock<I>>,
}

impl<P, G, U, S, I> CCM<P, G, U, S, I> {
    /// Construct a new CCM peripheral
    ///
    /// # Safety
    ///
    /// This should only be called once. Ideally, it's encapsulated behind another
    /// constructor that takes ownership of CCM peripheral memory. Calling this more
    /// than once will let you access global, mutable memory that's assumed to not
    /// be aliased.
    pub const unsafe fn new() -> Self {
        CCM {
            handle: Handle::new(),
            perclock: Disabled(PerClock::assume_enabled()),
            uart_clock: Disabled(UARTClock::assume_enabled()),
            spi_clock: Disabled(SPIClock::assume_enabled()),
            i2c_clock: Disabled(I2CClock::assume_enabled()),
        }
    }
}

/// Describes a clock gate setting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockGate {
    /// Clock is off during all modes
    ///
    /// Stop enter hardware handshake is disabled.
    Off = 0b00,
    /// Clock is on in run mode, but off in wait and stop modes
    OnlyRun = 0b01,
    /// Clock is on in all modes, except stop mode
    On = 0b11,
}

impl ClockGate {
    #[inline(always)]
    fn from_u8(raw: u8) -> ClockGate {
        match raw & 0b11 {
            off if off == ClockGate::Off as u8 => ClockGate::Off,
            only_run if only_run == ClockGate::OnlyRun as u8 => ClockGate::OnlyRun,
            on if on == ClockGate::On as u8 => ClockGate::On,
            _ => unreachable!(),
        }
    }
}

/// Crystal oscillator frequency
const OSCILLATOR_FREQUENCY_HZ: u32 = 24_000_000;

/// A disabled clock of type `Clock`
///
/// Call `enable` on your instance to enable the clock.
pub struct Disabled<Clock>(Clock);

/// The periodic clock root
///
/// `PerClock` is the input clock for GPT and PIT.
pub struct PerClock<P, G>(PhantomData<(P, G)>);

impl<P, G> PerClock<P, G> {
    /// Assume that the clock is enabled, and acquire the enabled clock
    ///
    /// # Safety
    ///
    /// This may create an alias to memory that is mutably owned by another instance.
    /// Users should only `assume_enabled` when configuring clocks through another
    /// API.
    pub const unsafe fn assume_enabled() -> Self {
        Self(PhantomData)
    }
}

/// The UART clock
///
/// The UART clock is based on the crystal oscillator.
pub struct UARTClock<C>(PhantomData<C>);

impl<C> UARTClock<C> {
    /// Assume that the clock is enabled, and acquire the enabled clock
    ///
    /// # Safety
    ///
    /// This may create an alias to memory that is mutably owned by another instance.
    /// Users should only `assume_enabled` when configuring clocks through another
    /// API.
    pub const unsafe fn assume_enabled() -> Self {
        Self(PhantomData)
    }
}

/// The SPI clock
///
/// The SPI clock is based on PLL2.
pub struct SPIClock<S>(PhantomData<S>);

impl<S> SPIClock<S> {
    /// Assume that the clock is enabled, and acquire the enabled clock
    ///
    /// # Safety
    ///
    /// This may create an alias to memory that is mutably owned by another instance.
    /// Users should only `assume_enabled` when configuring clocks through another
    /// API.
    pub const unsafe fn assume_enabled() -> Self {
        Self(PhantomData)
    }
}

/// The I2C clock
///
/// The I2C clock is based on the crystal oscillator.
pub struct I2CClock<I>(PhantomData<I>);

impl<I> I2CClock<I> {
    /// Assume that the clock is enabled, and acquire the enabled clock
    ///
    /// # Safety
    ///
    /// This may create an alias to memory that is mutably owned by another instance.
    /// Users should only `assume_enabled` when configuring clocks through another
    /// API.
    pub const unsafe fn assume_enabled() -> Self {
        Self(PhantomData)
    }
}

#[cfg(test)]
mod tests {
    assert_send!(super::Handle);
    assert_not_sync!(super::Handle);
}
