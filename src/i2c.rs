//! I2C clock control

use super::{set_clock_gate, ClockGate, Disabled, Handle, I2CClock, Instance, CCGR_BASE};

/// I2C peripheral clock frequency
///
/// If changing the root clock in `enable`, you'll need to update
/// this value.
const I2C_CLOCK_HZ: u32 = crate::OSCILLATOR_FREQUENCY_HZ / I2C_CLOCK_DIVIDER;
/// I2C peripheral clock divider
const I2C_CLOCK_DIVIDER: u32 = 3;

impl Disabled<I2CClock> {
    /// Enable the I2C clocks
    pub fn enable(self, _: &mut Handle) -> I2CClock {
        unsafe { configure() };
        self.0
    }
}

/// Peripheral instance identifier for I2C
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2C {
    I2C1,
    I2C2,
    I2C3,
    I2C4,
}

impl I2CClock {
    /// Set the clock gate gate for the I2C instance
    pub fn clock_gate<I: Instance<Inst = I2C>>(&mut self, i2c: &mut I, gate: ClockGate) {
        unsafe { clock_gate(i2c.instance(), gate) }
    }

    /// Returns the I2C clock frequency (Hz)
    pub const fn frequency() -> u32 {
        I2C_CLOCK_HZ
    }
}

/// Set the clock gate gate for a I2C peripheral
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`I2CClock`](struct.I2CClock.html) for a
/// safer interface.
pub unsafe fn clock_gate(i2c: I2C, gate: ClockGate) {
    let value = gate as u8;
    match i2c {
        I2C::I2C1 => set_clock_gate(CCGR_BASE.add(2), &[3], value),
        I2C::I2C2 => set_clock_gate(CCGR_BASE.add(2), &[4], value),
        I2C::I2C3 => set_clock_gate(CCGR_BASE.add(2), &[5], value),
        I2C::I2C4 => set_clock_gate(CCGR_BASE.add(6), &[12], value),
    }
}

/// Configure the I2C clock root
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`I2CClock`](struct.I2CClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn configure() {
    const CSCDR2: *mut u32 = 0x400F_C038 as *mut u32;
    const LPI2C_CLK_PODF_OFFSET: u32 = 19;
    const LPI2C_CLK_PODF_MASK: u32 = 0x3F << LPI2C_CLK_PODF_OFFSET;
    const LPI2C_CLK_SEL_OFFSET: u32 = 18;
    const LPI2C_CLK_SEL_MASK: u32 = 0x01 << LPI2C_CLK_SEL_OFFSET;
    const OSCILLATOR: u32 = 1;

    let mut cscdr2 = CSCDR2.read_volatile();
    cscdr2 &= !(LPI2C_CLK_PODF_MASK | LPI2C_CLK_SEL_MASK);
    cscdr2 |= I2C_CLOCK_DIVIDER.saturating_sub(1) << LPI2C_CLK_PODF_OFFSET;
    cscdr2 |= OSCILLATOR << LPI2C_CLK_SEL_OFFSET;
    CSCDR2.write_volatile(cscdr2);
}
