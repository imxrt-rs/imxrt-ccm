//! UART clock control

use super::{set_clock_gate, ClockGate, Disabled, Handle, Instance, UARTClock, CCGR_BASE};

impl Disabled<UARTClock> {
    /// Enable the UART clocks
    pub fn enable(self, _: &mut Handle) -> UARTClock {
        unsafe { enable() };
        self.0
    }
}

/// Peripheral instance identifier for I2C
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

impl UARTClock {
    /// Set the clock gate for the UART instance
    pub fn clock_gate<U: Instance<Inst = UART>>(&mut self, uart: &mut U, gate: ClockGate) {
        unsafe { clock_gate(uart.instance(), gate) }
    }

    /// Returns the UART clock frequency (Hz)
    pub const fn frequency() -> u32 {
        super::OSCILLATOR_FREQUENCY_HZ
    }
}

/// Set the clock gate for a UART peripheral
///
/// # Safety
///
/// This could be called anywhere, by anyone who uses the globally-accessible UART memory.
/// Consider using the safer `UARTClock::clock_gate` API.
pub unsafe fn clock_gate(uart: UART, gate: ClockGate) {
    let value = gate as u8;
    match uart {
        UART::UART1 => set_clock_gate(CCGR_BASE.add(5), &[12], value),
        UART::UART2 => set_clock_gate(CCGR_BASE.add(0), &[14], value),
        UART::UART3 => set_clock_gate(CCGR_BASE.add(0), &[6], value),
        UART::UART4 => set_clock_gate(CCGR_BASE.add(1), &[12], value),
        UART::UART5 => set_clock_gate(CCGR_BASE.add(3), &[1], value),
        UART::UART6 => set_clock_gate(CCGR_BASE.add(3), &[3], value),
        UART::UART7 => set_clock_gate(CCGR_BASE.add(5), &[13], value),
        UART::UART8 => set_clock_gate(CCGR_BASE.add(6), &[7], value),
    }
}

/// Enable the UART clock root
///
/// # Safety
///
/// This modifies easily-accessible global state. Consider using `UartClock::enable`
/// for a safery API.
#[inline(always)]
pub unsafe fn enable() {
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
