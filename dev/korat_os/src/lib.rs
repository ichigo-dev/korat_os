/*

    Korat OS

    ----------------------------------------------------------------------------

    A simple OS written in Rust. It contains a framework for running tests.

*/

#![no_std]
#![cfg_attr(test, no_main)]

#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
use bootloader::{ BootInfo, entry_point };

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;

#[cfg(test)]
entry_point!(test_kernel_main);

//------------------------------------------------------------------------------
//  Runs tests.
//------------------------------------------------------------------------------
pub fn test_runner( tests: &[&dyn Testable] )
{
    serial_println!("\nRunning {} tests", tests.len());
    for test in tests
    {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler( info: &PanicInfo ) -> !
{
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

//------------------------------------------------------------------------------
//  A trait for testable types.
//------------------------------------------------------------------------------
pub trait Testable
{
    fn run( &self ) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    //--------------------------------------------------------------------------
    //  Runs a test.
    //--------------------------------------------------------------------------
    fn run( &self )
    {
        //  Prints the function name.
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

//------------------------------------------------------------------------------
//  Quits QEMU automatically.
//------------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode
{
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu( exit_code: QemuExitCode )
{
    use x86_64::instructions::port::Port;

    unsafe
    {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

//------------------------------------------------------------------------------
//  Entry point for `cargo test`.
//------------------------------------------------------------------------------
#[cfg(test)]
fn test_kernel_main( _boot_info: &'static BootInfo ) -> !
{
    init();
    test_main();
    hlt_loop();
}

//------------------------------------------------------------------------------
//  The function is called on panic.
//------------------------------------------------------------------------------
#[cfg(test)]
#[panic_handler]
fn panic( info: &PanicInfo ) -> !
{
    test_panic_handler(info);
}

//------------------------------------------------------------------------------
//  Initialization function.
//------------------------------------------------------------------------------
pub fn init()
{
    gdt::init_gdt();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() }
    x86_64::instructions::interrupts::enable();
}

//------------------------------------------------------------------------------
//  Stop CPU.
//------------------------------------------------------------------------------
pub fn hlt_loop() -> !
{
    loop
    {
        x86_64::instructions::hlt();
    }
}
