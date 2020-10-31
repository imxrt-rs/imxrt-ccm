//! Clock Control Module (CCM) driver for i.MX RT systems
//!
//! `imxrt-ccm` supports peripheral clock gating, root clock configuration, and other clock
//! management features for i.MX RT processors. It is a lower-level driver, targeted for
//! HAL implementers. We encourage you to re-export `imxrt-ccm` APIs in your larger libraries.
//!
//! The rest of this documentation is for HAL implementers, or users who want to create higher-level
//! abstractions which require CCM configuration. HAL users who are using re-exported `imxrt-ccm` APIs
//! should consult their HAL documentation for more information on clock configuration.
//!
//! # Usage
//!
//! Implement [`Instance`](trait.Instance.html) on peripheral instances. It's your decision on what
//! qualifies as a "peripeheral instance." It could be a type that could represent the register block,
//! or MMIO. Or, it could represent your peripheral driver.
//!
//! This CCM driver supports clock gating for a variety of peripherals; look for "peripheral instance"
//! in documentation. You should encapsulate any chip-specific details in the `Instance` implementation.
//!
//! Then, create a safe wrapper around [`CCM::new`](struct.CCM.html#method.new) so that your users
//! can safely acquire the CCM peripheral. Your API must ensure that there is only one CCM instance.
//! The types wrapped by your CCM clocks should reflect your `Instance` implementations.
//!
//! Here's an example of implementing a I2C `Instance` for compatibility with the I2C clock. The example
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
//! receiver for constructing a peripheral, you guarantee that a user has configured the peripheral
//! clock.
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
//! i2c_clock.clock_gate(&mut i2c3, ccm::ClockGate::On);
//! // Create the higher-level driver, requires the I2C clock
//! let i2c = I2CDriver::new(i2c3, &i2c_clock);
//! ```
//!
//! # `imxrt-ral` support
//!
//! `imxrt-ccm` provides support for `imxrt-ral`. The support includes `Instance` implementations on
//! all supported `imxrt-ral` peripheral instances. The support also includes helper functions and types,
//! which are exported in the `ral` module. Use the `imxrt-ral` support if your HAL already depends on
//! the `imxrt-ral` crate.
//!
//! Enable the `imxrt-ral` feature. You must ensure that something else in your dependency graph enables the appropriate
//! `imxrt-ral` feature for your processor. See the `imxrt-ral` documentation for more information.
//!
//! If you enable the `imxrt-ral` feature, you must also enable a chip feature. `imxrt-ccm` provides
//! chip features that correlate to NXP datasheets and reference manuals. The list below describes the
//! available features:
//!
//! - `"imxrt1010"` for i.MX RT 1010 processors, like iMXRT1011
//! - `"imxrt1060"` for i.MX RT 1060 processors, like iMXRT1061 and iMXRT1062

#![no_std]

mod i2c;
mod perclock;
mod register;
mod spi;
mod uart;

#[cfg(feature = "imxrt-ral")]
pub mod ral;

pub use i2c::{
    clock_gate as clock_gate_i2c, configure as configure_i2c, frequency as frequency_i2c, I2C,
};
pub use perclock::{
    clock_gate_gpt, clock_gate_pit, configure as configure_perclock, frequency as frequency_perclk,
    GPT, PIT,
};
pub use spi::{
    clock_gate as clock_gate_spi, configure as configure_spi,
    CLOCK_FREQUENCY_HZ as SPI_CLOCK_FREQUENCY_HZ, SPI,
};
pub use uart::{
    clock_gate as clock_gate_uart, configure as configure_uart,
    CLOCK_FREQUENCY_HZ as UART_CLOCK_FREQUENCY_HZ, UART,
};

use core::marker::PhantomData;

/// A peripheral instance whose clock can be gated
///
/// # Safety
///
/// You should only implement `Instance` on a true i.MX RT peripheral instance.
/// `Instance`s are only used when you have both a mutable reference to the instance,
/// and a mutable reference to the CCM [`Handle`](struct.Handle.html). If you incorrectly
/// implement `Instance`, you can violate the safety associated with accessing global,
/// mutable state.
pub unsafe trait Instance {
    /// An identifier that describes the instance
    type Inst: Copy + PartialEq;
    /// Returns the identifier that describes this peripheral instance
    fn instance(&self) -> Self::Inst;
    /// Returns `true` if this instance is valid for a particular
    /// implementation.
    fn is_valid(inst: Self::Inst) -> bool;
}

/// Returns `Some(inst)` if `inst` is valid for this peripheral, or
/// `None` if `inst` is not valid.
#[inline(always)]
fn check_instance<I: Instance>(inst: I::Inst) -> Option<I::Inst> {
    Some(inst).filter(|inst| I::is_valid(*inst))
}

/// Peripheral instance identifier for DMA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DMA;

/// Peripheral instance identifier for ADCs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ADC {
    ADC1,
    ADC2,
}

/// Peripheral instance identifier for PWM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PWM {
    PWM1,
    PWM2,
    PWM3,
    PWM4,
}

/// Handle to the CCM register block
///
/// `Handle` also supports clock gating for peripherals that
/// don't have an obvious clock root, like DMA.
pub struct Handle(());

impl Handle {
    /// Set the clock gate for the DMA controller
    ///
    /// You should set the clock gate before creating DMA channels. Otherwise, the DMA
    /// peripheral may not work.
    #[inline(always)]
    pub fn clock_gate_dma<D>(&mut self, _: &mut D, gate: ClockGate)
    where
        D: Instance<Inst = DMA>,
    {
        unsafe { clock_gate_dma::<D>(gate) };
    }

    /// Set the clock gate for the ADC peripheral
    #[inline(always)]
    pub fn clock_gate_adc<A>(&mut self, adc: &mut A, gate: ClockGate)
    where
        A: Instance<Inst = ADC>,
    {
        unsafe { clock_gate_adc::<A>(adc.instance(), gate) }
    }

    /// Set the clock gate for the PWM peripheral
    #[inline(always)]
    pub fn clock_gate_pwm<P>(&mut self, pwm: &mut P, gate: ClockGate)
    where
        P: Instance<Inst = PWM>,
    {
        unsafe { clock_gate_pwm::<P>(pwm.instance(), gate) }
    }
}

/// Set the clock gate for the DMA controller
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the CCM [`Handle`](struct.Handle.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn clock_gate_dma<D: Instance<Inst = DMA>>(gate: ClockGate) {
    set_clock_gate(CCGR_BASE.add(5), &[3], gate as u8);
}

/// Set the clock gate for the ADC instance
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the CCM [`Handle`](struct.Handle.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn clock_gate_adc<A: Instance<Inst = ADC>>(adc: ADC, gate: ClockGate) {
    match check_instance::<A>(adc) {
        Some(ADC::ADC1) => set_clock_gate(CCGR_BASE.add(1), &[8], gate as u8),
        Some(ADC::ADC2) => set_clock_gate(CCGR_BASE.add(1), &[4], gate as u8),
        _ => (),
    }
}

/// Set the clock gate for the PWM instance
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the CCM [`Handle`](struct.Handle.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn clock_gate_pwm<P: Instance<Inst = PWM>>(pwm: PWM, gate: ClockGate) {
    match check_instance::<P>(pwm) {
        Some(PWM::PWM1) => set_clock_gate(CCGR_BASE.add(4), &[8], gate as u8),
        Some(PWM::PWM2) => set_clock_gate(CCGR_BASE.add(4), &[9], gate as u8),
        Some(PWM::PWM3) => set_clock_gate(CCGR_BASE.add(4), &[9], gate as u8),
        Some(PWM::PWM4) => set_clock_gate(CCGR_BASE.add(4), &[10], gate as u8),
        _ => (),
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
            handle: Handle(()),
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
/// The UART clock is based on the crystal oscillator. See
/// [`UART_CLOCK_FREQUENCY_HZ`](constant.UART_CLOCK_FREQUENCY_HZ.html) for its
/// constant value.
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
/// The SPI clock is based on the crystal oscillator. See
/// [`SPI_CLOCK_FREQUENCY_HZ`](constant.SPI_CLOCK_FREQUENCY_HZ.html) for its
/// constant value.
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

/// Starting address of the clock control gate registers
const CCGR_BASE: *mut u32 = 0x400F_C068 as *mut u32;

/// # Safety
///
/// Should only be used when you have a mutable reference to an enabled clock.
/// Should only be used on a valid clock gate register.
#[inline(always)]
unsafe fn set_clock_gate(ccgr: *mut u32, gates: &[usize], value: u8) {
    const MASK: u32 = 0b11;
    let mut register = core::ptr::read_volatile(ccgr);

    for gate in gates {
        let shift: usize = gate * 2;
        register &= !(MASK << shift);
        register |= (MASK & (value as u32)) << shift;
    }

    core::ptr::write_volatile(ccgr, register);
}

#[cfg(test)]
mod tests {
    use super::set_clock_gate;

    #[test]
    fn test_set_clock_gate() {
        let mut reg = 0;

        unsafe {
            set_clock_gate(&mut reg, &[3, 7], 0b11);
        }
        assert_eq!(reg, (0b11 << 14) | (0b11 << 6));

        unsafe {
            set_clock_gate(&mut reg, &[3], 0b1);
        }
        assert_eq!(reg, (0b11 << 14) | (0b01 << 6));
    }
}
