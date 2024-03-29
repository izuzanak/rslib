#![allow(dead_code)]

pub const CSTRING_MISSING_TERMINATING_ZERO:&str = "missing terminating zero of C string";
pub const TEST_FAILED:&str = "test failed";

#[derive(Debug)]
pub struct Error {
    pub descr:&'static str
}

#[macro_export]
macro_rules! err {
    ($descr:expr) => {{
        println!("ERROR: {}: in {} +{}",$descr,file!(),line!());
        Err($crate::Error{descr:$descr})
    }}
}

#[macro_export]
macro_rules! test_err {
    ($descr:expr) => {
        Err($crate::Error{descr:$descr})
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str
    {//{{{
        self.descr
    }//}}}
}

impl PartialEq for Error {
    fn eq(&self,other:&Self) -> bool 
    {//{{{
        self.descr.eq(other.descr)
    }//}}}
}

impl std::fmt::Display for Error {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result
    {//{{{
        std::fmt::Debug::fmt(self,f)
    }//}}}
}

#[cfg(test)]
mod tests {
use super::*;

fn error_test() -> Result<(),Error>
{//{{{
    return err!("error 0");
}//}}}

#[test]
fn dummy_0()
{//{{{
    match error_test() {
        Err(Error{descr:"error 0"}) => {},
        _ => panic!(err::TEST_FAILED)
    }
}//}}}

}

