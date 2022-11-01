//! The global allocator

use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;

#[global_allocator]
/// heap allocator instance
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty(); // 实例化一个全局动态内存分配器. 系统(使用者 alloc库)将利用它来进行动态内存的管理. LockedHeap是一个被Mutex保护的资源, 也就是说, 只能被一个线程访问, 任何线程在访问前, 需要先获取它的锁, 以避免数据竞争.

#[alloc_error_handler]
/// panic when heap allocation error occurs
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

/// heap space ([u8; KERNEL_HEAP_SIZE])
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE]; // 堆的定义, 是一个[u8; KERNEL_HEAP_SIZE]静态可变全局数组, 长度为0x30_0000, 类型是u8. 位于内核的bss段

/// initiate heap allocator
pub fn init_heap() { // 初始化全局动态内存分配器
    unsafe {
        HEAP_ALLOCATOR
            .lock() // 取锁.
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE); // 使用init方法, 通知HEAP_ALLOCATOR: 现在可以有HEAP_SPACE的属性(起始地址和大小)可以被分配.
    }
}

#[allow(unused)]
pub fn heap_test() {
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let bss_range = sbss as usize..ebss as usize;
    let a = Box::new(5);
    assert_eq!(*a, 5);
    assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
    drop(a);
    let mut v: Vec<usize> = Vec::new();
    for i in 0..500 {
        v.push(i);
    }
    for (i, vi) in v.iter().enumerate().take(500) {
        assert_eq!(*vi, i);
    }
    assert!(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);
    info!("heap_test passed!");
}
