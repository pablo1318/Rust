#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    
    
    
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();
    let but = pins.d12.into_pull_up_input();
    let mut pulsado  = false;
    led.set_low();
    loop {
        if but.is_high() && !pulsado{
            led.toggle();
            pulsado = true;
        }
        if but.is_low() && pulsado{
            pulsado = false;
        }
       
        arduino_hal::delay_ms(50);
    }
}
