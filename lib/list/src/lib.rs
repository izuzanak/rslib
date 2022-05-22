#![allow(dead_code)]

const IDX_NOT_EXIST:u32 = std::u32::MAX;

struct ListElement<T>
{//{{{
    next_idx:u32,
    prev_idx:u32,
    value:T,
}//}}}

pub struct List<T>
{//{{{
    free_idx:u32,
    first_idx:u32,
    last_idx:u32,
    count:u32,
    data:Vec<ListElement<T>>,
}//}}}

pub struct ListIter<'a,T>
{//{{{
    list:&'a List<T>,
    idx:u32,
}//}}}

impl<T> List<T>
{//{{{
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
        let new_idx:u32;

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
}//}}}

impl<T:std::cmp::PartialEq> List<T> {
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

impl<T:std::fmt::Display> std::fmt::Display for List<T> {
    fn fmt(&self,f:&mut std::fmt::Formatter) -> std::fmt::Result
    {//{{{
        write!(f,"[")?;

        let mut first = true;
        for value in self.iter() {
            write!(f,"{}{}",if first { first = false; ""} else { "," },*value)?;
        }

        write!(f,"]")
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

}

