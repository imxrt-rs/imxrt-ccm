//! Clock gate control

/// Starting address of the clock control gate registers
#[cfg(not(test))]
pub const CCGR_BASE: *mut u32 = 0x400F_C068 as *mut u32;

#[inline(always)]
unsafe fn set_clock_gate_(ccgr: *mut u32, gates: &[usize], value: u8) {
    const MASK: u32 = 0b11;
    let mut register = core::ptr::read_volatile(ccgr);

    for gate in gates {
        let shift: usize = gate * 2;
        register &= !(MASK << shift);
        register |= (MASK & (value as u32)) << shift;
    }

    core::ptr::write_volatile(ccgr, register);
}

#[inline(always)]
unsafe fn get_clock_gate_(ccgr: *const u32, gate: usize) -> u8 {
    const MASK: u32 = 0b11;
    let register = core::ptr::read_volatile(ccgr);
    let shift = gate * 2;
    ((register >> shift) & MASK) as u8
}

/// # Safety
///
/// Should only be used when you have a mutable reference to an enabled clock.
/// Should only be used on a valid clock gate register.
#[cfg(not(test))]
#[inline(always)]
pub unsafe fn set_clock_gate(ccgr: *mut u32, gates: &[usize], value: u8) {
    set_clock_gate_(ccgr, gates, value)
}

/// # Safety
///
/// Assumes memory is valid
#[cfg(not(test))]
#[inline(always)]
pub unsafe fn get_clock_gate(ccgr: *const u32, gate: usize) -> u8 {
    get_clock_gate_(ccgr, gate)
}

#[cfg(test)]
pub use tests::{get_clock_gate, set_clock_gate, CCGR_BASE};

#[cfg(test)]
mod tests {
    use super::{get_clock_gate_, set_clock_gate_};
    use std::{cell::Cell, thread::LocalKey};

    type Memory = [Cell<u32>; 8];
    const MEMORY_INIT: Memory = [
        Cell::new(0),
        Cell::new(0),
        Cell::new(0),
        Cell::new(0),
        Cell::new(0),
        Cell::new(0),
        Cell::new(0),
        Cell::new(0),
    ];

    /// A stub CCGR array
    pub struct ClockGateRegisterArray;

    pub struct ClockGateRegister {
        memory: &'static LocalKey<Memory>,
        offset: usize,
    }

    impl ClockGateRegisterArray {
        pub const fn new() -> Self {
            ClockGateRegisterArray
        }
        pub fn add(&self, offset: usize) -> ClockGateRegister {
            thread_local! { static MEMORY: Memory = MEMORY_INIT;  }
            ClockGateRegister {
                memory: &MEMORY,
                offset,
            }
        }
    }

    /// # Safety
    ///
    /// This function is actually safe, but marked as unsafe for compatibility with
    /// the real implementation.
    pub unsafe fn set_clock_gate(ccgr: ClockGateRegister, gates: &[usize], value: u8) {
        let offset = ccgr.offset;
        ccgr.memory.with(|array| {
            let mut mem = array[offset].get();
            set_clock_gate_(&mut mem, gates, value);
            array[offset].set(mem);
        });
    }

    /// # Safety
    ///
    /// This function is actually safe, but marked unsafe for compatibility with
    /// the real implementation.
    pub unsafe fn get_clock_gate(ccgr: ClockGateRegister, gate: usize) -> u8 {
        let offset = ccgr.offset;
        ccgr.memory.with(|array| {
            let mem = array[offset].get();
            get_clock_gate_(&mem, gate)
        })
    }

    pub static CCGR_BASE: ClockGateRegisterArray = ClockGateRegisterArray::new();

    #[test]
    fn test_set_clock_gate() {
        let mut reg = 0;

        unsafe {
            set_clock_gate_(&mut reg, &[3, 7], 0b11);
        }
        assert_eq!(reg, (0b11 << 14) | (0b11 << 6));

        unsafe {
            set_clock_gate_(&mut reg, &[3], 0b1);
        }
        assert_eq!(reg, (0b11 << 14) | (0b01 << 6));

        unsafe {
            set_clock_gate_(
                &mut reg,
                &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                0b01,
            );
        }
        assert_eq!(reg, 0x55555555);

        unsafe {
            set_clock_gate_(
                &mut reg,
                &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                0b10,
            );
        }
        assert_eq!(reg, 0xAAAAAAAA);
    }

    #[test]
    fn test_get_clock_gate() {
        let reg = 0x0000_0300;
        unsafe {
            assert_eq!(get_clock_gate_(&reg, 4), 0b11);
        }
    }
}
