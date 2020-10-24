//! SPI clock control

use super::{set_clock_gate, ClockGate, Disabled, Handle, Instance, SPIClock, CCGR_BASE};

const CLOCK_DIVIDER: u32 = 5;
/// SPI clock frequency (Hz)
pub const CLOCK_FREQUENCY_HZ: u32 = 528_000_000 / CLOCK_DIVIDER;

impl<S> Disabled<SPIClock<S>> {
    /// Enable the SPI clocks
    ///
    /// When `enable` returns, all SPI clock gates will be set to off.
    /// Use [`clock_gate`](struct.SPIClock.html#method.clock_gate)
    /// to turn on SPI clock gates.
    #[inline(always)]
    pub fn enable(self, _: &mut Handle) -> SPIClock<S>
    where
        S: Instance<Inst = SPI>,
    {
        unsafe {
            clock_gate::<S>(SPI::SPI1, ClockGate::Off);
            clock_gate::<S>(SPI::SPI2, ClockGate::Off);
            clock_gate::<S>(SPI::SPI3, ClockGate::Off);
            clock_gate::<S>(SPI::SPI4, ClockGate::Off);

            configure()
        };
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

impl<S> SPIClock<S> {
    /// Set the clock gate for the SPI instance
    #[inline(always)]
    pub fn clock_gate(&mut self, spi: &mut S, gate: ClockGate)
    where
        S: Instance<Inst = SPI>,
    {
        unsafe { clock_gate::<S>(spi.instance(), gate) }
    }
}

/// Set the clock gate for a SPI peripheral
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`SPIClock`](struct.SPIClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn clock_gate<S: Instance<Inst = SPI>>(spi: SPI, value: ClockGate) {
    let gate = match super::check_instance::<S>(spi) {
        Some(SPI::SPI1) => 0,
        Some(SPI::SPI2) => 1,
        Some(SPI::SPI3) => 2,
        Some(SPI::SPI4) => 3,
        _ => return,
    };

    set_clock_gate(CCGR_BASE.add(1), &[gate], value as u8);
}

/// Configure the SPI clock root
///
/// Configure will **not** disable peripheral clock gates. You should disable
/// clock gates yourself before calling this function.
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`SPIClock`](struct.SPIClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn configure() {
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
