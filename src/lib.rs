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
//!     type Inst = ccm::i2c::I2C;
//!     fn instance(&self) -> Self::Inst {
//!         match self.instance_id {
//!             1 => ccm::i2c::I2C::I2C1,
//!             2 => ccm::i2c::I2C::I2C2,
//!             #[cfg(feature = "imxrt1060")]
//!             3 => ccm::i2c::I2C::I2C3,
//!             #[cfg(feature = "imxrt1060")]
//!             4 => ccm::i2c::I2C::I2C4,
//!             _ => unreachable!()
//!         }
//!     }
//!     fn is_valid(inst: Self::Inst) -> bool {
//!         #[allow(unreachable_patterns)]
//!         match inst {
//!             ccm::i2c::I2C::I2C1 | ccm::i2c::I2C::I2C2 => true,
//!             #[cfg(feature = "imxrt1060")]
//!             ccm::i2c::I2C::I2C3 | ccm::i2c::I2C::I2C4 => true,
//!             _ => false,
//!         }
//!     }
//! }
//!
//! struct MyClocks;
//! impl ccm::Clocks for MyClocks {
//!     type I2C = I2C;
//!     // Other clock types...
//! #   type SPI = ();
//! #   type UART = ();
//! #   type GPT = ();
//! #   type PIT = ();
//! }
//! type CCM = ccm::CCM<MyClocks>;
//!
//! fn take_ccm() -> Option<CCM> {
//!   // TODO safety check that ensures
//!   // CCM only taken once!
//!   Some(unsafe { CCM::new() })
//! }
//!
//! let mut ccm = take_ccm().unwrap();
//! // Enable the clock, which disables all clock gates
//! let mut i2c_clock = ccm.i2c_clock_mut().configure_divider(8);
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
//! #     type Inst = ccm::i2c::I2C;
//! #     fn instance(&self) -> Self::Inst {
//! #         match self.instance_id {
//! #             1 => ccm::i2c::I2C::I2C1,
//! #             2 => ccm::i2c::I2C::I2C2,
//! #             #[cfg(feature = "imxrt1060")]
//! #             3 => ccm::i2c::I2C::I2C3,
//! #             #[cfg(feature = "imxrt1060")]
//! #             4 => ccm::i2c::I2C::I2C4,
//! #             _ => unreachable!()
//! #         }
//! #     }
//! #     fn is_valid(inst: Self::Inst) -> bool {
//! #         #[allow(unreachable_patterns)]
//! #         match inst {
//! #             ccm::i2c::I2C::I2C1 | ccm::i2c::I2C::I2C2 => true,
//! #             #[cfg(feature = "imxrt1060")]
//! #             ccm::i2c::I2C::I2C3 | ccm::i2c::I2C::I2C4 => true,
//! #             _ => false,
//! #         }
//! #     }
//! # }
//! # struct MyClocks;
//! # impl ccm::Clocks for MyClocks {
//! #     type I2C = I2C;
//! #     // Other clock types...
//! #   type SPI = ();
//! #   type UART = ();
//! #   type GPT = ();
//! #   type PIT = ();
//! # }
//! # type CCM = ccm::CCM<MyClocks>;
//!
//! struct I2CDriver { inst: I2C }
//! impl I2CDriver {
//!     pub fn new(inst: I2C, clock: &ccm::i2c::I2CClock<I2C>) -> I2CDriver {
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
//! # let mut ccm = unsafe { CCM::new() };
//! // Enable I2C3 clock gate
//! ccm.i2c_clock_mut().set_clock_gate(&mut i2c3, ccm::ClockGate::On);
//! // Create the higher-level driver, requires the I2C clock
//! let i2c = I2CDriver::new(i2c3, ccm.i2c_clock());
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
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod arm;
mod gate;
pub mod i2c;
pub mod perclock;
mod register;
pub mod spi;
pub mod uart;

#[cfg(feature = "imxrt-ral")]
#[cfg_attr(docsrs, doc(cfg(feature = "imxrt-ral")))]
pub mod ral;

use core::marker::PhantomData;

use perclock::PerClock;

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
    impl Sealed for super::perclock::GPT {}
    impl Sealed for super::i2c::I2C {}
    impl Sealed for super::perclock::PIT {}
    impl Sealed for super::PWM {}
    impl Sealed for super::spi::SPI {}
    impl Sealed for super::uart::UART {}
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
/// # use imxrt_ccm::{Instance, perclock::GPT};
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
/// # use imxrt_ccm::{Instance, perclock::GPT};
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
/// and a mutable reference to the [`CCM`]. An incorrect
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

/// Correlates an instance type to a CCM clock root
///
/// If you're usage doesn't require a clock, fill in an empty
/// tuple, `()`, or any type that _doesn't_ implement [`Instance`].
pub trait Clocks {
    /// PIT instance
    type PIT;
    /// GPT instance
    type GPT;
    /// UART instance
    type UART;
    /// SPI instance
    type SPI;
    /// I2C instance
    type I2C;
}

/// The clock control module (CCM)
#[non_exhaustive]
pub struct CCM<C: Clocks> {
    /// The periodic clock handle
    ///
    /// `perclock` is used for timers, including GPT and PIT timers
    perclock: perclock::PerClock<C::PIT, C::GPT>,
    /// The UART clock
    ///
    /// `uart_clock` is for UART peripherals.
    uart_clock: uart::UARTClock<C::UART>,
    /// The SPI clock
    ///
    /// `spi_clock` is for SPI peripherals.
    spi_clock: spi::SPIClock<C::SPI>,
    /// The I2C clock
    ///
    /// `i2c_clock` is for I2C peripherals.
    i2c_clock: i2c::I2CClock<C::I2C>,
    /// Marker to prevent default Sync implementation
    _not_sync: PhantomData<*const ()>,
}

unsafe impl<C: Clocks> Send for CCM<C> {}

impl<C: Clocks> CCM<C> {
    /// Construct a new CCM peripheral
    ///
    /// # Safety
    ///
    /// This should only be called once. Ideally, it's encapsulated behind another
    /// constructor that takes ownership of CCM peripheral memory. Calling this more
    /// than once will let you access global, mutable memory that's assumed to not
    /// be aliased.
    pub unsafe fn new() -> Self {
        CCM {
            perclock: perclock::PerClock::new(),
            uart_clock: uart::UARTClock::new(),
            spi_clock: spi::SPIClock::new(),
            i2c_clock: i2c::I2CClock::new(),
            _not_sync: PhantomData,
        }
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
    pub fn set_frequency_arm(&mut self, hz: u32) -> (arm::ARMClock, arm::IPGClock) {
        // Safety: we own the CCM peripheral memory
        unsafe { arm::set_frequency(hz) }
    }

    /// Returns the ARM and IPG clock frequencies
    #[inline(always)]
    pub fn frequency_arm(&self) -> (arm::ARMClock, arm::IPGClock) {
        // Safety: we own the CCM peripheral memory
        unsafe { arm::frequency() }
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

impl<C> CCM<C>
where
    C: Clocks,
    C::PIT: Instance<Inst = perclock::PIT>,
    C::GPT: Instance<Inst = perclock::GPT>,
{
    /// Returns a reference to the periodic clock
    pub fn perclock(&self) -> &PerClock<C::PIT, C::GPT> {
        &self.perclock
    }
    /// Returns a mutable reference to the periodic clock
    pub fn perclock_mut(&mut self) -> &mut PerClock<C::PIT, C::GPT> {
        &mut self.perclock
    }
}

impl<C> CCM<C>
where
    C: Clocks,
    C::I2C: Instance<Inst = i2c::I2C>,
{
    /// Returns a reference to the I2C clock
    pub fn i2c_clock(&self) -> &i2c::I2CClock<C::I2C> {
        &self.i2c_clock
    }
    /// Returns a mutable reference to the I2C clock
    pub fn i2c_clock_mut(&mut self) -> &mut i2c::I2CClock<C::I2C> {
        &mut self.i2c_clock
    }
}

impl<C> CCM<C>
where
    C: Clocks,
    C::SPI: Instance<Inst = spi::SPI>,
{
    /// Returns a reference to the SPI clock
    pub fn spi_clock(&self) -> &spi::SPIClock<C::SPI> {
        &self.spi_clock
    }
    /// Returns a mutable reference to the SPI clock
    pub fn spi_clock_mut(&mut self) -> &mut spi::SPIClock<C::SPI> {
        &mut self.spi_clock
    }
}

impl<C> CCM<C>
where
    C: Clocks,
    C::UART: Instance<Inst = uart::UART>,
{
    /// Returns a reference to the UART clock
    pub fn uart_clock(&self) -> &uart::UARTClock<C::UART> {
        &self.uart_clock
    }
    /// Returns a mutable reference to the uart clock
    pub fn uart_clock_mut(&mut self) -> &mut uart::UARTClock<C::UART> {
        &mut self.uart_clock
    }
}
