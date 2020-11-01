//! Clock gate control

use super::ClockGateLocation;

const MASK: u32 = 0b11;
const CCGR_BASE: *mut u32 = 0x400F_C068 as *mut u32;

/// # Safety
///
/// Modifies global, mutable memory. The read-modify-write operation is not
/// atomic.
#[inline(always)]
pub unsafe fn set(location: &ClockGateLocation, value: u8) {
    let ccgr = CCGR_BASE.add(location.offset);
    let mut register = ccgr.read_volatile();
    for gate in location.gates {
        let shift: usize = gate * 2;
        register &= !(MASK << shift);
        register |= (MASK & (value as u32)) << shift;
    }
    ccgr.write_volatile(register);
}

#[inline(always)]
pub fn get(location: &ClockGateLocation) -> u8 {
    // Safety: pointer in range
    let ccgr = unsafe { CCGR_BASE.add(location.offset) };
    // Safety: pointer valid
    let register = unsafe { ccgr.read_volatile() };
    let shift = location.gates[0] * 2;
    ((register >> shift) & MASK) as u8
}
