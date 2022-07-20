#![no_std]
#![no_main]

use embedded_hal::Capture;
use panic_halt as _;
use ufmt;
const BAUD_RATE: u32 = 57200;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, BAUD_RATE);

    //Pines para la derecha e izquierda del encoder
    let pina = pins.d2.into_pull_up_input();
    let pinb = pins.d3.into_pull_up_input();
    let pinc = pins.d4.into_pull_up_input();

    let mut pulsado = false;

    let mut contador = 0;
    let mut pulsaciones = 0;
    let mut estado_pin = false;

    _ = ufmt::uwriteln!(&mut serial, "Contador: {}", contador);
    _ = ufmt::uwriteln!(&mut serial, "Pulsaciones: {}", pulsaciones);
    let mut ant_estado_pin = pina.is_low();
    loop {
        estado_pin = pina.is_high();

        if ant_estado_pin != estado_pin {
            if estado_pin == pinb.is_low() {
                contador += 1;
            } else {
                contador -= 1;
            }

            _ = ufmt::uwriteln!(&mut serial, "Contador: {}", contador);
            _ = ufmt::uwriteln!(&mut serial, "Pulsaciones: {}", pulsaciones);
        }

        //La parte del pulsador del boton
        if pinc.is_low() && !pulsado {
            pulsaciones += 1;
            pulsado = true;
            _ = ufmt::uwriteln!(&mut serial, "Contador: {}", contador);
            _ = ufmt::uwriteln!(&mut serial, "Pulsaciones: {}", pulsaciones);
        }
        if pinc.is_high() && pulsado {
            pulsado = false;
        }
        ant_estado_pin = estado_pin;
        arduino_hal::delay_ms(50); //Esperamos 10ms para encuestar
    }
}
