#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// System tick - control and status register
const SYST_CSR: u32 = 0xE000E010;

// System tick - reload value register
const SYST_RVR: u32 = 0xE000E014;

// System tick - current value register
const SYST_CVR: u32 = 0xE000E018;

// CPU Frequency (12.5MHz)
const CPU_FREQ: u32 = 12_500_000;

#[entry]
fn main() -> ! {
    hprintln!("Hello World!");

    let sleep_time: u32 = CPU_FREQ; // 1s

    unsafe {
        // set timer value (reload value register)
        *(SYST_RVR as *mut u32) = sleep_time;

        // Clear current value register
        *(SYST_CVR as *mut u32) = 0;

        // enable timer
        *(SYST_CSR as *mut u32) = 0b111;
    }

    loop {
        asm::wfi();
    }
}

#[exception]
fn SysTick() {
    hprintln!("ugh, woke up :(")
}
