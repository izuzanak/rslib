#![allow(dead_code)]

extern crate libc;
#[macro_use] extern crate err;

static SYSTEM_MISSING_PROGRAM_NAME:&str = "missing name of program to execute";
static SYSTEM_CANNOT_CREATE_NEW_PROCESS:&str = "cannot create new process";
static CLOCK_CANNOT_GET_TIME:&str = "cannot get clock time";

fn execute(a_args:Vec<&str>) -> Result<libc::pid_t,err::Error>
{//{{{
    if a_args.is_empty() {
        return err!(SYSTEM_MISSING_PROGRAM_NAME);
    }

    for arg in &a_args {
        if arg.as_bytes().last() != Some(&0u8) {
            return err!(err::CSTRING_MISSING_TERMINATING_ZERO);
        }
    }

    unsafe {
        let pid = libc::fork();
        if pid == -1 {
            return err!(SYSTEM_CANNOT_CREATE_NEW_PROCESS);
        }

        // - process is child -
        if pid == 0 {
            let mut arguments:Vec<*const i8> = vec![];

            // - initialize argument list -
            for arg in &a_args {
                arguments.push(arg.as_ptr() as *const i8);
            }

            // - add terminating argument -
            arguments.push(0 as *const i8);

            // - execute target process -
            if libc::execvp(a_args[0].as_ptr() as *const i8,arguments.as_ptr()) == -1 {
                libc::exit(0);
            }
        }

        Ok(pid)
    }
}//}}}

struct Clock {}

impl Clock {
    fn gettime() -> Result<i64,err::Error>
    {//{{{
        unsafe {
            let mut tp = std::mem::MaybeUninit::<libc::timespec>::uninit();

            if libc::clock_gettime(libc::CLOCK_MONOTONIC,tp.as_mut_ptr()) != 0 {
                return err!(CLOCK_CANNOT_GET_TIME);
            }

            let tp = tp.assume_init();

            Ok(tp.tv_sec as i64 *1000000000i64 + tp.tv_nsec as i64)
        }
    }//}}}
}

#[cfg(test)]
mod tests {
use super::*;

static ERROR_TEST_FAILED:&str = "Test failed";

#[test]
fn execute_t0()
{//{{{
    let pid = execute(vec!["ls\0","-l\0"]).unwrap();

    unsafe {
        let mut status = 0;
        assert_eq!(pid,libc::waitpid(pid,&mut status as *mut libc::c_int,0));
    }
}//}}}

#[test]
fn gettime_t0()
{//{{{
    println!("gettime: {}",Clock::gettime().unwrap());
}//}}}

}

