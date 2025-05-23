use atsamd_hal::clock::GenericClockController;
use atsamd_hal::pac::{self, Mclk};
use atsamd_hal::sercom::{uart, IoSet2, Sercom2};
use atsamd_hal::time::Hertz;

#[cfg(feature = "usb")]
use atsamd_hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};
#[cfg(feature = "usb")]
use pac::gclk::{genctrl::Srcselect, pchctrl::Genselect};

use super::pins::aliases::*;

/// Uart pins (uses `SERCOM2`)
pub struct Uart {
    /// UART transmit pin
    pub tx: UartTxReset,

    /// UART receive pin
    pub rx: UartRxReset,
}

/// UART pads for the labelled RX & TX pins
pub type UartPads = uart::Pads<Sercom2, IoSet2, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type HalUart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

impl Uart {
    /// Set up the labelled TX/RX pins to operate as a UART device at the
    /// specified baud rate.
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        baud: F,
        sercom2: Sercom2,
        mclk: &mut Mclk,
    ) -> HalUart {
        let gclk0 = clocks.gclk0();
        let pads = uart::Pads::default().rx(self.rx).tx(self.tx);
        uart::Config::new(
            mclk,
            sercom2,
            pads,
            clocks.sercom2_core(&gclk0).unwrap().freq(),
        )
        .baud(
            baud.into(),
            uart::BaudMode::Fractional(uart::Oversampling::Bits16),
        )
        .enable()
    }
}

/// USB pins
pub struct Usb {
    /// USB data-minus pin
    pub dm: UsbDmReset,

    /// USB data-plus pin
    pub dp: UsbDpReset,
}

impl Usb {
    #[cfg(feature = "usb")]
    /// Create a USB allocator.
    pub fn usb_allocator(
        self,
        usb: pac::Usb,
        clocks: &mut GenericClockController,
        mclk: &mut Mclk,
    ) -> UsbBusAllocator<UsbBus> {
        clocks.configure_gclk_divider_and_source(Genselect::Gclk2, 1, Srcselect::Dfll, false);
        let usb_gclk = clocks.get_gclk(Genselect::Gclk2).unwrap();
        let usb_clock = &clocks.usb(&usb_gclk).unwrap();

        UsbBusAllocator::new(UsbBus::new(usb_clock, mclk, self.dm, self.dp, usb))
    }
}
