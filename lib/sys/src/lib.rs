#![allow(dead_code)]

extern crate libc;
#[macro_use] extern crate err;

static CLOCK_CANNOT_GET_TIME:&str = "cannot get clock time";

struct Clock {}

impl Clock {
    fn gettime() -> Result<i64,err::Error>
    {//{{{
        unsafe {
            let mut tp:libc::timespec = std::mem::uninitialized();

            if libc::clock_gettime(libc::CLOCK_MONOTONIC,&mut tp as *mut _) != 0 {
                return err!(CLOCK_CANNOT_GET_TIME);
            }

            Ok(tp.tv_sec as i64 *1000000000i64 + tp.tv_nsec as i64)
        }
    }//}}}
}

#[cfg(test)]
mod tests {
use super::*;

static ERROR_TEST_FAILED:&str = "Test failed";

#[test]
fn gettime_t0()
{//{{{
    println!("gettime: {}",Clock::gettime().unwrap());
}//}}}

}

