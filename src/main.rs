#![no_std]
#![no_main]

use arduino_nano33iot::{
    hal::{
        clock::GenericClockController,
        delay::Delay,
        pac::{CorePeripherals, Peripherals},
        prelude::*,
    },
    Led, Pins,
};
use panic_halt as _;

#[arduino_nano33iot::entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = Pins::new(peripherals.PORT);
    let mut led: Led = pins.led_sck.into();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        delay.delay_ms(200u8);
        led.set_high().unwrap();
        delay.delay_ms(200u8);
        led.set_low().unwrap();
    }
}
