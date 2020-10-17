//! Instance implementations for the `imxrt-ral` crate
//!
//! # Safety
//!
//! We know all of these trait implementations to be safe,
//! since we've studied the RAL and know its guarantees.

use crate::{Instance, ADC, DMA, GPT, I2C, PIT, PWM, SPI, UART};
use imxrt_ral as ral;

impl crate::CCM {
    /// Converts the `imxrt-ral` CCM instance into the `CCM` driver
    ///
    /// This is safer than using `new()`, since we take ownership of the
    /// only other CCM instance in the system.
    pub fn from_ral_ccm(_: ral::ccm::Instance) -> Self {
        // Safety: we "own" the CCM instance, so no one
        // else can (safely) access it.
        unsafe { crate::CCM::new() }
    }
}

unsafe impl Instance for ral::dma0::Instance {
    type Inst = DMA;
    fn instance(&self) -> DMA {
        DMA
    }
}

unsafe impl Instance for ral::lpi2c::Instance {
    type Inst = I2C;
    fn instance(&self) -> I2C {
        #[cfg(not(any(feature = "imxrt1011", feature = "imxrt1062")))]
        compile_error!("Ensure that LPI2C instances are correct");

        match &**self as *const _ {
            ral::lpi2c::LPI2C1 => I2C::I2C1,
            ral::lpi2c::LPI2C2 => I2C::I2C2,
            #[cfg(feature = "imxrt1062")]
            ral::lpi2c::LPI2C3 => I2C::I2C3,
            #[cfg(feature = "imxrt1062")]
            ral::lpi2c::LPI2C4 => I2C::I2C4,
            _ => unreachable!(),
        }
    }
}

unsafe impl Instance for ral::gpt::Instance {
    type Inst = GPT;
    fn instance(&self) -> GPT {
        match &**self as *const _ {
            ral::gpt::GPT1 => GPT::GPT1,
            ral::gpt::GPT2 => GPT::GPT2,
            _ => unreachable!(),
        }
    }
}

unsafe impl Instance for ral::pit::Instance {
    type Inst = PIT;
    fn instance(&self) -> PIT {
        PIT
    }
}

unsafe impl Instance for ral::lpspi::Instance {
    type Inst = SPI;
    fn instance(&self) -> SPI {
        #[cfg(not(any(feature = "imxrt1011", feature = "imxrt1062")))]
        compile_error!("Ensure that LPSPI instances are correct");

        match &**self as *const _ {
            ral::lpspi::LPSPI1 => SPI::SPI1,
            ral::lpspi::LPSPI2 => SPI::SPI2,
            #[cfg(feature = "imxrt1062")]
            ral::lpspi::LPSPI3 => SPI::SPI3,
            #[cfg(feature = "imxrt1062")]
            ral::lpspi::LPSPI4 => SPI::SPI4,
            _ => unreachable!(),
        }
    }
}

unsafe impl Instance for ral::lpuart::Instance {
    type Inst = UART;
    fn instance(&self) -> UART {
        #[cfg(not(any(feature = "imxrt1011", feature = "imxrt1062")))]
        compile_error!("Ensure that LPUART instances are correct");

        match &**self as *const _ {
            ral::lpuart::LPUART1 => UART::UART1,
            ral::lpuart::LPUART2 => UART::UART2,
            ral::lpuart::LPUART3 => UART::UART3,
            ral::lpuart::LPUART4 => UART::UART4,
            #[cfg(feature = "imxrt1062")]
            ral::lpuart::LPUART5 => UART::UART5,
            #[cfg(feature = "imxrt1062")]
            ral::lpuart::LPUART6 => UART::UART6,
            #[cfg(feature = "imxrt1062")]
            ral::lpuart::LPUART7 => UART::UART7,
            #[cfg(feature = "imxrt1062")]
            ral::lpuart::LPUART8 => UART::UART8,
            _ => unreachable!(),
        }
    }
}

unsafe impl Instance for ral::adc::Instance {
    type Inst = ADC;
    fn instance(&self) -> ADC {
        #[cfg(not(any(feature = "imxrt1011", feature = "imxrt1062")))]
        compile_error!("Ensure that ADC instances are correct");

        match &**self as *const _ {
            ral::adc::ADC1 => ADC::ADC1,
            #[cfg(feature = "imxrt1062")]
            ral::adc::ADC2 => ADC::ADC2,
            _ => unreachable!(),
        }
    }
}

unsafe impl Instance for ral::pwm::Instance {
    type Inst = PWM;
    fn instance(&self) -> PWM {
        #[cfg(not(any(feature = "imxrt1011", feature = "imxrt1062")))]
        compile_error!("Ensure that PWM instances are correct");

        match &**self as *const _ {
            ral::pwm::PWM1 => PWM::PWM1,
            #[cfg(feature = "imxrt1062")]
            ral::pwm::PWM2 => PWM::PWM2,
            #[cfg(feature = "imxrt1062")]
            ral::pwm::PWM3 => PWM::PWM3,
            #[cfg(feature = "imxrt1062")]
            ral::pwm::PWM4 => PWM::PWM4,
            _ => unreachable!(),
        }
    }
}
