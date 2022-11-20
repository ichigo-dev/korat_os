/*

    GDT: Global Descriptor Table

    ----------------------------------------------------------------------------

    In computing, a task is a unit of execution or a unit of work.

    The task state segment (TSS) is a structure on x86-based computers which 
    holds information about a task.

    The Global Descriptor Table (GDT) is a binary data structure specific to 
    the x86-64 architectures. It contains entries telling the CPU about memory 
    segments. A similar Interrupt Descriptor Table exists containing task and 
    interrupt descriptors.

*/

use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{
    GlobalDescriptorTable,
    Descriptor,
    SegmentSelector,
};
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static!
{
    static ref TSS: TaskStateSegment =
    {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] =
        {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static!
{
    static ref GDT: (GlobalDescriptorTable, Selectors) =
    {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, data_selector, tss_selector })
    };
}

struct Selectors
{
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init_gdt()
{
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::Segment;
    use x86_64::registers::segmentation::{ CS, SS };

    GDT.0.load();
    unsafe
    {
        CS::set_reg(GDT.1.code_selector);
        SS::set_reg(GDT.1.data_selector);
        load_tss(GDT.1.tss_selector);
    }
}
