#![allow(dead_code)]

const IDX_NOT_EXIST:usize = std::usize::MAX;

struct ListElement<T> {
    next_idx:usize,
    prev_idx:usize,
    value:T
}

struct List<T> {
    free_idx:usize,
    first_idx:usize,
    last_idx:usize,
    count:usize,
    data:Vec<ListElement<T>>,
}

impl<T:std::cmp::PartialEq> List<T> {
    pub fn new() -> List<T>
    {//{{{
        List{
            free_idx:IDX_NOT_EXIST,
            first_idx:IDX_NOT_EXIST,
            last_idx:IDX_NOT_EXIST,
            count:0,
            data:vec![],
        }
    }//}}}

    pub fn new_init(a_src:Vec<T>) -> List<T>
    {//{{{
        let mut data:Vec<ListElement<T>> = vec![];

        if a_src.is_empty() {
            List::<T>::new()
        }
        else {
            let count = a_src.len();
            let mut prev_idx = IDX_NOT_EXIST;

            for value in a_src {
                let idx = data.len();

                data.push(ListElement{
                    next_idx:idx + 1,
                    prev_idx:prev_idx,
                    value:value,
                });

                prev_idx = idx;
            }

            let last_idx = data.len() - 1;
            data[last_idx].next_idx = IDX_NOT_EXIST;

            List{
                free_idx:IDX_NOT_EXIST,
                first_idx:0,
                last_idx:last_idx,
                count:count,
                data:data,
            }
        }
    }//}}}

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn prepend(&mut self,a_value:T) -> usize
    {//{{{
        let new_idx:usize;

        if self.free_idx != IDX_NOT_EXIST {
            new_idx = self.free_idx;
            self.free_idx = self.data[new_idx].next_idx;

            self.data[new_idx] = ListElement{
                next_idx:self.first_idx,
                prev_idx:IDX_NOT_EXIST,
                value:a_value,
            }
        }
        else {
            new_idx = self.data.len();
            self.data.push(ListElement{
                next_idx:self.first_idx,
                prev_idx:IDX_NOT_EXIST,
                value:a_value,
            });
        }

        self.first_idx = new_idx;
        self.count += 1;

        new_idx
    }//}}}

    pub fn append(&mut self,a_value:T) -> usize
    {//{{{
        let new_idx:usize;

        if self.free_idx != IDX_NOT_EXIST {
            new_idx = self.free_idx;
            self.free_idx = self.data[new_idx].next_idx;

            self.data[new_idx] = ListElement{
                next_idx:IDX_NOT_EXIST,
                prev_idx:self.last_idx,
                value:a_value,
            }
        }
        else {
            new_idx = self.data.len();
            self.data.push(ListElement{
                next_idx:IDX_NOT_EXIST,
                prev_idx:self.last_idx,
                value:a_value,
            });
        }

        if self.last_idx != IDX_NOT_EXIST {
            self.data[self.last_idx].next_idx = new_idx;
        }
        else {
            self.first_idx = new_idx;
        }

        self.last_idx = new_idx;
        self.count += 1;

        new_idx
    }//}}}

    pub fn remove(&mut self,a_idx:usize) -> &mut List<T>
    {//{{{
        debug_assert!(a_idx < self.data.len());

        let rm_next_idx = self.data[a_idx].next_idx;
        let rm_prev_idx = self.data[a_idx].prev_idx;

        if rm_next_idx != IDX_NOT_EXIST {
            self.data[rm_next_idx].prev_idx = rm_prev_idx;
        }
        else {
            self.last_idx = rm_prev_idx;
        }

        if rm_prev_idx != IDX_NOT_EXIST {
            self.data[rm_prev_idx].next_idx = rm_next_idx;
        }
        else {
            self.first_idx = rm_next_idx;
        }

        self.data[a_idx].next_idx = self.free_idx;
        self.free_idx = a_idx;
        self.count -= 1;

        self
    }//}}}

    pub fn get_idx(&self,a_value:T) -> usize
    {//{{{
        let mut idx = self.first_idx;
        while idx != IDX_NOT_EXIST {
            let element = &self.data[idx];
            if element.value == a_value {
                return idx
            }

            idx = element.next_idx;
        }

        IDX_NOT_EXIST
    }//}}}
}

impl<T:std::fmt::Display> std::fmt::Display for List<T> {
    fn fmt(&self,f:&mut std::fmt::Formatter) -> std::fmt::Result
    {//{{{
        let mut idx = self.first_idx;
        let mut first = true;
        while idx != IDX_NOT_EXIST {
            let element = &self.data[idx];
            idx = element.next_idx;
            write!(f,"{}{}",if first { first = false; '['} else { ',' },element.value)?
        }
        write!(f,"]")
    }//}}}
}

#[cfg(test)]
mod tests {
use super::*;

static ERROR_TEST_FAILED:&str = "Test failed";

#[test]
fn create_t0()
{//{{{
    let list = List::<u32>::new();
    assert_eq!(list.len(),0);
}//}}}

#[test]
fn create_t1()
{//{{{
    let list = List::<u32>::new_init(vec![1,2,3,4,5,6]);
    assert_eq!(list.len(),6);
    assert_eq!(format!("{}",list),"[1,2,3,4,5,6]");

    let list1 = List::<u32>::new_init(vec![5,4,3,2,1,0]);
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
    let mut list = List::<u32>::new_init(vec![0,1,2,3,4]);
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
    let list = List::<u32>::new_init(vec![4,3,2,1,0]);
    assert_eq!(list.len(),5);
    assert_eq!(format!("{}",list),"[4,3,2,1,0]");

    let mut idx:u32 = 0;
    while idx < 4 {
        assert_eq!(list.get_idx(idx),(4 - idx) as usize);
        idx += 1;
    }
}//}}}

}

