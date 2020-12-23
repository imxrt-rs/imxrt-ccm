//! ARM clock control
//!
//! The module provides routines to control the ARM clock frequency.
//! Since the IPG clock runs on the AHB_CLK_ROOT signal, this module
//! also controls IPG clock speeds.
//!
//! # Approach
//!
//! 1. Switch the AHB_CLK_ROOT to use the 24MHz clock provided by
//!    peripheral clock 2. Use the glitchless muxes.
//! 2. Compute the new ARM & IPG clock divider values, and the PLL1
//!    loop divider value. Commit those values to registers.
//! 3. Switch (back) to PLL1 as the AHB_CLK_ROOT.
//!
//! # References
//!
//! i.MX RT 1060 reference manual
//! - Chapter 14: Clock Control Module (CCM)
//!   - System Clocks
//!   - CCM Internal Clock Generation

use crate::register::Field;

/// The ARM clock frequency
///
/// See [`Handle::set_frequency_arm`](crate::Handle::set_frequency_arm`)
/// and [`Handle::frequency_arm`](crate::Handle::frequency_arm`) for safe
/// mutators and accessors.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ARMClock(pub u32);
/// The IPG clock frequency
///
/// The IPG clock frequency runs on the AHB_CLOCK_ROOT. It's a divided
/// ARM clock.
///
/// See [`Handle::set_frequency_arm`](crate::Handle::set_frequency_arm`)
/// and [`Handle::frequency_arm`](crate::Handle::frequency_arm`) for safe
/// mutators and accessors.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IPGClock(pub u32);

const CCM_CACCR: *mut u32 = 0x400F_C010 as _;
const CCM_CBCDR: *mut u32 = 0x400F_C014 as _;

/// Wait for all divider and mux handshakes to complete
#[inline(always)]
unsafe fn wait_for_handshake() {
    const CCM_CDHIPR: *mut u32 = 0x400F_C048 as _;
    while CCM_CDHIPR.read_volatile() != 0 {}
}

/// Runs the function when the AHB_CLK_ROOT is powered by the
/// 24MHz crystal oscillator. When the function returns, AH_BCLK_ROOT
/// is powered by the PRE_PERIPH_CLK source.
///
/// # Safety
///
/// Modifies CCM register memory.
unsafe fn on_ahb_clk_oscillator<R>(func: impl FnOnce() -> R) -> R {
    const CCM_CBCMR: *mut u32 = 0x400F_C018 as _;

    const PERIPH_CLK2_PODF: Field = Field::new(27, 0b111);
    const PERIPH_CLK2_SEL: Field = Field::new(12, 0b11);

    PERIPH_CLK2_PODF.modify(CCM_CBCDR, 0); // Divide by 1
    PERIPH_CLK2_SEL.modify(CCM_CBCMR, 1); // Derive from oscillator
    wait_for_handshake();

    // Switch main peripheral clock to PERIPH_CLK2
    const PERIPH_CLK_SEL: Field = Field::new(25, 1);
    PERIPH_CLK_SEL.modify(CCM_CBCDR, 1);
    wait_for_handshake();

    let result = func();

    // Switch back to PRE_PERIPH_CLK
    const PRE_PERIPH_CLK_SEL: Field = Field::new(18, 0x3);
    PRE_PERIPH_CLK_SEL.modify(CCM_CBCMR, 3); // Select PLL1

    PERIPH_CLK_SEL.modify(CCM_CBCDR, 0);
    wait_for_handshake();

    result
}

/// ARM clock timings
#[derive(PartialEq, Eq, Debug)]
struct Timings {
    /// PLL_ARM DIV_SEL
    ///
    /// Valid range for divider value: 54-108. `Fout = Fin * div_select/2.0`
    /// Fin is the 24MHz crystal oscillator
    pll_arm_div_sel: u32,
    /// Divider value for CACRR[ARM_PODF], in between the PLL
    /// and the pre peripheral mux
    ///
    /// Note that this value is off-by-one to support runtime math.
    /// Subtract 1 before writing to the field.
    div_arm: u32,
    /// Divider for CBCDR[AHB_PODF], right before the ARM
    /// core clock input
    ///
    /// Note that this value is off-by-one to support runtime math.
    /// Subtract 1 before writing to the field.
    div_ahb: u32,
    /// ARM clock frequency that we're using
    arm_hz: u32,
    /// IPG divider (off-by-one for runtime math; subtract 1 before writing)
    div_ipg: u32,
}

#[inline(always)]
fn compute_arm_hz(div_arm: u32, div_ahb: u32, pll_arm_div_sel: u32) -> u32 {
    pll_arm_div_sel * 12_000_000 / div_arm / div_ahb
}

impl Timings {
    /// Returns a `Timings` that approximates the target ARM clock `arm_hz`
    fn target(arm_hz: u32) -> Self {
        let (mut div_arm, mut div_ahb) = (1, 1);
        while arm_hz * div_arm * div_ahb < 648_000_000 {
            if div_arm < 8 {
                div_arm += 1;
            } else if div_ahb < 5 {
                div_ahb += 1;
                div_arm = 1;
            } else {
                break;
            }
        }

        let pll_arm_div_sel = (arm_hz * div_arm * div_ahb + 6_000_000) / 12_000_000;
        let pll_arm_div_sel = pll_arm_div_sel.min(108).max(54);
        let arm_hz = compute_arm_hz(div_arm, div_ahb, pll_arm_div_sel);

        let div_ipg = (arm_hz + 149_999_999) / 150_000_000;
        let div_ipg = div_ipg.min(4);

        Timings {
            pll_arm_div_sel,
            div_arm,
            div_ahb,
            arm_hz,
            div_ipg,
        }
    }

    /// Returns the IPG clock frequency described by these timings
    fn ipg_hz(&self) -> u32 {
        self.arm_hz / self.div_ipg
    }
}

pub const CCM_ANALOG_PLL_ARM: *mut u32 = 0x400D_8000 as _;

const DIV_SEL: Field = Field::new(0, 0x7f);

/// Restart the ARM PLL with a new `div_sel` value
///
/// # Safety
///
/// Unsynchronized writes to CCM memory.
unsafe fn restart_pll_arm(div_sel: u32) {
    const POWERDOWN: Field = Field::new(12, 1);
    const ENABLE: Field = Field::new(13, 1);

    // Clear all bits except POWERDOWN
    POWERDOWN.write_zero(CCM_ANALOG_PLL_ARM, 1);
    // Clear POWERDOWN write above
    DIV_SEL.write_zero(CCM_ANALOG_PLL_ARM, div_sel);
    // Enable the PLL
    ENABLE.modify(CCM_ANALOG_PLL_ARM, 1);

    const LOCK: u32 = 1 << 31;
    while CCM_ANALOG_PLL_ARM.read_volatile() & LOCK == 0 {}
}

const ARM_PODF: Field = Field::new(0, 0x7);
const AHB_PODF: Field = Field::new(10, 0x7);
const IPG_PODF: Field = Field::new(8, 0x3);

/// Write the ARM timings throughout the CCM
///
/// # Safety
///
/// Unsynchronized writes to CCM memory.
unsafe fn set_timings(timings: &Timings) {
    ARM_PODF.modify(CCM_CACCR, timings.div_arm.saturating_sub(1));
    wait_for_handshake();

    AHB_PODF.modify(CCM_CBCDR, timings.div_ahb.saturating_sub(1));
    wait_for_handshake();

    IPG_PODF.modify(CCM_CBCDR, timings.div_ipg.saturating_sub(1));
}

/// Returns the ARM timings read from the CCM peripheral
///
/// Assumes that the ARM clock was configured using this module's API.
/// If the ARM clock is not running on PLL1, these timings may be meaningless.
///
/// # Safety
///
/// Reads global, mutable memory.
#[inline(always)]
unsafe fn timings() -> Timings {
    timings_(CCM_CACCR, CCM_CBCDR, CCM_ANALOG_PLL_ARM)
}

#[inline(always)]
unsafe fn timings_(caccr: *const u32, cbcdr: *const u32, pll_arm: *const u32) -> Timings {
    let div_arm = ARM_PODF.read(caccr) + 1;
    let div_ahb = AHB_PODF.read(cbcdr) + 1;
    let div_ipg = IPG_PODF.read(cbcdr) + 1;
    let pll_arm_div_sel = DIV_SEL.read(pll_arm);
    let arm_hz = compute_arm_hz(div_arm, div_ahb, pll_arm_div_sel);
    Timings {
        div_arm,
        div_ahb,
        pll_arm_div_sel,
        arm_hz,
        div_ipg,
    }
}

/// Set the ARM clock frequency, returning the ARM and IPG clock speeds
///
/// The function will temporarily switch the ARM clock to the 24MHz clock
/// while it modifies the clock speed. While in this switched state, the ARM
/// core will execute instructions much more slowly than usual. To avoid
/// negative performance, consider setting the ARM clock speed early in system
/// startup, or by calling this function in a critical section.
///
/// The function lets you reconfigure the IPG clock frequency. Any peripherals
/// that use the IPG clock may not be aware of this new clock frequency. You're
/// responsible for updating any peripherals to reference the new clock speed.
///
/// When this function returns, the ARM clock runs on PLL1 (the "ARM PLL").
///
/// # Safety
///
/// Modifies CCM and CCM_ANALOG peripheral memory. This may be aliased
/// elsewhere, and could be in the middle of a modification. Users should
/// prefer the safer [`Handle::set_frequency_arm`](crate::Handle::set_frequency_arm)
/// method.
pub unsafe fn set_frequency(hz: u32) -> (ARMClock, IPGClock) {
    on_ahb_clk_oscillator(|| {
        let timings = Timings::target(hz);
        restart_pll_arm(timings.pll_arm_div_sel);
        set_timings(&timings);
        (ARMClock(timings.arm_hz), IPGClock(timings.ipg_hz()))
    })
}

/// Returns the ARM and IPG clock frequencies
///
/// The function assumes that the ARM clock runs on PLL1.
/// The clock values may be incorrect until after the first call to
/// [`set_frequency_arm`](crate::set_frequency_arm).
///
/// # Safety
///
/// Reads multiple CCM registers without synchronization. It's safer to use
/// [`Handle::frequency_arm`](crate::Handle::frequency_arm) to read the frequencies.
pub unsafe fn frequency() -> (ARMClock, IPGClock) {
    let timings = timings();
    (ARMClock(timings.arm_hz), IPGClock(timings.ipg_hz()))
}

#[cfg(test)]
mod tests {
    use super::{timings_, Timings};

    #[test]
    fn imxrt1060_target_freq() {
        let timings = Timings::target(600_000_000);
        assert_eq!(timings.arm_hz, 600_000_000);
        assert_eq!(timings.ipg_hz(), 150_000_000);
        assert!(54 <= timings.pll_arm_div_sel && timings.pll_arm_div_sel <= 108);

        let timings = Timings::target(600_000_100);
        assert_eq!(timings.arm_hz, 600_000_000);
    }

    #[test]
    fn imxrt1060_frequency() {
        let expected = Timings::target(600_000_000);
        let caccr = expected.div_arm.saturating_sub(1);
        let cbcdr =
            expected.div_ahb.saturating_sub(1) << 10 | expected.div_ipg.saturating_sub(1) << 8;
        let pll_arm = expected.pll_arm_div_sel;

        let actual = unsafe { timings_(&caccr, &cbcdr, &pll_arm) };
        assert_eq!(actual, expected);
    }
}
