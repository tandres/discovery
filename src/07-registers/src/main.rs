#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln, ITM, RegisterBlock};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    gpioe.bsrr.write(|w| w.bs9().set_bit());
    gpioe.bsrr.write(|w| w.bs11().set_bit());
    gpioe.bsrr.write(|w| w.br9().set_bit());
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}
