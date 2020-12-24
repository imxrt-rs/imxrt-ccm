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

#[allow(unused)]
fn adc_compiles() {
    let mut handle = unsafe { ccm::Handle::new() };
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
    let mut spi_clock = unsafe { ccm::spi::SPIClock::<SPI>::assume_enabled() };
    let mut spi = SPI;
    spi_clock.set_clock_gate(&mut spi, ccm::ClockGate::Off);
    spi_clock.clock_gate(&spi);
}
