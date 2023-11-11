#![allow(dead_code)]

use std::cmp::{PartialEq,Eq,PartialOrd,Ord,Ordering};
use std::fmt::{Debug,Display,Formatter};

const IDX_NOT_EXIST:u32 = std::u32::MAX;

struct ListElement<T> {
    next_idx:u32,
    prev_idx:u32,
    value:T,
}

pub struct List<T> {
    free_idx:u32,
    first_idx:u32,
    last_idx:u32,
    count:u32,
    data:Vec<ListElement<T>>,
}

pub struct ListIter<'a,T> {
    list:&'a List<T>,
    idx:u32,
}

impl<T> List<T> {
    pub fn new() -> List<T>
    {//{{{
        List{
            free_idx: IDX_NOT_EXIST,
            first_idx:IDX_NOT_EXIST,
            last_idx: IDX_NOT_EXIST,
            count:0,
            data:vec![],
        }
    }//}}}

    pub fn from(src:Vec<T>) -> List<T>
    {//{{{
        let mut data:Vec<ListElement<T>> = vec![];

        if src.is_empty() {
            List::<T>::new()
        }
        else {
            let count = src.len() as u32;
            let mut prev_idx = IDX_NOT_EXIST;

            for value in src {
                let idx = data.len() as u32;

                data.push(ListElement{
                    next_idx:idx + 1,
                    prev_idx:prev_idx,
                    value:value,
                });

                prev_idx = idx;
            }

            let last_idx = data.len() as u32 - 1;
            data[last_idx as usize].next_idx = IDX_NOT_EXIST;

            List{
                free_idx:IDX_NOT_EXIST,
                first_idx:0,
                last_idx:last_idx,
                count:count,
                data:data,
            }
        }
    }//}}}

    pub fn len(&self) -> u32
    {//{{{
        self.count
    }//}}}

    pub fn prepend(&mut self,value:T) -> u32
    {//{{{
        let new_idx:u32;

        if self.free_idx != IDX_NOT_EXIST {
            new_idx = self.free_idx;
            self.free_idx = self.data[new_idx as usize].next_idx;

            self.data[new_idx as usize] = ListElement{
                next_idx:self.first_idx,
                prev_idx:IDX_NOT_EXIST,
                value:value,
            }
        }
        else {
            new_idx = self.data.len() as u32;
            self.data.push(ListElement{
                next_idx:self.first_idx,
                prev_idx:IDX_NOT_EXIST,
                value:value,
            });
        }

        self.first_idx = new_idx;
        self.count += 1;

        new_idx
    }//}}}

    pub fn append(&mut self,value:T) -> u32
    {//{{{
        let new_idx;

        if self.free_idx != IDX_NOT_EXIST {
            new_idx = self.free_idx;
            self.free_idx = self.data[new_idx as usize].next_idx;

            self.data[new_idx as usize] = ListElement{
                next_idx:IDX_NOT_EXIST,
                prev_idx:self.last_idx,
                value:value,
            }
        }
        else {
            new_idx = self.data.len() as u32;
            self.data.push(ListElement{
                next_idx:IDX_NOT_EXIST,
                prev_idx:self.last_idx,
                value:value,
            });
        }

        if self.last_idx != IDX_NOT_EXIST {
            self.data[self.last_idx as usize].next_idx = new_idx;
        }
        else {
            self.first_idx = new_idx;
        }

        self.last_idx = new_idx;
        self.count += 1;

        new_idx
    }//}}}

    pub fn insert_before(&mut self,idx:u32,value:T) -> u32
    {//{{{
        debug_assert!(idx < self.data.len() as u32);

        let new_idx;
        let idx_element_prev_idx = self.data[idx as usize].prev_idx;

        if self.free_idx != IDX_NOT_EXIST {
            new_idx = self.free_idx;
            self.free_idx = self.data[new_idx as usize].next_idx;

            self.data[new_idx as usize] = ListElement{
                next_idx:idx,
                prev_idx:idx_element_prev_idx,
                value:value,
            }
        }
        else {
            new_idx = self.data.len() as u32;
            self.data.push(ListElement{
                next_idx:idx,
                prev_idx:idx_element_prev_idx,
                value:value,
            });
        }

        if idx_element_prev_idx != IDX_NOT_EXIST {
            self.data[idx_element_prev_idx as usize].next_idx = new_idx;
        }
        else {
            self.first_idx = new_idx;
        }

        self.data[idx as usize].prev_idx = new_idx;
        self.count += 1;

        new_idx
    }//}}}

    pub fn insert_after(&mut self,idx:u32,value:T) -> u32
    {//{{{
        debug_assert!(idx < self.data.len() as u32);

        let new_idx;
        let idx_element_next_idx = self.data[idx as usize].next_idx;

        if self.free_idx != IDX_NOT_EXIST {
            new_idx = self.free_idx;
            self.free_idx = self.data[new_idx as usize].next_idx;

            self.data[new_idx as usize] = ListElement{
                next_idx:idx_element_next_idx,
                prev_idx:idx,
                value:value,
            };
        }
        else {
            new_idx = self.data.len() as u32;
            self.data.push(ListElement{
                next_idx:idx_element_next_idx,
                prev_idx:idx,
                value:value,
            });
        }

        if idx_element_next_idx != IDX_NOT_EXIST {
            self.data[idx_element_next_idx as usize].prev_idx = new_idx;
        }
        else {
            self.last_idx = new_idx;
        }

        self.data[idx as usize].next_idx = new_idx;
        self.count += 1;

        new_idx
    }//}}}

    pub fn remove(&mut self,idx:u32) -> &mut Self
    {//{{{
        debug_assert!(idx < self.data.len() as u32);

        let rm_next_idx = self.data[idx as usize].next_idx;
        let rm_prev_idx = self.data[idx as usize].prev_idx;

        if rm_next_idx != IDX_NOT_EXIST {
            self.data[rm_next_idx as usize].prev_idx = rm_prev_idx;
        }
        else {
            self.last_idx = rm_prev_idx;
        }

        if rm_prev_idx != IDX_NOT_EXIST {
            self.data[rm_prev_idx as usize].next_idx = rm_next_idx;
        }
        else {
            self.first_idx = rm_next_idx;
        }

        self.data[idx as usize].next_idx = self.free_idx;
        self.free_idx = idx;
        self.count -= 1;

        self
    }//}}}

    pub fn iter(&self) -> ListIter<T>
    {//{{{
        ListIter{
            list:&self,
            idx:self.first_idx,
        }
    }//}}}
}

impl<T:PartialEq> List<T> {
    pub fn get_idx(&self,value:&T) -> u32
    {//{{{
        let mut idx = self.first_idx;
        while idx != IDX_NOT_EXIST {
            let element = &self.data[idx as usize];
            if element.value == *value {
                return idx;
            }

            idx = element.next_idx;
        }

        IDX_NOT_EXIST
    }//}}}
}

impl<T:PartialEq> PartialEq for List<T> {
    fn eq(&self,other:&Self) -> bool
    {//{{{
        if self.len() != other.len() {
            return false;
        }

        let mut idx = self.first_idx;
        let mut o_idx = other.first_idx;

        while idx != IDX_NOT_EXIST {
            let element = &self.data[idx as usize];
            let o_element = &other.data[o_idx as usize];

            if element.value != o_element.value {
                return false;
            }

            idx = element.next_idx;
            o_idx = o_element.next_idx;
        }

        debug_assert!(o_idx == IDX_NOT_EXIST);

        true
    }//}}}
}

impl<T:Eq> Eq for List<T> {}

impl<T:Ord> PartialOrd for List<T> {
    fn partial_cmp(&self,other:&Self) -> Option<Ordering>
    {//{{{
        Some(Ord::cmp(self,other))
    }//}}}
}

impl<T:Ord> Ord for List<T> {
    fn cmp(&self,other:&Self) -> Ordering
    {//{{{
        let mut idx = self.first_idx;
        let mut o_idx = other.first_idx;

        while idx != IDX_NOT_EXIST && o_idx != IDX_NOT_EXIST {
            let element = &self.data[idx as usize];
            let o_element = &other.data[o_idx as usize];

            match Ord::cmp(&element.value,&o_element.value) {
                Ordering::Equal => {},
                result => return result,
            }

            idx = element.next_idx;
            o_idx = o_element.next_idx;
        }

        match (idx,o_idx) {
            (IDX_NOT_EXIST,IDX_NOT_EXIST) => Ordering::Equal,
            (IDX_NOT_EXIST,_) => Ordering::Less,
            (_,IDX_NOT_EXIST) => Ordering::Greater,
            _ => panic!(),
        }
    }//}}}
}

impl<'a,T> Iterator for ListIter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>
    {//{{{
        match self.idx {
            IDX_NOT_EXIST => None,
            _ => {
                let element = &self.list.data[self.idx as usize];
                self.idx = element.next_idx;
                Some(&element.value)
            }
        }
    }//}}}
}

impl<T:Display> Display for List<T> {
    fn fmt(&self,f:&mut Formatter) -> std::fmt::Result
    {//{{{
        write!(f,"[")?;

        let mut first = true;
        for value in self.iter() {
            write!(f,"{}{}",if first { first = false; ""} else { "," },*value)?;
        }

        write!(f,"]")
    }//}}}
}

impl<T:Debug> Debug for List<T> {
    fn fmt(&self,f:&mut Formatter) -> std::fmt::Result
    {//{{{
        write!(f,"[")?;

        let mut first = true;
        for value in self.iter() {
            write!(f,"{}{:?}",if first { first = false; ""} else { ", " },*value)?;
        }

        write!(f,"]")
    }//}}}
}

impl<T> std::ops::Index<u32> for List<T> {
    type Output = T;
    fn index(&self,idx: u32) -> &T
    {//{{{
        &self.data[idx as usize].value
    }//}}}
}

impl<T> std::ops::IndexMut<u32> for List<T> {
    fn index_mut(&mut self,idx: u32) -> &mut Self::Output
    {//{{{
        &mut self.data[idx as usize].value
    }//}}}
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn create_t0()
{//{{{
    let list = List::<u32>::new();
    assert_eq!(list.len(),0);
}//}}}

#[test]
fn create_t1()
{//{{{
    let list = List::<u32>::from(vec![1,2,3,4,5,6]);
    assert_eq!(list.len(),6);
    assert_eq!(format!("{}",list),"[1,2,3,4,5,6]");

    let list1 = List::<u32>::from(vec![5,4,3,2,1,0]);
    assert_eq!(list1.len(),6);
    assert_eq!(format!("{}",list1),"[5,4,3,2,1,0]");
}//}}}

#[test]
fn prepend_t0()
{//{{{
    let mut list = List::<u32>::new();
    let mut idx:u32 = 0;
    while idx < 5 {
        list.prepend(idx);
        idx += 1
    }
    assert_eq!(list.len(),5);
    assert_eq!(format!("{}",list),"[4,3,2,1,0]");
    assert_eq!(list,List::<u32>::from(vec![4,3,2,1,0]));
}//}}}

#[test]
fn append_t0()
{//{{{
    let mut list = List::<u32>::new();
    let mut idx:u32 = 0;
    while idx < 5 {
        list.append(idx);
        idx += 1
    }

    assert_eq!(list.len(),5);
    assert_eq!(format!("{}",list),"[0,1,2,3,4]");
    assert_eq!(list,List::<u32>::from(vec![0,1,2,3,4]));
}//}}}

#[test]
fn insert_before_t0()
{//{{{
    let mut list = List::<u32>::from(vec![0,1,2,3,4,5]);
    let mut idx:u32 = 0;
    while idx < 5 {
        list.insert_before(3,idx);
        idx += 1
    }

    assert_eq!(list.len(),11);
    assert_eq!(format!("{}",list),"[0,1,2,0,1,2,3,4,3,4,5]");
    assert_eq!(list,List::<u32>::from(vec![0,1,2,0,1,2,3,4,3,4,5]));
}//}}}

#[test]
fn insert_after_t0()
{//{{{
    let mut list = List::<u32>::from(vec![0,1,2,3,4,5]);
    let mut idx:u32 = 0;
    while idx < 5 {
        list.insert_after(3,idx);
        idx += 1
    }

    assert_eq!(list.len(),11);
    assert_eq!(format!("{}",list),"[0,1,2,3,4,3,2,1,0,4,5]");
    assert_eq!(list,List::<u32>::from(vec![0,1,2,3,4,3,2,1,0,4,5]));
}//}}}

#[test]
fn remove_t0()
{//{{{
    let mut list = List::<u32>::from(vec![0,1,2,3,4]);
    assert_eq!(list.len(),5);
    assert_eq!(format!("{}",list),"[0,1,2,3,4]");

    list.remove(2);
    assert_eq!(list.len(),4);
    assert_eq!(format!("{}",list),"[0,1,3,4]");

    list.remove(0);
    assert_eq!(list.len(),3);
    assert_eq!(format!("{}",list),"[1,3,4]");

    list.remove(4);
    assert_eq!(list.len(),2);
    assert_eq!(format!("{}",list),"[1,3]");
}//}}}

#[test]
fn get_idx_t0()
{//{{{
    let list = List::<u32>::from(vec![4,3,2,1,0]);
    assert_eq!(list.len(),5);
    assert_eq!(format!("{}",list),"[4,3,2,1,0]");

    let mut idx:u32 = 0;
    while idx < 4 {
        assert_eq!(list.get_idx(&idx),4 - idx);
        idx += 1;
    }
}//}}}

#[test]
fn iter_t0()
{//{{{
    let list = List::<u32>::from(vec![4,3,2,1,0]);
    assert_eq!(list.len(),5);
    assert_eq!(format!("{}",list),"[4,3,2,1,0]");

    let mut vec = vec![];
    for value in list.iter() {
        vec.push(*value);
    }
    assert_eq!(vec,vec![4,3,2,1,0]);
}//}}}

#[test]
fn fmt_t0()
{//{{{
    let list = List::<u32>::new();
    assert_eq!(list.len(),0);
    assert_eq!(format!("{}",list),"[]");

    let list = List::<u32>::from(vec![4,3,2,1,0]);
    assert_eq!(list.len(),5);
    assert_eq!(format!("{}",list),"[4,3,2,1,0]");
}//}}}

#[test]
fn equal_t0()
{//{{{
    let list = List::<u32>::from(vec![0,1,2,3,4]);
    let mut list1 = List::<u32>::new();

    let mut vec = vec![];
    for idx in 0..9 {
        list1.append(idx);
        vec.push(list == list1);
    }
    assert_eq!(vec,vec![false,false,false,false,true,false,false,false,false]);
}//}}}

#[test]
fn ord_t0()
{//{{{
    let list = List::<u32>::from(vec![0,1,2,3,4]);
    let mut list1 = List::<u32>::new();

    let mut vec = vec![];
    for idx in 0..9 {
        list1.append(idx);
        vec.push(Ord::cmp(&list,&list1) as i8);
    }
    assert_eq!(vec,vec![1,1,1,1,0,-1,-1,-1,-1]);
}//}}}

#[test]
fn vec_list_t0()
{//{{{
    let mut list = List::<Vec<u32>>::new();
    for count in 1 ..=10 {
        let vec = (0u32 .. count).collect();
        list.append(vec);
    }
}//}}}

#[test]
fn index_t0()
{//{{{
    let values = [0,1,2,3,4,5,6,7,8,9];
    let list = List::<u32>::from(values.to_vec());
    for value in values.to_vec() {
        let idx = list.get_idx(&value);
        let ret_value = list[idx];
        assert_eq!(value,ret_value)
    }
}//}}}

#[test]
fn index_mut_t0()
{//{{{
    let values = [0,1,2,3,4,5,6,7,8,9];
    let mut list = List::<u32>::from(values.to_vec());
    for value in values.to_vec() {
        let idx = list.get_idx(&value);
        list[idx] = 9;
    }
    assert_eq!(format!("{}",list),"[9,9,9,9,9,9,9,9,9,9]");
}//}}}

}

