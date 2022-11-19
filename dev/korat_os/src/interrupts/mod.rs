/*

    Interrupts

    ----------------------------------------------------------------------------

                         _____________             ______
    Timer ------------> |            |            |     |
    Keyboard ---------> | Interrupt  | ---------> | CPU |
    Other Hardware ---> | Controller |            |_____|
    Etc. -------------> |____________|


    # Exceptions

    The Interrupt Descriptor Table (IDT) is a data structure used by the x86 
    architecture to implement an interrupt vector table. 

    | Number | Description                   |
    | ------ | ----------------------------- |
    | 0x00   | Division by zero              |
    | 0x01   | Single-step interrupt         |
    | 0x02   | NMI                           |
    | 0x03   | Breakpoint                    |
    | 0x04   | Overflow                      |
    | 0x05   | Bound Range Exceeded          |
    | 0x06   | Invalid Opcode                |
    | 0x07   | Coprocessor not available     |
    | 0x08   | Double Fault                  |
    | 0x09   | Coprocessor Segment Overrun   |
    | 0x0A   | Invalid Task State Segment    |
    | 0x0B   | Segment not present           |
    | 0x0C   | Stack Segment Fault           |
    | 0x0D   | General Protection Fault      |
    | 0x0E   | Page Fault                    |
    | 0x0F   | reserved                      |
    | 0x10   | X87 Floating Point Exception  |
    | 0x11   | Alignment Check               |
    | 0x12   | Machine Check                 |
    | 0x13   | SIMD Floating-Point Exception |
    | 0x14   | Virtualization Exception      |
    | 0x15   | Control Protection Exception  |

    - [Interrupt Descriptor Table(wikipedia)](https://en.wikipedia.org/wiki/Interrupt_descriptor_table)

*/

use crate::{ print, println };
use crate::gdt;

use lazy_static::lazy_static;
use x86_64::structures::idt::{ InterruptDescriptorTable, InterruptStackFrame };
use pic8259::ChainedPics;
use spin;

lazy_static!
{
    static ref IDT: InterruptDescriptorTable =
    {
        let mut idt = InterruptDescriptorTable::new();

        //  Hook handler functions
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe
        {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);

        idt
    };
}

pub fn init_idt()
{
    IDT.load();
}

//------------------------------------------------------------------------------
//  8259 PIC
//                        _____________                         _____________
//  Real Time Clock ---> |            |   Timer -------------> |            |
//  ACPI --------------> |            |   Keyboard ----------> |            |      ______
//  Available ---------> | Secondary  |----------------------> | Primary    |     |     |
//  Available ---------> | Interrupt  |   Serial Port 2 -----> | Interrupt  |---> | CPU |
//  Mouse -------------> | Controller |   Serial Port 1 -----> | Controller |     |_____|
//  Co-Processor ------> |            |   Parallel Port 2/3 -> |            |
//  Primary ATA -------> |            |   Floppy disk -------> |            |
//  Secondary ATA -----> |____________|   Parallel Port 1 ---> |____________|
//------------------------------------------------------------------------------
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

//------------------------------------------------------------------------------
//  Various interrupt processing.
//------------------------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex
{
    Timer = PIC_1_OFFSET,
}

impl InterruptIndex
{
    fn as_u8( self ) -> u8
    {
        self as u8
    }

    fn as_usize( self ) -> usize
    {
        usize::from(self.as_u8())
    }
}

//------------------------------------------------------------------------------
//  A exception breakpoint is executed by suspending the program when the 
//  breakpoint instruction `int3` is executed.
//------------------------------------------------------------------------------
extern "x86-interrupt" fn breakpoint_handler( stack_frame: InterruptStackFrame )
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

//------------------------------------------------------------------------------
//  A double-fault exception is executed when the CPU fails to call an 
//  exception handler. If the call to the double-fault exception fails, a more 
//  fatal triple fault exception is raised and attempts to reset the system.
//------------------------------------------------------------------------------
extern "x86-interrupt" fn double_fault_handler
(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT(code: {})\n{:#?}", error_code, stack_frame);
}

//------------------------------------------------------------------------------
//  A timer interrupt hander.
//------------------------------------------------------------------------------
extern "x86-interrupt" fn timer_interrupt_handler
(
    _stack_frame: InterruptStackFrame
)
{
    print!(".");

    unsafe
    {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

//------------------------------------------------------------------------------
//  tests
//------------------------------------------------------------------------------
#[test_case]
fn test_breakpoint_exception()
{
    x86_64::instructions::interrupts::int3();
}
