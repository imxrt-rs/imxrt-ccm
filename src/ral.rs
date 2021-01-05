//! Implementations for the imxrt-ral
//!
//! Use [`CCM::from_ral`](../struct.CCM.html#from_ral) to safely
//! acquire the CCM handle and clock roots.

use crate::{
    i2c::I2C,
    perclock::{GPT, PIT},
    spi::SPI,
    uart::UART,
    Instance, ADC, DCDC, DMA, PWM,
};
use imxrt_ral as ral;

/// Pairs the RAL instances to CCM clocks
pub struct Clocks;

impl crate::Clocks for Clocks {
    type PIT = ral::pit::Instance;
    type GPT = ral::gpt::Instance;
    type UART = ral::lpuart::Instance;
    type SPI = ral::lpspi::Instance;
    type I2C = ral::lpi2c::Instance;
}

/// Helper for a clock control module designed to the
/// RAL interface.
pub type CCM = crate::CCM<Clocks>;

/// A periodic clock that controls RAL PIT and GPT timings
pub type PerClock = crate::perclock::PerClock<ral::pit::Instance, ral::gpt::Instance>;
/// A UART clock that controls RAL LPUART timing
pub type UARTClock = crate::uart::UARTClock<ral::lpuart::Instance>;
/// A SPI clock that controls RAL LPSPI timing
pub type SPIClock = crate::spi::SPIClock<ral::lpspi::Instance>;
/// An I2C clock that contorls RAL LPI2C timing
pub type I2CClock = crate::i2c::I2CClock<ral::lpi2c::Instance>;

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
    pub fn from_ral(_: ral::ccm::Instance) -> Self {
        // Safety: we "own" the CCM instance, so no one
        // else can (safely) access it.
        unsafe { crate::CCM::new() }
    }
}

unsafe impl Instance for ral::dcdc::Instance {
    type Inst = DCDC;
    #[inline(always)]
    fn instance(&self) -> DCDC {
        DCDC
    }
    #[inline(always)]
    fn is_valid(_: DCDC) -> bool {
        true
    }
}

/// ```no_run
/// use imxrt_ccm::{CCM, ClockGate};
/// use imxrt_ral::ccm;
/// use imxrt_ral::dcdc::DCDC;
///
/// let mut handle = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut dcdc = DCDC::take().unwrap();
/// handle.set_clock_gate_dcdc(&mut dcdc, ClockGate::On);
/// handle.clock_gate_dcdc(&dcdc);
/// ```
#[cfg(doctest)]
struct DCDCClockGate;

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
/// let mut handle = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut dma = DMA0::take().unwrap();
/// handle.set_clock_gate_dma(&mut dma, ClockGate::On);
/// handle.clock_gate_dma(&dma);
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
/// let mut ccm = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut i2c_clock = ccm.i2c_clock_mut();
/// i2c_clock.configure();
/// let mut i2c = LPI2C2::take().unwrap();
/// i2c_clock.set_clock_gate(&mut i2c, ClockGate::On);
/// i2c_clock.clock_gate(&i2c);
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
/// let mut ccm = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut perclock = ccm.perclock_mut();
/// perclock.configure();
/// let mut gpt = GPT2::take().unwrap();
/// perclock.set_clock_gate_gpt(&mut gpt, ClockGate::On);
/// perclock.clock_gate_gpt(&gpt);
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
/// let mut ccm = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut perclock = ccm.perclock_mut();
/// perclock.configure();
/// let mut pit = PIT::take().unwrap();
/// perclock.set_clock_gate_pit(&mut pit, ClockGate::On);
/// perclock.clock_gate_pit(&pit);
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
/// let mut ccm = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut spi_clock = ccm.spi_clock_mut();
/// spi_clock.configure();
/// let mut spi = LPSPI1::take().unwrap();
/// spi_clock.set_clock_gate(&mut spi, ClockGate::On);
/// spi_clock.clock_gate(&spi);
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
/// let mut ccm = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut uart_clock = ccm.uart_clock_mut();
/// uart_clock.configure();
/// let mut uart = LPUART4::take().unwrap();
/// uart_clock.set_clock_gate(&mut uart, ClockGate::On);
/// uart_clock.clock_gate(&uart);
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
/// let mut handle = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut adc = ADC1::take().unwrap();
/// handle.set_clock_gate_adc(&mut adc, ClockGate::On);
/// handle.clock_gate_adc(&adc);
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
/// let mut handle = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut pwm = PWM1::take().unwrap();
/// handle.set_clock_gate_pwm(&mut pwm, ClockGate::On);
/// handle.clock_gate_pwm(&pwm);
/// ```
#[cfg(doctest)]
struct PWMClockGate;

#[cfg(test)]
mod tests {

    macro_rules! assert_send {
        ($type:ty) => {
            ::static_assertions::assert_impl_all!($type: Send);
        };
    }

    macro_rules! assert_not_sync {
        ($type:ty) => {
            ::static_assertions::assert_not_impl_any!($type: Sync);
        };
    }

    assert_send!(super::CCM);
    assert_not_sync!(super::CCM);

    assert_send!(super::PerClock);
    assert_not_sync!(super::PerClock);

    assert_send!(super::UARTClock);
    assert_not_sync!(super::UARTClock);

    assert_send!(super::SPIClock);
    assert_not_sync!(super::SPIClock);

    assert_send!(super::I2CClock);
    assert_not_sync!(super::I2CClock);
}
