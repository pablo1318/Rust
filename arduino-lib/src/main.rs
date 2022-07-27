#![no_std]
#![no_main]

use init;
use panic_halt as _;

#[arduino_hal::entry]
unsafe fn main() -> ! {
    init::init();

    loop {
        //led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
