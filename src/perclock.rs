//! Periodic clock implementations

use super::{set_clock_gate, ClockGate, Disabled, Handle, Instance, PerClock, CCGR_BASE};
use crate::register::{Field, Register};

/// Peripheral instance identifier for GPT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPT {
    GPT1,
    GPT2,
}

/// Peripheral instance identifier for PIT
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PIT;

/// Periodic clock frequency (Hz)
///
/// This may be further divided by internal GPT dividers.
const CLOCK_FREQUENCY_HZ: u32 = super::OSCILLATOR_FREQUENCY_HZ;
const DEFAULT_CLOCK_DIVIDER: u32 = 24;

impl<P, G> PerClock<P, G> {
    /// Set the clock gate for the GPT
    #[inline(always)]
    pub fn clock_gate_gpt(&mut self, gpt: &mut G, gate: ClockGate)
    where
        G: Instance<Inst = GPT>,
    {
        unsafe { clock_gate_gpt::<G>(gpt.instance(), gate) };
    }
    /// Set the clock gate for the PIT
    #[inline(always)]
    pub fn clock_gate_pit(&mut self, _: &mut P, gate: ClockGate)
    where
        P: Instance<Inst = PIT>,
    {
        unsafe { clock_gate_pit::<P>(gate) };
    }
    /// Returns the configured periodic clock frequency
    #[inline(always)]
    pub fn frequency(&self) -> u32 {
        frequency()
    }
}

impl<P, G> Disabled<PerClock<P, G>>
where
    P: Instance<Inst = PIT>,
    G: Instance<Inst = GPT>,
{
    /// Enable the periodic clock root, specifying the clock divider
    ///
    /// The divider should be between [1, 64]. The function will treat a 0 as 1,
    /// and anything greater than 64 as 64.
    ///
    /// When `enable` returns, all GPT and PIT clock gates will be set to off. To
    /// re-enable clock gates, use the clock gate methods on [`PerClock`](struct.PerClock.html).
    #[inline(always)]
    pub fn enable_divider(self, _: &mut Handle, divider: u32) -> PerClock<P, G> {
        unsafe {
            clock_gate_gpt::<G>(GPT::GPT1, ClockGate::Off);
            clock_gate_gpt::<G>(GPT::GPT2, ClockGate::Off);
            clock_gate_pit::<P>(ClockGate::Off);
            configure(divider);
        };
        self.0
    }

    /// Enable the periodic clock root with a default divider. The default divider will result
    /// in a periodic clock frequency of **1MHz**.
    ///
    /// When `enable` returns, all GPT and PIT clock gates will be set to off. To
    /// re-enable clock gates, use the clock gate methods on [`PerClock`](struct.PerClock.html).
    #[inline(always)]
    pub fn enable(self, handle: &mut Handle) -> PerClock<P, G> {
        self.enable_divider(handle, DEFAULT_CLOCK_DIVIDER)
    }
}

/// Set the GPT clock gate
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`PerClock`](struct.PerClock.html) for a
/// safer interface.
#[inline(always)]
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
#[inline(always)]
pub unsafe fn clock_gate_pit<P: Instance<Inst = PIT>>(gate: ClockGate) {
    set_clock_gate(CCGR_BASE.add(1), &[6], gate as u8);
}

const PERCLK_PODF: Field = Field::new(0, 0x3F);
const PERCLK_SEL: Field = Field::new(6, 0x01);
const CSCMR1: Register = unsafe { Register::new(PERCLK_PODF, PERCLK_SEL, 0x400F_C01C as *mut u32) };

/// Configure the periodic clock root
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
/// the CCM. Consider using the [`PerClock`](struct.PerClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn configure(divider: u32) {
    configure_(divider, CSCMR1);
}

#[inline(always)]
unsafe fn configure_(divider: u32, reg: Register) {
    const OSCILLATOR: u32 = 1;
    reg.set(divider.min(64).max(1).saturating_sub(1), OSCILLATOR);
}

/// Returns the periodic clock frequency
#[inline(always)]
pub fn frequency() -> u32 {
    frequency_(CSCMR1)
}

#[inline(always)]
fn frequency_(reg: Register) -> u32 {
    let divider = reg.divider() + 1;
    CLOCK_FREQUENCY_HZ / divider
}

#[cfg(test)]
mod tests {

    use super::{configure_, frequency_, Register, CLOCK_FREQUENCY_HZ, PERCLK_PODF, PERCLK_SEL};

    unsafe fn register(mem: &mut u32) -> Register {
        Register::new(PERCLK_PODF, PERCLK_SEL, mem)
    }

    #[test]
    fn perclk_divider_upper_bound() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(65, reg);
            assert_eq!(frequency_(reg), CLOCK_FREQUENCY_HZ / 64);
        }
    }

    #[test]
    fn perclk_divider_lower_bound() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(0, reg);
            assert_eq!(frequency_(reg), CLOCK_FREQUENCY_HZ);
        }
    }

    #[test]
    fn perclk_divider() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(7, reg);
            assert_eq!(frequency_(reg), CLOCK_FREQUENCY_HZ / 7);
        }
    }
}
