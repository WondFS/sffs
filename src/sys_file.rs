//
// File-system system calls.
//

use std::sync::Arc;
use crate::fake_proc;
use crate::common::file_table;

// fetch system call argument as a file descriptor
// and return both the descriptor and the corresoinding struct file.
pub fn arg_fd() -> Option<(u32, file_table::FileLink)> {
    let fd = 0;
    let proc = fake_proc::my_proc();
    let mut file = None;
    if fd < 0 || fd >= proc.max_file || proc.file[fd as usize].is_none()  {
        return None;
    }
    file = Some(Arc::clone(&proc.file[fd as usize].as_ref().unwrap()));
    Some((fd as u32, file.unwrap()))
}

// Allocate a file descriptor for the given file.
pub fn fdalloc(file: file_table::FileLink) -> i32 {
    let mut proc = fake_proc::my_proc();
    for fd in 0..proc.max_file {
        if proc.file[fd as usize].is_none() {
            proc.file[fd as usize] = Some(file);
            return fd as i32;
        }
    }
    -1
}

pub fn sys_dup() -> i32 {
    let mut proc = fake_proc::my_proc();
    let (_, file) = arg_fd().unwrap();
    let fd = fdalloc(file);
    if fd < 0 {
        return -1;
    }
    proc.file_table.file_dup(&proc.file[fd as usize].as_ref().unwrap());
    fd
}

pub fn sys_read() -> i32 {
    let (_, file) = arg_fd().unwrap();
    let len = 0;
    let mut buf = vec![];
    // ？ 为啥直接返回函数结果会报错捏
    let ret = file.borrow_mut().file_read(len, &mut buf);
    ret
}

pub fn sys_write() -> i32 {
    let (_, file) = arg_fd().unwrap();
    let len = 0;
    let buf = vec![];
    let ret = file.borrow_mut().file_write(len, &buf);
    ret
}

pub fn sys_close() -> i32 {
    let mut proc = fake_proc::my_proc();
    let (fd, file) = arg_fd().unwrap();
    proc.file[fd as usize] = None;
    proc.file_table.file_close(file);
    0
}

pub fn sys_fstat() -> i32 {
    let (_, file) = arg_fd().unwrap();
    let _ = file.borrow().file_stat().unwrap();
    0
}

// Create the path new as a link to the same inode as old.
pub fn sys_link() {
    
}

// Is the directory dp empty except for "." and ".." ?
pub fn is_dir_empty() {

}

pub fn sys_unlink() {

}

pub fn create() {

}

pub fn sys_open() {

}

pub fn sys_mkdir() {

}

pub fn sys_mknod() {

}

pub fn sys_chdir() {

}

pub fn sys_exec() {

}

pub fn sys_pipe() {

}