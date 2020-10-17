//! Periodic clock implementations

use super::{set_clock_gate, ClockGate, Disabled, Handle, Instance, PerClock, CCGR_BASE};

/// Peripheral instance identifier for GPT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPT {
    GPT1,
    GPT2,
}

/// Peripheral instance identifier for PIT
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PIT;

const PERIODIC_CLOCK_FREQUENCY_HZ: u32 = super::OSCILLATOR_FREQUENCY_HZ / PERIODIC_CLOCK_DIVIDER;
const PERIODIC_CLOCK_DIVIDER: u32 = 24;

impl PerClock {
    /// Set the clock gate for the GPT
    pub fn clock_gate_gpt<G: Instance<Inst = GPT>>(&mut self, gpt: &mut G, gate: ClockGate) {
        unsafe { clock_gate_gpt(gpt.instance(), gate) };
    }
    /// Set the clock gate for the PIT
    pub fn clock_gate_pit<P: Instance<Inst = PIT>>(&mut self, _: &mut P, gate: ClockGate) {
        unsafe { clock_gate_pit(gate) };
    }
    /// Returns the periodic clock frequency (Hz)
    pub const fn frequency() -> u32 {
        PERIODIC_CLOCK_FREQUENCY_HZ
    }
}

impl Disabled<PerClock> {
    /// Enable the periodic clock root
    pub fn enable(self, _: &mut Handle) -> PerClock {
        unsafe {
            configure();
        };
        self.0
    }
}

/// Set the GPT clock gate
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`PerClock`](struct.PerClock.html) for a
/// safer interface.
pub unsafe fn clock_gate_gpt(gpt: GPT, gate: ClockGate) {
    let value = gate as u8;
    match gpt {
        GPT::GPT1 => set_clock_gate(CCGR_BASE.add(1), &[10, 11], value),
        GPT::GPT2 => set_clock_gate(CCGR_BASE.add(0), &[12, 13], value),
    }
}

/// Set the PIT clock gate
///
/// # Safety
///
/// This could be used by anyone who supplies a PIT register block, which is globally
/// available. Consider using [`PerClock::clock_gate_pit`](struct.PerClock.html#method.clock_gate_pit)
/// for a safer interface.
pub unsafe fn clock_gate_pit(gate: ClockGate) {
    set_clock_gate(CCGR_BASE.add(1), &[6], gate as u8);
}

/// Configure the periodic clock root
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`PerClock`](struct.PerClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn configure() {
    const CSCMR1: *mut u32 = 0x400F_C01C as *mut u32;
    const PERCLK_PODF_OFFSET: u32 = 0;
    const PERCLK_PODF_MASK: u32 = 0x1F << PERCLK_PODF_OFFSET;
    const PERCLK_SEL_OFFSET: u32 = 6;
    const PERCLK_SEL_MASK: u32 = 0x01 << PERCLK_SEL_OFFSET;
    const OSCILLATOR: u32 = 1;

    let mut cscmr1 = CSCMR1.read_volatile();
    cscmr1 &= !(PERCLK_PODF_MASK | PERCLK_SEL_MASK);
    cscmr1 |= PERIODIC_CLOCK_DIVIDER.saturating_sub(1) << PERCLK_PODF_OFFSET;
    cscmr1 |= OSCILLATOR << PERCLK_SEL_OFFSET;
    CSCMR1.write_volatile(cscmr1);
}
