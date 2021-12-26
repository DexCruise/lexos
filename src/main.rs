#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![no_std]
#![no_main]

use lexos::{println, test_runner};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    lexos::init();

    x86_64::instructions::interrupts::int3();

    // page fault


    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

