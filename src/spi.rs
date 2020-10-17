//! SPI clock control

use super::{set_clock_gate, ClockGate, Disabled, Handle, Instance, SPIClock, CCGR_BASE};

const CLOCK_DIVIDER: u32 = 5;
/// If changing this, make sure to update `clock`
const CLOCK_HZ: u32 = 528_000_000 / CLOCK_DIVIDER;

impl Disabled<SPIClock> {
    /// Enable the SPI clocks
    pub fn enable(self, _: &mut Handle) -> SPIClock {
        unsafe { enable() };
        self.0
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

impl SPIClock {
    /// Set the clock gate for the SPI instance
    pub fn clock_gate<I: Instance<Inst = SPI>>(&mut self, spi: &mut I, gate: ClockGate) {
        unsafe { clock_gate(spi.instance(), gate) }
    }

    /// Returns the SPI clock frequency (Hz)
    pub const fn frequency() -> u32 {
        CLOCK_HZ
    }
}

/// Set the clock gate for a SPI peripheral
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`SPIClock`](struct.SPIClock.html) for a
/// safer interface.
pub unsafe fn clock_gate(spi: SPI, value: ClockGate) {
    let ccgr = CCGR_BASE.add(1);
    let gate = match spi {
        SPI::SPI1 => 0,
        SPI::SPI2 => 1,
        SPI::SPI3 => 2,
        SPI::SPI4 => 3,
    };

    set_clock_gate(ccgr, &[gate], value as u8);
}

/// Enable the SPI clock root
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`SPIClock`](struct.SPIClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn enable() {
    const CBCMR: *mut u32 = 0x400F_C018 as *mut u32;
    const LPSPI_PODF_OFFSET: u32 = 26;
    const LPSPI_PODF_MASK: u32 = 0xF << LPSPI_PODF_OFFSET;
    const LPSPI_SEL_OFFSET: u32 = 4;
    const LPSPI_SEL_MASK: u32 = 0x3 << LPSPI_SEL_OFFSET;
    const PLL2: u32 = 2; // Consistent for 1062, 1011 chips

    let mut cbcmr = CBCMR.read_volatile();
    cbcmr &= !(LPSPI_PODF_MASK | LPSPI_SEL_MASK);
    cbcmr |= CLOCK_DIVIDER.saturating_sub(1) << LPSPI_PODF_OFFSET;
    cbcmr |= PLL2 << LPSPI_SEL_OFFSET;
    CBCMR.write_volatile(cbcmr);
}
