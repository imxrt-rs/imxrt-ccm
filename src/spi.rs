//! SPI clock control

use super::{ClockGate, ClockGateLocation, ClockGateLocator, Disabled, Handle, Instance, SPIClock};
use crate::register::{Field, Register};

const DEFAULT_CLOCK_DIVIDER: u32 = 5;
/// SPI clock frequency (Hz)
const CLOCK_FREQUENCY_HZ: u32 = 528_000_000;

impl<S> Disabled<SPIClock<S>>
where
    S: Instance<Inst = SPI>,
{
    /// Enable the SPI clocks, specifying the clock divider
    ///
    /// The divider should be between [1, 8]. If you supply a divider
    /// outside of that closed range, the implementation will saturate the
    /// divider at the nearest extreme.
    ///
    /// **1010 only:** the divider range is [1, 16].
    ///
    /// When `enable` returns, all SPI clock gates will be set to off.
    /// Use [`clock_gate`](struct.SPIClock.html#method.clock_gate)
    /// to turn on SPI clock gates.
    #[inline(always)]
    pub fn enable_divider(self, _: &mut Handle, divider: u32) -> SPIClock<S> {
        unsafe {
            super::set_clock_gate::<S>(SPI::SPI1, ClockGate::Off);
            super::set_clock_gate::<S>(SPI::SPI2, ClockGate::Off);
            super::set_clock_gate::<S>(SPI::SPI3, ClockGate::Off);
            super::set_clock_gate::<S>(SPI::SPI4, ClockGate::Off);

            configure(divider)
        };
        self.0
    }

    /// Enable the SPI clocks with a default divider
    ///
    /// When `enable` returns, all SPI clock gates will be set to off.
    /// Use [`clock_gate`](struct.SPIClock.html#method.clock_gate)
    /// to turn on SPI clock gates.
    #[inline(always)]
    pub fn enable(self, handle: &mut Handle) -> SPIClock<S> {
        self.enable_divider(handle, DEFAULT_CLOCK_DIVIDER)
    }
}

/// Peripheral instance identifier for SPI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPI {
    SPI1,
    SPI2,
    SPI3,
    SPI4,
}

impl<S> SPIClock<S>
where
    S: Instance<Inst = SPI>,
{
    /// Returns the clock gate setting for the SPI instance
    pub fn clock_gate(&self, spi: &S) -> ClockGate {
        // Unwrap OK: instance must be valid to call this function,
        // or the Instance implementation is invalid.
        super::get_clock_gate::<S>(spi.instance()).unwrap()
    }

    /// Set the clock gate for the SPI instance
    #[inline(always)]
    pub fn set_clock_gate(&mut self, spi: &mut S, gate: ClockGate)
    where
        S: Instance<Inst = SPI>,
    {
        unsafe { super::set_clock_gate::<S>(spi.instance(), gate) }
    }

    /// Returns the SPI clock frequency
    #[inline(always)]
    pub fn frequency(&self) -> u32 {
        frequency()
    }
}

impl ClockGateLocator for SPI {
    #[inline(always)]
    fn location(&self) -> ClockGateLocation {
        let gates = match self {
            SPI::SPI1 => &[0],
            SPI::SPI2 => &[1],
            SPI::SPI3 => &[2],
            SPI::SPI4 => &[3],
        };
        ClockGateLocation { offset: 1, gates }
    }
}

const LPSPI_PODF: Field = Field::new(
    26,
    #[cfg(not(feature = "imxrt1010"))]
    0x7,
    #[cfg(feature = "imxrt1010")]
    0xF,
);
const LPSPI_SEL: Field = Field::new(4, 3);
const CBCMR: Register = unsafe { Register::new(LPSPI_PODF, LPSPI_SEL, 0x400F_C018 as *mut u32) };

/// Configure the SPI clock root
///
/// Configure will **not** disable peripheral clock gates. You should disable
/// clock gates yourself before calling this function.
///
/// The divider should be between [1, 8]. If you supply a divider
/// outside of that closed range, the implementation will saturate the
/// divider at the nearest extreme.
///
/// **1010 only:** the divider range is [1, 16].
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`SPIClock`](struct.SPIClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn configure(divider: u32) {
    configure_(divider, &CBCMR);
}

#[inline(always)]
unsafe fn configure_(divider: u32, reg: &Register) {
    const PLL2: u32 = 2; // Consistent for 1062, 1011 chips
    #[cfg(not(feature = "imxrt1010"))]
    const MAX_DIVIDER: u32 = 8;
    #[cfg(feature = "imxrt1010")]
    const MAX_DIVIDER: u32 = 16;

    reg.set(divider.min(MAX_DIVIDER).max(1).saturating_sub(1), PLL2);
}

/// Returns the SPI clock frequency
#[inline(always)]
pub fn frequency() -> u32 {
    frequency_(&CBCMR)
}

#[inline(always)]
fn frequency_(reg: &Register) -> u32 {
    let divider = reg.divider() + 1;
    CLOCK_FREQUENCY_HZ / divider
}

#[cfg(test)]
mod tests {

    use super::{configure_, frequency_, Register, CLOCK_FREQUENCY_HZ, LPSPI_PODF, LPSPI_SEL};

    unsafe fn register(mem: &mut u32) -> Register {
        Register::new(LPSPI_PODF, LPSPI_SEL, mem)
    }

    #[cfg(not(feature = "imxrt1010"))]
    #[test]
    fn spi_divider_upper_bound() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(9, &reg);
            assert_eq!(frequency_(&reg), CLOCK_FREQUENCY_HZ / 8);
        }
    }

    #[cfg(feature = "imxrt1010")]
    #[test]
    fn spi_divider_upper_bound() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(17, &reg);
            assert_eq!(frequency_(&reg), CLOCK_FREQUENCY_HZ / 16);
        }
    }

    #[test]
    fn spi_divider_lower_bound() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(0, &reg);
            assert_eq!(frequency_(&reg), CLOCK_FREQUENCY_HZ);
        }
    }

    #[test]
    fn spi_divider() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(7, &reg);
            assert_eq!(frequency_(&reg), CLOCK_FREQUENCY_HZ / 7);
        }
    }
}
