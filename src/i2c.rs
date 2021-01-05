//! I2C clock control

use super::{set_clock_gate, ClockGate, ClockGateLocation, ClockGateLocator, Instance};
use crate::register::{Field, Register};
use core::marker::PhantomData;

/// Base I2C clock frequency (Hz)
const CLOCK_FREQUENCY_HZ: u32 = crate::OSCILLATOR_FREQUENCY_HZ;
/// Default I2C peripheral clock divider
const DEFAULT_CLOCK_DIVIDER: u32 = 3;

/// The I2C clock
///
/// The I2C clock is based on the crystal oscillator.
pub struct I2CClock<I>(PhantomData<I>);

impl<I> I2CClock<I> {
    pub(crate) const fn new() -> Self {
        I2CClock(PhantomData)
    }
}

impl<I> I2CClock<I>
where
    I: Instance<Inst = I2C>,
{
    /// Configure the I2C clocks, and supply the clock divider.
    ///
    /// The divider should be between [1, 64]. The function will treat a 0 as 1,
    /// and anything greater than 64 as 64.
    ///
    /// When `configure` returns, all I2C clock gates will be set to off.
    /// Use [`clock_gate`](struct.I2CClock.html#method.clock_gate)
    /// to turn on I2C clock gates.
    #[inline(always)]
    pub fn configure_divider(&mut self, divider: u32) {
        unsafe {
            set_clock_gate::<I>(I2C::I2C1, ClockGate::Off);
            set_clock_gate::<I>(I2C::I2C2, ClockGate::Off);
            set_clock_gate::<I>(I2C::I2C3, ClockGate::Off);
            set_clock_gate::<I>(I2C::I2C4, ClockGate::Off);

            configure(divider)
        };
    }

    /// Configure the I2C clocks with a default divider
    ///
    /// The default divider will allow the I2C peripheral to support both
    /// 100KHz and 400KHz clock speeds.
    ///
    /// When `configure` returns, all I2C clock gates will be set to off.
    /// Use [`clock_gate`](struct.I2CClock.html#method.clock_gate)
    /// to turn on I2C clock gates.
    #[inline(always)]
    pub fn configure(&mut self) {
        self.configure_divider(DEFAULT_CLOCK_DIVIDER);
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

impl ClockGateLocator for I2C {
    #[inline(always)]
    fn location(&self) -> ClockGateLocation {
        match self {
            I2C::I2C1 => ClockGateLocation {
                offset: 2,
                gates: &[3],
            },
            I2C::I2C2 => ClockGateLocation {
                offset: 2,
                gates: &[4],
            },
            I2C::I2C3 => ClockGateLocation {
                offset: 2,
                gates: &[5],
            },
            I2C::I2C4 => ClockGateLocation {
                offset: 6,
                gates: &[12],
            },
        }
    }
}

impl<I> I2CClock<I>
where
    I: Instance<Inst = I2C>,
{
    /// Set the clock gate setting for the I2C instance
    #[inline(always)]
    pub fn set_clock_gate(&mut self, i2c: &mut I, gate: ClockGate) {
        unsafe { set_clock_gate::<I>(i2c.instance(), gate) }
    }

    /// Returns the clock gate setting for the I2C instance
    #[inline(always)]
    pub fn clock_gate(&self, i2c: &I) -> ClockGate {
        // Unwrap OK: instance must be valid to call this function,
        // or the Instance implementation is invalid.
        super::get_clock_gate::<I>(i2c.instance()).unwrap()
    }

    /// Returns the configured I2C clock frequency
    #[inline(always)]
    pub fn frequency(&self) -> u32 {
        frequency()
    }
}

const LPI2C_CLK_PODF: Field = Field::new(19, 0x3F);
const LPI2C_CLK_SEL: Field = Field::new(18, 0x01);
const CSCDR2: Register =
    unsafe { Register::new(LPI2C_CLK_PODF, LPI2C_CLK_SEL, 0x400F_C038 as *mut u32) };

/// Configure the I2C clock root, specifying a clock divider
///
/// Configure will **not** disable peripheral clock gates. You should disable
/// clock gates yourself before calling this function.
///
/// Clock divider should be between [1, 64]. The function will treat a 0 as 1,
/// and anything greater than 64 as 64.
///
/// # Safety
///
/// This could be called anywhere, modifying global memory that's owned by
/// the CCM. Consider using the [`I2CClock`](struct.I2CClock.html) for a
/// safer interface.
#[inline(always)]
pub unsafe fn configure(divider: u32) {
    configure_(divider, &CSCDR2);
}

#[inline(always)]
unsafe fn configure_(divider: u32, reg: &Register) {
    const OSCILLATOR: u32 = 1;
    reg.set(divider.min(64).max(1).saturating_sub(1), OSCILLATOR);
}

/// Returns the I2C clock frequency
#[inline(always)]
pub fn frequency() -> u32 {
    frequency_(&CSCDR2)
}

#[inline(always)]
fn frequency_(reg: &Register) -> u32 {
    let divider = reg.divider() + 1;
    CLOCK_FREQUENCY_HZ / divider
}

#[cfg(test)]
mod tests {

    use super::{
        configure_, frequency_, Register, CLOCK_FREQUENCY_HZ, LPI2C_CLK_PODF, LPI2C_CLK_SEL,
    };

    unsafe fn register(mem: &mut u32) -> Register {
        Register::new(LPI2C_CLK_PODF, LPI2C_CLK_SEL, mem)
    }

    #[test]
    fn i2c_divider_upper_bound() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(65, &reg);
            assert_eq!(frequency_(&reg), CLOCK_FREQUENCY_HZ / 64);
        }
    }

    #[test]
    fn i2c_divider_lower_bound() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(0, &reg);
            assert_eq!(frequency_(&reg), CLOCK_FREQUENCY_HZ);
        }
    }

    #[test]
    fn i2c_divider() {
        let mut mem: u32 = 0;
        unsafe {
            let reg = register(&mut mem);
            configure_(7, &reg);
            assert_eq!(frequency_(&reg), CLOCK_FREQUENCY_HZ / 7);
        }
    }
}
