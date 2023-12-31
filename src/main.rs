#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use rp_pico as bsp;

mod exception;
mod systimer;

pub static mut COUNTER: u32 = 0;

#[entry]
fn main() -> ! {
    info!("Program start");
    println!("*** hoge");

    let mut st = systimer::SystemTimer::new();
    st.init();

    loop {
        info!("1");
        print_unsafe_counter();
        st.delay_ms(500);

        info!("2");
        print_unsafe_counter();
        st.delay_ms(500);
    }
}

pub fn print_unsafe_counter() {
    unsafe {
        info!("unsafe counter: {}", crate::COUNTER);
    }
}
