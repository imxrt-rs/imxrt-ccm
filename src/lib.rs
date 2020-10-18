//! i.MX RT Clock Control Module (CCM)

#![no_std]

mod i2c;
mod perclock;
mod spi;
mod uart;

#[cfg(all(
    feature = "imxrt-ral",
    any(feature = "imxrt1010", feature = "imxrt1060")
))]
pub mod ral;

pub use i2c::{clock_gate as clock_gate_i2c, configure as configure_i2c, I2C};
pub use perclock::{clock_gate_gpt, clock_gate_pit, configure as configure_perclock, GPT, PIT};
pub use spi::{clock_gate as clock_gate_spi, configure as configure_spi, SPI};
pub use uart::{clock_gate as clock_gate_uart, configure as configure_uart, UART};

use core::marker::PhantomData;

/// A peripheral instance whose clock can be gated
///
/// # Safety
///
/// You should only implement `Instance` on a true i.MX RT peripheral instance.
/// `Instance`s are only used when you have both a mutable reference to the instance,
/// and a mutable reference to the CCM [`Handle`](struct.Handle.html). If you incorrectly
/// implement `Instance`, you can violate the safety associted with accessing global,
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
    pub fn clock_gate_dma<D>(&mut self, _: &mut D, gate: ClockGate)
    where
        D: Instance<Inst = DMA>,
    {
        unsafe { clock_gate_dma::<D>(gate) };
    }

    /// Set the clock gate for the ADC peripheral
    pub fn clock_gate_adc<A>(&mut self, adc: &mut A, gate: ClockGate)
    where
        A: Instance<Inst = ADC>,
    {
        unsafe { clock_gate_adc::<A>(adc.instance(), gate) }
    }

    /// Set the clock gate for the PWM peripheral
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
    /// `perclock` is used for timers, including [`GPT`](../struct.GPT.html) and [`PIT`](../struct.PIT.html).
    pub perclock: Disabled<PerClock<P, G>>,
    /// The UART clock
    ///
    /// `uart_clock` is for [`UART`](../struct.UART.html) peripherals.
    pub uart_clock: Disabled<UARTClock<U>>,
    /// The SPI clock
    ///
    /// `spi_clock` is for [`SPI`](../struct.SPI.html) peripherals.
    pub spi_clock: Disabled<SPIClock<S>>,
    /// The I2C clock
    ///
    /// `i2c_clock` is for [`I2C`](../struct.I2C.html) peripherals.
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
/// `PerClock` is the input clock for GPT and PIT. It runs at
/// 1MHz.
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
#[allow(unused)] // Used when features are enabled
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
