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

use alloc::vec;
use alloc::vec::Vec;
use alloc::rc::Rc;
use alloc::boxed::Box;
use core::panic::PanicInfo;
use bootloader::{ BootInfo, entry_point };

extern crate alloc;

entry_point!(kernel_main);

//------------------------------------------------------------------------------
//  The entry point function.
//
//  Linker looks for a function named `_start` by default.
//------------------------------------------------------------------------------
fn kernel_main( boot_info: &'static BootInfo ) -> !
{
    use korat_os::allocator;
    use korat_os::memory::{ self, BootInfoFrameAllocator };
    use x86_64::VirtAddr;
    use x86_64::structures::paging::Page;

    println!("Hello, world");
    korat_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe
    {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    //  Map an unused page.
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    //  Write the string `New!` to the screen through the new mapping.
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500
    {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    korat_os::hlt_loop();
}

//------------------------------------------------------------------------------
//  The function is called on panic.
//------------------------------------------------------------------------------
#[cfg(not(test))]
#[panic_handler]
fn panic( info: &PanicInfo ) -> !
{
    println!("{}", info);
    korat_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic( info: &PanicInfo ) -> !
{
    korat_os::test_panic_handler(info);
}
