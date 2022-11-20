#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use korat_os::{ exit_qemu, QemuExitCode, serial_print, serial_println };

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{ InterruptDescriptorTable, InterruptStackFrame };

lazy_static!
{
    static ref TEST_IDT: InterruptDescriptorTable =
    {
        let mut idt = InterruptDescriptorTable::new();
        unsafe
        {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(korat_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

#[no_mangle]
pub extern "C" fn _start() -> !
{
    serial_print!("stack_overflow::stack_overflow...\t");

    korat_os::gdt::init_gdt();
    init_test_idt();

    stack_overflow();

    panic!("Exception continued after stack overflow");
}

extern "x86-interrupt" fn test_double_fault_handler
(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> !
{
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[allow(unconditional_recursion)]
fn stack_overflow()
{
    stack_overflow();
    volatile::Volatile::new(0).read();
}

#[panic_handler]
fn panic( info: &PanicInfo ) -> !
{
    korat_os::test_panic_handler(info)
}

pub fn init_test_idt()
{
    TEST_IDT.load();
}
