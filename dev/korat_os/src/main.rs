/*

    Korat OS

*/

#![no_std]
#![no_main]

mod vga_buffer;

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
    loop {}
}

//------------------------------------------------------------------------------
//  The function is called on panic.
//------------------------------------------------------------------------------
#[panic_handler]
fn panic( info: &PanicInfo ) -> !
{
    println!("{}", info);
    loop {}
}
