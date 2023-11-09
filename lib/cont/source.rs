
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::{c_char,c_int,c_uint,c_void};

#[link(name = "base_cll")]
#[link(name = "rscont_cll")]

extern {
    fn memcmp(s1:*const c_void,s2:*const c_void,n:usize) -> c_int;
    fn free(ptr:*mut c_void);
}

type bc = c_char;
type ui = c_uint;

const c_array_add:u32 = 4;
const c_idx_not_exist:u32 = std::u32::MAX;
const c_log_of_2:f32 = 0.69314718055994530941;

pub trait Cont {
    fn default() -> Self;
    fn to_string(&self,a_trg:&mut bc_array_s)
    {//{{{
        a_trg.append(b"{}");
    }//}}}

    fn to_json(&self,a_trg:&mut bc_array_s)
    {//{{{
        a_trg.append(b"null");
    }//}}}

    fn to_json_nice(&self,_a_json_nice:&mut json_nice_s,a_trg:&mut bc_array_s)
    {//{{{
        self.to_json(a_trg)
    }//}}}
}

impl Cont for ui {
    fn default() -> Self
    {//{{{
        0
    }//}}}

    fn to_string(&self,a_trg:&mut bc_array_s)
    {//{{{
        let buffer = format!("{}",*self);
        a_trg.append(&buffer.as_bytes());
    }//}}}

    fn to_json(&self,a_trg:&mut bc_array_s)
    {//{{{
        Cont::to_string(self,a_trg);
    }//}}}
}

// - pointer typedef -
type pointer = *mut c_void;

@begin
define pointer basic
@end

impl Cont for pointer {
    fn default() -> Self
    {//{{{
        std::ptr::null_mut()
    }//}}}

    fn to_string(&self,a_trg:&mut bc_array_s)
    {//{{{
        let buffer = format!("{:?}",*self);
        a_trg.append(&buffer.as_bytes());
    }//}}}

    fn to_json(&self,a_trg:&mut bc_array_s)
    {//{{{
        Cont::to_string(self,a_trg);
    }//}}}
}

// -- bc_array_s --
@begin
rust_array<bc>
bc_array_s;
@end

// -- bc_array_s --
@begin
inlines bc_array_s
@end

// -- bc_array_s --
@begin
methods bc_array_s
@end

impl bc_array_s {

    fn append(&mut self,a_data:&[u8])
    {//{{{
        let old_used = self.used;
        self.push_blanks(a_data.len() as u32);
        unsafe {
            std::ptr::copy_nonoverlapping(
                    a_data.as_ptr(),
                    self.data.offset(old_used as isize) as *mut u8,
                    a_data.len());
        }
    }//}}}
}

impl std::fmt::Debug for bc_array_s {
    fn fmt(&self,f:&mut std::fmt::Formatter) -> std::fmt::Result
    {//{{{
        let string;
        unsafe {
            string = std::str::from_utf8(
                std::slice::from_raw_parts(self.data as *const u8,self.used as usize)).expect("Error");
        }
        write!(f,"{}",string)
    }//}}}
}

// -- json_nice_s --
@begin
rust_struct
<
bc_array_s:tabulator
bc_array_s:indent_buffer
ui:indent_size
pointer:user
>
json_nice_s;
@end

impl json_nice_s {
    fn create(&mut self,a_tabulator:&[u8],a_indentation:&[u8],a_user:pointer)
    {//{{{
        self.clear();

        self.tabulator.append(a_tabulator);
        self.indent_buffer.push(b'\n' as bc);
        self.indent_buffer.append(a_indentation);
        self.indent_size = self.indent_buffer.used;
        self.user = a_user;
    }//}}}

    fn push_indent(&mut self,a_trg:&mut bc_array_s)
    {//{{{
        self.indent_size += self.tabulator.used;
        if self.indent_size > self.indent_buffer.used
        {
            self.indent_buffer.append(unsafe{std::slice::from_raw_parts(
                self.tabulator.data as *const u8,self.tabulator.used as usize)});
        }

        a_trg.append(unsafe{std::slice::from_raw_parts(
            self.indent_buffer.data as *const u8,self.indent_size as usize)});
    }//}}}

    fn pop_indent(&mut self,a_trg:&mut bc_array_s)
    {//{{{
        self.indent_size -= self.tabulator.used;
        a_trg.append(unsafe {std::slice::from_raw_parts(
            self.indent_buffer.data as *const u8,self.indent_size as usize)});
    }//}}}

    fn indent(&self,a_trg:&mut bc_array_s)
    {//{{{
        a_trg.append(unsafe{std::slice::from_raw_parts(
            self.indent_buffer.data as *const u8,self.indent_size as usize)});
    }//}}}
}

// -- json_nice_s --
@begin
inlines json_nice_s
@end

// -- json_nice_s --
@begin
methods json_nice_s
@end

// -- ui_array_s --
@begin
rust_array<ui>
options ( to_json to_json_nice )
ui_array_s;
@end

// -- ui_array_s --
@begin
inlines ui_array_s
@end

// -- ui_array_s --
@begin
methods ui_array_s
@end

// -- bc_arrays_s --
@begin
rust_array<bc_array_s> bc_arrays_s;
@end

// -- bc_arrays_s --
@begin
inlines bc_arrays_s
@end

// -- bc_arrays_s --
@begin
methods bc_arrays_s
@end

// -- record_s --
@begin
rust_struct
<
ui:first
ui:second
ui_array_s:array0
ui_array_s:array1
>
options ( to_json to_json_nice )
record_s
rec_s
rec1_s
rec2_s;
@end

// -- record_s --
@begin
inlines record_s
@end

// -- record_s --
@begin
methods record_s
@end

// -- ui_tree_s --
@begin
rust_rb_tree<ui
ui:first
ui_array_s:second
> ui_tree_s;
@end

// -- ui_tree_s --
@begin
inlines ui_tree_s
@end

// -- ui_tree_s --
@begin
methods ui_tree_s
@end

// -- record_tree_s --
@begin
rust_safe_rb_tree<record_s> record_tree_s;
@end

// -- record_tree_s --
@begin
inlines record_tree_s
@end

// -- record_tree_s --
@begin
methods record_tree_s
@end

fn main() {
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn array_new_0()
{//{{{
    assert!(ui_array_s::new().compare(&ui_array_s::new()));
}//}}}

#[test]
fn array_init_0()
{//{{{
    let mut array = ui_array_s::new();
    array.init();
    assert!(array.compare(&ui_array_s{size:0,used:0,data:std::ptr::null_mut()}));
}//}}}

#[test]
fn array_init_size_0()
{//{{{
    let mut array = ui_array_s::new();
    array.init_size(100);
    assert_eq!(array.used,0);
    assert!(array.size >= 100);
    assert_ne!(array.data,std::ptr::null_mut());
}//}}}

#[test]
fn array_clear_0()
{//{{{
    let mut array = ui_array_s::new();
    for value in 0..100 {
        array.push(value);
    }
    array.clear();
    assert!(array.compare(&ui_array_s{size:0,used:0,data:std::ptr::null_mut()}));
}//}}}

#[test]
fn array_set_0()
{//{{{
    let mut array = ui_array_s::new();
    let slice = &[1,2,3,4,5,6];
    array.set(slice.len() as u32,slice.as_ptr());

    let mut buffer = bc_array_s::new();
    array.to_json(&mut buffer);
    assert_eq!(format!("{:?}",buffer),"[1,2,3,4,5,6]");
}//}}}

#[test]
fn array_flush_0()
{//{{{
    let mut array = ui_array_s::new();
    for value in 0..100 {
        array.push(value);
    }
    assert_ne!(array.used,array.size);
    array.flush();
    assert_eq!(array.used,array.size);
}//}}}

#[test]
fn array_flush_all_0()
{//{{{
    let mut arrays = bc_arrays_s::new();
    for _ in 0..10 {
        let mut array = bc_array_s::new();
        for value in 0..100 {
            array.push(value);
        }

        arrays.push_blank();
        arrays.last().swap(&mut array);
    }

    arrays.flush_all();
    assert_eq!(arrays.size,10);

    let mut len_array = ui_array_s::new();
    for idx in 0..arrays.used {
        len_array.push(arrays.at(idx).size);
    }

    let mut buffer = bc_array_s::new();
    len_array.to_json(&mut buffer);
    assert_eq!(format!("{:?}",buffer),"[100,100,100,100,100,100,100,100,100,100]");
}//}}}

#[test]
fn array_copy_resize_0()
{//{{{
    let mut array = ui_array_s::new();
    for value in 0..10 {
        array.push(value);
    }
    assert!(array.size >= 10);
    array.copy_resize(20);
    assert_eq!(array.size,20);
}//}}}

#[test]
fn array_fill_0()
{//{{{
    let mut array = ui_array_s::new();
    array.copy_resize(10);
    array.fill(42);

    let mut buffer = bc_array_s::new();
    array.to_json(&mut buffer);
    assert_eq!(format!("{:?}",buffer),"[42,42,42,42,42,42,42,42,42,42]");
}//}}}

#[test]
fn array_get_idx_0()
{//{{{
    let mut arrays = bc_arrays_s::new();
    let mut array = bc_array_s::new();
    for value in 0..10 {
        array.push(value);
        arrays.push(&array);
    }

    let mut res_array = ui_array_s::new();
    for _ in 0..10 {
        res_array.push(arrays.get_idx(&array));
        array.pop();
    }

    let mut buffer = bc_array_s::new();
    res_array.to_json(&mut buffer);
    assert_eq!(format!("{:?}",buffer),"[9,8,7,6,5,4,3,2,1,0]");
}//}}}

#[test]
fn array_copy_0()
{//{{{
    let mut arrays = bc_arrays_s::new();
    let mut array = bc_array_s::new();
    for value in 0..10 {
        array.push(value);
        arrays.push(&array);
    }

    let mut arrays_copy = bc_arrays_s::new();
    arrays_copy.copy(&arrays);
    assert!(arrays_copy.compare(&arrays));
}//}}}

#[test]
fn array_compare_0()
{//{{{
    let mut arrays = bc_arrays_s::new();
    let mut array = bc_array_s::new();
    for value in 0..10 {
        array.push(value);
        arrays.push(&array);
    }

    let mut search_array = bc_array_s::new();
    let mut res_array = ui_array_s::new();
    for value in 0..10 {
        search_array.push(value);
        res_array.push(arrays.at(5).compare(&search_array) as u32);
    }

    let mut buffer = bc_array_s::new();
    res_array.to_json(&mut buffer);
    assert_eq!(format!("{:?}",buffer),"[0,0,0,0,0,1,0,0,0,0]");
}//}}}

#[test]
fn struct_init_0()
{//{{{
    let mut record = record_s::new();
    record.init();
}//}}}

#[test]
fn struct_clear_0()
{//{{{
    let mut record = record_s::new();
    for value in 0..10 {
        record.array0.push(value);
    }
}//}}}

#[test]
fn struct_set_0()
{//{{{
    let mut record = record_s::new();
    for value in 0..10 {
        record.array0.push(value);
    }

    let mut record_set = record_s::new();
    record_set.set(0,0,&record.array0,&record.array0);
    assert!(record_set.array0.compare(&record.array0));
    assert!(record_set.array1.compare(&record.array0));
}//}}}

#[test]
fn struct_flush_all_0()
{//{{{
    let mut record = record_s::new();
    for value in 0..10 {
        record.array0.push(value);
    }
    for value in 0..100 {
        record.array1.push(value);
    }
    assert_eq!(record.array0.size,12);
    assert_eq!(record.array1.size,124);
    record.flush_all();
    assert_eq!(record.array0.size,10);
    assert_eq!(record.array1.size,100);
}//}}}

#[test]
fn struct_swap_0()
{//{{{
    let mut array = ui_array_s::new();
    for value in 0..10 {
        array.push(value);
    }
    let mut record = record_s::new();
    record.set(0,0,&array,&ui_array_s::new());
    assert!(record.array0.compare(&array));

    let mut record_swap = record_s::new();
    record_swap.swap(&mut record);
    assert!(!record.array0.compare(&array));
    assert!(record_swap.array0.compare(&array));
}//}}}

#[test]
fn struct_copy_0()
{//{{{
    let mut array = ui_array_s::new();
    for value in 0..10 {
        array.push(value);
    }
    let mut record = record_s::new();
    record.set(0,0,&array,&ui_array_s::new());
    assert!(record.array0.compare(&array));

    let mut record_copy = record_s::new();
    record_copy.copy(&record);
    assert!(record_copy.compare(&record));
    //assert!(record_copy.compare(&record_copy));
}//}}}

#[test]
fn struct_compare_0()
{//{{{
    let mut array = ui_array_s::new();
    for value in 0..10 {
        array.push(value);
    }
    let mut record = record_s::new();
    record.set(0,0,&array,&ui_array_s::new());
    assert!(record.array0.compare(&array));

    let mut record_copy = record_s::new();
    record_copy.copy(&record);
    assert!(record_copy.compare(&record));
    assert!(!record_copy.compare(&record_s::new()));
}//}}}

#[test]
fn struct_to_string_0()
{//{{{
    let mut array = ui_array_s::new();
    for value in 0..10 {
        array.push(value);
    }
    let mut record = record_s::new();
    record.set(0,0,&array,&ui_array_s::new());
    assert!(record.array0.compare(&array));

    let mut buffer = bc_array_s::new();
    record.to_string(&mut buffer);
    assert_eq!(format!("{:?}",buffer),
            "{first:0,second:0,array0:[0,1,2,3,4,5,6,7,8,9],array1:[]}");

    buffer.used = 0;
    record.to_string_separator(&mut buffer,b" > ");
    assert_eq!(format!("{:?}",buffer),
            "first:0 > second:0 > array0:[0,1,2,3,4,5,6,7,8,9] > array1:[]");
}//}}}

#[test]
fn struct_to_json_0()
{//{{{
    let mut array = ui_array_s::new();
    for value in 0..10 {
        array.push(value);
    }
    let mut record = record_s::new();
    record.set(0,0,&array,&ui_array_s::new());
    assert!(record.array0.compare(&array));

    let mut buffer = bc_array_s::new();
    record.to_json(&mut buffer);
    assert_eq!(format!("{:?}",buffer),
            "{\"first\":0,\"second\":0,\"array0\":[0,1,2,3,4,5,6,7,8,9],\"array1\":[]}");
    
    let mut json_nice = json_nice_s::new();
    json_nice.create(b"--",b"==",std::ptr::null_mut());
    
    buffer.used = 0;
    record.to_json_nice(&mut json_nice,&mut buffer);
    assert_eq!(format!("{:?}",buffer),
"{
==--\"first\": 0,
==--\"second\": 0,
==--\"array0\": [
==----0,
==----1,
==----2,
==----3,
==----4,
==----5,
==----6,
==----7,
==----8,
==----9
==--],
==--\"array1\": []
==}");
}//}}}

#[test]
fn tree_new_0()
{//{{{
    assert!(ui_tree_s::new().compare(&ui_tree_s::new()));
    assert!(record_tree_s::new().compare(&record_tree_s::new()));
}//}}}

#[test]
fn tree_init_0()
{//{{{
    let mut tree = ui_tree_s::new();
    tree.init();
    assert!(tree.compare(&ui_tree_s{
        size:0,
        used:0,
        data:std::ptr::null_mut(),
        free_idx:c_idx_not_exist,
        root_idx:c_idx_not_exist,
        leaf_idx:c_idx_not_exist,
        first:0,
        second:ui_array_s::new(),
    }));
}//}}}

#[test]
fn tree_clear_0()
{//{{{
    let mut tree = ui_tree_s::new();
    for value in 0..100 {
        tree.insert(value);
    }
    tree.clear();
    assert!(tree.compare(&ui_tree_s::new()));
}//}}}

}
