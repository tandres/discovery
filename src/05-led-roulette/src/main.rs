#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let time_slice = 50_u16;

    // infinite loop; just so we don't leave this stack frame
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().ok();
            delay.delay_ms(time_slice);
            leds[curr].off().ok();
            delay.delay_ms(time_slice);
        }
    }
}
