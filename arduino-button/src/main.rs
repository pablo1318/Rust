#![no_std]
#![no_main]

use panic_halt as _;
use ufmt::{self, uwrite};

const BAUD_RATE: u32 = 57600; //BAUD RATE FOR SERIAL PRINTING
#[arduino_hal::entry]
fn main() -> ! {
    //SETUP
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();
    let but = pins.d12.into_pull_up_input();
    let mut serial = arduino_hal::default_serial!(dp, pins, BAUD_RATE);

    //BEGIN
    let mut pulsado = true;
    led.set_low();

    loop {
        if but.is_high() && !pulsado {
            led.set_low();
            pulsado = true;
            _ = ufmt::uwriteln!(&mut serial, "APAGAMOS EL LED");
        }
        if but.is_low() && pulsado {
            led.set_high();
            pulsado = false;
            _ = ufmt::uwriteln!(&mut serial, "ENCENDEMOS EL LED");
        }

        arduino_hal::delay_ms(50);
    }
}
