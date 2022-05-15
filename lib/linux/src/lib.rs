#![allow(dead_code)]

extern crate libc;
#[macro_use] extern crate err;

const FD_DUP_ERROR:&str = "error while duplicating file descriptor";
const FD_WRITE_ERROR:&str = "error while writing to file descriptor";
const FD_READ_ERROR:&str = "error while reading from file descriptor";

const READ_BUFFER_ADD:usize = 4096;

use std::os::raw::{c_int,c_ulong};

extern
{//{{{
    fn dup(oldfd:c_int) -> c_int;
    fn close(fd:c_int) -> c_int;
    fn write(fd:c_int,buf:*const u8,count:usize) -> isize;
    fn read(fd:c_int,buf:*mut u8,count:usize) -> isize;
    fn ioctl(fd:c_int,request:c_ulong,...) -> c_int;
}//}}}

pub struct Fd {
    fd:c_int
}

impl Fd {
    pub fn take(a_fd:c_int) -> Fd {
        Fd{fd:a_fd}
    }

    pub fn dup(a_fd:c_int) -> Result<Fd,err::Error>
    {//{{{
        let new_fd;
        unsafe {
            new_fd = dup(a_fd);
        }
        if new_fd == -1 {
            return err!(FD_DUP_ERROR);
        }

        Ok(Fd{fd:new_fd})
    }//}}}

    pub fn write(&self,a_src:&[u8]) -> Result<(),err::Error>
    {//{{{
        debug_assert!(self.fd != -1);

        if !a_src.is_empty() {
            let size = a_src.len() as isize;
            let mut writed:isize = 0;

            while writed < size {
                let read_cnt;
                unsafe {
                    read_cnt = write(self.fd,a_src.as_ptr().offset(writed),(size - writed) as usize);
                    if read_cnt == -1 {
                        return err!(FD_WRITE_ERROR);
                    }
                }
                writed += read_cnt;
            }
        }

        Ok({})
    }//}}}

    pub fn read(&self,a_trg:&mut Vec<u8>) -> Result<(),err::Error>
    {//{{{
        debug_assert!(self.fd != -1);

        let mut inq_cnt:c_int = 0;
        loop {
            a_trg.reserve(READ_BUFFER_ADD);

            unsafe {
                let read_cnt = read(self.fd,a_trg.as_mut_ptr().offset(a_trg.len() as isize),READ_BUFFER_ADD);
                if read_cnt == -1 {
                    return err!(FD_READ_ERROR);
                }

                a_trg.set_len(a_trg.len() + read_cnt as usize);

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

const ERROR_TEST_FAILED:&str = "Test failed";

#[cfg(test)]
mod tests {
//use super::*;

#[test]
fn dummy_t0()
{//{{{
}//}}}

}

