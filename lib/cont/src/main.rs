
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
// --- struct bc_array_s definition --- 


#[repr(C)] pub struct bc_array_s
{
  size:u32,
  used:u32,
  data:*mut bc,
}

extern {
fn bc_array_s_copy_resize(this:*mut bc_array_s,a_size:c_uint);
fn bc_array_s_get_idx(this:*const bc_array_s,a_value:bc) -> c_uint;
fn bc_array_s___to_string(this:*const bc_array_s,a_trg:*mut bc_array_s);
fn bc_array_s_to_string_separator(this:*const bc_array_s,a_trg:*mut bc_array_s,a_count:c_uint,a_data:*const c_char);
}



// -- bc_array_s --
// --- struct bc_array_s inline method definition ---

impl bc_array_s {

fn new() -> Self
{/*{{{*/
  <Self as Cont>::default()
}/*}}}*/

fn init(&mut self)
{/*{{{*/
  self.size = 0;
  self.used = 0;
  self.data = std::ptr::null_mut();
}/*}}}*/

fn init_size(&mut self,a_size:u32)
{/*{{{*/
  self.init();
  self.copy_resize(a_size);
}/*}}}*/

fn clear(&mut self)
{/*{{{*/
  if !self.data.is_null()
  {
    unsafe{free(self.data as *mut c_void);}
  }

  self.init();
}/*}}}*/

fn set(&mut self,a_used:u32,a_data:*const bc)
{/*{{{*/
  self.clear();

  if a_used == 0
  {
    return;
  }

  debug_assert!(!a_data.is_null());

  self.copy_resize(a_used);

  unsafe{std::ptr::copy_nonoverlapping(a_data,self.data,a_used as usize);}
  self.used = a_used;
}/*}}}*/

fn flush(&mut self)
{/*{{{*/
  self.copy_resize(self.used);
}/*}}}*/

fn flush_all(&mut self)
{/*{{{*/
  self.copy_resize(self.used);
}/*}}}*/

fn swap(&mut self,a_second:&mut Self)
{/*{{{*/
  std::mem::swap(self,a_second);
}/*}}}*/

fn at(&mut self,a_idx:u32) -> &mut bc
{/*{{{*/
  debug_assert!(a_idx < self.used);
  unsafe{&mut *self.data.offset(a_idx as isize)}
}/*}}}*/

fn push(&mut self,a_value:bc)
{/*{{{*/
  if self.used >= self.size
  {
    let new_size = (self.size << 1) + c_array_add;
    debug_assert!(new_size != 0);

    self.copy_resize(new_size);
  }

  unsafe{*self.data.offset(self.used as isize) = a_value;}
  self.used += 1;
}/*}}}*/

fn push_blank(&mut self)
{/*{{{*/
  if self.used >= self.size
  {
    let new_size = (self.size << 1) + c_array_add;
    debug_assert!(new_size != 0);

    self.copy_resize(new_size);
  }

  self.used += 1;
}/*}}}*/

fn reserve(&mut self,a_cnt:u32)
{/*{{{*/
  let required_cnt = self.used + a_cnt;
  if required_cnt > self.size
  {
    let mut r_size = self.size;
    loop {
      r_size = (r_size << 1) + c_array_add;
      if r_size >= required_cnt { break; }
    }

    debug_assert!(r_size != 0);
    self.copy_resize(r_size);
  }
}/*}}}*/

fn push_blanks(&mut self,a_cnt:u32)
{/*{{{*/
  let required_cnt = self.used + a_cnt;
  if required_cnt > self.size
  {
    let mut r_size = self.size;
    loop {
      r_size = (r_size << 1) + c_array_add;
      if r_size >= required_cnt { break; }
    }

    debug_assert!(r_size != 0);
    self.copy_resize(r_size);
  }

  self.used += a_cnt;
}/*}}}*/

fn push_clear(&mut self)
{/*{{{*/
  if self.used >= self.size
  {
    let new_size = (self.size << 1) + c_array_add;
    debug_assert!(new_size != 0);

    self.copy_resize(new_size);
  }

  self.used += 1;
}/*}}}*/

fn pop(&mut self) -> bc
{/*{{{*/
  debug_assert!(self.used > 0);
  self.used -= 1;
  unsafe{*self.data.offset(self.used as isize)}
}/*}}}*/

fn last(&self) -> &mut bc
{/*{{{*/
  debug_assert!(self.used > 0);
  unsafe{&mut *self.data.offset(self.used as isize - 1)}
}/*}}}*/

fn fill(&mut self,a_value:bc)
{/*{{{*/
  if self.size == 0
  {
    return;
  }

  unsafe{self.data.write_bytes(a_value as u8,self.size as usize);}

  self.used = self.size;
}/*}}}*/

fn copy(&mut self,a_src:&Self)
{/*{{{*/
  self.clear();

  if a_src.used == 0
  {
    return;
  }

  self.copy_resize(a_src.used);

  unsafe{std::ptr::copy_nonoverlapping(a_src.data,self.data,a_src.used as usize);}

  self.used = a_src.used;
}/*}}}*/

fn compare(&self,a_second:&Self) -> bool
{/*{{{*/
  if self.used != a_second.used
  {
    return false;
  }

  if self.used == 0
  {
    return true;
  }

  unsafe{memcmp(self.data as *const c_void,a_second.data as *const c_void,self.used as usize*std::mem::size_of::<bc>()) == 0}
}/*}}}*/

}



// -- bc_array_s --
// --- struct bc_array_s method definition ---

impl bc_array_s {

fn copy_resize(&mut self,a_size:u32)
{/*{{{*/
  unsafe{bc_array_s_copy_resize(self as *mut Self,a_size as c_uint);}
}/*}}}*/

fn get_idx(&self,a_value:bc) -> u32
{/*{{{*/
  unsafe{bc_array_s_get_idx(self as *const Self,a_value)}
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn to_string_separator(&self,a_trg:&mut bc_array_s,a_sep:&str)
{/*{{{*/
  unsafe{bc_array_s_to_string_separator(self as *const Self,a_trg as *mut bc_array_s,a_sep.len() as c_uint,a_sep.as_ptr() as *const c_char);}
}/*}}}*/

}

impl Cont for bc_array_s
{

fn default() -> Self
{/*{{{*/
  bc_array_s{
    size:0,
    used:0,
    data:std::ptr::null_mut(),
  }
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn to_string(&self,a_trg:&mut bc_array_s)
{/*{{{*/
  unsafe{bc_array_s___to_string(self as *const Self,a_trg as *mut bc_array_s);}
}/*}}}*/

}

impl Drop for bc_array_s
{
  fn drop(&mut self) {
    self.clear();
  }
}



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
// --- struct json_nice_s definition --- 


#[repr(C)] pub struct json_nice_s
{
  tabulator:bc_array_s,
  indent_buffer:bc_array_s,
  indent_size:ui,
  user:pointer,
}




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
// --- struct json_nice_s inline method definition ---

impl json_nice_s {

fn new() -> Self
{/*{{{*/
  <Self as Cont>::default()
}/*}}}*/

fn init(&mut self)
{/*{{{*/
  self.tabulator.init();
  self.indent_buffer.init();
}/*}}}*/

fn clear(&mut self)
{/*{{{*/
  self.tabulator.clear();
  self.indent_buffer.clear();
}/*}}}*/

fn set(&mut self,a_tabulator:&bc_array_s,a_indent_buffer:&bc_array_s,a_indent_size:ui,a_user:pointer)
{/*{{{*/
  self.tabulator.copy(a_tabulator);
  self.indent_buffer.copy(a_indent_buffer);
  self.indent_size = a_indent_size;
  self.user = a_user;
}/*}}}*/

fn flush_all(&mut self)
{/*{{{*/
  self.tabulator.flush_all();
  self.indent_buffer.flush_all();
}/*}}}*/

fn swap(&mut self,a_second:&mut Self)
{/*{{{*/
  std::mem::swap(self,a_second);
}/*}}}*/

fn copy(&mut self,a_src:&Self)
{/*{{{*/
  self.tabulator.copy(&a_src.tabulator);
  self.indent_buffer.copy(&a_src.indent_buffer);
  self.indent_size = a_src.indent_size;
  self.user = a_src.user;
}/*}}}*/

fn compare(&self,a_second:&Self) -> bool
{/*{{{*/
  (self.tabulator.compare(&a_second.tabulator) &&
          self.indent_buffer.compare(&a_second.indent_buffer) &&
          self.indent_size == a_second.indent_size &&
          self.user == a_second.user)
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn to_string_separator(&self,a_trg:&mut bc_array_s,a_data:&[u8])
{/*{{{*/
  a_trg.append(b"tabulator:");
  Cont::to_string(&self.tabulator,a_trg);
  a_trg.append(a_data);
  a_trg.append(b"indent_buffer:");
  Cont::to_string(&self.indent_buffer,a_trg);
  a_trg.append(a_data);
  a_trg.append(b"indent_size:");
  Cont::to_string(&self.indent_size,a_trg);
  a_trg.append(a_data);
  a_trg.append(b"user:");
  Cont::to_string(&self.user,a_trg);
}/*}}}*/

}

impl Cont for json_nice_s
{

fn default() -> Self
{/*{{{*/
  json_nice_s{
    tabulator:<bc_array_s as Cont>::default(),
    indent_buffer:<bc_array_s as Cont>::default(),
    indent_size:<ui as Cont>::default(),
    user:<pointer as Cont>::default(),
  }
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn to_string(&self,a_trg:&mut bc_array_s)
{/*{{{*/
  a_trg.append(b"{tabulator:");
  Cont::to_string(&self.tabulator,a_trg);
  a_trg.append(b",indent_buffer:");
  Cont::to_string(&self.indent_buffer,a_trg);
  a_trg.append(b",indent_size:");
  Cont::to_string(&self.indent_size,a_trg);
  a_trg.append(b",user:");
  Cont::to_string(&self.user,a_trg);
  a_trg.push(b'}' as bc);
}/*}}}*/

}



// -- json_nice_s --
// --- struct json_nice_s method definition ---

impl json_nice_s {

}

impl Drop for json_nice_s
{
  fn drop(&mut self) {
    self.clear();
  }
}



// -- ui_array_s --
// --- struct ui_array_s definition --- 


#[repr(C)] pub struct ui_array_s
{
  size:u32,
  used:u32,
  data:*mut ui,
}

extern {
fn ui_array_s_copy_resize(this:*mut ui_array_s,a_size:c_uint);
fn ui_array_s_fill(this:*mut ui_array_s,a_value:ui);
fn ui_array_s_get_idx(this:*const ui_array_s,a_value:ui) -> c_uint;
fn ui_array_s___to_string(this:*const ui_array_s,a_trg:*mut bc_array_s);
fn ui_array_s_to_string_separator(this:*const ui_array_s,a_trg:*mut bc_array_s,a_count:c_uint,a_data:*const c_char);
fn ui_array_s_to_json(this:*const ui_array_s,a_trg:*mut bc_array_s);
fn ui_array_s_to_json_nice(this:*const ui_array_s,a_json_nice:*mut json_nice_s,a_trg:*mut bc_array_s);
}



// -- ui_array_s --
// --- struct ui_array_s inline method definition ---

impl ui_array_s {

fn new() -> Self
{/*{{{*/
  <Self as Cont>::default()
}/*}}}*/

fn init(&mut self)
{/*{{{*/
  self.size = 0;
  self.used = 0;
  self.data = std::ptr::null_mut();
}/*}}}*/

fn init_size(&mut self,a_size:u32)
{/*{{{*/
  self.init();
  self.copy_resize(a_size);
}/*}}}*/

fn clear(&mut self)
{/*{{{*/
  if !self.data.is_null()
  {
    unsafe{free(self.data as *mut c_void);}
  }

  self.init();
}/*}}}*/

fn set(&mut self,a_used:u32,a_data:*const ui)
{/*{{{*/
  self.clear();

  if a_used == 0
  {
    return;
  }

  debug_assert!(!a_data.is_null());

  self.copy_resize(a_used);

  unsafe{std::ptr::copy_nonoverlapping(a_data,self.data,a_used as usize);}
  self.used = a_used;
}/*}}}*/

fn flush(&mut self)
{/*{{{*/
  self.copy_resize(self.used);
}/*}}}*/

fn flush_all(&mut self)
{/*{{{*/
  self.copy_resize(self.used);
}/*}}}*/

fn swap(&mut self,a_second:&mut Self)
{/*{{{*/
  std::mem::swap(self,a_second);
}/*}}}*/

fn at(&mut self,a_idx:u32) -> &mut ui
{/*{{{*/
  debug_assert!(a_idx < self.used);
  unsafe{&mut *self.data.offset(a_idx as isize)}
}/*}}}*/

fn push(&mut self,a_value:ui)
{/*{{{*/
  if self.used >= self.size
  {
    let new_size = (self.size << 1) + c_array_add;
    debug_assert!(new_size != 0);

    self.copy_resize(new_size);
  }

  unsafe{*self.data.offset(self.used as isize) = a_value;}
  self.used += 1;
}/*}}}*/

fn push_blank(&mut self)
{/*{{{*/
  if self.used >= self.size
  {
    let new_size = (self.size << 1) + c_array_add;
    debug_assert!(new_size != 0);

    self.copy_resize(new_size);
  }

  self.used += 1;
}/*}}}*/

fn reserve(&mut self,a_cnt:u32)
{/*{{{*/
  let required_cnt = self.used + a_cnt;
  if required_cnt > self.size
  {
    let mut r_size = self.size;
    loop {
      r_size = (r_size << 1) + c_array_add;
      if r_size >= required_cnt { break; }
    }

    debug_assert!(r_size != 0);
    self.copy_resize(r_size);
  }
}/*}}}*/

fn push_blanks(&mut self,a_cnt:u32)
{/*{{{*/
  let required_cnt = self.used + a_cnt;
  if required_cnt > self.size
  {
    let mut r_size = self.size;
    loop {
      r_size = (r_size << 1) + c_array_add;
      if r_size >= required_cnt { break; }
    }

    debug_assert!(r_size != 0);
    self.copy_resize(r_size);
  }

  self.used += a_cnt;
}/*}}}*/

fn push_clear(&mut self)
{/*{{{*/
  if self.used >= self.size
  {
    let new_size = (self.size << 1) + c_array_add;
    debug_assert!(new_size != 0);

    self.copy_resize(new_size);
  }

  self.used += 1;
}/*}}}*/

fn pop(&mut self) -> ui
{/*{{{*/
  debug_assert!(self.used > 0);
  self.used -= 1;
  unsafe{*self.data.offset(self.used as isize)}
}/*}}}*/

fn last(&self) -> &mut ui
{/*{{{*/
  debug_assert!(self.used > 0);
  unsafe{&mut *self.data.offset(self.used as isize - 1)}
}/*}}}*/

fn copy(&mut self,a_src:&Self)
{/*{{{*/
  self.clear();

  if a_src.used == 0
  {
    return;
  }

  self.copy_resize(a_src.used);

  unsafe{std::ptr::copy_nonoverlapping(a_src.data,self.data,a_src.used as usize);}

  self.used = a_src.used;
}/*}}}*/

fn compare(&self,a_second:&Self) -> bool
{/*{{{*/
  if self.used != a_second.used
  {
    return false;
  }

  if self.used == 0
  {
    return true;
  }

  unsafe{memcmp(self.data as *const c_void,a_second.data as *const c_void,self.used as usize*std::mem::size_of::<ui>()) == 0}
}/*}}}*/

}



// -- ui_array_s --
// --- struct ui_array_s method definition ---

impl ui_array_s {

fn copy_resize(&mut self,a_size:u32)
{/*{{{*/
  unsafe{ui_array_s_copy_resize(self as *mut Self,a_size as c_uint);}
}/*}}}*/

fn fill(&mut self,a_value:ui)
{/*{{{{*/
  unsafe{ui_array_s_fill(self as *mut Self,a_value);}
}/*}}}*/

fn get_idx(&self,a_value:ui) -> u32
{/*{{{*/
  unsafe{ui_array_s_get_idx(self as *const Self,a_value)}
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn to_string_separator(&self,a_trg:&mut bc_array_s,a_sep:&str)
{/*{{{*/
  unsafe{ui_array_s_to_string_separator(self as *const Self,a_trg as *mut bc_array_s,a_sep.len() as c_uint,a_sep.as_ptr() as *const c_char);}
}/*}}}*/

}

impl Cont for ui_array_s
{

fn default() -> Self
{/*{{{*/
  ui_array_s{
    size:0,
    used:0,
    data:std::ptr::null_mut(),
  }
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn to_string(&self,a_trg:&mut bc_array_s)
{/*{{{*/
  unsafe{ui_array_s___to_string(self as *const Self,a_trg as *mut bc_array_s);}
}/*}}}*/

#[cfg(OPTION_TO_JSON = "ENABLED")]
fn to_json(&self,a_trg:&mut bc_array_s)
{/*{{{*/
  unsafe{ui_array_s_to_json(self as *const Self,a_trg as *mut bc_array_s);}
}/*}}}*/

#[cfg(OPTION_TO_JSON = "ENABLED")]
fn to_json_nice(&self,a_json_nice:&mut json_nice_s,a_trg:&mut bc_array_s)
{/*{{{*/
  unsafe{ui_array_s_to_json_nice(self as *const Self,a_json_nice as *mut json_nice_s,a_trg as *mut bc_array_s);}
}/*}}}*/

}

impl Drop for ui_array_s
{
  fn drop(&mut self) {
    self.clear();
  }
}



// -- bc_arrays_s --
// --- struct bc_arrays_s definition --- 


#[repr(C)] pub struct bc_arrays_s
{
  size:u32,
  used:u32,
  data:*mut bc_array_s,
}

extern {
fn bc_arrays_s_clear(this:*mut bc_arrays_s);
fn bc_arrays_s_set(this:*mut bc_arrays_s,a_used:c_uint,a_data:*const bc_array_s);
fn bc_arrays_s_flush_all(this:*mut bc_arrays_s);
fn bc_arrays_s_copy_resize(this:*mut bc_arrays_s,a_size:c_uint);
fn bc_arrays_s_fill(this:*mut bc_arrays_s,a_value:*const bc_array_s);
fn bc_arrays_s_get_idx(this:*const bc_arrays_s,a_value:*const bc_array_s) -> c_uint;
fn bc_arrays_s_copy(this:*mut bc_arrays_s,a_src:*const bc_arrays_s);
fn bc_arrays_s_compare(this:*const bc_arrays_s,a_second:*const bc_arrays_s) -> c_int;
fn bc_arrays_s___to_string(this:*const bc_arrays_s,a_trg:*mut bc_array_s);
fn bc_arrays_s_to_string_separator(this:*const bc_arrays_s,a_trg:*mut bc_array_s,a_count:c_uint,a_data:*const c_char);
}



// -- bc_arrays_s --
// --- struct bc_arrays_s inline method definition ---

impl bc_arrays_s {

fn new() -> Self
{/*{{{*/
  <Self as Cont>::default()
}/*}}}*/

fn init(&mut self)
{/*{{{*/
  self.size = 0;
  self.used = 0;
  self.data = std::ptr::null_mut();
}/*}}}*/

fn init_size(&mut self,a_size:u32)
{/*{{{*/
  self.init();
  self.copy_resize(a_size);
}/*}}}*/

fn flush(&mut self)
{/*{{{*/
  self.copy_resize(self.used);
}/*}}}*/

fn swap(&mut self,a_second:&mut Self)
{/*{{{*/
  std::mem::swap(self,a_second);
}/*}}}*/

fn at(&mut self,a_idx:u32) -> &mut bc_array_s
{/*{{{*/
  debug_assert!(a_idx < self.used);
  unsafe{&mut *self.data.offset(a_idx as isize)}
}/*}}}*/

fn push(&mut self,a_value:&bc_array_s)
{/*{{{*/
  if self.used >= self.size
  {
    let new_size = (self.size << 1) + c_array_add;
    debug_assert!(new_size != 0);

    self.copy_resize(new_size);
  }

  unsafe{(*self.data.offset(self.used as isize)).copy(a_value);}
  self.used += 1;
}/*}}}*/

fn push_blank(&mut self)
{/*{{{*/
  if self.used >= self.size
  {
    let new_size = (self.size << 1) + c_array_add;
    debug_assert!(new_size != 0);

    self.copy_resize(new_size);
  }

  self.used += 1;
}/*}}}*/

fn reserve(&mut self,a_cnt:u32)
{/*{{{*/
  let required_cnt = self.used + a_cnt;
  if required_cnt > self.size
  {
    let mut r_size = self.size;
    loop {
      r_size = (r_size << 1) + c_array_add;
      if r_size >= required_cnt { break; }
    }

    debug_assert!(r_size != 0);
    self.copy_resize(r_size);
  }
}/*}}}*/

fn push_blanks(&mut self,a_cnt:u32)
{/*{{{*/
  let required_cnt = self.used + a_cnt;
  if required_cnt > self.size
  {
    let mut r_size = self.size;
    loop {
      r_size = (r_size << 1) + c_array_add;
      if r_size >= required_cnt { break; }
    }

    debug_assert!(r_size != 0);
    self.copy_resize(r_size);
  }

  self.used += a_cnt;
}/*}}}*/

fn push_clear(&mut self)
{/*{{{*/
  if self.used >= self.size
  {
    let new_size = (self.size << 1) + c_array_add;
    debug_assert!(new_size != 0);

    self.copy_resize(new_size);
  }

  unsafe{(*self.data.offset(self.used as isize)).clear();}
  self.used += 1;
}/*}}}*/

fn pop(&mut self) -> &mut bc_array_s
{/*{{{*/
  debug_assert!(self.used > 0);
  self.used -= 1;
  unsafe{&mut *self.data.offset(self.used as isize)}
}/*}}}*/

fn last(&self) -> &mut bc_array_s
{/*{{{*/
  debug_assert!(self.used > 0);
  unsafe{&mut *self.data.offset(self.used as isize - 1)}
}/*}}}*/

}



// -- bc_arrays_s --
// --- struct bc_arrays_s method definition ---

impl bc_arrays_s {

fn clear(&mut self)
{/*{{{*/
  unsafe{bc_arrays_s_clear(self as *mut Self);}
}/*}}}*/

fn set(&mut self,a_used:u32,a_data:*const bc_array_s)
{/*{{{*/
  unsafe{bc_arrays_s_set(self as *mut Self,a_used as c_uint,a_data);}
}/*}}}*/

fn flush_all(&mut self)
{/*{{{*/
  unsafe{bc_arrays_s_flush_all(self as *mut Self);}
}/*}}}*/

fn copy_resize(&mut self,a_size:u32)
{/*{{{*/
  unsafe{bc_arrays_s_copy_resize(self as *mut Self,a_size as c_uint);}
}/*}}}*/

fn fill(&mut self,a_value:&bc_array_s)
{/*{{{{*/
  unsafe{bc_arrays_s_fill(self as *mut Self,a_value as *const bc_array_s);}
}/*}}}*/

fn get_idx(&self,a_value:&bc_array_s) -> u32
{/*{{{*/
  unsafe{bc_arrays_s_get_idx(self as *const Self,a_value as *const bc_array_s)}
}/*}}}*/

fn copy(&mut self,a_src:&Self)
{/*{{{*/
  unsafe{bc_arrays_s_copy(self as *mut Self,a_src as *const Self);}
}/*}}}*/

fn compare(&self,a_second:&Self) -> bool
{/*{{{*/
  unsafe{bc_arrays_s_compare(self as *const Self,a_second as *const Self) != 0}
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn to_string_separator(&self,a_trg:&mut bc_array_s,a_sep:&str)
{/*{{{*/
  unsafe{bc_arrays_s_to_string_separator(self as *const Self,a_trg as *mut bc_array_s,a_sep.len() as c_uint,a_sep.as_ptr() as *const c_char);}
}/*}}}*/

}

impl Cont for bc_arrays_s
{

fn default() -> Self
{/*{{{*/
  bc_arrays_s{
    size:0,
    used:0,
    data:std::ptr::null_mut(),
  }
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn to_string(&self,a_trg:&mut bc_array_s)
{/*{{{*/
  unsafe{bc_arrays_s___to_string(self as *const Self,a_trg as *mut bc_array_s);}
}/*}}}*/

}

impl Drop for bc_arrays_s
{
  fn drop(&mut self) {
    self.clear();
  }
}



// -- record_s --
// --- struct record_s definition --- 

type rec_s = record_s;
type rec1_s = record_s;
type rec2_s = record_s;

#[repr(C)] pub struct record_s
{
  first:ui,
  second:ui,
  array0:ui_array_s,
  array1:ui_array_s,
}




// -- record_s --
// --- struct record_s inline method definition ---

impl record_s {

fn new() -> Self
{/*{{{*/
  <Self as Cont>::default()
}/*}}}*/

fn init(&mut self)
{/*{{{*/
  self.array0.init();
  self.array1.init();
}/*}}}*/

fn clear(&mut self)
{/*{{{*/
  self.array0.clear();
  self.array1.clear();
}/*}}}*/

fn set(&mut self,a_first:ui,a_second:ui,a_array0:&ui_array_s,a_array1:&ui_array_s)
{/*{{{*/
  self.first = a_first;
  self.second = a_second;
  self.array0.copy(a_array0);
  self.array1.copy(a_array1);
}/*}}}*/

fn flush_all(&mut self)
{/*{{{*/
  self.array0.flush_all();
  self.array1.flush_all();
}/*}}}*/

fn swap(&mut self,a_second:&mut Self)
{/*{{{*/
  std::mem::swap(self,a_second);
}/*}}}*/

fn copy(&mut self,a_src:&Self)
{/*{{{*/
  self.first = a_src.first;
  self.second = a_src.second;
  self.array0.copy(&a_src.array0);
  self.array1.copy(&a_src.array1);
}/*}}}*/

fn compare(&self,a_second:&Self) -> bool
{/*{{{*/
  (self.first == a_second.first &&
          self.second == a_second.second &&
          self.array0.compare(&a_second.array0) &&
          self.array1.compare(&a_second.array1))
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn to_string_separator(&self,a_trg:&mut bc_array_s,a_data:&[u8])
{/*{{{*/
  a_trg.append(b"first:");
  Cont::to_string(&self.first,a_trg);
  a_trg.append(a_data);
  a_trg.append(b"second:");
  Cont::to_string(&self.second,a_trg);
  a_trg.append(a_data);
  a_trg.append(b"array0:");
  Cont::to_string(&self.array0,a_trg);
  a_trg.append(a_data);
  a_trg.append(b"array1:");
  Cont::to_string(&self.array1,a_trg);
}/*}}}*/

}

impl Cont for record_s
{

fn default() -> Self
{/*{{{*/
  record_s{
    first:<ui as Cont>::default(),
    second:<ui as Cont>::default(),
    array0:<ui_array_s as Cont>::default(),
    array1:<ui_array_s as Cont>::default(),
  }
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn to_string(&self,a_trg:&mut bc_array_s)
{/*{{{*/
  a_trg.append(b"{first:");
  Cont::to_string(&self.first,a_trg);
  a_trg.append(b",second:");
  Cont::to_string(&self.second,a_trg);
  a_trg.append(b",array0:");
  Cont::to_string(&self.array0,a_trg);
  a_trg.append(b",array1:");
  Cont::to_string(&self.array1,a_trg);
  a_trg.push(b'}' as bc);
}/*}}}*/

#[cfg(OPTION_TO_JSON = "ENABLED")]
fn to_json(&self,a_trg:&mut bc_array_s)
{/*{{{*/
  a_trg.append(b"{\"first\":");
  Cont::to_json(&self.first,a_trg);
  a_trg.append(b",\"second\":");
  Cont::to_json(&self.second,a_trg);
  a_trg.append(b",\"array0\":");
  Cont::to_json(&self.array0,a_trg);
  a_trg.append(b",\"array1\":");
  Cont::to_json(&self.array1,a_trg);
  a_trg.push('}' as bc);
}/*}}}*/

#[cfg(OPTION_TO_JSON = "ENABLED")]
fn to_json_nice(&self,a_json_nice:&mut json_nice_s,a_trg:&mut bc_array_s)
{/*{{{*/
  a_trg.push(b'{' as bc);
  a_json_nice.push_indent(a_trg);
  a_trg.append(b"\"first\": ");
  Cont::to_json_nice(&self.first,a_json_nice,a_trg);
  a_trg.push(b',' as bc);
  a_json_nice.indent(a_trg);
  a_trg.append(b"\"second\": ");
  Cont::to_json_nice(&self.second,a_json_nice,a_trg);
  a_trg.push(b',' as bc);
  a_json_nice.indent(a_trg);
  a_trg.append(b"\"array0\": ");
  Cont::to_json_nice(&self.array0,a_json_nice,a_trg);
  a_trg.push(b',' as bc);
  a_json_nice.indent(a_trg);
  a_trg.append(b"\"array1\": ");
  Cont::to_json_nice(&self.array1,a_json_nice,a_trg);
  a_json_nice.pop_indent(a_trg);
  a_trg.push(b'}' as bc);
}/*}}}*/

}



// -- record_s --
// --- struct record_s method definition ---

impl record_s {

}

impl Drop for record_s
{
  fn drop(&mut self) {
    self.clear();
  }
}



// -- ui_tree_s --
// struct ui_tree_s definition


#[repr(C)] pub struct ui_tree_s_node
{
  parent_idx:u32,
  left_idx:u32,
  right_idx:u32,
  color:u8,
  object:ui,
}

#[repr(C)] pub struct ui_tree_s
{
  size:u32,
  used:u32,
  data:*mut ui_tree_s_node,
  free_idx:u32,
  root_idx:u32,
  leaf_idx:u32,
  first:ui,
  second:ui_array_s,
}

extern {
fn ui_tree_s_get_stack_min_value_idx(this:*const ui_tree_s,a_idx:c_uint,a_s_ptr:*mut *mut c_uint) -> c_uint;
fn ui_tree_s_get_min_value_idx(this:*const ui_tree_s,a_idx:c_uint) -> c_uint;
fn ui_tree_s_get_max_value_idx(this:*const ui_tree_s,a_idx:c_uint) -> c_uint;
fn ui_tree_s_get_next_idx(this:*const ui_tree_s,a_idx:c_uint) -> c_uint;
fn ui_tree_s_get_prev_idx(this:*const ui_tree_s,a_idx:c_uint) -> c_uint;
fn ui_tree_s___binary_tree_insert(this:*mut ui_tree_s,a_new_idx:c_uint,a_value:*const ui,a_unique:c_int) -> c_uint;
fn ui_tree_s___remove_black_black(this:*mut ui_tree_s,a_idx:c_uint);
fn ui_tree_s___insert_operation(this:*mut ui_tree_s,a_idx:c_uint);
fn ui_tree_s_remove(this:*mut ui_tree_s,a_idx:c_uint);
fn ui_tree_s_copy_resize(this:*mut ui_tree_s,a_size:c_uint);
fn ui_tree_s_get_idx(this:*const ui_tree_s,a_value:ui) -> c_uint;
fn ui_tree_s_get_idx_left(this:*const ui_tree_s,a_value:ui) -> c_uint;
fn ui_tree_s_get_gre_idx(this:*const ui_tree_s,a_value:ui) -> c_uint;
fn ui_tree_s_get_lee_idx(this:*const ui_tree_s,a_value:ui) -> c_uint;
fn ui_tree_s_get_idxs(this:*const ui_tree_s,a_value:ui,a_idxs_array:*mut ui_array_s);
fn ui_tree_s_compare(this:*const ui_tree_s,a_second:*const ui_tree_s) -> c_int;
fn ui_tree_s___to_string(this:*const ui_tree_s,a_trg:*mut bc_array_s);
fn ui_tree_s_to_string_separator(this:*const ui_tree_s,a_trg:*mut bc_array_s,a_count:c_uint,a_data:*const c_char);
}



// -- ui_tree_s --
// --- struct ui_tree_s inline method definition ---

impl ui_tree_s {

fn new() -> Self
{/*{{{*/
  <Self as Cont>::default()
}/*}}}*/

fn __get_grandparent_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  let node = unsafe{&*self.data.offset(a_idx as isize)};

  if node.parent_idx != c_idx_not_exist
  {
    unsafe{&*self.data.offset(node.parent_idx as isize)}.parent_idx
  }
  else
  {
    c_idx_not_exist
  }
}/*}}}*/

fn __get_uncle_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  let gp_idx = self.__get_grandparent_idx(a_idx);

  if gp_idx != c_idx_not_exist
  {
    let gp = unsafe{&*self.data.offset(gp_idx as isize)};
    if gp.left_idx == unsafe{&*self.data.offset(a_idx as isize)}.parent_idx
    {
      gp.right_idx
    }
    else
    {
      gp.left_idx
    }
  }
  else
  {
    c_idx_not_exist
  }
}/*}}}*/

fn __get_sibling_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  let p = unsafe{&*self.data.offset((*self.data.offset(a_idx as isize)).parent_idx as isize)};
  if p.left_idx == a_idx { p.right_idx } else { p.left_idx }
}/*}}}*/

fn get_descent_stack_size(&self) -> u32
{/*{{{*/
  (((self.used as f32).ln()/c_log_of_2) as u32) << 1
}/*}}}*/

fn get_stack_next_idx(&self,a_idx:u32,a_s_ptr:*mut *mut u32,a_stack_base:*const u32) -> u32
{/*{{{*/
  debug_assert!(a_idx < self.used);

  let node = unsafe{&*self.data.offset(a_idx as isize)};

  if node.right_idx != self.leaf_idx
  {
    self.get_stack_min_value_idx(node.right_idx,a_s_ptr)
  }
  else
  {
    if unsafe{*a_s_ptr} > a_stack_base as *mut u32
    {
      unsafe{
        (*a_s_ptr) = (*a_s_ptr).offset(-1);
        *(*a_s_ptr)
      }
    }
    else
    {
      c_idx_not_exist
    }
  }
}/*}}}*/

fn __rotate_left(&mut self,a_idx:u32)
{/*{{{*/
  let root = unsafe{&mut *self.data.offset(a_idx as isize)};
  let pivot = unsafe{&mut *self.data.offset(root.right_idx as isize)};

  if a_idx == self.root_idx
  {
    self.root_idx = root.right_idx;
    pivot.parent_idx = c_idx_not_exist;
  }
  else
  {
    let rp = unsafe{&mut *self.data.offset(root.parent_idx as isize)};

    if rp.right_idx == a_idx
    {
      rp.right_idx = root.right_idx;
    }
    else
    {
      rp.left_idx = root.right_idx;
    }

    pivot.parent_idx = root.parent_idx;
  }

  root.parent_idx = root.right_idx;

  root.right_idx = pivot.left_idx;
  unsafe{&mut *self.data.offset(root.right_idx as isize)}.parent_idx = a_idx;

  pivot.left_idx = a_idx;
}/*}}}*/

fn __rotate_right(&mut self,a_idx:u32)
{/*{{{*/
  let root = unsafe{&mut *self.data.offset(a_idx as isize)};
  let pivot = unsafe{&mut *self.data.offset(root.left_idx as isize)};

  if a_idx == self.root_idx
  {
    self.root_idx = root.left_idx;
    pivot.parent_idx = c_idx_not_exist;
  }
  else
  {
    let rp = unsafe{&mut *self.data.offset(root.parent_idx as isize)};

    if rp.right_idx == a_idx
    {
      rp.right_idx = root.left_idx;
    }
    else
    {
      rp.left_idx = root.left_idx;
    }

    pivot.parent_idx = root.parent_idx;
  }

  root.parent_idx = root.left_idx;

  root.left_idx = pivot.right_idx;
  unsafe{&mut *self.data.offset(root.left_idx as isize)}.parent_idx = a_idx;

  pivot.right_idx = a_idx;
}/*}}}*/

fn __get_new_index(&mut self) -> u32
{/*{{{*/
  let new_idx;

  if self.free_idx != c_idx_not_exist
  {
    new_idx = self.free_idx;
    self.free_idx = unsafe{&*self.data.offset(new_idx as isize)}.parent_idx;
  }
  else
  {
    if self.used >= self.size
    {
      let new_size = (self.size << 1) + c_array_add;
      debug_assert!(new_size != 0);

      unsafe{ui_tree_s_copy_resize(self as *mut Self,new_size)};
    }

    if self.leaf_idx == c_idx_not_exist
    {
      self.leaf_idx = self.used;
      self.used += 1;
      let leaf = unsafe{&mut *self.data.offset(self.leaf_idx as isize)};

      leaf.color = 1;
    }

    new_idx = self.used;
    self.used += 1;
  }

  new_idx
}/*}}}*/

fn __replace_delete_node_by_child(&mut self,a_idx:u32,a_ch_idx:u32)
{/*{{{*/
  let node = unsafe{&*self.data.offset(a_idx as isize)};

  if node.parent_idx != c_idx_not_exist
  {
    let parent = unsafe{&mut *self.data.offset(node.parent_idx as isize)};

    if parent.left_idx == a_idx
    {
      parent.left_idx = a_ch_idx;
    }
    else
    {
      parent.right_idx = a_ch_idx;
    }

    unsafe{&mut *self.data.offset(a_ch_idx as isize)}.parent_idx = node.parent_idx;
  }
  else
  {
    self.root_idx = if a_ch_idx == self.leaf_idx { c_idx_not_exist } else { a_ch_idx };
    unsafe{&mut *self.data.offset(a_ch_idx as isize)}.parent_idx = c_idx_not_exist;
  }
}/*}}}*/

fn __remove_one_child(&mut self,a_idx:u32,a_ch_idx:u32)
{/*{{{*/
  let node = unsafe{&mut *self.data.offset(a_idx as isize)};
  self.__replace_delete_node_by_child(a_idx,a_ch_idx);

  node.parent_idx = self.free_idx;
  self.free_idx = a_idx;

  if node.color != 0
  {
    let child_node = unsafe{&mut *self.data.offset(a_ch_idx as isize)};

    if child_node.color == 0
    {
      child_node.color = 1;
    }
    else
    {
      self.__remove_black_black(a_ch_idx);
    }
  }
}/*}}}*/

fn init(&mut self)
{/*{{{*/
  self.size = 0;
  self.used = 0;
  self.data = std::ptr::null_mut();
  self.free_idx = c_idx_not_exist;
  self.root_idx = c_idx_not_exist;
  self.leaf_idx = c_idx_not_exist;

  self.second.init();
}/*}}}*/

fn clear(&mut self)
{/*{{{*/
  if self.data != std::ptr::null_mut()
  {
    unsafe{free(self.data as *mut c_void);}
  }

  self.second.clear();

  self.size = 0;
  self.used = 0;
  self.data = std::ptr::null_mut();
  self.free_idx = c_idx_not_exist;
  self.root_idx = c_idx_not_exist;
  self.leaf_idx = c_idx_not_exist;
}/*}}}*/

fn flush(&mut self)
{/*{{{*/
  self.copy_resize(self.used);
}/*}}}*/

fn flush_all(&mut self)
{/*{{{*/
  self.copy_resize(self.used);

  self.second.flush_all();
}/*}}}*/

fn swap(&mut self,a_second:&mut Self)
{/*{{{*/
  std::mem::swap(self,a_second);
}/*}}}*/

fn at(&self,a_idx:u32) -> &mut ui
{/*{{{*/
  debug_assert!(a_idx < self.used);
  unsafe{&mut (*self.data.offset(a_idx as isize)).object}
}/*}}}*/

fn insert(&mut self,a_value:ui) -> u32
{/*{{{*/
  let new_node_idx = self.__get_new_index();

  self.__binary_tree_insert(new_node_idx,&a_value,false);
  self.__insert_operation(new_node_idx);

  unsafe{(*self.data.offset(new_node_idx as isize)).object = a_value};

  new_node_idx
}/*}}}*/

fn unique_insert(&mut self,a_value:ui) -> u32
{/*{{{*/
  let new_node_idx = self.__get_new_index();
  let old_node_idx = self.__binary_tree_insert(new_node_idx,&a_value,true);

  if old_node_idx != c_idx_not_exist
  {
    let new_node = unsafe {&mut *self.data.offset(new_node_idx as isize)};

    new_node.parent_idx = self.free_idx;
    self.free_idx = new_node_idx;

    return old_node_idx;
  }

  self.__insert_operation(new_node_idx);

  unsafe{(*self.data.offset(new_node_idx as isize)).object = a_value};

  new_node_idx
}/*}}}*/

fn copy(&mut self,a_src:&Self)
{/*{{{*/
  self.clear();

  if self.root_idx == c_idx_not_exist
  {
    return;
  }

  debug_assert!(a_src.used != 0);
  self.copy_resize(a_src.used);

  unsafe{std::ptr::copy_nonoverlapping(a_src.data,self.data,a_src.used as usize);}

  self.used = a_src.used;
  self.free_idx = a_src.free_idx;
  self.root_idx = a_src.root_idx;
  self.leaf_idx = a_src.leaf_idx;

  self.first = a_src.first;
  self.second.copy(&a_src.second);
}/*}}}*/

}



// -- ui_tree_s --
// --- struct ui_tree_s method definition ---

impl ui_tree_s {

fn get_stack_min_value_idx(&self,a_idx:u32,a_s_ptr:*mut *mut u32) -> u32
{/*{{{*/
  unsafe{ ui_tree_s_get_stack_min_value_idx(self as *const Self,a_idx,a_s_ptr) }
}/*}}}*/

fn get_min_value_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  unsafe{ui_tree_s_get_min_value_idx(self as *const Self,a_idx)}
}/*}}}*/

fn get_max_value_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  unsafe{ui_tree_s_get_max_value_idx(self as *const Self,a_idx)}
}/*}}}*/

fn get_next_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  unsafe{ui_tree_s_get_next_idx(self as *const Self,a_idx)}
}/*}}}*/

fn get_prev_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  unsafe{ui_tree_s_get_prev_idx(self as *const Self,a_idx)}
}/*}}}*/

fn __binary_tree_insert(&mut self,a_new_idx:u32,a_value:&ui,a_unique:bool) -> u32
{/*{{{*/
  unsafe{ui_tree_s___binary_tree_insert(self as *mut Self,a_new_idx,a_value as *const ui,a_unique as i32)}
}/*}}}*/

fn __remove_black_black(&mut self,a_idx:u32)
{/*{{{*/
  unsafe{ui_tree_s___remove_black_black(self as *mut Self,a_idx)}
}/*}}}*/

fn __insert_operation(&mut self,a_idx:u32)
{/*{{{*/
  unsafe{ui_tree_s___insert_operation(self as *mut Self,a_idx)}
}/*}}}*/

fn remove(&mut self,a_idx:u32)
{/*{{{*/
  unsafe{ui_tree_s_remove(self as *mut Self,a_idx)}
}/*}}}*/

fn copy_resize(&mut self,a_size:u32)
{/*{{{*/
  unsafe{ui_tree_s_copy_resize(self as *mut Self,a_size)}
}/*}}}*/

fn get_idx(&self,a_value:ui) -> u32
{/*{{{*/
  unsafe{ui_tree_s_get_idx(self as *const Self,a_value)}
}/*}}}*/

fn get_idx_left(&self,a_value:ui) -> u32
{/*{{{*/
  unsafe{ui_tree_s_get_idx_left(self as *const Self,a_value)}
}/*}}}*/

fn get_gre_idx(&self,a_value:ui) -> u32
{/*{{{*/
  unsafe{ui_tree_s_get_gre_idx(self as *const Self,a_value)}
}/*}}}*/

fn get_lee_idx(&self,a_value:ui) -> u32
{/*{{{*/
  unsafe{ui_tree_s_get_lee_idx(self as *const Self,a_value)}
}/*}}}*/

fn get_idxs(&self,a_value:ui,a_idxs_array:&mut ui_array_s)
{/*{{{*/
  unsafe{ui_tree_s_get_idxs(self as *const Self,a_value,a_idxs_array as *mut ui_array_s)}
}/*}}}*/

fn compare(&self,a_second:&ui_tree_s) -> bool
{/*{{{*/
  unsafe{ui_tree_s_compare(self as *const Self,a_second as *const Self) != 0}
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn __to_string(&self,a_trg:&mut bc_array_s)
{/*{{{*/
  unsafe{ui_tree_s___to_string(self as *const Self,a_trg as *mut bc_array_s);}
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn to_string_separator(&self,a_trg:&mut bc_array_s,a_sep:&str)
{/*{{{*/
  unsafe{ui_tree_s_to_string_separator(self as *const Self,a_trg as *mut bc_array_s,a_sep.len() as c_uint,a_sep.as_ptr() as *const c_char);}
}/*}}}*/

}

impl Cont for ui_tree_s
{

fn default() -> Self
{/*{{{*/
  ui_tree_s{
    size:0,
    used:0,
    data:std::ptr::null_mut(),
    free_idx:c_idx_not_exist,
    root_idx:c_idx_not_exist,
    leaf_idx:c_idx_not_exist,

    first:<ui as Cont>::default(),
    second:<ui_array_s as Cont>::default(),
  }
}/*}}}*/

}

impl Drop for ui_tree_s
{
  fn drop(&mut self) {
    self.clear();
  }
}



// -- record_tree_s --
// struct record_tree_s definition


#[repr(C)] pub struct record_tree_s_node
{
  parent_idx:u32,
  left_idx:u32,
  right_idx:u32,
  color:u8,
  valid:u8,
  object:record_s,
}

#[repr(C)] pub struct record_tree_s
{
  size:u32,
  used:u32,
  count:u32,
  data:*mut record_tree_s_node,
  free_idx:u32,
  root_idx:u32,
  leaf_idx:u32,
}

extern {
fn record_tree_s_get_stack_min_value_idx(this:*const record_tree_s,a_idx:c_uint,a_s_ptr:*mut *mut c_uint) -> c_uint;
fn record_tree_s_get_min_value_idx(this:*const record_tree_s,a_idx:c_uint) -> c_uint;
fn record_tree_s_get_max_value_idx(this:*const record_tree_s,a_idx:c_uint) -> c_uint;
fn record_tree_s_get_next_idx(this:*const record_tree_s,a_idx:c_uint) -> c_uint;
fn record_tree_s_get_prev_idx(this:*const record_tree_s,a_idx:c_uint) -> c_uint;
fn record_tree_s___binary_tree_insert(this:*mut record_tree_s,a_new_idx:c_uint,a_value:*const record_s,a_unique:c_int) -> c_uint;
fn record_tree_s___remove_black_black(this:*mut record_tree_s,a_idx:c_uint);
fn record_tree_s___insert_operation(this:*mut record_tree_s,a_idx:c_uint);
fn record_tree_s_clear(this:*mut record_tree_s);
fn record_tree_s_flush_all(this:*mut record_tree_s);
fn record_tree_s_remove(this:*mut record_tree_s,a_idx:c_uint);
fn record_tree_s_copy_resize(this:*mut record_tree_s,a_size:c_uint);
fn record_tree_s_get_idx(this:*const record_tree_s,a_value:*const record_s) -> c_uint;
fn record_tree_s_get_idx_left(this:*const record_tree_s,a_value:*const record_s) -> c_uint;
fn record_tree_s_get_gre_idx(this:*const record_tree_s,a_value:*const record_s) -> c_uint;
fn record_tree_s_get_lee_idx(this:*const record_tree_s,a_value:*const record_s) -> c_uint;
fn record_tree_s_get_idxs(this:*const record_tree_s,a_value:*const record_s,a_idxs_array:*mut ui_array_s);
fn record_tree_s_copy(this:*mut record_tree_s,a_src:*const record_tree_s);
fn record_tree_s_compare(this:*const record_tree_s,a_second:*const record_tree_s) -> c_int;
fn record_tree_s___to_string(this:*const record_tree_s,a_trg:*mut bc_array_s);
fn record_tree_s_to_string_separator(this:*const record_tree_s,a_trg:*mut bc_array_s,a_count:c_uint,a_data:*const c_char);
}



// -- record_tree_s --
// --- struct record_tree_s inline method definition ---

impl record_tree_s {

fn new() -> Self
{/*{{{*/
  <Self as Cont>::default()
}/*}}}*/

fn __get_grandparent_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  let node = unsafe{&*self.data.offset(a_idx as isize)};

  if node.parent_idx != c_idx_not_exist
  {
    unsafe{&*self.data.offset(node.parent_idx as isize)}.parent_idx
  }
  else
  {
    c_idx_not_exist
  }
}/*}}}*/

fn __get_uncle_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  let gp_idx = self.__get_grandparent_idx(a_idx);

  if gp_idx != c_idx_not_exist
  {
    let gp = unsafe{&*self.data.offset(gp_idx as isize)};
    if gp.left_idx == unsafe{&*self.data.offset(a_idx as isize)}.parent_idx
    {
      gp.right_idx
    }
    else
    {
      gp.left_idx
    }
  }
  else
  {
    c_idx_not_exist
  }
}/*}}}*/

fn __get_sibling_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  let p = unsafe{&*self.data.offset((*self.data.offset(a_idx as isize)).parent_idx as isize)};
  if p.left_idx == a_idx { p.right_idx } else { p.left_idx }
}/*}}}*/

fn get_descent_stack_size(&self) -> u32
{/*{{{*/
  (((self.used as f32).ln()/c_log_of_2) as u32) << 1
}/*}}}*/

fn get_stack_next_idx(&self,a_idx:u32,a_s_ptr:*mut *mut u32,a_stack_base:*const u32) -> u32
{/*{{{*/
  debug_assert!(a_idx < self.used && unsafe{&*self.data.offset(a_idx as isize)}.valid != 0);

  let node = unsafe{&*self.data.offset(a_idx as isize)};

  if node.right_idx != self.leaf_idx
  {
    self.get_stack_min_value_idx(node.right_idx,a_s_ptr)
  }
  else
  {
    if unsafe{*a_s_ptr} > a_stack_base as *mut u32
    {
      unsafe{
        (*a_s_ptr) = (*a_s_ptr).offset(-1);
        *(*a_s_ptr)
      }
    }
    else
    {
      c_idx_not_exist
    }
  }
}/*}}}*/

fn __rotate_left(&mut self,a_idx:u32)
{/*{{{*/
  let root = unsafe{&mut *self.data.offset(a_idx as isize)};
  let pivot = unsafe{&mut *self.data.offset(root.right_idx as isize)};

  if a_idx == self.root_idx
  {
    self.root_idx = root.right_idx;
    pivot.parent_idx = c_idx_not_exist;
  }
  else
  {
    let rp = unsafe{&mut *self.data.offset(root.parent_idx as isize)};

    if rp.right_idx == a_idx
    {
      rp.right_idx = root.right_idx;
    }
    else
    {
      rp.left_idx = root.right_idx;
    }

    pivot.parent_idx = root.parent_idx;
  }

  root.parent_idx = root.right_idx;

  root.right_idx = pivot.left_idx;
  unsafe{&mut *self.data.offset(root.right_idx as isize)}.parent_idx = a_idx;

  pivot.left_idx = a_idx;
}/*}}}*/

fn __rotate_right(&mut self,a_idx:u32)
{/*{{{*/
  let root = unsafe{&mut *self.data.offset(a_idx as isize)};
  let pivot = unsafe{&mut *self.data.offset(root.left_idx as isize)};

  if a_idx == self.root_idx
  {
    self.root_idx = root.left_idx;
    pivot.parent_idx = c_idx_not_exist;
  }
  else
  {
    let rp = unsafe{&mut *self.data.offset(root.parent_idx as isize)};

    if rp.right_idx == a_idx
    {
      rp.right_idx = root.left_idx;
    }
    else
    {
      rp.left_idx = root.left_idx;
    }

    pivot.parent_idx = root.parent_idx;
  }

  root.parent_idx = root.left_idx;

  root.left_idx = pivot.right_idx;
  unsafe{&mut *self.data.offset(root.left_idx as isize)}.parent_idx = a_idx;

  pivot.right_idx = a_idx;
}/*}}}*/

fn __get_new_index(&mut self) -> u32
{/*{{{*/
  let new_idx;

  if self.free_idx != c_idx_not_exist
  {
    new_idx = self.free_idx;
    self.free_idx = unsafe{&*self.data.offset(new_idx as isize)}.parent_idx;
  }
  else
  {
    if self.used >= self.size
    {
      let new_size = (self.size << 1) + c_array_add;
      debug_assert!(new_size != 0);

      unsafe{record_tree_s_copy_resize(self as *mut Self,new_size)};
    }

    if self.leaf_idx == c_idx_not_exist
    {
      self.leaf_idx = self.used;
      self.used += 1;
      let leaf = unsafe{&mut *self.data.offset(self.leaf_idx as isize)};

      leaf.valid = 0;
      leaf.color = 1;
    }

    new_idx = self.used;
    self.used += 1;
  }

  unsafe{&mut *self.data.offset(new_idx as isize)}.valid = 1;
  self.count += 1;

  new_idx
}/*}}}*/

fn __replace_delete_node_by_child(&mut self,a_idx:u32,a_ch_idx:u32)
{/*{{{*/
  let node = unsafe{&*self.data.offset(a_idx as isize)};

  if node.parent_idx != c_idx_not_exist
  {
    let parent = unsafe{&mut *self.data.offset(node.parent_idx as isize)};

    if parent.left_idx == a_idx
    {
      parent.left_idx = a_ch_idx;
    }
    else
    {
      parent.right_idx = a_ch_idx;
    }

    unsafe{&mut *self.data.offset(a_ch_idx as isize)}.parent_idx = node.parent_idx;
  }
  else
  {
    self.root_idx = if a_ch_idx == self.leaf_idx { c_idx_not_exist } else { a_ch_idx };
    unsafe{&mut *self.data.offset(a_ch_idx as isize)}.parent_idx = c_idx_not_exist;
  }
}/*}}}*/

fn __remove_one_child(&mut self,a_idx:u32,a_ch_idx:u32)
{/*{{{*/
  let node = unsafe{&mut *self.data.offset(a_idx as isize)};
  self.__replace_delete_node_by_child(a_idx,a_ch_idx);

  node.parent_idx = self.free_idx;
  self.free_idx = a_idx;

  node.valid = 0;
  self.count -= 1;

  if node.color != 0
  {
    let child_node = unsafe{&mut *self.data.offset(a_ch_idx as isize)};

    if child_node.color == 0
    {
      child_node.color = 1;
    }
    else
    {
      self.__remove_black_black(a_ch_idx);
    }
  }
}/*}}}*/

fn init(&mut self)
{/*{{{*/
  self.size = 0;
  self.used = 0;
  self.count = 0;
  self.data = std::ptr::null_mut();
  self.free_idx = c_idx_not_exist;
  self.root_idx = c_idx_not_exist;
  self.leaf_idx = c_idx_not_exist;
}/*}}}*/

fn flush(&mut self)
{/*{{{*/
  self.copy_resize(self.used);
}/*}}}*/

fn swap(&mut self,a_second:&mut Self)
{/*{{{*/
  std::mem::swap(self,a_second);
}/*}}}*/

fn at(&self,a_idx:u32) -> &mut record_s
{/*{{{*/
  debug_assert!(a_idx < self.used && unsafe{(*self.data.offset(a_idx as isize)).valid} != 0);
  unsafe{&mut (*self.data.offset(a_idx as isize)).object}
}/*}}}*/

fn insert(&mut self,a_value:&record_s) -> u32
{/*{{{*/
  let new_node_idx = self.__get_new_index();

  self.__binary_tree_insert(new_node_idx,a_value,false);
  self.__insert_operation(new_node_idx);

  unsafe{(*self.data.offset(new_node_idx as isize)).object.copy(a_value)};

  new_node_idx
}/*}}}*/

fn swap_insert(&mut self,a_value:&mut record_s) -> u32
{/*{{{*/
  let new_node_idx = self.__get_new_index();

  self.__binary_tree_insert(new_node_idx,a_value,false);
  self.__insert_operation(new_node_idx);

  unsafe{(*self.data.offset(new_node_idx as isize)).object.swap(a_value)};

  new_node_idx
}/*}}}*/

fn unique_insert(&mut self,a_value:&record_s) -> u32
{/*{{{*/
  let new_node_idx = self.__get_new_index();
  let old_node_idx = self.__binary_tree_insert(new_node_idx,a_value,true);

  if old_node_idx != c_idx_not_exist
  {
    let new_node = unsafe {&mut *self.data.offset(new_node_idx as isize)};

    new_node.parent_idx = self.free_idx;
    self.free_idx = new_node_idx;

    new_node.valid = 0;
    self.count -= 1;

    return old_node_idx;
  }

  self.__insert_operation(new_node_idx);

  unsafe{(*self.data.offset(new_node_idx as isize)).object.copy(a_value)};

  new_node_idx
}/*}}}*/

fn unique_swap_insert(&mut self,a_value:&mut record_s) -> u32
{/*{{{*/
  let new_node_idx = self.__get_new_index();
  let old_node_idx = self.__binary_tree_insert(new_node_idx,a_value,true);

  if old_node_idx != c_idx_not_exist
  {
    let new_node = unsafe {&mut *self.data.offset(new_node_idx as isize)};

    new_node.parent_idx = self.free_idx;
    self.free_idx = new_node_idx;

    new_node.valid = 0;
    self.count -= 1;

    return old_node_idx;
  }

  self.__insert_operation(new_node_idx);

  unsafe{(*self.data.offset(new_node_idx as isize)).object.swap(a_value)};

  new_node_idx
}/*}}}*/

}



// -- record_tree_s --
// --- struct record_tree_s method definition ---

impl record_tree_s {

fn get_stack_min_value_idx(&self,a_idx:u32,a_s_ptr:*mut *mut u32) -> u32
{/*{{{*/
  unsafe{ record_tree_s_get_stack_min_value_idx(self as *const Self,a_idx,a_s_ptr) }
}/*}}}*/

fn get_min_value_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  unsafe{record_tree_s_get_min_value_idx(self as *const Self,a_idx)}
}/*}}}*/

fn get_max_value_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  unsafe{record_tree_s_get_max_value_idx(self as *const Self,a_idx)}
}/*}}}*/

fn get_next_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  unsafe{record_tree_s_get_next_idx(self as *const Self,a_idx)}
}/*}}}*/

fn get_prev_idx(&self,a_idx:u32) -> u32
{/*{{{*/
  unsafe{record_tree_s_get_prev_idx(self as *const Self,a_idx)}
}/*}}}*/

fn __binary_tree_insert(&mut self,a_new_idx:u32,a_value:&record_s,a_unique:bool) -> u32
{/*{{{*/
  unsafe{record_tree_s___binary_tree_insert(self as *mut Self,a_new_idx,a_value as *const record_s,a_unique as i32)}
}/*}}}*/

fn __remove_black_black(&mut self,a_idx:u32)
{/*{{{*/
  unsafe{record_tree_s___remove_black_black(self as *mut Self,a_idx)}
}/*}}}*/

fn __insert_operation(&mut self,a_idx:u32)
{/*{{{*/
  unsafe{record_tree_s___insert_operation(self as *mut Self,a_idx)}
}/*}}}*/

fn clear(&mut self)
{/*{{{*/
  unsafe{record_tree_s_clear(self as *mut Self);}
}/*}}}*/

fn flush_all(&mut self)
{/*{{{*/
  unsafe{record_tree_s_flush_all(self as *mut Self)}
}/*}}}*/

fn remove(&mut self,a_idx:u32)
{/*{{{*/
  unsafe{record_tree_s_remove(self as *mut Self,a_idx)}
}/*}}}*/

fn copy_resize(&mut self,a_size:u32)
{/*{{{*/
  unsafe{record_tree_s_copy_resize(self as *mut Self,a_size)}
}/*}}}*/

fn get_idx(&self,a_value:&record_s) -> u32
{/*{{{*/
  unsafe{record_tree_s_get_idx(self as *const Self,a_value as *const record_s)}
}/*}}}*/

fn get_idx_left(&self,a_value:&record_s) -> u32
{/*{{{*/
  unsafe{record_tree_s_get_idx_left(self as *const Self,a_value as *const record_s)}
}/*}}}*/

fn get_gre_idx(&self,a_value:&record_s) -> u32
{/*{{{*/
  unsafe{record_tree_s_get_gre_idx(self as *const Self,a_value as *const record_s)}
}/*}}}*/

fn get_lee_idx(&self,a_value:&record_s) -> u32
{/*{{{*/
  unsafe{record_tree_s_get_lee_idx(self as *const Self,a_value as *const record_s)}
}/*}}}*/

fn get_idxs(&self,a_value:*const record_s,a_idxs_array:&mut ui_array_s)
{/*{{{*/
  unsafe{record_tree_s_get_idxs(self as *const Self,a_value,a_idxs_array as *mut ui_array_s)}
}/*}}}*/

fn copy(&mut self,a_src:&Self)
{/*{{{*/
  unsafe{record_tree_s_copy(self as *mut Self,a_src as *const Self);}
}/*}}}*/

fn compare(&self,a_second:&record_tree_s) -> bool
{/*{{{*/
  unsafe{record_tree_s_compare(self as *const Self,a_second as *const Self) != 0}
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn __to_string(&self,a_trg:&mut bc_array_s)
{/*{{{*/
  unsafe{record_tree_s___to_string(self as *const Self,a_trg as *mut bc_array_s);}
}/*}}}*/

#[cfg(OPTION_TO_STRING = "ENABLED")]
fn to_string_separator(&self,a_trg:&mut bc_array_s,a_sep:&str)
{/*{{{*/
  unsafe{record_tree_s_to_string_separator(self as *const Self,a_trg as *mut bc_array_s,a_sep.len() as c_uint,a_sep.as_ptr() as *const c_char);}
}/*}}}*/

}

impl Cont for record_tree_s
{

fn default() -> Self
{/*{{{*/
  record_tree_s{
    size:0,
    used:0,
    count:0,
    data:std::ptr::null_mut(),
    free_idx:c_idx_not_exist,
    root_idx:c_idx_not_exist,
    leaf_idx:c_idx_not_exist,
  }
}/*}}}*/

}

impl Drop for record_tree_s
{
  fn drop(&mut self) {
    self.clear();
  }
}



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
