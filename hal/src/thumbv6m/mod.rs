pub mod eic;

mod reset_cause;
pub use reset_cause::*;

mod serial_number;
pub use serial_number::*;

pub mod calibration;
pub mod clock;
pub mod timer;

#[cfg(feature = "unproven")]
pub mod adc;

#[cfg(feature = "unproven")]
pub mod pwm;

#[cfg(feature = "unproven")]
pub mod watchdog;

// Note that SAMD10 is identical to SAMD11 except it lacks USB, we treat SAMD10
// as an alias to SAMD11.  When SAMD11 USB is implemented, exclude SAMD10 here.
#[cfg(all(feature = "usb", feature = "samd21"))]
pub mod usb;

pub(crate) mod sercom;
