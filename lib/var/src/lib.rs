#[macro_use] extern crate err;

static ERROR_INVALID_VALUE_TYPE:&str = "Invalid value type";

#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub enum Data {
    Blank,
    Bool   (bool),
    Int    (i64),
    Float  (f64),
    String (String),
    Array  (Vec<Var>),
    Dict   (std::collections::BTreeMap<Var,Var>),
}

impl Eq for Data {}
impl Ord for Data {
    fn cmp(&self,other:&Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result
    {//{{{
        match *self {
            Data::Blank => write!(f,"<blank>"),
            Data::Bool(value) => write!(f,"{}",value),
            Data::Int(value) => write!(f,"{}",value),
            Data::Float(value) => write!(f,"{}",value),
            Data::String(ref value) => write!(f,"{}",value),
            Data::Array(ref value) => {
                if value.is_empty() { write!(f,"[]") }
                else {
                    let mut first = true;
                    for item in value {
                        try!(write!(f,"{}{}",if first { first = false; '[' } else { ',' },item))
                    }
                    write!(f,"]")
                }
            },
            Data::Dict(ref value) => {
                if value.is_empty() { write!(f,"[]") }
                else {
                    let mut first = true;
                    for (key,item) in value {
                        try!(write!(f,"{}{}:{}",if first { first = false; '[' } else { ',' },key,item))
                    }
                    write!(f,"]")
                }
            }
        }
    }//}}}
}

struct Loc {
    data:Data,
    refs:std::sync::atomic::AtomicUsize
}

impl Loc {
    fn unref(&mut self) -> usize {
        self.refs.fetch_sub(1,std::sync::atomic::Ordering::Relaxed)
    }
}

pub struct Var {
    loc:*mut Loc
}

impl Var {
    pub fn blank() -> Var {
        let loc = Box::new(Loc{data:Data::Blank,refs:std::sync::atomic::AtomicUsize::new(1)});
        Var{loc:Box::into_raw(loc)}
    }
    pub fn bool(a_val:bool) -> Var {
        let loc = Box::new(Loc{data:Data::Bool(a_val),refs:std::sync::atomic::AtomicUsize::new(1)});
        Var{loc:Box::into_raw(loc)}
    }
    pub fn int(a_val:i64) -> Var {
        let loc = Box::new(Loc{data:Data::Int(a_val),refs:std::sync::atomic::AtomicUsize::new(1)});
        Var{loc:Box::into_raw(loc)}
    }
    pub fn float(a_val:f64) -> Var {
        let loc = Box::new(Loc{data:Data::Float(a_val),refs:std::sync::atomic::AtomicUsize::new(1)});
        Var{loc:Box::into_raw(loc)}
    }
    pub fn str(a_val:&str) -> Var {
        let loc = Box::new(Loc{data:Data::String(String::from(a_val)),refs:std::sync::atomic::AtomicUsize::new(1)});
        Var{loc:Box::into_raw(loc)}
    }
    pub fn array(a_val:Vec<Var>) -> Var {
        let loc = Box::new(Loc{data:Data::Array(a_val),refs:std::sync::atomic::AtomicUsize::new(1)});
        Var{loc:Box::into_raw(loc)}
    }
    pub fn dict(a_val:std::collections::BTreeMap<Var,Var>) -> Var {
        let loc = Box::new(Loc{data:Data::Dict(a_val),refs:std::sync::atomic::AtomicUsize::new(1)});
        Var{loc:Box::into_raw(loc)}
    }
    pub fn var(a_val:&Var) -> Var {
        unsafe {
            (*a_val.loc).refs.fetch_add(1,std::sync::atomic::Ordering::Relaxed);
            Var{loc:a_val.loc}
        }
    }
    pub fn data(&self) -> &Data {
        unsafe {
            &(*self.loc).data
        }
    }
    pub fn to_bool(&self) -> Result<bool,err::Error>
    {//{{{
        unsafe {
            match &mut (*self.loc).data {
                &mut Data::Bool(value) => Ok(value),
                &mut Data::Blank     |
                &mut Data::Int(_)    |
                &mut Data::Float(_)  |
                &mut Data::String(_) |
                &mut Data::Array(_)  |
                &mut Data::Dict(_)   => err!(ERROR_INVALID_VALUE_TYPE)
            }
        }
    }//}}}
    pub fn to_int(&self) -> Result<i64,err::Error>
    {//{{{
        unsafe {
            match &mut (*self.loc).data {
                &mut Data::Int(value) => Ok(value),
                &mut Data::Blank     |
                &mut Data::Bool(_)   |
                &mut Data::Float(_)  |
                &mut Data::String(_) |
                &mut Data::Array(_)  |
                &mut Data::Dict(_)   => err!(ERROR_INVALID_VALUE_TYPE)
            }
        }
    }//}}}
    pub fn to_float(&self) -> Result<f64,err::Error>
    {//{{{
        unsafe {
            match &mut (*self.loc).data {
                &mut Data::Float(value) => Ok(value),
                &mut Data::Blank     |
                &mut Data::Bool(_)   |
                &mut Data::Int(_)    |
                &mut Data::String(_) |
                &mut Data::Array(_)  |
                &mut Data::Dict(_)   => err!(ERROR_INVALID_VALUE_TYPE)
            }
        }
    }//}}}
    pub fn to_str(&self) -> Result<&String,err::Error>
    {//{{{
        unsafe {
            match &mut (*self.loc).data {
                &mut Data::String(ref value) => Ok(value),
                &mut Data::Blank     |
                &mut Data::Bool(_)   |
                &mut Data::Int(_)    |
                &mut Data::Float(_)  |
                &mut Data::Array(_)  |
                &mut Data::Dict(_)   => err!(ERROR_INVALID_VALUE_TYPE)
            }
        }
    }//}}}
    pub fn to_array(&self) -> Result<&mut Vec<Var>,err::Error>
    {//{{{
        unsafe {
            match &mut (*self.loc).data {
                &mut Data::Array(ref mut value) => Ok(value),
                &mut Data::Blank     |
                &mut Data::Bool(_)   |
                &mut Data::Int(_)    |
                &mut Data::Float(_)  |
                &mut Data::String(_) |
                &mut Data::Dict(_)   => err!(ERROR_INVALID_VALUE_TYPE)
            }
        }
    }//}}}
    pub fn to_dict(&self) -> Result<&mut std::collections::BTreeMap<Var,Var>,err::Error>
    {//{{{
        unsafe {
            match &mut (*self.loc).data {
                &mut Data::Dict(ref mut value) => Ok(value),
                &mut Data::Blank     |
                &mut Data::Bool(_)   |
                &mut Data::Int(_)    |
                &mut Data::Float(_)  |
                &mut Data::String(_) |
                &mut Data::Array(_)  => err!(ERROR_INVALID_VALUE_TYPE)
            }
        }
    }//}}}
}

impl PartialEq for Var {
    fn eq(&self,other:&Self) -> bool {
        unsafe { (*self.loc).data.eq(&(*other.loc).data) }
    }
}

impl PartialOrd for Var {
    fn partial_cmp(&self,other:&Self) -> Option<std::cmp::Ordering> {
        unsafe { (*self.loc).data.partial_cmp(&(*other.loc).data) }
    }
}

impl Eq for Var {}

impl Ord for Var {
    fn cmp(&self,other:&Self) -> std::cmp::Ordering {
        unsafe { (*self.loc).data.cmp(&(*other.loc).data) }
    }
}

impl Clone for Var {
    fn clone(&self) -> Self {
        unsafe {
            let loc = Box::new(Loc{data:(*self.loc).data.clone(),refs:std::sync::atomic::AtomicUsize::new(1)});
            Var{loc:Box::into_raw(loc)}
        }
    }
}

impl Drop for Var {
    fn drop(&mut self) {
        unsafe { if (*self.loc).unref() <= 1 { drop(Box::from_raw(self.loc)); } }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result
    {//{{{
        unsafe { std::fmt::Display::fmt(&(*self.loc).data,f) }
    }//}}}
}

impl std::fmt::Debug for Var {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe {
            let loc = &*self.loc;
            write!(f,"{:?}#{}",loc.data,loc.refs.load(std::sync::atomic::Ordering::Relaxed))
        }
    }
}

#[macro_export]
macro_rules! var {
    ($($var:tt)+) => {
        var_internal!($($var)+)
    };
}

#[macro_export]
macro_rules! var_internal
{//{{{
    // - done with trailing comma -
    (@array [$($elems:expr,)*]) => { vec![$($elems,)*] };

    // - done without trailing comma -
    (@array [$($elems:expr),*]) => { vec![$($elems),*] };

    (@array [$($elems:expr,)*] blank $($rest:tt)*) => {
        var_internal!(@array [$($elems,)* var_internal!(blank)] $($rest)*)
    };
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        var_internal!(@array [$($elems,)* var_internal!(false)] $($rest)*)
    };
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        var_internal!(@array [$($elems,)* var_internal!(true)] $($rest)*)
    };
    (@array [$($elems:expr,)*] i($value:expr) $($rest:tt)*) => {
        var_internal!(@array [$($elems,)* var_internal!(i($value))] $($rest)*)
    };
    (@array [$($elems:expr,)*] f($value:expr) $($rest:tt)*) => {
        var_internal!(@array [$($elems,)* var_internal!(f($value))] $($rest)*)
    };
    (@array [$($elems:expr,)*] s($value:expr) $($rest:tt)*) => {
        var_internal!(@array [$($elems,)* var_internal!(s($value))] $($rest)*)
    };
    (@array [$($elems:expr,)*] v($value:expr) $($rest:tt)*) => {
        var_internal!(@array [$($elems,)* var_internal!(v($value))] $($rest)*)
    };
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        var_internal!(@array [$($elems,)* var_internal!([$($array)*])] $($rest)*)
    };
    (@array [$($elems:expr,)*] {$($dict:tt)*} $($rest:tt)*) => {
        var_internal!(@array [$($elems,)* var_internal!({$($dict)*})] $($rest)*)
    };

    // - comma after the most recent element -
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        var_internal!(@array [$($elems,)*] $($rest)*)
    };

    // - done -
    (@dict $object:ident () () ()) => {};

    // - insert the current entry followed by trailing comma -
    (@dict $object:ident [$key:expr] ($value:expr) , $($rest:tt)*) => {
        $object.insert($key,$value);
        var_internal!(@dict $object () ($($rest)*) ($($rest)*));
    };

    // - insert the last entry without trailing comma -
    (@dict $object:ident [$key:expr] ($value:expr)) => {
        $object.insert($key,$value);
    };

    // - process values -
    (@dict $object:ident ($($key:tt)+) (: blank $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object [$($key)+] (var_internal!(blank)) $($rest)*);
    };
    (@dict $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object [$($key)+] (var_internal!(true)) $($rest)*);
    };
    (@dict $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object [$($key)+] (var_internal!(false)) $($rest)*);
    };
    (@dict $object:ident ($($key:tt)+) (: i($value:expr) $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object [$($key)+] (var_internal!(i($value))) $($rest)*);
    };
    (@dict $object:ident ($($key:tt)+) (: f($value:expr) $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object [$($key)+] (var_internal!(f($value))) $($rest)*);
    };
    (@dict $object:ident ($($key:tt)+) (: s($value:expr) $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object [$($key)+] (var_internal!(s($value))) $($rest)*);
    };
    (@dict $object:ident ($($key:tt)+) (: v($value:expr) $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object [$($key)+] (var_internal!(v($value))) $($rest)*);
    };
    (@dict $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object [$($key)+] (var_internal!([$($array)*])) $($rest)*);
    };
    (@dict $object:ident ($($key:tt)+) (: {$($dict:tt)*} $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object [$($key)+] (var_internal!({$($dict)*})) $($rest)*);
    };

    // - next value is an expression followed by comma -
    (@dict $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object [$($key)+] (var_internal!($value)) , $($rest)*);
    };

    // - last value is an expression with no trailing comma -
    (@dict $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        var_internal!(@dict $object [$($key)+] (var_internal!($value)));
    };

    // - key is fully parenthesized -
    (@dict $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // - process keys -
    (@dict $object:ident ($($key:tt)*) (blank $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object ($($key)* var_internal!(blank)) ($($rest)*) ($($rest)*));
    };
    (@dict $object:ident ($($key:tt)*) (false $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object ($($key)* var_internal!(false)) ($($rest)*) ($($rest)*));
    };
    (@dict $object:ident ($($key:tt)*) (true $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object ($($key)* var_internal!(true)) ($($rest)*) ($($rest)*));
    };
    (@dict $object:ident ($($key:tt)*) (i($value:expr) $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object ($($key)* var_internal!(i($value))) ($($rest)*) ($($rest)*));
    };
    (@dict $object:ident ($($key:tt)*) (f($value:expr) $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object ($($key)* var_internal!(f($value))) ($($rest)*) ($($rest)*));
    };
    (@dict $object:ident ($($key:tt)*) (s($value:expr) $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object ($($key)* var_internal!(s($value))) ($($rest)*) ($($rest)*));
    };
    (@dict $object:ident ($($key:tt)*) (v($value:expr) $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object ($($key)* var_internal!(v($value))) ($($rest)*) ($($rest)*));
    };
    (@dict $object:ident ($($key:tt)*) ([$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object ($($key)* var_internal!([$($array)*])) ($($rest)*) ($($rest)*));
    };
    (@dict $object:ident ($($key:tt)*) ({$($dict:tt)*} $($rest:tt)*) $copy:tt) => {
        var_internal!(@dict $object ($($key)* var_internal!({$($dict)*})) ($($rest)*) ($($rest)*));
    };

    (blank) => { $crate::Var::blank() };
    (false) => { $crate::Var::bool(false) };
    (true) => { $crate::Var::bool(true) };
    (i($value:expr)) => { $crate::Var::int($value) };
    (f($value:expr)) => { $crate::Var::float($value) };
    (s($value:expr)) => { $crate::Var::str($value) };
    (v($value:expr)) => { $crate::Var::var(&$value) };
    ([]) => { $crate::Var::array(vec![]) };
    ({}) => { $crate::Var::dict(std::collections::BTreeMap::new()) };
    ([ $($tt:tt)+ ]) => {
        $crate::Var::array(var_internal!(@array [] $($tt)+))
    };
    ({ $($tt:tt)+ }) => {
        $crate::Var::dict({
            let mut map = std::collections::BTreeMap::new();
            var_internal!(@dict map () ($($tt)+) ($($tt)+));
            map
        })
    };
}//}}}

#[cfg(test)]
mod tests {
use super::*;

static ERROR_TEST_FAILED:&str = "Test failed";

#[test]
fn blank_t0()
{//{{{
    match Var::blank().data() { &Data::Blank => {} _ => panic!(ERROR_TEST_FAILED) }
}//}}}

#[test]
fn bool_t0()
{//{{{
    match Var::bool(true).data() { &Data::Bool(true) => {} _ => panic!(ERROR_TEST_FAILED) }
    match Var::bool(false).data() { &Data::Bool(false) => {} _ => panic!(ERROR_TEST_FAILED) }

    assert_eq!(Var::bool(false).to_bool(),Ok(false));
    assert_eq!(Var::bool(true).to_bool(),Ok(true));
    if let Err(_) = Var::bool(true).to_int() {} else { panic!(ERROR_TEST_FAILED) }
}//}}}

#[test]
fn int_t0()
{//{{{
    match Var::int(10).data() { &Data::Int(10) => {} _ => panic!(ERROR_TEST_FAILED) }
    match Var::int(123).data() { &Data::Int(123) => {} _ => panic!(ERROR_TEST_FAILED) }
    match Var::int(-156).data() { &Data::Int(-156) => {} _ => panic!(ERROR_TEST_FAILED) }

    assert_eq!(Var::int(10).to_int(),Ok(10));
    assert_eq!(Var::int(123).to_int(),Ok(123));
    if let Err(_) = Var::int(20).to_bool() {} else { panic!(ERROR_TEST_FAILED) }
}//}}}

#[test]
fn float_t0()
{//{{{
    match Var::float(1.236).data() {
        &Data::Float(value) => { assert_eq!(value,1.236); }
        _ => panic!(ERROR_TEST_FAILED)
    }

    match Var::float(1256.2).data() {
        &Data::Float(value) => { assert_eq!(value,1256.2); }
        _ => panic!(ERROR_TEST_FAILED)
    }

    match Var::float(-325.2).data() {
        &Data::Float(value) => { assert_eq!(value,-325.2); }
        _ => panic!(ERROR_TEST_FAILED)
    }

    assert_eq!(Var::float(1.234).to_float(),Ok(1.234));
    assert_eq!(Var::float(-325.2).to_float(),Ok(-325.2));
    if let Err(_) = Var::float(20.0).to_bool() {} else { panic!(ERROR_TEST_FAILED) }
}//}}}

#[test]
fn string_t0()
{//{{{
    match Var::str("Hello world").data() {
        &Data::String(ref value) => { assert_eq!(value,"Hello world"); }
        _ => panic!(ERROR_TEST_FAILED)
    }

    assert_eq!(Var::str("Hello world").to_str(),Ok(&String::from("Hello world")));
    assert_eq!(Var::str("Hello universe").to_str(),Ok(&String::from("Hello universe")));
    if let Err(_) = Var::str("Hello world").to_bool() {} else { panic!(ERROR_TEST_FAILED) }
}//}}}

#[test]
fn array_t0()
{//{{{
    let array = Var::array(vec![
        Var::blank(),
        Var::bool(true),
        Var::int(123),
        Var::float(123.45),
        Var::str("Hello world"),
    ]);

    match array.data() {
        &Data::Array(_) => {}
        _ => panic!(ERROR_TEST_FAILED)
    };
}//}}}

#[test]
fn array_t1()
{//{{{
    let array = Var::array(vec![
        Var::bool(true),
        Var::bool(false),
    ]);

    let ref mut vec = array.to_array().unwrap();
    vec.push(Var::bool(false));
    vec.push(Var::bool(false));
    vec.push(Var::bool(false));
}//}}}

#[test]
fn dict_t0()
{//{{{
    let dict = Var::dict(std::collections::BTreeMap::new());

    let ref mut map = dict.to_dict().unwrap();
    map.insert(Var::str("One"),Var::int(1));
    map.insert(Var::str("Two"),Var::int(2));
    map.insert(Var::str("Three"),Var::int(3));
    map.insert(Var::str("Four"),Var::int(4));

    assert_eq!(format!("{}",dict),"[Four:4,One:1,Three:3,Two:2]");
}//}}}

#[test]
fn data_t0()
{//{{{
    assert!(Data::Blank < Data::Bool(true));
    assert!(Data::Bool(false) < Data::Int(123));
    assert!(Data::Int(123) < Data::Float(12.345));
    assert!(Data::Float(12.345) < Data::String(String::from("Hello world")));
}//}}}

#[test]
fn var_t0()
{//{{{
    let array = Var::array(vec![
        Var::bool(true),
        Var::bool(false),
    ]);

    let var_0 = Var::var(&array);
    let var_1 = Var::var(&array);

    var_0.to_array().unwrap().push(Var::bool(true));
    var_1.to_array().unwrap().push(Var::bool(false));
    assert_eq!(array.to_array().unwrap().len(),4);
}//}}}

#[test]
fn var_partial_eq_t0()
{//{{{
    assert_eq!(Var::blank(),Var::blank());
    assert_ne!(Var::blank(),Var::bool(true));
    assert_eq!(Var::bool(false),Var::bool(false));
    assert_ne!(Var::bool(false),Var::bool(true));
    assert_eq!(Var::int(123),Var::int(123));
    assert_ne!(Var::int(123),Var::int(124));
    assert_eq!(Var::float(12.345),Var::float(12.345));
    assert_ne!(Var::float(12.345),Var::float(13.345));
    assert_eq!(Var::str("Hello world"),Var::str("Hello world"));
    assert_ne!(Var::str("Hello world"),Var::str("Hello universe"));
    assert_eq!(Var::array(vec![Var::bool(false),Var::bool(true)]),Var::array(vec![Var::bool(false),Var::bool(true)]));
    assert_ne!(Var::array(vec![Var::bool(false),Var::bool(true)]),Var::array(vec![Var::bool(false),Var::bool(false)]));
}//}}}

#[test]
fn var_partial_ord_t0()
{//{{{
    assert!(Var::blank() < Var::bool(true));
    assert!(Var::bool(false) < Var::bool(true));
    assert!(Var::bool(true) < Var::int(123));
    assert!(Var::int(123) < Var::int(124));
    assert!(Var::int(123) < Var::float(12.345));
    assert!(Var::float(12.345) < Var::float(12.346));
    assert!(Var::float(12.345) < Var::str("Hello world"));
    assert!(Var::str("Hello world") < Var::str("Hello world!"));
    assert!(Var::str("Hello world") < Var::array(vec![]));
    assert!(Var::array(vec![]) < Var::array(vec![Var::blank()]));
}//}}}

#[test]
fn var_clone_t0()
{//{{{
    let array_0 = Var::array(vec![
        Var::int(1), Var::int(2), Var::int(3)
    ]);

    let array_1 = Var::var(&array_0);
    let array_2 = array_0.clone();

    array_0.to_array().unwrap().push(Var::int(4));
    assert_eq!(array_0,array_1);
    assert_ne!(array_0,array_2);

    let dict_0 = Var::dict(std::collections::BTreeMap::new());
    let dmap = dict_0.to_dict().unwrap();

    dmap.insert(Var::str("One"),Var::int(1));
    dmap.insert(Var::str("Two"),Var::int(2));
    dmap.insert(Var::str("Three"),Var::int(3));

    let dict_1 = Var::var(&dict_0);
    let dict_2 = dict_0.clone();

    dmap.insert(Var::str("Four"),Var::int(4));
    assert_eq!(dict_0,dict_1);
    assert_ne!(dict_0,dict_2);
}//}}}

#[test]
fn var_display_fmt_t0()
{//{{{
    assert_eq!(format!("{}",Var::blank()),"<blank>");
    assert_eq!(format!("{}",Var::bool(true)),"true");
    assert_eq!(format!("{}",Var::bool(false)),"false");
    assert_eq!(format!("{}",Var::int(10)),"10");
    assert_eq!(format!("{}",Var::int(123)),"123");
    assert_eq!(format!("{}",Var::int(-156)),"-156");
    assert_eq!(format!("{}",Var::float(10.123)),"10.123");
    assert_eq!(format!("{}",Var::float(123.123)),"123.123");
    assert_eq!(format!("{}",Var::float(-156.123)),"-156.123");
    assert_eq!(format!("{}",Var::str("Hello world")),"Hello world");
    assert_eq!(format!("{}",Var::array(vec![
        Var::blank(),
        Var::bool(true),
        Var::bool(false),
        Var::array(vec![
            Var::bool(true),
            Var::bool(false),
        ]),
    ])),"[<blank>,true,false,[true,false]]");
}//}}}

#[test]
fn var_debug_fmt_t0()
{//{{{
    assert_eq!(format!("{:?}",Var::blank()),"Blank#1");
    assert_eq!(format!("{:?}",Var::bool(true)),"Bool(true)#1");
    assert_eq!(format!("{:?}",Var::bool(false)),"Bool(false)#1");
    assert_eq!(format!("{:?}",Var::int(10)),"Int(10)#1");
    assert_eq!(format!("{:?}",Var::int(123)),"Int(123)#1");
    assert_eq!(format!("{:?}",Var::int(-156)),"Int(-156)#1");
    assert_eq!(format!("{:?}",Var::float(10.123)),"Float(10.123)#1");
    assert_eq!(format!("{:?}",Var::float(123.123)),"Float(123.123)#1");
    assert_eq!(format!("{:?}",Var::float(-156.123)),"Float(-156.123)#1");
    assert_eq!(format!("{:?}",Var::str("Hello world")),"String(\"Hello world\")#1");
    assert_eq!(format!("{:?}",Var::array(vec![
        Var::blank(),
        Var::bool(true),
        Var::bool(false),
        Var::array(vec![Var::bool(true),Var::bool(false)]),
    ])),"Array([Blank#1, Bool(true)#1, Bool(false)#1, Array([Bool(true)#1, Bool(false)#1])#1])#1");

    let var = Var::bool(true);
    assert_eq!(format!("{:?}",Var::array(vec![
        Var::var(&var),
        Var::var(&var),
        Var::var(&var),
        Var::var(&var),
        Var::var(&var),
    ])),"Array([Bool(true)#6, Bool(true)#6, Bool(true)#6, Bool(true)#6, Bool(true)#6])#1");
}//}}}

#[test]
fn macro_t0()
{//{{{
    assert_eq!(var!(blank),Var::blank());
    assert_eq!(var!(false),Var::bool(false));
    assert_eq!(var!(true),Var::bool(true));
    assert_eq!(var!(i(123)),Var::int(123));
    assert_eq!(var!(f(123.45)),Var::float(123.45));
    assert_eq!(var!(s("Hello world")),Var::str("Hello world"));
    assert_eq!(var!([]),Var::array(vec![]));
    assert_eq!(var!({}),Var::dict(std::collections::BTreeMap::new()));
}//}}}

#[test]
fn macro_t1()
{//{{{
    let var = Var::int(123);
    assert_eq!(var!(v(var)),var);

    let var1 = var!({
        s("blank") : blank,
        s("bool") : true,
        s("integer") : i(123),
        s("float") : f(123.45),
        s("string") : s("Hello world"),
        s("array") : [i(1),i(2),i(3)],
        s("dict") : {
            s("One") : i(1),
            s("Two") : i(2),
            s("Three") : i(3),
        },
    });
    assert_eq!(format!("{}",var1),"[array:[1,2,3],blank:<blank>,bool:true,dict:[One:1,Three:3,Two:2],float:123.45,integer:123,string:Hello world]");

    let var2 = var!({
        blank : s("blank"),
        true : s("bool"),
        i(123) : s("integer"),
        f(123.45) : s("float"),
        s("Hello world") : s("string"),
        [i(1),i(2),i(3)] : s("array"),
        {
            s("One") : i(1),
            s("Two") : i(2),
            s("Three") : i(3),
        } : s("dict"),
    });
    assert_eq!(format!("{}",var2),"[<blank>:blank,true:bool,123:integer,123.45:float,Hello world:string,[1,2,3]:array,[One:1,Three:3,Two:2]:dict]")
}//}}}
}

