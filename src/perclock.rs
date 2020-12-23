//! Periodic clock implementations

use super::{
    arm, ClockGate, ClockGateLocation, ClockGateLocator, Disabled, Handle, Instance, PerClock,
};
use crate::{
    register::{Field, Register},
    OSCILLATOR_FREQUENCY_HZ,
};

/// Peripheral instance identifier for GPT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPT {
    GPT1,
    GPT2,
}

/// Periodic clock selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selection {
    /// Use the IPG clock root
    ///
    /// This assumes that you've configured the IPG clock elsewhere.
    IPG,
    /// Use the crystal oscillator
    Oscillator,
}

impl ClockGateLocator for GPT {
    #[inline(always)]
    fn location(&self) -> ClockGateLocation {
        match self {
            GPT::GPT1 => ClockGateLocation {
                offset: 1,
                gates: &[10, 11],
            },
            GPT::GPT2 => ClockGateLocation {
                offset: 0,
                gates: &[12, 13],
            },
        }
    }
}

/// Peripheral instance identifier for PIT
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PIT;

impl ClockGateLocator for PIT {
    #[inline(always)]
    fn location(&self) -> ClockGateLocation {
        ClockGateLocation {
            offset: 1,
            gates: &[6],
        }
    }
}

const DEFAULT_CLOCK_DIVIDER: u32 = 24;

impl<P, G> PerClock<P, G> {
    /// Returns the configured periodic clock frequency
    ///
    /// The method requires a reference to the CCM `Handle`, since it may need to read
    /// the IPG clock frequency.
    #[inline(always)]
    pub fn frequency(&self, _: &Handle) -> u32 {
        // Safety: we satisfy the safety requirements for both the ARM frequency
        // call, and also the periodic clock frequency call.
        unsafe { frequency() }
    }
    /// Try to read the periodic clock frequency, returning the frequency if it can
    /// be safely read
    ///
    /// If the periodic clocks run on the IPG frequency, it is not safe to read the
    /// frequencies. `try_frequency` would return `None`. But, if the periodic clocks
    /// run on the oscillator, we can safely compute the frequency.
    #[inline(always)]
    pub fn try_frequency(&self) -> Option<u32> {
        if self.selection() == Selection::Oscillator {
            Some(unsafe { frequency() })
        } else {
            None
        }
    }
    /// Returns the periodic clock selection
    #[inline(always)]
    pub fn selection(&self) -> Selection {
        selection()
    }
}

impl<P, G> PerClock<P, G>
where
    G: Instance<Inst = GPT>,
{
    /// Returns the clock gate setting for the GPT
    #[inline(always)]
    pub fn clock_gate_gpt(&self, gpt: &G) -> ClockGate {
        // Unwrap OK: instance must be valid to call this function,
        // or the Instance implementation is invalid.
        super::get_clock_gate::<G>(gpt.instance()).unwrap()
    }

    /// Set the clock gate for the GPT
    #[inline(always)]
    pub fn set_clock_gate_gpt(&mut self, gpt: &mut G, gate: ClockGate) {
        unsafe { super::set_clock_gate::<G>(gpt.instance(), gate) };
    }
}

impl<P, G> PerClock<P, G>
where
    P: Instance<Inst = PIT>,
{
    /// Returns the clock gate setting for the PIT
    #[inline(always)]
    pub fn clock_gate_pit(&self, pit: &P) -> ClockGate {
        // Unwrap OK: instance must be valid to call this function,
        // or the Instance implementation is invalid.
        super::get_clock_gate::<P>(pit.instance()).unwrap()
    }

    /// Set the clock gate for the PIT
    #[inline(always)]
    pub fn set_clock_gate_pit(&mut self, pit: &mut P, gate: ClockGate) {
        unsafe { super::set_clock_gate::<P>(pit.instance(), gate) };
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
    pub fn enable_selection_divider(
        self,
        _: &mut Handle,
        selection: Selection,
        divider: u32,
    ) -> PerClock<P, G> {
        unsafe {
            super::set_clock_gate::<G>(GPT::GPT1, ClockGate::Off);
            super::set_clock_gate::<G>(GPT::GPT2, ClockGate::Off);
            super::set_clock_gate::<P>(PIT, ClockGate::Off);
            configure(selection, divider);
        };
        self.0
    }

    /// Enable the periodic clock root with a default divider. The default divider will result
    /// in a periodic clock frequency of **1MHz** from the crystal oscillator.
    ///
    /// When `enable` returns, all GPT and PIT clock gates will be set to off. To
    /// re-enable clock gates, use the clock gate methods on [`PerClock`](struct.PerClock.html).
    #[inline(always)]
    pub fn enable(self, handle: &mut Handle) -> PerClock<P, G> {
        self.enable_selection_divider(handle, Selection::Oscillator, DEFAULT_CLOCK_DIVIDER)
    }
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
pub unsafe fn configure(selection: Selection, divider: u32) {
    configure_(selection, divider, &CSCMR1);
}

#[inline(always)]
unsafe fn configure_(selection: Selection, divider: u32, reg: &Register) {
    let selection: u32 = match selection {
        Selection::Oscillator => 1,
        Selection::IPG => 0,
    };
    reg.set(divider.min(64).max(1).saturating_sub(1), selection);
}

/// Returns the periodic clock frequency
///
/// # Safety
///
/// Reads multiple CCM registers without synchronization.
#[inline(always)]
pub unsafe fn frequency() -> u32 {
    frequency_(&arm::ARM_CONTEXT, &CSCMR1)
}

unsafe fn frequency_(ctx: &arm::Context, reg: &Register) -> u32 {
    let divider = reg.divider() + 1;
    match selection_(reg) {
        Selection::IPG => ctx.timings().ipg_hz() / divider,
        Selection::Oscillator => OSCILLATOR_FREQUENCY_HZ / divider,
    }
}

/// Returns the periodic clock selection
#[inline(always)]
pub fn selection() -> Selection {
    selection_(&CSCMR1)
}

#[inline(always)]
fn selection_(reg: &Register) -> Selection {
    match reg.selection() {
        1 => Selection::Oscillator,
        0 => Selection::IPG,
        sel => unreachable!("Periodic clock selection unknown value {}", sel),
    }
}

#[cfg(test)]
mod tests {

    use super::{
        arm::tests::TestContext, configure_, frequency_, Register, Selection,
        OSCILLATOR_FREQUENCY_HZ, PERCLK_PODF, PERCLK_SEL,
    };

    unsafe fn register(mem: &mut u32) -> Register {
        Register::new(PERCLK_PODF, PERCLK_SEL, mem)
    }

    #[test]
    fn perclk_divider_upper_bound() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(Selection::Oscillator, 65, &reg);
            assert_eq!(
                frequency_(&TestContext::new().context(), &reg),
                OSCILLATOR_FREQUENCY_HZ / 64
            );
        }
    }

    #[test]
    fn perclk_divider_lower_bound() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(Selection::Oscillator, 0, &reg);
            assert_eq!(
                frequency_(&TestContext::new().context(), &reg),
                OSCILLATOR_FREQUENCY_HZ
            );
        }
    }

    #[test]
    fn perclk_divider() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(Selection::Oscillator, 7, &reg);
            assert_eq!(
                frequency_(&TestContext::new().context(), &reg),
                OSCILLATOR_FREQUENCY_HZ / 7
            );
        }
    }

    #[test]
    fn perclk_ipg() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(Selection::IPG, 2, &reg);
            let mut ctx = TestContext::from_timings(&crate::arm::Timings::target(600_000_000));
            assert_eq!(frequency_(&ctx.context(), &reg), 150_000_000 / 2);
        }
    }
}
