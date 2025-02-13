pub(crate) use embassy_executor::InterruptExecutor as Executor;
pub use embassy_nrf::interrupt::SWI0_EGU0 as SWI;
pub use embassy_nrf::{init, OptionalPeripherals};
pub use embassy_nrf::{interrupt, peripherals};

#[cfg(feature = "usb")]
use embassy_nrf::{bind_interrupts, rng, usb as nrf_usb};

#[cfg(feature = "usb")]
bind_interrupts!(struct Irqs {
    USBD => nrf_usb::InterruptHandler<peripherals::USBD>;
    POWER_CLOCK => nrf_usb::vbus_detect::InterruptHandler;
    RNG => rng::InterruptHandler<peripherals::RNG>;
});

#[interrupt]
unsafe fn SWI0_EGU0() {
    crate::EXECUTOR.on_interrupt()
}

#[cfg(feature = "usb")]
pub mod usb {
    use embassy_nrf::{
        peripherals,
        usb::{vbus_detect::HardwareVbusDetect, Driver},
    };

    use crate::arch;

    pub type UsbDriver = Driver<'static, peripherals::USBD, HardwareVbusDetect>;

    pub fn driver(peripherals: &mut arch::OptionalPeripherals) -> UsbDriver {
        use super::Irqs;

        let usbd = peripherals.USBD.take().unwrap();
        Driver::new(usbd, Irqs, HardwareVbusDetect::new(Irqs))
    }
}
