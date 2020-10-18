//! UART clock control

use super::{set_clock_gate, ClockGate, Disabled, Handle, Instance, UARTClock, CCGR_BASE};

/// UART clock frequency (Hz)
pub const CLOCK_FREQUENCY_HZ: u32 = super::OSCILLATOR_FREQUENCY_HZ;

impl<U> Disabled<UARTClock<U>> {
    /// Enable the UART clocks
    ///
    /// When `enable` returns, all UART clock gates will be set to off.
    /// Use [`clock_gate`](struct.UARTClock.html#method.clock_gate)
    /// to turn on UART clock gates.
    #[inline(always)]
    pub fn enable(self, _: &mut Handle) -> UARTClock<U>
    where
        U: Instance<Inst = UART>,
    {
        unsafe {
            clock_gate::<U>(UART::UART1, ClockGate::Off);
            clock_gate::<U>(UART::UART2, ClockGate::Off);
            clock_gate::<U>(UART::UART3, ClockGate::Off);
            clock_gate::<U>(UART::UART4, ClockGate::Off);
            clock_gate::<U>(UART::UART5, ClockGate::Off);
            clock_gate::<U>(UART::UART6, ClockGate::Off);
            clock_gate::<U>(UART::UART7, ClockGate::Off);
            clock_gate::<U>(UART::UART8, ClockGate::Off);

            configure()
        };
        self.0
    }
}

/// Peripheral instance identifier for UART
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UART {
    UART1,
    UART2,
    UART3,
    UART4,
    UART5,
    UART6,
    UART7,
    UART8,
}

impl<U> UARTClock<U> {
    /// Set the clock gate for the UART instance
    #[inline(always)]
    pub fn clock_gate(&mut self, uart: &mut U, gate: ClockGate)
    where
        U: Instance<Inst = UART>,
    {
        unsafe { clock_gate::<U>(uart.instance(), gate) }
    }
}

/// Set the clock gate for a UART peripheral
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`UARTClock`](struct.UARTClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn clock_gate<U: Instance<Inst = UART>>(uart: UART, gate: ClockGate) {
    let value = gate as u8;
    match super::check_instance::<U>(uart) {
        Some(UART::UART1) => set_clock_gate(CCGR_BASE.add(5), &[12], value),
        Some(UART::UART2) => set_clock_gate(CCGR_BASE.add(0), &[14], value),
        Some(UART::UART3) => set_clock_gate(CCGR_BASE.add(0), &[6], value),
        Some(UART::UART4) => set_clock_gate(CCGR_BASE.add(1), &[12], value),
        Some(UART::UART5) => set_clock_gate(CCGR_BASE.add(3), &[1], value),
        Some(UART::UART6) => set_clock_gate(CCGR_BASE.add(3), &[3], value),
        Some(UART::UART7) => set_clock_gate(CCGR_BASE.add(5), &[13], value),
        Some(UART::UART8) => set_clock_gate(CCGR_BASE.add(6), &[7], value),
        _ => (),
    }
}

/// Configure the UART clock root
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`UARTClock`](struct.UARTClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn configure() {
    const CSCDR1: *mut u32 = 0x400F_C024 as *mut u32;
    const UART_CLK_PODF_OFFSET: u32 = 0;
    const UART_CLK_PODF_MASK: u32 = 0x3F << UART_CLK_PODF_OFFSET;
    const UART_CLK_SEL_OFFSET: u32 = 6;
    const UART_CLK_SEL_MASK: u32 = 0x3 << UART_CLK_SEL_OFFSET; // Note that the mask is 1 for 1011, but the adjacent bit is reserved
    const OSCILLATOR: u32 = 1; // Same value for 1062, 1011
    const DIVIDE_1: u32 = 0;

    let mut cscdr1 = CSCDR1.read_volatile();
    cscdr1 &= !(UART_CLK_PODF_MASK | UART_CLK_SEL_MASK);
    cscdr1 |= DIVIDE_1 << UART_CLK_PODF_OFFSET;
    cscdr1 |= OSCILLATOR << UART_CLK_SEL_OFFSET;
    CSCDR1.write_volatile(cscdr1);
}
