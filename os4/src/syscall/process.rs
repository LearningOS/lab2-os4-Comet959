//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::mm::{PhysPageNum, VirtAddr, VirtPageNum, PageTable, PhysAddr};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, set_task_info, get_mmap, get_munmap};
use crate::timer::{get_time_us, get_time};
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    // 1. 找到虚拟地址_ts对应的物理地址address
    let virt_addr: VirtAddr = VirtAddr::from(_ts as usize);
    let virt_page_offset: usize = virt_addr.page_offset();
    let virt_page_nr: VirtPageNum = virt_addr.floor();
    let page_physical_nr = PageTable::from_token(current_user_token()).translate(virt_page_nr).map(|entry| entry.ppn()).unwrap();
    let ph_page_addr: PhysAddr = PhysAddr::getPhaddrByOffset(page_physical_nr, virt_page_offset);

    // 2. 把这个物理地址强制转成TimeVal类型数据(ts = address as * mut TimeVal) 设为ts
    let _us = get_time_us();
    let ts = ph_page_addr.0 as *mut TimeVal;

    // 3. 将ts赋值成_us  PS: 此时_ts是数据虚拟地址(应用地址空间), ts是该数据的物理地址
    unsafe {
        *ts = TimeVal {
            sec: _us / 1_000_000,
            usec: _us % 1_000_000,
        };
    }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    get_mmap(_start, _len, _port)
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    get_munmap(_start, _len)
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    // 1. 找到虚拟地址ti对应的物理地址address
    let virt_addr: VirtAddr = VirtAddr::from(ti as usize);
    let virt_page_offset: usize = virt_addr.page_offset();
    let virt_page_nr: VirtPageNum = virt_addr.floor();
    let page_physical_nr = PageTable::from_token(current_user_token()).translate(virt_page_nr).map(|entry| entry.ppn()).unwrap();
    let ph_page_addr: PhysAddr = PhysAddr::getPhaddrByOffset(page_physical_nr, virt_page_offset); // 物理地址

    // 2. 把这个物理地址强制转成TimeVal类型数据(ts = address as * mut TimeVal) 设为ti2
    let ti2 = ph_page_addr.0 as *mut TaskInfo;

    // 3. 设置信息
    set_task_info(ti2);
    0
}
