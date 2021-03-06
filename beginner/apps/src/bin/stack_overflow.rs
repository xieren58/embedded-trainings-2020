#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_log as _; // panic handler

#[entry]
fn main() -> ! {
    // board initialization
    dk::init().unwrap();

    log::info!("fib(100) = {:?}", fib(100));

    loop {
        asm::bkpt();
    }
}

#[inline(never)]
fn fib(n: u32) -> u32 {
    // allocate and initialize one kilobyte of stack memory to provoke stack overflow
    let _use_stack = [0xAA; 1024];

    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2) // recursion
    }
}
