//! UART clock control

use super::{
    set_clock_gate, ClockGate, ClockGateLocation, ClockGateLocator, Disabled, Handle, Instance,
    UARTClock,
};
use crate::register::{Field, Register};

/// UART clock frequency (Hz)
const CLOCK_FREQUENCY_HZ: u32 = super::OSCILLATOR_FREQUENCY_HZ;
const DEFAULT_CLOCK_DIVIDER: u32 = 1;

impl<U> Disabled<UARTClock<U>>
where
    U: Instance<Inst = UART>,
{
    /// Enable the UART clocks with default divider
    ///
    /// When `enable` returns, all UART clock gates will be set to off.
    /// Use [`clock_gate`](struct.UARTClock.html#method.clock_gate)
    /// to turn on UART clock gates.
    #[inline(always)]
    pub fn enable(self, handle: &mut Handle) -> UARTClock<U> {
        self.enable_divider(handle, DEFAULT_CLOCK_DIVIDER)
    }

    /// Enable the UART clocks with a clock divider.
    ///
    /// The divider should be between [1, 64]. The function will treat a 0 as 1,
    /// and anything greater than 64 as 64.
    ///
    /// When `enable` returns, all UART clock gates will be set to off.
    /// Use [`clock_gate`](struct.UARTClock.html#method.clock_gate)
    /// to turn on UART clock gates.
    #[inline(always)]
    pub fn enable_divider(self, _: &mut Handle, divider: u32) -> UARTClock<U> {
        unsafe {
            set_clock_gate::<U>(UART::UART1, ClockGate::Off);
            set_clock_gate::<U>(UART::UART2, ClockGate::Off);
            set_clock_gate::<U>(UART::UART3, ClockGate::Off);
            set_clock_gate::<U>(UART::UART4, ClockGate::Off);
            set_clock_gate::<U>(UART::UART5, ClockGate::Off);
            set_clock_gate::<U>(UART::UART6, ClockGate::Off);
            set_clock_gate::<U>(UART::UART7, ClockGate::Off);
            set_clock_gate::<U>(UART::UART8, ClockGate::Off);

            configure(divider)
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

impl<U> UARTClock<U>
where
    U: Instance<Inst = UART>,
{
    /// Returns the clock gate setting for the UART instance
    #[inline(always)]
    pub fn clock_gate(&self, uart: &U) -> ClockGate {
        // Unwrap OK: instance must be valid to call this function,
        // or the Instance implementation is invalid.
        super::get_clock_gate::<U>(uart.instance()).unwrap()
    }

    /// Set the clock gate for the UART instance
    #[inline(always)]
    pub fn set_clock_gate(&mut self, uart: &mut U, gate: ClockGate) {
        unsafe { set_clock_gate::<U>(uart.instance(), gate) }
    }

    /// Returns the UART clock frequency
    #[inline(always)]
    pub fn frequency(&self) -> u32 {
        frequency()
    }
}

impl ClockGateLocator for UART {
    #[inline(always)]
    fn location(&self) -> ClockGateLocation {
        match self {
            UART::UART1 => ClockGateLocation {
                offset: 5,
                gates: &[12],
            },
            UART::UART2 => ClockGateLocation {
                offset: 0,
                gates: &[14],
            },
            UART::UART3 => ClockGateLocation {
                offset: 0,
                gates: &[6],
            },
            UART::UART4 => ClockGateLocation {
                offset: 1,
                gates: &[12],
            },
            UART::UART5 => ClockGateLocation {
                offset: 3,
                gates: &[1],
            },
            UART::UART6 => ClockGateLocation {
                offset: 3,
                gates: &[3],
            },
            UART::UART7 => ClockGateLocation {
                offset: 5,
                gates: &[13],
            },
            UART::UART8 => ClockGateLocation {
                offset: 6,
                gates: &[7],
            },
        }
    }
}

const UART_CLK_PODF: Field = Field::new(0, 0x3F);
// Note that the mask is 1 for 1011, but the adjacent bit is reserved
const UART_CLK_SEL: Field = Field::new(6, 0x3);
const CSCDR1: Register =
    unsafe { Register::new(UART_CLK_PODF, UART_CLK_SEL, 0x400F_C024 as *mut u32) };

/// Configure the UART clock root
///
/// Configure will **not** disable peripheral clock gates. You should disable
/// clock gates yourself before calling this function.
///
/// The divider should be between [1, 64]. The function will treat a 0 as 1,
/// and anything greater than 64 as 64.
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`UARTClock`](struct.UARTClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn configure(divider: u32) {
    configure_(divider, CSCDR1);
}

#[inline(always)]
unsafe fn configure_(divider: u32, reg: Register) {
    const OSCILLATOR: u32 = 1; // Same value for 1060, 1010
    reg.set(divider.min(64).max(1).saturating_sub(1), OSCILLATOR);
}

/// Returns the UART clock frequency
#[inline(always)]
pub fn frequency() -> u32 {
    frequency_(CSCDR1)
}

#[inline(always)]
fn frequency_(reg: Register) -> u32 {
    let divider = reg.divider() + 1;
    CLOCK_FREQUENCY_HZ / divider
}

#[cfg(test)]
mod tests {

    use super::{
        configure_, frequency_, Register, CLOCK_FREQUENCY_HZ, UART_CLK_PODF, UART_CLK_SEL,
    };

    unsafe fn register(mem: &mut u32) -> Register {
        Register::new(UART_CLK_PODF, UART_CLK_SEL, mem)
    }

    #[test]
    fn uart_divider_upper_bound() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(65, reg);
            assert_eq!(frequency_(reg), CLOCK_FREQUENCY_HZ / 64);
        }
    }

    #[test]
    fn uart_divider_lower_bound() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(0, reg);
            assert_eq!(frequency_(reg), CLOCK_FREQUENCY_HZ);
        }
    }

    #[test]
    fn uart_divider() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(7, reg);
            assert_eq!(frequency_(reg), CLOCK_FREQUENCY_HZ / 7);
        }
    }
}
