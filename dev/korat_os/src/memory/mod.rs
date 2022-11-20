/*

    Memory

    ----------------------------------------------------------------------------

    The OS uses hardware functions to prevent other processes from interfering 
    with the memory area of a process.

    Virtual memory is to allocate part of physical memory to a virtual address 
    as working memory dedicated to a certain program. This allows the memory 
    used by a program to be placed anywhere on the physical memory, making it 
    easier to manage multiple programs.

    The physical memory of multiple programs may not be next to each other, and 
    a small amount of freed physical memory may not be available to the next 
    program. This is called fragmentation. To avoid this, the OS needs to 
    manage the memory used by programs.

    Segmentation manages programs and data in units of variable size called 
    segments or sections. This is the process of stopping programs execution 
    and rearranging segments in memory so that in memory so that they are 
    adjacent to each other.

    Paging divides virtual and physical memory into small fixed-size blocks. 
    This allows physical memory to be used without fragmentation. Paging uses 
    a page table to map virtual memory to physical memory.

*/

use bootloader::bootinfo::{ MemoryMap, MemoryRegionType };
use x86_64::{ VirtAddr, PhysAddr };
use x86_64::structures::paging::{
    Page,
    PageTable,
    Mapper,
    Size4KiB,
    FrameAllocator,
    OffsetPageTable,
    PhysFrame,
};

//------------------------------------------------------------------------------
//  Initialize a new OffsetPageTable.
//
//  This function is unsafe: the caller must ensure that all physical memory is 
//  mapped into virtual memory offset by the passed `physical_memory_offset`. 
//  Also, this function should only be called once, as it would leads to `&mut` 
//  reference having multiple names.
//------------------------------------------------------------------------------
pub unsafe fn init( physical_memory_offset: VirtAddr )
    -> OffsetPageTable<'static>
{
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

//------------------------------------------------------------------------------
//  Returns a mutable reference to the active level 4 table.
//
//  This function is unsafe: the caller must ensure that all physical memory is 
//  mapped into virtual memory offset by the passed `physical_memory_offset`. 
//  Also, this function should only be called once, as it would leads to `&mut` 
//  reference having multiple names.
//------------------------------------------------------------------------------
unsafe fn active_level_4_table( physical_memory_offset: VirtAddr )
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

//------------------------------------------------------------------------------
//  Creates an example mapping for the given page to frame `0xb8000`.
//------------------------------------------------------------------------------
pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
)
{
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe
    {
        mapper.map_to(page, frame, flags, frame_allocator)
    };

    map_to_result.expect("map_to failed").flush();
}

//------------------------------------------------------------------------------
//  A FrameAllocator that always returns `None`.
//------------------------------------------------------------------------------
pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator
{
    //--------------------------------------------------------------------------
    //  allocate_frame
    //--------------------------------------------------------------------------
    fn allocate_frame( &mut self ) -> Option<PhysFrame>
    {
        None
    }
}

//------------------------------------------------------------------------------
//  A FrameAllocator that returns usable from the bootloader's memory map.
//------------------------------------------------------------------------------
pub struct BootInfoFrameAllocator
{
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator
{
    //--------------------------------------------------------------------------
    //  Create a FrameAllocator from the passed  memory map.
    //
    //  This function is unsafe because the caller must guarantee that the 
    //  passed memory map is valid. The main requirement is that all frames 
    //  that are marked as `USABLE` in it are really unused.
    //--------------------------------------------------------------------------
    pub unsafe fn init( memory_map: &'static MemoryMap )
        -> BootInfoFrameAllocator
    {
        BootInfoFrameAllocator
        {
            memory_map,
            next: 0,
        }
    }

    //--------------------------------------------------------------------------
    //  Returns an iterator over the usable frames specified inthe memory map.
    //--------------------------------------------------------------------------
    fn usable_frames( &self ) -> impl Iterator<Item = PhysFrame>
    {
        //  Get usable regions from memory map.
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r|
            r.region_type == MemoryRegionType::Usable
        );

        //  Map each region to its address range.
        let addr_ranges = usable_regions.map(|r|
            r.range.start_addr()..r.range.end_addr()
        );

        //  Transform to an iterator of frame start addresses.
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

        //  Create `PhysFrame` types from the start addresses.
        frame_addresses.map(|addr|
            PhysFrame::containing_address(PhysAddr::new(addr))
        )
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator
{
    //--------------------------------------------------------------------------
    //  allocate_frame
    //--------------------------------------------------------------------------
    fn allocate_frame( &mut self ) -> Option<PhysFrame>
    {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
