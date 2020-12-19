#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myx_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    myx_os::init();

    println!("Hello World{}", "!");
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    myx_os::test_panic_handler(info)
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1 + 1, 2);
}
