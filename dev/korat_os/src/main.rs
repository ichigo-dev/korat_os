/*

    Korat OS

    ----------------------------------------------------------------------------

    A simple OS written in Rust.

*/

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(korat_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use korat_os::println;
use core::panic::PanicInfo;

//------------------------------------------------------------------------------
//  The entry point function.
//
//  Linker looks for a function named `_start` by default.
//------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn _start() -> !
{
    println!("Hello, world");

    #[cfg(test)]
    test_main();

    loop {}
}

//------------------------------------------------------------------------------
//  The function is called on panic.
//------------------------------------------------------------------------------
#[cfg(not(test))]
#[panic_handler]
fn panic( info: &PanicInfo ) -> !
{
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic( info: &PanicInfo ) -> !
{
    korat_os::test_panic_handler(info);
}
