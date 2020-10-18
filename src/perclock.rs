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

impl<P, G> PerClock<P, G> {
    /// Set the clock gate for the GPT
    pub fn clock_gate_gpt(&mut self, gpt: &mut G, gate: ClockGate)
    where
        G: Instance<Inst = GPT>,
    {
        unsafe { clock_gate_gpt::<G>(gpt.instance(), gate) };
    }
    /// Set the clock gate for the PIT
    pub fn clock_gate_pit(&mut self, _: &mut P, gate: ClockGate)
    where
        P: Instance<Inst = PIT>,
    {
        unsafe { clock_gate_pit::<P>(gate) };
    }
    /// Returns the periodic clock frequency (Hz)
    pub const fn frequency() -> u32 {
        PERIODIC_CLOCK_FREQUENCY_HZ
    }
}

impl<P, G> Disabled<PerClock<P, G>> {
    /// Enable the periodic clock root
    ///
    /// When `enable` returns, all GPT and PIT clock gates will be set to off. To
    /// re-enable clock gates, use the clock gate methods on [`PerClock`](struct.PerClock.html).
    pub fn enable(self, _: &mut Handle) -> PerClock<P, G>
    where
        P: Instance<Inst = PIT>,
        G: Instance<Inst = GPT>,
    {
        unsafe {
            clock_gate_gpt::<G>(GPT::GPT1, ClockGate::Off);
            clock_gate_gpt::<G>(GPT::GPT2, ClockGate::Off);
            clock_gate_pit::<P>(ClockGate::Off);
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
pub unsafe fn clock_gate_gpt<G: Instance<Inst = GPT>>(gpt: GPT, gate: ClockGate) {
    let value = gate as u8;
    match super::check_instance::<G>(gpt) {
        Some(GPT::GPT1) => set_clock_gate(CCGR_BASE.add(1), &[10, 11], value),
        Some(GPT::GPT2) => set_clock_gate(CCGR_BASE.add(0), &[12, 13], value),
        _ => (),
    }
}

/// Set the PIT clock gate
///
/// # Safety
///
/// This could be used by anyone who supplies a PIT register block, which is globally
/// available. Consider using [`PerClock::clock_gate_pit`](struct.PerClock.html#method.clock_gate_pit)
/// for a safer interface.
pub unsafe fn clock_gate_pit<P: Instance<Inst = PIT>>(gate: ClockGate) {
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
