#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::sync::atomic::{AtomicBool, Ordering};

use arduino_hal::{hal::wdt::WdtOps, pac::tc1};
use panic_halt as _;

static INT: AtomicBool = AtomicBool::new(false);
const order: Ordering = Ordering::SeqCst;

#[avr_device::interrupt(atmega328p)]
fn INT0() {
    INT.store(true, order);
}

#[arduino_hal::entry]
unsafe fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    dp.EXINT.eicra.modify(|_, w| w.isc0().bits(0x01));
    dp.EXINT.eimsk.modify(|_, w| w.int0().set_bit());

    avr_device::interrupt::enable();
    pins.d2.into_pull_up_input();

    let mut led = pins.d13.into_output();
    led.set_low();
    _ = ufmt::uwriteln!(&mut serial, "Ready ...");

    loop {
        if INT.load(order) {
            _ = ufmt::uwriteln!(&mut serial, "Interrupcion");
            INT.store(false, order);
            led.toggle();
        }
    }
}
