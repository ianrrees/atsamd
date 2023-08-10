#![no_std]
#![no_main]

use metro_m4 as bsp;

use bsp::ehal;
use bsp::hal;
use bsp::pac;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use pac::{CorePeripherals, Peripherals};

use ehal::blocking::delay::DelayMs;
use ehal::digital::v2::OutputPin;
use hal::delay::Delay;

use core::convert::Infallible;

#[derive(Debug)]
enum Error {}

type Result<T> = core::result::Result<T, Error>;

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

struct LedStruct<P> {
    led: P,
    delay: Delay,
}

impl <P> LedStruct<P>
where
P: OutputPin<Error=Infallible> {
    fn on(&mut self, duration_ms: u8)->Result<()> {
        self.led.set_low()?;
        self.delay.delay_ms(duration_ms);
        Ok(())
    }

    fn off(&mut self, duration_ms: u8)->Result<()> {
        self.led.set_high()?;
        self.delay.delay_ms(duration_ms);
        Ok(())
    }
}

fn blink<P>(led_struct: &mut LedStruct<P>) -> Result<()>
where
P: OutputPin<Error=Infallible> {
    led_struct.on(200u8)?;
    led_struct.off(200u8)?;
    Ok(())
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let delay = Delay::new(core.SYST, &mut clocks);

    let mut led = LedStruct {led: pins.d13.into_push_pull_output(), delay};
    loop {
        blink(&mut led).unwrap();
    }
}
