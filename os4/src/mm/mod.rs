//! Memory management implementation
//! 
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//! 
//! Every task or process has a memory_set to control its virtual memory.


mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
pub use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{translated_byte_buffer, PageTableEntry};
pub use page_table::{PTEFlags, PageTable};


/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    // 动态内存分配器初始化 HEAP_ALLOCATOR  : 用于初始化Rust的堆数据结构
    heap_allocator::init_heap();

    // 物理页帧管理器初始化 FRAME_ALLOCATOR : 物理页帧的分配与回收能力
    frame_allocator::init_frame_allocator();
    
    // 创建内核地址空间    KERNEL_SPACE    : CPU开启分页模式, MMU使用内核的多级页表
    KERNEL_SPACE.lock().activate();
}
