//! Implementations for the imxrt-ral
//!
//! Use [`CCM::from_ral`](../struct.CCM.html#from_ral) to safely
//! acquire the CCM handle and clock roots.
//!
//! The functions in this module are the same as the `clock_gate_*` functions
//! in the library root, but they inline the RAL instance.

use crate::{Instance, ADC, DMA, GPT, I2C, PIT, PWM, SPI, UART};
use imxrt_ral as ral;

/// Helper for a clock control module designed to the
/// RAL interface.
pub type CCM = crate::CCM<
    ral::pit::Instance,
    ral::gpt::Instance,
    ral::lpuart::Instance,
    ral::lpspi::Instance,
    ral::lpi2c::Instance,
>;

/// A periodic clock that controls RAL PIT and GPT timings
pub type PerClock = crate::PerClock<ral::pit::Instance, ral::gpt::Instance>;
/// A UART clock that controls RAL LPUART timing
pub type UARTClock = crate::UARTClock<ral::lpuart::Instance>;
/// A SPI clock that controls RAL LPSPI timing
pub type SPIClock = crate::SPIClock<ral::lpspi::Instance>;
/// An I2C clock that contorls RAL LPI2C timing
pub type I2CClock = crate::I2CClock<ral::lpi2c::Instance>;

impl CCM {
    /// Converts the `imxrt-ral` CCM instance into the `CCM` driver
    ///
    /// This is safer than using `new()`, since we take ownership of the
    /// only other CCM instance in the system.
    ///
    /// ```no_run
    /// use imxrt_ccm::CCM;
    /// use imxrt_ral::ccm;
    ///
    /// let ccm = ccm::CCM::take().map(CCM::from_ral).unwrap();
    /// ```
    pub const fn from_ral(_: ral::ccm::Instance) -> Self {
        // Safety: we "own" the CCM instance, so no one
        // else can (safely) access it.
        unsafe { crate::CCM::new() }
    }
}

unsafe impl Instance for ral::dma0::Instance {
    type Inst = DMA;
    #[inline(always)]
    fn instance(&self) -> DMA {
        DMA
    }
    #[inline(always)]
    fn is_valid(_: DMA) -> bool {
        true
    }
}

/// ```no_run
/// use imxrt_ccm::{CCM, ClockGate};
/// use imxrt_ral::ccm;
/// use imxrt_ral::dma0::DMA0;
///
/// let CCM{ mut handle, .. } = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// handle.set_clock_gate_dma(&mut DMA0::take().unwrap(), ClockGate::On);
/// ```
#[cfg(doctest)]
struct DMAClockGate;

#[cfg(not(any(feature = "imxrt1010", feature = "imxrt1060")))]
compile_error!("Ensure that LPI2C instances are correct");
unsafe impl Instance for ral::lpi2c::Instance {
    type Inst = I2C;
    #[inline(always)]
    fn instance(&self) -> I2C {
        match &**self as *const _ {
            ral::lpi2c::LPI2C1 => I2C::I2C1,
            ral::lpi2c::LPI2C2 => I2C::I2C2,
            #[cfg(feature = "imxrt1060")]
            ral::lpi2c::LPI2C3 => I2C::I2C3,
            #[cfg(feature = "imxrt1060")]
            ral::lpi2c::LPI2C4 => I2C::I2C4,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn is_valid(i2c: I2C) -> bool {
        matches!(i2c, I2C::I2C1 | I2C::I2C2)
            || (cfg!(feature = "imxrt1060") && matches!(i2c, I2C::I2C3 | I2C::I2C4))
    }
}

/// ```no_run
/// use imxrt_ccm::{CCM, ClockGate};
/// use imxrt_ral::ccm;
/// use imxrt_ral::lpi2c::LPI2C2;
///
/// let CCM{ mut handle, i2c_clock, .. } = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut i2c_clock = i2c_clock.enable(&mut handle);
/// i2c_clock.set_clock_gate(&mut LPI2C2::take().unwrap(), ClockGate::On);
/// ```
#[cfg(doctest)]
struct I2CClockGate;

unsafe impl Instance for ral::gpt::Instance {
    type Inst = GPT;
    #[inline(always)]
    fn instance(&self) -> GPT {
        match &**self as *const _ {
            ral::gpt::GPT1 => GPT::GPT1,
            ral::gpt::GPT2 => GPT::GPT2,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn is_valid(gpt: GPT) -> bool {
        match gpt {
            GPT::GPT1 | GPT::GPT2 => true,
        }
    }
}

/// ```no_run
/// use imxrt_ccm::{CCM, ClockGate};
/// use imxrt_ral::ccm;
/// use imxrt_ral::gpt::GPT2;
///
/// let CCM{ mut handle, perclock, .. } = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut perclock = perclock.enable(&mut handle);
/// perclock.set_clock_gate_gpt(&mut GPT2::take().unwrap(), ClockGate::On);
/// ```
#[cfg(doctest)]
struct GPTClockGate;

unsafe impl Instance for ral::pit::Instance {
    type Inst = PIT;
    #[inline(always)]
    fn instance(&self) -> PIT {
        PIT
    }
    #[inline(always)]
    fn is_valid(_: PIT) -> bool {
        true
    }
}

/// ```no_run
/// use imxrt_ccm::{CCM, ClockGate};
/// use imxrt_ral::ccm;
/// use imxrt_ral::pit::PIT;
///
/// let CCM{ mut handle, perclock, .. } = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut perclock = perclock.enable(&mut handle);
/// perclock.set_clock_gate_pit(&mut PIT::take().unwrap(), ClockGate::On);
/// ```
#[cfg(doctest)]
struct PITClockGate;

#[cfg(not(any(feature = "imxrt1010", feature = "imxrt1060")))]
compile_error!("Ensure that LPSPI instances are correct");
unsafe impl Instance for ral::lpspi::Instance {
    type Inst = SPI;
    #[inline(always)]
    fn instance(&self) -> SPI {
        match &**self as *const _ {
            ral::lpspi::LPSPI1 => SPI::SPI1,
            ral::lpspi::LPSPI2 => SPI::SPI2,
            #[cfg(feature = "imxrt1060")]
            ral::lpspi::LPSPI3 => SPI::SPI3,
            #[cfg(feature = "imxrt1060")]
            ral::lpspi::LPSPI4 => SPI::SPI4,
            _ => unreachable!(),
        }
    }
    #[inline(always)]
    fn is_valid(spi: SPI) -> bool {
        matches!(spi, SPI::SPI1 | SPI::SPI2)
            || (cfg!(feature = "imxrt1060") && matches!(spi, SPI::SPI3 | SPI::SPI4))
    }
}

/// ```no_run
/// use imxrt_ccm::{CCM, ClockGate};
/// use imxrt_ral::ccm;
/// use imxrt_ral::lpspi::LPSPI1;
///
/// let CCM{ mut handle, spi_clock, .. } = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut spi_clock = spi_clock.enable(&mut handle);
/// spi_clock.set_clock_gate(&mut LPSPI1::take().unwrap(), ClockGate::On);
/// ```
#[cfg(doctest)]
struct SPIClockGate;

#[cfg(not(any(feature = "imxrt1010", feature = "imxrt1060")))]
compile_error!("Ensure that LPUART instances are correct");
unsafe impl Instance for ral::lpuart::Instance {
    type Inst = UART;
    #[inline(always)]
    fn instance(&self) -> UART {
        match &**self as *const _ {
            ral::lpuart::LPUART1 => UART::UART1,
            ral::lpuart::LPUART2 => UART::UART2,
            ral::lpuart::LPUART3 => UART::UART3,
            ral::lpuart::LPUART4 => UART::UART4,
            #[cfg(feature = "imxrt1060")]
            ral::lpuart::LPUART5 => UART::UART5,
            #[cfg(feature = "imxrt1060")]
            ral::lpuart::LPUART6 => UART::UART6,
            #[cfg(feature = "imxrt1060")]
            ral::lpuart::LPUART7 => UART::UART7,
            #[cfg(feature = "imxrt1060")]
            ral::lpuart::LPUART8 => UART::UART8,
            _ => unreachable!(),
        }
    }
    #[inline(always)]
    fn is_valid(uart: UART) -> bool {
        matches!(uart, UART::UART1 | UART::UART2 | UART::UART3 | UART::UART4)
            || (cfg!(feature = "imxrt1060")
                && matches!(uart, UART::UART5 | UART::UART6 | UART::UART7 | UART::UART8))
    }
}

/// ```no_run
/// use imxrt_ccm::{CCM, ClockGate};
/// use imxrt_ral::ccm;
/// use imxrt_ral::lpuart::LPUART4;
///
/// let CCM{ mut handle, uart_clock, .. } = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut uart_clock = uart_clock.enable(&mut handle);
/// uart_clock.set_clock_gate(&mut LPUART4::take().unwrap(), ClockGate::On);
/// ```
#[cfg(doctest)]
struct UARTClockGate;

#[cfg(feature = "imxrt1060")]
use ral::adc;
#[cfg(feature = "imxrt1010")]
use ral::adc1 as adc;

#[cfg(not(any(feature = "imxrt1010", feature = "imxrt1060")))]
compile_error!("Ensure that ADC instances are correct");
unsafe impl Instance for adc::Instance {
    type Inst = ADC;
    #[inline(always)]
    fn instance(&self) -> ADC {
        match &**self as *const _ {
            adc::ADC1 => ADC::ADC1,
            #[cfg(feature = "imxrt1060")]
            adc::ADC2 => ADC::ADC2,
            _ => unreachable!(),
        }
    }
    #[inline(always)]
    fn is_valid(adc: ADC) -> bool {
        matches!(adc, ADC::ADC1) || (cfg!(feature = "imxrt1060") && matches!(adc, ADC::ADC2))
    }
}

/// ```no_run
/// use imxrt_ccm::{CCM, ClockGate};
/// use imxrt_ral::ccm;
/// #[cfg(feature = "imxrt1060")]
/// use imxrt_ral::adc::ADC1;
/// #[cfg(feature = "imxrt1010")]
/// use imxrt_ral::adc1::ADC1;
///
/// let CCM{ mut handle, .. } = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// handle.set_clock_gate_adc(&mut ADC1::take().unwrap(), ClockGate::On);
/// ```
#[cfg(doctest)]
struct ADCClockGate;

#[cfg(feature = "imxrt1060")]
use ral::pwm;
#[cfg(feature = "imxrt1010")]
use ral::pwm1 as pwm;

#[cfg(not(any(feature = "imxrt1010", feature = "imxrt1060")))]
compile_error!("Ensure that PWM instances are correct");
unsafe impl Instance for pwm::Instance {
    type Inst = PWM;
    #[inline(always)]
    fn instance(&self) -> PWM {
        match &**self as *const _ {
            pwm::PWM1 => PWM::PWM1,
            #[cfg(feature = "imxrt1060")]
            pwm::PWM2 => PWM::PWM2,
            #[cfg(feature = "imxrt1060")]
            pwm::PWM3 => PWM::PWM3,
            #[cfg(feature = "imxrt1060")]
            pwm::PWM4 => PWM::PWM4,
            _ => unreachable!(),
        }
    }
    #[inline(always)]
    fn is_valid(pwm: PWM) -> bool {
        matches!(pwm, PWM::PWM1)
            || (cfg!(feature = "imxrt1060") && matches!(pwm, PWM::PWM2 | PWM::PWM3 | PWM::PWM4))
    }
}

/// ```no_run
/// use imxrt_ccm::{CCM, ClockGate};
/// use imxrt_ral::ccm;
/// #[cfg(feature = "imxrt1060")]
/// use imxrt_ral::pwm::PWM1;
/// #[cfg(feature = "imxrt1010")]
/// use imxrt_ral::pwm1::PWM1;
///
/// let CCM{ mut handle, .. } = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// handle.set_clock_gate_pwm(&mut PWM1::take().unwrap(), ClockGate::On);
/// ```
#[cfg(doctest)]
struct PWMClockGate;
