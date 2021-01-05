//! This tests a simple ADC Instance implementation. The test ensures
//! that the API handle will work for more than just the RAL
//! implementation.
//!
//! This test doesn't run. If the test compiles, the test passes.

use imxrt_ccm as ccm;

struct ADC;

unsafe impl ccm::Instance for ADC {
    type Inst = ccm::ADC;
    fn instance(&self) -> Self::Inst {
        unreachable!("This test doesn't run")
    }
    fn is_valid(_: Self::Inst) -> bool {
        unreachable!("This test doesn't run")
    }
}

struct TestClocks;
impl ccm::Clocks for TestClocks {
    type PIT = ();
    type GPT = ();
    type SPI = SPI;
    type I2C = ();
    type UART = ();
}

#[allow(unused)]
fn adc_compiles() {
    let mut handle = unsafe { ccm::CCM::<TestClocks>::new() };
    let mut adc = ADC;
    handle.set_clock_gate_adc(&mut adc, ccm::ClockGate::Off);
    handle.clock_gate_adc(&adc);
}

struct SPI;

unsafe impl ccm::Instance for SPI {
    type Inst = ccm::spi::SPI;
    fn instance(&self) -> Self::Inst {
        unreachable!("This test doesn't run")
    }
    fn is_valid(_: Self::Inst) -> bool {
        unreachable!("This test doesn't run")
    }
}

#[allow(unused)]
fn spi_compiles() {
    let mut handle = unsafe { ccm::CCM::<TestClocks>::new() };
    let mut spi = SPI;
    handle
        .spi_clock_mut()
        .set_clock_gate(&mut spi, ccm::ClockGate::Off);
    handle.spi_clock().clock_gate(&spi);
}
