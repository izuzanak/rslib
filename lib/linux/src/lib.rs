#![allow(dead_code)]

extern crate libc;
#[macro_use] extern crate err;

const FD_DUP_ERROR:&str = "error while duplicating file descriptor";
const FD_OPEN_ERROR:&str = "error while opening file";
const FD_WRITE_ERROR:&str = "error while writing to file descriptor";
const FD_READ_ERROR:&str = "error while reading from file descriptor";

const READ_BUFFER_ADD:usize = 4096;

use std::os::raw::{c_int,c_ulong};

extern
{//{{{
    fn dup(oldfd:c_int) -> c_int;
    fn open(pathname:*const u8,flags:c_int,mode:libc::mode_t) -> c_int;
    fn close(fd:c_int) -> c_int;
    fn write(fd:c_int,buf:*const u8,count:usize) -> isize;
    fn read(fd:c_int,buf:*mut u8,count:usize) -> isize;
    fn ioctl(fd:c_int,request:c_ulong,...) -> c_int;
}//}}}

pub struct Fd {
    fd:c_int
}

impl Fd {
    pub fn take(fd:c_int) -> Fd {
        Fd{fd:fd}
    }

    pub fn dup(fd:c_int) -> Result<Fd,err::Error>
    {//{{{
        let new_fd;
        unsafe {
            new_fd = dup(fd);
        }
        if new_fd == -1 {
            return err!(FD_DUP_ERROR);
        }

        Ok(Fd{fd:new_fd})
    }//}}}

    pub fn open(pathname:&str,flags:c_int,mode:libc::mode_t) -> Result<Fd,err::Error>
    {//{{{
        if pathname.as_bytes().last() != Some(&0u8) {
            return err!(err::CSTRING_MISSING_TERMINATING_ZERO);
        }

        unsafe {
            let fd = open(pathname.as_ptr(),flags,mode);
            if fd == -1 {
                return err!{FD_OPEN_ERROR}
            }

            Ok(Fd{fd:fd})
        }
    }//}}}

    pub fn write(&self,src:&[u8]) -> Result<(),err::Error>
    {//{{{
        debug_assert!(self.fd != -1);

        if !src.is_empty() {
            let size = src.len() as isize;
            let mut writed:isize = 0;

            while writed < size {
                let read_cnt;
                unsafe {
                    read_cnt = write(self.fd,src.as_ptr().offset(writed),(size - writed) as usize);
                    if read_cnt == -1 {
                        return err!(FD_WRITE_ERROR);
                    }
                }
                writed += read_cnt;
            }
        }

        Ok({})
    }//}}}

    pub fn read(&self,trg:&mut Vec<u8>) -> Result<(),err::Error>
    {//{{{
        debug_assert!(self.fd != -1);

        let mut inq_cnt:c_int = 0;
        loop {
            trg.reserve(READ_BUFFER_ADD);

            unsafe {
                let read_cnt = read(self.fd,trg.as_mut_ptr().offset(trg.len() as isize),READ_BUFFER_ADD);
                if read_cnt == -1 {
                    return err!(FD_READ_ERROR);
                }

                trg.set_len(trg.len() + read_cnt as usize);

                let res = ioctl(self.fd,libc::TIOCINQ,&mut inq_cnt as *mut c_int);
                if res == -1 {
                    return err!(FD_READ_ERROR);
                }
            }

            if inq_cnt <= 0 {
                break;
            }
        }

        Ok({})
    }//}}}
}

impl Drop for Fd {
    fn drop(&mut self)
    {//{{{
        if self.fd != -1 {
            unsafe {
                close(self.fd);
            }
        }
    }//}}}
}

impl std::fmt::Display for Fd {
    fn fmt(&self,f:&mut std::fmt::Formatter) -> std::fmt::Result
    {//{{{
        write!(f,"Fd({})",self.fd)
    }//}}}
}

const TEST_FAILED:&str = "Test failed";

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn open_t0()
{//{{{
    let fd = Fd::open("test.txt\0",libc::O_CREAT | libc::O_WRONLY,0o666).unwrap();

    let mut idx = 0;
    while idx < 10 {
        fd.write(format!("Hello there, idx: {}\n",idx).as_str().as_bytes()).unwrap();
        idx += 1;
    }

    let fd1 = Fd::open("test.txt\0",libc::O_RDONLY,0).unwrap();

    let mut buffer:Vec<u8> = vec![];
    fd1.read(&mut buffer).unwrap();
    assert_eq!(buffer,
"Hello there, idx: 0
Hello there, idx: 1
Hello there, idx: 2
Hello there, idx: 3
Hello there, idx: 4
Hello there, idx: 5
Hello there, idx: 6
Hello there, idx: 7
Hello there, idx: 8
Hello there, idx: 9
".as_bytes());
}//}}}

}

