//! Helpers for clock registers

/// A field in a CCM register
#[derive(Clone, Copy)]
pub struct Field {
    offset: u32,
    mask: u32,
}

impl Field {
    /// Create a register field
    ///
    /// Callers should not apply offset to the mask. See the
    /// tests for an example.
    pub const fn new(offset: u32, mask: u32) -> Self {
        Field {
            offset,
            mask: mask << offset,
        }
    }
}

/// A CCM register
#[derive(Clone, Copy)]
pub struct Register {
    /// The clock divider field
    divider: Field,
    /// The clock selection field
    select: Field,
    /// Register address
    address: *mut u32,
}

impl Register {
    /// # Safety
    ///
    /// Caller must ensure that `address` is valid.
    pub const unsafe fn new(divider: Field, select: Field, address: *mut u32) -> Self {
        Register {
            divider,
            select,
            address,
        }
    }
    /// # Safety
    ///
    /// Caller must ensure that this read-modify-write operation is atomic
    #[inline(always)]
    pub unsafe fn set(&self, divider: u32, select: u32) {
        let mut reg = self.address.read_volatile();
        reg &= !(self.divider.mask | self.select.mask);
        reg |= (divider << self.divider.offset) & self.divider.mask;
        reg |= (select << self.select.offset) & self.select.mask;
        self.address.write_volatile(reg);
    }
    /// Returns the clock divider
    #[inline(always)]
    pub fn divider(&self) -> u32 {
        // Safety: assumed valid through `new`, atomic read
        let reg = unsafe { self.address.read_volatile() };
        (reg & self.divider.mask) >> self.divider.offset
    }
}

#[cfg(test)]
mod tests {
    use super::{Field, Register};

    const LPI2C_CLK_PODF: Field = Field::new(19, 0x3F);
    const LPI2C_CLK_SEL: Field = Field::new(18, 0x01);

    #[test]
    fn set() {
        let mut reg = 0;
        unsafe {
            let reg = Register::new(LPI2C_CLK_PODF, LPI2C_CLK_SEL, &mut reg);
            reg.set(u32::max_value(), u32::max_value());
        }
        assert_eq!(reg, 0x01FC_0000);
        unsafe {
            let reg = Register::new(LPI2C_CLK_PODF, LPI2C_CLK_SEL, &mut reg);
            reg.set(0, 0);
        }
        assert_eq!(reg, 0);
        reg = u32::max_value();
        unsafe {
            let reg = Register::new(LPI2C_CLK_PODF, LPI2C_CLK_SEL, &mut reg);
            reg.set(3, 1);
        }
        assert_eq!(reg, 0xFE1F_FFFF);
    }

    #[test]
    fn divider() {
        let mut reg = u32::max_value();
        unsafe {
            let reg = Register::new(LPI2C_CLK_PODF, LPI2C_CLK_SEL, &mut reg);
            reg.set(3, 1);
            assert_eq!(reg.divider(), 3);
        }
    }
}