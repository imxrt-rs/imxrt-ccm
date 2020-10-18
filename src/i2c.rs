//! I2C clock control

use super::{set_clock_gate, ClockGate, Disabled, Handle, I2CClock, Instance, CCGR_BASE};

/// I2C clock frequency (Hz)
pub const CLOCK_FREQUENCY_HZ: u32 = crate::OSCILLATOR_FREQUENCY_HZ / I2C_CLOCK_DIVIDER;
/// I2C peripheral clock divider
const I2C_CLOCK_DIVIDER: u32 = 3;

impl<I> Disabled<I2CClock<I>> {
    /// Enable the I2C clocks
    ///
    /// When `enable` returns, all I2C clock gates will be set to off.
    /// Use [`clock_gate`](struct.I2CClock.html#method.clock_gate)
    /// to turn on I2C clock gates.
    pub fn enable(self, _: &mut Handle) -> I2CClock<I>
    where
        I: Instance<Inst = I2C>,
    {
        unsafe {
            clock_gate::<I>(I2C::I2C1, ClockGate::Off);
            clock_gate::<I>(I2C::I2C2, ClockGate::Off);
            clock_gate::<I>(I2C::I2C3, ClockGate::Off);
            clock_gate::<I>(I2C::I2C4, ClockGate::Off);

            configure()
        };
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

impl<I> I2CClock<I> {
    /// Set the clock gate gate for the I2C instance
    pub fn clock_gate(&mut self, i2c: &mut I, gate: ClockGate)
    where
        I: Instance<Inst = I2C>,
    {
        unsafe { clock_gate::<I>(i2c.instance(), gate) }
    }
}

/// Set the clock gate gate for a I2C peripheral
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`I2CClock`](struct.I2CClock.html) for a
/// safer interface.
pub unsafe fn clock_gate<I: Instance<Inst = I2C>>(i2c: I2C, gate: ClockGate) {
    let value = gate as u8;
    match super::check_instance::<I>(i2c) {
        Some(I2C::I2C1) => set_clock_gate(CCGR_BASE.add(2), &[3], value),
        Some(I2C::I2C2) => set_clock_gate(CCGR_BASE.add(2), &[4], value),
        Some(I2C::I2C3) => set_clock_gate(CCGR_BASE.add(2), &[5], value),
        Some(I2C::I2C4) => set_clock_gate(CCGR_BASE.add(6), &[12], value),
        _ => (),
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
