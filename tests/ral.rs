//! Tests for RAL implementation details

#![cfg(feature = "imxrt-ral")]

use imxrt_ccm::*;
use imxrt_ral as ral;

const IMXRT1060: bool = cfg!(feature = "imxrt1060");

#[test]
fn dcdc_is_valid() {
    assert!(ral::dcdc::Instance::is_valid(DCDC));
}

#[test]
fn dma_is_valid() {
    assert!(ral::dma0::Instance::is_valid(DMA));
}

#[test]
fn i2c_is_valid() {
    assert!(ral::lpi2c::Instance::is_valid(I2C::I2C1));
    assert!(ral::lpi2c::Instance::is_valid(I2C::I2C2));
    assert_eq!(ral::lpi2c::Instance::is_valid(I2C::I2C3), IMXRT1060);
    assert_eq!(ral::lpi2c::Instance::is_valid(I2C::I2C4), IMXRT1060);
}

#[test]
fn gpt_is_valid() {
    assert!(ral::gpt::Instance::is_valid(GPT::GPT1));
    assert!(ral::gpt::Instance::is_valid(GPT::GPT2));
}

#[test]
fn pit_is_valid() {
    assert!(ral::pit::Instance::is_valid(PIT))
}

#[test]
fn spi_is_valid() {
    assert!(ral::lpspi::Instance::is_valid(SPI::SPI1));
    assert!(ral::lpspi::Instance::is_valid(SPI::SPI2));
    assert_eq!(ral::lpspi::Instance::is_valid(SPI::SPI3), IMXRT1060);
    assert_eq!(ral::lpspi::Instance::is_valid(SPI::SPI3), IMXRT1060);
}

#[test]
fn uart_is_valid() {
    assert!(ral::lpuart::Instance::is_valid(UART::UART1));
    assert!(ral::lpuart::Instance::is_valid(UART::UART2));
    assert!(ral::lpuart::Instance::is_valid(UART::UART3));
    assert!(ral::lpuart::Instance::is_valid(UART::UART4));
    assert_eq!(ral::lpuart::Instance::is_valid(UART::UART5), IMXRT1060);
    assert_eq!(ral::lpuart::Instance::is_valid(UART::UART6), IMXRT1060);
    assert_eq!(ral::lpuart::Instance::is_valid(UART::UART7), IMXRT1060);
    assert_eq!(ral::lpuart::Instance::is_valid(UART::UART8), IMXRT1060);
}

#[cfg(feature = "imxrt1060")]
use ral::adc;
#[cfg(feature = "imxrt1010")]
use ral::adc1 as adc;

#[test]
fn adc_is_valid() {
    assert!(adc::Instance::is_valid(ADC::ADC1));
    assert_eq!(adc::Instance::is_valid(ADC::ADC2), IMXRT1060);
}

#[cfg(feature = "imxrt1060")]
use ral::pwm;
#[cfg(feature = "imxrt1010")]
use ral::pwm1 as pwm;

#[test]
fn pwm_is_valid() {
    assert!(pwm::Instance::is_valid(PWM::PWM1));
    assert_eq!(pwm::Instance::is_valid(PWM::PWM2), IMXRT1060);
    assert_eq!(pwm::Instance::is_valid(PWM::PWM3), IMXRT1060);
    assert_eq!(pwm::Instance::is_valid(PWM::PWM4), IMXRT1060);
}
