
#![allow(dead_code)]

use std::os::raw::{c_int};
use std::default::{Default};
use std::cmp::{PartialEq,Eq,PartialOrd,Ord,Ordering};
use std::fmt::{Debug,Display,Formatter};

const IDX_NOT_EXIST:u32 = std::u32::MAX;

extern
{//{{{
    fn rand() -> c_int;
}//}}}

struct TreeNode<T>
{//{{{
    parent_idx:u32,
    left_idx:u32,
    right_idx:u32,
    color:bool,
    valid:bool,
    value:T,
}//}}}

pub struct Tree<T>
{//{{{
    free_idx:u32,
    root_idx:u32,
    leaf_idx:u32,
    count:u32,
    data:Vec<TreeNode<T>>,
}//}}}

pub struct TreeIter<'a,T>
{//{{{
    tree:&'a Tree<T>,
    idx:u32,
}//}}}

pub struct TreeOrdIter<'a,T>
{//{{{
    tree:&'a Tree<T>,
    stack:Vec<u32>,
    idx:u32,
}//}}}

impl<T:Default + Ord> Tree<T>
{//{{{
    fn __get_grandparent_idx(&self,idx:u32) -> u32
    {//{{{
        let node_parent_idx = self.data[idx as usize].parent_idx;

        if node_parent_idx != IDX_NOT_EXIST {
            self.data[node_parent_idx as usize].parent_idx
        }
        else {
            IDX_NOT_EXIST
        }
    }//}}}

    fn __get_uncle_idx(&self,idx:u32) -> u32
    {//{{{
        let gp_idx = self.__get_grandparent_idx(idx);

        if gp_idx != IDX_NOT_EXIST {
          let gp = &self.data[gp_idx as usize];
          if gp.left_idx == self.data[idx as usize].parent_idx { gp.right_idx } else { gp.left_idx }
        }
        else {
            IDX_NOT_EXIST
        }
    }//}}}

    fn __get_sibling_idx(&self,idx:u32) -> u32
    {//{{{
        let p = &self.data[self.data[idx as usize].parent_idx as usize];
        if p.left_idx == idx { p.right_idx } else { p.left_idx }
    }//}}}

    fn __rotate_left(&mut self,idx:u32) -> &mut Self
    {//{{{
        let root_idx = idx;
        let pivot_idx = self.data[root_idx as usize].right_idx;

        if idx == self.root_idx {
            self.root_idx = pivot_idx;
            self.data[pivot_idx as usize].parent_idx = IDX_NOT_EXIST;
        }
        else {
          let root_parent_idx = self.data[root_idx as usize].parent_idx;
          let rp = &mut self.data[root_parent_idx as usize];

          if rp.right_idx == idx {
              rp.right_idx = pivot_idx;
          }
          else {
              rp.left_idx = pivot_idx;
          }

          self.data[pivot_idx as usize].parent_idx = root_parent_idx;
        }

        let pivot_left_idx = self.data[pivot_idx as usize].left_idx;
        let root = &mut self.data[root_idx as usize];
        root.parent_idx = pivot_idx;

        root.right_idx = pivot_left_idx;

        let root_right_idx = root.right_idx;
        self.data[root_right_idx as usize].parent_idx = idx;

        self.data[pivot_idx as usize].left_idx = idx;

        self
    }//}}}

    fn __rotate_right(&mut self,idx:u32) -> &mut Self
    {//{{{
        let root_idx = idx;
        let pivot_idx = self.data[root_idx as usize].left_idx;

        if idx == self.root_idx {
            self.root_idx = pivot_idx;
            self.data[pivot_idx as usize].parent_idx = IDX_NOT_EXIST;
        }
        else {
            let root_parent_idx = self.data[root_idx as usize].parent_idx;
            let rp = &mut self.data[root_parent_idx as usize];

            if rp.right_idx == idx {
                rp.right_idx = pivot_idx;
            }
            else {
                rp.left_idx = pivot_idx;
            }

            self.data[pivot_idx as usize].parent_idx = root_parent_idx;
        }

        let pivot_right_idx = self.data[pivot_idx as usize].right_idx;
        let root = &mut self.data[root_idx as usize];
        root.parent_idx = pivot_idx;

        root.left_idx = pivot_right_idx;

        let root_left_idx = root.left_idx;
        self.data[root_left_idx as usize].parent_idx = idx;

        self.data[pivot_idx as usize].right_idx = idx;

        self
    }//}}}

    fn __get_new_index(&mut self) -> u32
    {//{{{
        let new_idx;

        if self.free_idx != IDX_NOT_EXIST {
            new_idx = self.free_idx;
            self.free_idx = self.data[new_idx as usize].parent_idx;
        }
        else {
            if self.leaf_idx == IDX_NOT_EXIST {
                self.leaf_idx = self.data.len() as u32;
                self.data.push(TreeNode{
                    parent_idx:IDX_NOT_EXIST,
                    left_idx:  IDX_NOT_EXIST,
                    right_idx: IDX_NOT_EXIST,
                    color:true,
                    valid:false,
                    value:T::default(),
                });
            }

            new_idx = self.data.len() as u32;
            self.data.push(TreeNode{
                parent_idx:IDX_NOT_EXIST,
                left_idx:  IDX_NOT_EXIST,
                right_idx: IDX_NOT_EXIST,
                color:true,
                valid:false,
                value:T::default(),
            });
        }

        self.data[new_idx as usize].valid = true;
        self.count += 1;

        new_idx
    }//}}}

    fn __binary_tree_insert(&mut self,new_idx:u32,value:&T,unique:bool) -> u32
    {//{{{
        if self.root_idx == IDX_NOT_EXIST {
            self.data[new_idx as usize].parent_idx = IDX_NOT_EXIST;
            self.root_idx = new_idx;
        }
        else {
            let mut node_idx = self.root_idx;
            loop {
                let node = &mut self.data[node_idx as usize];

                let comp_result = Ord::cmp(value,&node.value);
                if comp_result == Ordering::Less {
                    if node.left_idx == self.leaf_idx {
                        node.left_idx = new_idx;
                        break;
                    }
                    node_idx = node.left_idx;
                }
                else {
                    if unique && comp_result == Ordering::Equal {
                        return node_idx;
                    }

                    if node.right_idx == self.leaf_idx {
                        node.right_idx = new_idx;
                        break;
                    }
                    node_idx = node.right_idx;
                }
            }

            self.data[new_idx as usize].parent_idx = node_idx;
        }

        let new_node = &mut self.data[new_idx as usize];
        new_node.left_idx = self.leaf_idx;
        new_node.right_idx = self.leaf_idx;
        new_node.color = false;

        IDX_NOT_EXIST
    }//}}}

    fn __replace_delete_node_by_child(&mut self,idx:u32,ch_idx:u32) -> &mut Self
    {//{{{
        let node_parent_idx = self.data[idx as usize].parent_idx;

        if node_parent_idx != IDX_NOT_EXIST {
            let parent = &mut self.data[node_parent_idx as usize];

            if parent.left_idx == idx {
                parent.left_idx = ch_idx;
            }
            else {
                parent.right_idx = ch_idx;
            }

            self.data[ch_idx as usize].parent_idx = node_parent_idx;
        }
        else {
            self.root_idx = if ch_idx == self.leaf_idx { IDX_NOT_EXIST } else { ch_idx };
            self.data[ch_idx as usize].parent_idx = IDX_NOT_EXIST;
        }

        self
    }//}}}

    fn __remove_black_black(&mut self,idx:u32) -> &mut Self
    {//{{{
        let mut node_idx = idx;
        loop {
            let node_parent_idx = self.data[node_idx as usize].parent_idx;
            if node_parent_idx == IDX_NOT_EXIST {
                return self;
            }

            let parent_idx = node_parent_idx;

            let parent = &self.data[parent_idx as usize];
            let parent_left_idx = parent.left_idx;
            let parent_right_idx = parent.right_idx;

            let sibling_idx = if parent_left_idx == node_idx { parent_right_idx } else { parent_left_idx };
            let sibling = &mut self.data[sibling_idx as usize];

            if !sibling.color {
                sibling.color = true;
                self.data[parent_idx as usize].color = false;

                if node_idx == parent_left_idx {
                    self.__rotate_left(parent_idx);
                }
                else {
                    self.__rotate_right(parent_idx);
                }
            }

            let parent = &self.data[parent_idx as usize];
            let parent_left_idx = parent.left_idx;
            let parent_right_idx = parent.right_idx;
            let parent_color = parent.color;

            let sibling_idx = if parent_left_idx == node_idx { parent_right_idx } else { parent_left_idx };
            let sibling = &self.data[sibling_idx as usize];
            let sibling_left_idx = sibling.left_idx;
            let sibling_right_idx = sibling.right_idx;
            let sibling_color = sibling.color;

            if parent_color && sibling_color && self.data[sibling_left_idx as usize].color && self.data[sibling_right_idx as usize].color {
                self.data[sibling_idx as usize].color = false;
                node_idx = parent_idx;
                continue;
            }

            if !parent_color && sibling_color && self.data[sibling_left_idx as usize].color && self.data[sibling_right_idx as usize].color {
                self.data[sibling_idx as usize].color = false;
                self.data[parent_idx as usize].color = true;
                return self;
            }

            if sibling_color {
                if node_idx == parent_left_idx && self.data[sibling_right_idx as usize].color && !self.data[sibling_left_idx as usize].color {
                    self.data[sibling_idx as usize].color = false;
                    self.data[sibling_left_idx as usize].color = true;
                    self.__rotate_right(sibling_idx);
                }
                else if node_idx == parent_right_idx && self.data[sibling_left_idx as usize].color && !self.data[sibling_right_idx as usize].color {
                    self.data[sibling_idx as usize].color = false;
                    self.data[sibling_right_idx as usize].color = true;
                    self.__rotate_left(sibling_idx);
                }
            }

            let parent = &self.data[parent_idx as usize];
            let parent_left_idx = parent.left_idx;
            let parent_right_idx = parent.right_idx;
            let parent_color = parent.color;

            let sibling_idx = if parent_left_idx == node_idx { parent_right_idx } else { parent_left_idx };
            let sibling = &self.data[sibling_idx as usize];
            let sibling_left_idx = sibling.left_idx;
            let sibling_right_idx = sibling.right_idx;

            self.data[sibling_idx as usize].color = parent_color;
            self.data[parent_idx as usize].color = true;

            if node_idx == parent_left_idx {
                self.data[sibling_right_idx as usize].color = true;
                self.__rotate_left(parent_idx);
            }
            else {
                self.data[sibling_left_idx as usize].color = true;
                self.__rotate_right(parent_idx);
            }

            return self;
        }
    }//}}}

    fn __remove_one_child(&mut self,idx:u32,ch_idx:u32) -> &mut Self
    {//{{{
        self.__replace_delete_node_by_child(idx,ch_idx);

        let node = &mut self.data[idx as usize];
        node.parent_idx = self.free_idx;
        self.free_idx = idx;

        node.valid = false;
        self.count -= 1;

        if node.color {
            let child_node = &mut self.data[ch_idx as usize];

            if !child_node.color {
                child_node.color = true;
            }
            else {
                self.__remove_black_black(ch_idx);
            }
        }

        self
    }//}}}

    fn __insert_operation(&mut self,idx:u32) -> &mut Self
    {//{{{
        let mut node_idx = idx;
        loop {
            let node = &self.data[node_idx as usize];

            if node.parent_idx == IDX_NOT_EXIST {
                self.data[node_idx as usize].color = true;
                return self;
            }

            if self.data[node.parent_idx as usize].color {
                return self;
            }

            let uncle_idx = self.__get_uncle_idx(node_idx);
            if uncle_idx != IDX_NOT_EXIST && !self.data[uncle_idx as usize].color {
                let node_parent_idx = node.parent_idx;
                self.data[node_parent_idx as usize].color = true;
                self.data[uncle_idx as usize].color = true;

                node_idx = self.__get_grandparent_idx(node_idx);
                self.data[node_idx as usize].color = false;

                continue;
            }

            let grandparent_idx = self.__get_grandparent_idx(node_idx);
            let node_parent_idx = node.parent_idx;

            if node_idx == self.data[node_parent_idx as usize].right_idx && node_parent_idx == self.data[grandparent_idx as usize].left_idx {
                self.__rotate_left(node_parent_idx);
                node_idx = self.data[node_idx as usize].left_idx;
            }
            else if node_idx == self.data[node_parent_idx as usize].left_idx && node_parent_idx == self.data[grandparent_idx as usize].right_idx {
                self.__rotate_right(node_parent_idx);
                node_idx = self.data[node_idx as usize].right_idx;
            }

            let grandparent_idx = self.__get_grandparent_idx(node_idx);
            let node_parent_idx = self.data[node_idx as usize].parent_idx;

            self.data[node_parent_idx as usize].color = true;
            self.data[grandparent_idx as usize].color = false;

            if node_idx == self.data[node_parent_idx as usize].left_idx && node_parent_idx == self.data[grandparent_idx as usize].left_idx {
                self.__rotate_right(grandparent_idx);
            }
            else {
                self.__rotate_left(grandparent_idx);
            }

            return self;
        }
    }//}}}

    pub fn new() -> Tree<T>
    {//{{{
        Tree{
            free_idx:IDX_NOT_EXIST,
            root_idx:IDX_NOT_EXIST,
            leaf_idx:IDX_NOT_EXIST,
            count:0,
            data:vec![],
        }
    }//}}}

    pub fn from(src:Vec<T>) -> Tree<T>
    {//{{{
        let mut tree = Tree::new();

        for value in src {
            tree.insert(value);
        }

        return tree;
    }//}}}

    pub fn len(&self) -> u32
    {//{{{
        self.count
    }//}}}

    pub fn get_stack_min_value_idx(&self,idx:u32,stack:&mut Vec<u32>) -> u32
    {//{{{
        debug_assert!((idx as usize) < self.data.len() && self.data[idx as usize].valid);

        let mut node_idx = idx;
        loop {
          let node = &self.data[node_idx as usize];

          if node.left_idx == self.leaf_idx {
            return node_idx;
          }

          stack.push(node_idx);
          node_idx = node.left_idx;
        }
    }//}}}

    pub fn get_stack_next_idx(&self,idx:u32,stack:&mut Vec<u32>) -> u32
    {//{{{
        debug_assert!((idx as usize) < self.data.len() && self.data[idx as usize].valid);

        let node = &self.data[idx as usize];

        if node.right_idx != self.leaf_idx {
            return self.get_stack_min_value_idx(node.right_idx,stack);
        }

        match stack.pop() {
            Some(idx) => idx,
            None => IDX_NOT_EXIST,
        }
    }//}}}

    pub fn get_min_value_idx(&self,idx:u32) -> u32
    {//{{{
        debug_assert!((idx as usize) < self.data.len() && self.data[idx as usize].valid);

        let mut node_idx = idx;
        loop {
            let node = &self.data[node_idx as usize];

            if node.left_idx == self.leaf_idx {
                return node_idx;
            }

            node_idx = node.left_idx;
        }
    }//}}}

    pub fn get_max_value_idx(&self,idx:u32) -> u32
    {//{{{
        debug_assert!((idx as usize) < self.data.len() && self.data[idx as usize].valid);

        let mut node_idx = idx;
        loop {
          let node = &self.data[node_idx as usize];

          if node.right_idx == self.leaf_idx {
            return node_idx;
          }

          node_idx = node.right_idx;
        }
    }//}}}

    pub fn get_next_idx(&self,idx:u32) -> u32
    {//{{{
        debug_assert!((idx as usize) < self.data.len() && self.data[idx as usize].valid);

        let node_right_idx = self.data[idx as usize].right_idx;

        if node_right_idx != self.leaf_idx {
            return self.get_min_value_idx(node_right_idx);
        }

        let mut node_idx = idx;
        loop {
            let node = &self.data[node_idx as usize];

            if node.parent_idx == IDX_NOT_EXIST {
                return IDX_NOT_EXIST;
            }

            if self.data[node.parent_idx as usize].right_idx != node_idx {
                return node.parent_idx;
            }

            node_idx = node.parent_idx;
        }
    }//}}}

    pub fn get_prev_idx(&self,idx:u32) -> u32
    {//{{{
        debug_assert!((idx as usize) < self.data.len() && self.data[idx as usize].valid);

        let node = &self.data[idx as usize];

        if node.left_idx != self.leaf_idx {
            return self.get_max_value_idx(node.left_idx);
        }

        let mut node_idx = idx;
        loop {
            let node = &self.data[node_idx as usize];

            if node.parent_idx == IDX_NOT_EXIST {
                return IDX_NOT_EXIST;
            }

            if self.data[node.parent_idx as usize].left_idx != node_idx {
                return node.parent_idx;
            }

            node_idx = node.parent_idx;
        }
    }//}}}

    pub fn insert(&mut self,value:T) -> u32
    {//{{{
        let new_node_idx = self.__get_new_index();

        self.__binary_tree_insert(new_node_idx,&value,false);
        self.__insert_operation(new_node_idx);
        self.data[new_node_idx as usize].value = value;

        new_node_idx
    }//}}}

    pub fn unique_insert(&mut self,value:T) -> u32
    {//{{{
        let new_node_idx = self.__get_new_index();
        let old_node_idx = self.__binary_tree_insert(new_node_idx,&value,true);

        if old_node_idx != IDX_NOT_EXIST {
          let new_node = &mut self.data[new_node_idx as usize];

          new_node.parent_idx = self.free_idx;
          self.free_idx = new_node_idx;

          new_node.valid = false;
          self.count -= 1;

          return old_node_idx;
        }

        self.__insert_operation(new_node_idx);
        self.data[new_node_idx as usize].value = value;

        new_node_idx
    }//}}}

    pub fn remove(&mut self,idx:u32) -> &mut Self
    {//{{{
        debug_assert!((idx as usize) < self.data.len() && self.data[idx as usize].valid);

        let del_node = &self.data[idx as usize];
        let del_node_left_idx = del_node.left_idx;
        let del_node_right_idx = del_node.right_idx;

        if del_node_left_idx != self.leaf_idx {
            if del_node_right_idx != self.leaf_idx {
                let del_node_parent_idx = del_node.parent_idx;
                let del_node_color = del_node.color;

                let mut found_idx = del_node_right_idx;
                loop {
                    let node = &self.data[found_idx as usize];

                    if node.left_idx == self.leaf_idx {
                        break;
                    }

                    found_idx = node.left_idx;
                }

                // - process del_node parent_idx -
                if del_node_parent_idx != IDX_NOT_EXIST {
                    let del_node_parent = &mut self.data[del_node_parent_idx as usize];

                    if del_node_parent.left_idx == idx {
                        del_node_parent.left_idx = found_idx;
                    }
                    else {
                        del_node_parent.right_idx = found_idx;
                    }
                }
                else {
                    self.root_idx = found_idx;
                }

                // - process del_node left_idx -
                self.data[del_node_left_idx as usize].parent_idx = found_idx;

                let found_node = &self.data[found_idx as usize];
                let found_node_right_idx = found_node.right_idx;
                let found_node_parent_idx = found_node.parent_idx;
                let found_node_color = found_node.color;

                // - process found_node right_idx -
                if found_node_right_idx != self.leaf_idx {
                    self.data[found_node_right_idx as usize].parent_idx = idx;
                }

                if del_node_right_idx == found_idx {
                    // - found node is right child of deleted node -
                    let del_node = &mut self.data[idx as usize];
                    del_node.right_idx = found_node_right_idx;
                    del_node.parent_idx = found_idx;
                    del_node.left_idx = self.leaf_idx;
                    del_node.color = found_node_color;

                    let found_node = &mut self.data[found_idx as usize];
                    found_node.right_idx = idx;
                    found_node.parent_idx = del_node_parent_idx;
                    found_node.left_idx = del_node_left_idx;
                    found_node.color = del_node_color;
                }
                else {
                    // - process found_node parent -
                    let found_node_parent = &mut self.data[found_node_parent_idx as usize];

                    if found_node_parent.left_idx == found_idx {
                        found_node_parent.left_idx = idx;
                    }
                    else {
                        found_node_parent.right_idx = idx;
                    }

                    // - process del_node right_idx -
                    self.data[del_node_right_idx as usize].parent_idx = found_idx;

                    // - swap index pointers between nodes -
                    let del_node = &mut self.data[idx as usize];
                    del_node.parent_idx = found_node_parent_idx;
                    del_node.left_idx = self.leaf_idx;
                    del_node.right_idx = found_node_right_idx;
                    del_node.color = found_node_color;

                    let found_node = &mut self.data[found_idx as usize];
                    found_node.parent_idx = del_node_parent_idx;
                    found_node.left_idx = del_node_left_idx;
                    found_node.right_idx = del_node_right_idx;
                    found_node.color = del_node_color;
                }

                self.__remove_one_child(idx,self.data[idx as usize].right_idx);
            }
            else {
                self.__remove_one_child(idx,del_node_left_idx);
            }
        }
        else {
            self.__remove_one_child(idx,del_node_right_idx);
        }

        self
    }//}}}

    pub fn get_idx(&self,value:&T) -> u32
    {//{{{
        if self.root_idx == IDX_NOT_EXIST {
            return IDX_NOT_EXIST;
        }

        let mut node_idx = self.root_idx;
        while node_idx != self.leaf_idx {
            let node = &self.data[node_idx as usize];

            let comp_result = Ord::cmp(value,&node.value);
            if comp_result == Ordering::Less {
                node_idx = node.left_idx;
            }
            else {
                if comp_result == Ordering::Equal {
                    return node_idx;
                }

                node_idx = node.right_idx;
            }
        }

        IDX_NOT_EXIST
    }//}}}

    pub fn get_left_idx(&self,value:&T) -> u32
    {//{{{
        if self.root_idx == IDX_NOT_EXIST {
          return IDX_NOT_EXIST;
        }

        let mut good_idx = IDX_NOT_EXIST;
        let mut node_idx = self.root_idx;
        while node_idx != self.leaf_idx {
          let node = &self.data[node_idx as usize];

          let comp_result = Ord::cmp(value,&node.value);
          if comp_result == Ordering::Less {
            node_idx = node.left_idx;
          }
          else {
            if comp_result == Ordering::Equal {
              good_idx = node_idx;
              node_idx = node.left_idx;
            }
            else {
              node_idx = node.right_idx;
            }
          }
        }

        good_idx
    }//}}}

    pub fn get_gre_idx(&self,value:&T) -> u32
    {//{{{
        if self.root_idx == IDX_NOT_EXIST {
          return IDX_NOT_EXIST;
        }

        let mut good_idx = IDX_NOT_EXIST;
        let mut node_idx = self.root_idx;
        while node_idx != self.leaf_idx {
          let node = &self.data[node_idx as usize];

          let comp_result = Ord::cmp(value,&node.value);
          if comp_result == Ordering::Less {
            good_idx = node_idx;
            node_idx = node.left_idx;
          }
          else {
            if comp_result == Ordering::Equal {
              return node_idx;
            }

            node_idx = node.right_idx;
          }
        }

        good_idx
    }//}}}

    pub fn get_lee_idx(&self,value:&T) -> u32
    {//{{{
        if self.root_idx == IDX_NOT_EXIST {
          return IDX_NOT_EXIST;
        }

        let mut good_idx = IDX_NOT_EXIST;
        let mut node_idx = self.root_idx;
        while node_idx != self.leaf_idx {
          let node = &self.data[node_idx as usize];

          let comp_result = Ord::cmp(value,&node.value);
          if comp_result == Ordering::Less {
            node_idx = node.left_idx;
          }
          else {
            if comp_result  == Ordering::Equal {
              return node_idx;
            }

            good_idx = node_idx;
            node_idx = node.right_idx;
          }
        }

        good_idx
    }//}}}

    pub fn get_idxs(&self,value:&T) -> Vec<u32>
    {//{{{
        let mut result:Vec<u32> = vec![];

        if self.root_idx == IDX_NOT_EXIST {
            return result;
        }

        let mut stack:Vec<u32> = vec![];
        stack.push(self.root_idx);

        while let Some(node_idx) = stack.pop() {
            let node = &self.data[node_idx as usize];

            let comp_result = Ord::cmp(value,&node.value);
            if comp_result == Ordering::Less {
                if node.left_idx != self.leaf_idx {
                    stack.push(node.left_idx);
                }
            }
            else {
                if comp_result == Ordering::Equal {
                    result.push(node_idx);

                    if node.left_idx != self.leaf_idx {
                        stack.push(node.left_idx);
                    }
                }

                if node.right_idx != self.leaf_idx {
                    stack.push(node.right_idx);
                }
            }
        }

        result
    }//}}}

    pub fn iter(&self) -> TreeIter<T>
    {//{{{
        TreeIter{
            tree:&self,
            idx:0,
        }
    }//}}}

    pub fn ord_iter(&self) -> TreeOrdIter<T>
    {//{{{
        let mut stack:Vec<u32> = vec![];
        let idx = if self.root_idx != IDX_NOT_EXIST {
            self.get_stack_min_value_idx(self.root_idx,&mut stack)
        }
        else {
            IDX_NOT_EXIST
        };

        TreeOrdIter{
            tree:&self,
            stack:stack,
            idx:idx,
        }
    }//}}}
}//}}}

impl<T:Default + Ord> PartialEq for Tree<T>
{//{{{
    fn eq(&self,other:&Self) -> bool
    {//{{{
        if self.len() != other.len() {
            return false;
        }

        match (self.root_idx,other.root_idx) {
            (IDX_NOT_EXIST,IDX_NOT_EXIST) => return true,
            (IDX_NOT_EXIST,_) |
            (_,IDX_NOT_EXIST) => return false,
            _ => {}
        }

        let mut stack:Vec<u32> = vec![];
        let mut o_stack:Vec<u32> = vec![];

        let mut idx = self.get_stack_min_value_idx(self.root_idx,&mut stack);
        let mut o_idx = other.get_stack_min_value_idx(other.root_idx,&mut o_stack);

        while idx != IDX_NOT_EXIST {
            if self.data[idx as usize].value != other.data[o_idx as usize].value {
                return false;
            }

            idx = self.get_stack_next_idx(idx,&mut stack);
            o_idx = other.get_stack_next_idx(o_idx,&mut o_stack);
        }

        debug_assert!(o_idx == IDX_NOT_EXIST);

        true
    }//}}}
}//}}}

impl<T:Default + Ord> Eq for Tree<T> {}

impl<T:Default + Ord> PartialOrd for Tree<T>
{//{{{
    fn partial_cmp(&self,other:&Self) -> Option<Ordering>
    {//{{{
        Some(Ord::cmp(self,other))
    }//}}}
}//}}}

impl<T:Default + Ord> Ord for Tree<T>
{//{{{
    fn cmp(&self,other:&Self) -> Ordering
    {//{{{
        match (self.root_idx,other.root_idx) {
            (IDX_NOT_EXIST,IDX_NOT_EXIST) => return Ordering::Equal,
            (IDX_NOT_EXIST,_) => return Ordering::Less,
            (_,IDX_NOT_EXIST) => return Ordering::Greater,
            _ => {}
        }

        let mut stack:Vec<u32> = vec![];
        let mut o_stack:Vec<u32> = vec![];

        let mut idx = self.get_stack_min_value_idx(self.root_idx,&mut stack);
        let mut o_idx = other.get_stack_min_value_idx(other.root_idx,&mut o_stack);

        while idx != IDX_NOT_EXIST && o_idx != IDX_NOT_EXIST {
            let node = &self.data[idx as usize];
            let o_node = &other.data[o_idx as usize];

            match Ord::cmp(&node.value,&o_node.value) {
                Ordering::Equal => {},
                result => return result,
            }

            idx = self.get_stack_next_idx(idx,&mut stack);
            o_idx = other.get_stack_next_idx(o_idx,&mut o_stack);
        }

        match (idx,o_idx) {
            (IDX_NOT_EXIST,IDX_NOT_EXIST) => Ordering::Equal,
            (IDX_NOT_EXIST,_) => Ordering::Less,
            (_,IDX_NOT_EXIST) => Ordering::Greater,
            _ => panic!(),
        }
    }//}}}
}//}}}

impl<'a,T:Default + Ord> Iterator for TreeOrdIter<'a,T>
{//{{{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>
    {//{{{
        match self.idx {
            IDX_NOT_EXIST => None,
            _ => {
                let node = &self.tree.data[self.idx as usize];
                self.idx = self.tree.get_stack_next_idx(self.idx,&mut self.stack);
                Some(&node.value)
            }
        }
    }//}}}
}//}}}

impl<'a,T:Default + Ord> Iterator for TreeIter<'a,T>
{//{{{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>
    {//{{{
        while let Some(ref node) = self.tree.data.get(self.idx as usize) {
            self.idx += 1;
            if node.valid {
                return Some(&node.value);
            }
        }

        None
    }//}}}
}//}}}

impl<T:Display + Default + Ord> Display for Tree<T>
{//{{{
    fn fmt(&self,f:&mut Formatter) -> std::fmt::Result
    {//{{{
        write!(f,"[")?;

        let mut first = true;
        for value in self.ord_iter() {
            write!(f,"{}{}",if first { first = false; ""} else { "," },*value)?;
        }

        write!(f,"]")
    }//}}}
}//}}}

impl<T:Debug + Default + Ord> Debug for Tree<T>
{//{{{
    fn fmt(&self,f:&mut Formatter) -> std::fmt::Result
    {//{{{
        write!(f,"[")?;

        let mut first = true;
        for value in self.ord_iter() {
            write!(f,"{:?}{:?}",if first { first = false; ""} else { "," },*value)?;
        }

        write!(f,"]")
    }//}}}
}//}}}

#[cfg(test)]
mod tests {
use super::*;

impl<T:Default + Ord> Tree<T>
{//{{{
    fn check_properties(&self) -> Result<(),&str>
    {//{{{
        let leaf = &self.data[self.leaf_idx as usize];
        if !leaf.color {
            return Err("ERROR: leaf_node color");
        }

        if leaf.left_idx != IDX_NOT_EXIST || leaf.right_idx != IDX_NOT_EXIST {
            return Err("ERROR: leaf_node indexes");
        }

        if self.root_idx != IDX_NOT_EXIST {
            // - check if root node is black -
            let r_node = &self.data[self.root_idx as usize];
            if !r_node.color {
                return Err("ERROR: root node is not black");
            }

            // - create node index and path length stacks -
            let mut ni_stack:Vec<u32> = vec![];
            let mut pl_stack:Vec<u32> = vec![];

            // - insert root on stack -
            ni_stack.push(self.root_idx);
            pl_stack.push(0);

            let mut r_path_length = IDX_NOT_EXIST;
            while let (Some(node_idx),Some(mut path_length)) = (ni_stack.pop(),pl_stack.pop()) {
                let stack_depth = ni_stack.len();

                let node = &self.data[node_idx as usize];

                if node.color {
                    path_length += 1;
                }
                else {
                    if node.left_idx == IDX_NOT_EXIST || node.right_idx == IDX_NOT_EXIST {
                        return Err("ERROR: red node has not two childs!");
                    }

                    if !self.data[node.left_idx as usize].color || !self.data[node.right_idx as usize].color {
                        return Err("ERROR: child of red node is not black!");
                    }
                }

                if node.left_idx != IDX_NOT_EXIST {
                    ni_stack.push(node.left_idx);
                    pl_stack.push(path_length);
                }

                if node.right_idx != IDX_NOT_EXIST {
                    ni_stack.push(node.right_idx);
                    pl_stack.push(path_length);
                }

                // - if node is leaf node -
                if stack_depth == ni_stack.len() {
                    if r_path_length != IDX_NOT_EXIST {
                        if r_path_length != path_length {
                            return Err("ERROR: all path have no same length!");
                        }
                    }
                    else {
                        r_path_length = path_length;
                    }
                }

            }
        }

        // - test if are node values sorted -
        if self.root_idx != IDX_NOT_EXIST {
            let mut stack:Vec<u32> = vec![];

            let mut idx = self.get_stack_min_value_idx(self.root_idx,&mut stack);
            loop {
                let l_idx = idx;
                idx = self.get_stack_next_idx(idx,&mut stack);
                if idx == IDX_NOT_EXIST {
                    break;
                }

                match Ord::cmp(&self.data[l_idx as usize].value,&self.data[idx as usize].value) {
                    Ordering::Greater => return Err("ERROR: values in rb_tree are not sorted"),
                    _ => {}
                }
            }
        }

        Ok(())
    }//}}}
}//}}}

#[test]
fn get_stack_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![0,1,2,3,4]);
    assert_eq!(tree.get_min_value_idx(tree.root_idx),1);

    let mut stack:Vec<u32> = vec![];
    let mut vec:Vec<u32> = vec![];
    let mut idx = tree.get_stack_min_value_idx(tree.root_idx,&mut stack);
    while idx != IDX_NOT_EXIST {
        vec.push(idx);
        idx = tree.get_stack_next_idx(idx,&mut stack);
    }
    assert_eq!(vec,vec![1,2,3,4,5]);
    assert_eq!(tree.check_properties(),Ok(()));
}//}}}

#[test]
fn get_min_value_idx_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![0,1,2,3,4]);
    assert_eq!(tree.get_min_value_idx(tree.root_idx),1);

    let tree = Tree::<u32>::from(vec![5,4,3,2,1]);
    assert_eq!(tree.get_min_value_idx(tree.root_idx),5);
    assert_eq!(tree.check_properties(),Ok(()));
}//}}}

#[test]
fn get_max_value_idx_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![0,1,2,3,4]);
    assert_eq!(tree.get_max_value_idx(tree.root_idx),5);

    let tree = Tree::<u32>::from(vec![5,4,3,2,1]);
    assert_eq!(tree.get_max_value_idx(tree.root_idx),1);
}//}}}

#[test]
fn get_next_idx_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![5,4,3,2,1]);

    let mut vec:Vec<u32> = vec![];
    let mut idx = tree.get_min_value_idx(tree.root_idx);
    while idx != IDX_NOT_EXIST {
        vec.push(idx);
        idx = tree.get_next_idx(idx);
    }
    assert_eq!(vec,vec![5,4,3,2,1]);

    let mut vec:Vec<u32> = vec![];
    let mut idx = tree.get_max_value_idx(tree.root_idx);
    while idx != IDX_NOT_EXIST {
        vec.push(idx);
        idx = tree.get_next_idx(idx);
    }
    assert_eq!(vec,vec![1]);
}//}}}

#[test]
fn get_prev_idx_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![5,4,3,2,1]);

    let mut vec:Vec<u32> = vec![];
    let mut idx = tree.get_min_value_idx(tree.root_idx);
    while idx != IDX_NOT_EXIST {
        vec.push(idx);
        idx = tree.get_prev_idx(idx);
    }
    assert_eq!(vec,vec![5]);

    let mut vec:Vec<u32> = vec![];
    let mut idx = tree.get_max_value_idx(tree.root_idx);
    while idx != IDX_NOT_EXIST {
        vec.push(idx);
        idx = tree.get_prev_idx(idx);
    }
    assert_eq!(vec,vec![1,2,3,4,5]);
}//}}}

#[test]
fn insert_t0()
{//{{{
    let mut tree = Tree::<u32>::new();
    for idx in 0..5 {
        tree.insert(idx);
    }
    assert_eq!(tree.len(),5);
    assert_eq!(format!("{}",tree),"[0,1,2,3,4]");
    assert_eq!(tree,Tree::<u32>::from(vec![0,1,2,3,4]));
}//}}}

#[test]
fn unique_insert_t0()
{//{{{
    let mut tree = Tree::<u32>::new();
    for idx in 0..20 {
        tree.unique_insert(idx % 5);
    }
    assert_eq!(tree.len(),5);
    assert_eq!(format!("{}",tree),"[0,1,2,3,4]");
    assert_eq!(tree,Tree::<u32>::from(vec![0,1,2,3,4]));
}//}}}

#[test]
fn remove_t0()
{//{{{
    let mut tree = Tree::<u32>::from(vec![0,1,2,3,4,5,6,7,8,9]);
    for idx in 4..9 {
        tree.remove(idx);
    }
    assert_eq!(tree.len(),5);
    assert_eq!(format!("{}",tree),"[0,1,2,8,9]");
    assert_eq!(tree,Tree::<u32>::from(vec![0,1,2,8,9]));
}//}}}

#[test]
fn get_idx_t0()
{//{{{
    let mut tree = Tree::<u32>::from(vec![5,4,3,2,1]);
    assert_eq!(tree.get_idx(&4),2);
    assert_eq!(tree.get_idx(&2),4);

    let vec:Vec<u32> = (1u32..=5).into_iter().map(|x| tree.get_idx(&x)).collect();
    assert_eq!(vec,vec![5,4,3,2,1]);

    tree.remove(2);
    let vec:Vec<u32> = (1u32..=5).into_iter().map(|x| tree.get_idx(&x)).collect();
    assert_eq!(vec,vec![5,4,3,IDX_NOT_EXIST,1]);
}//}}}

#[test]
fn get_left_idx_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![5,4,4,4,3,3,3,3,2,2,1]);
    assert_eq!(tree.len(),11);
    assert_eq!(tree.check_properties(),Ok(()));

    let vec:Vec<u32> = (1u32..=5).into_iter().rev().map(|x| tree.get_left_idx(&x)).collect();
    assert_eq!(vec,vec![1,2,5,9,11]);
}//}}}

#[test]
fn get_gre_idx_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![12,10,8,5,3,1]);
    assert_eq!(tree.len(),6);
    assert_eq!(tree.check_properties(),Ok(()));

    let vec:Vec<u32> = (0u32..=13).into_iter().map(|x| tree.get_gre_idx(&x)).collect();
    assert_eq!(vec,vec![6,6,5,5,4,4,3,3,3,2,2,1,1,IDX_NOT_EXIST]);
}//}}}

#[test]
fn get_lee_idx_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![12,10,8,5,3,1]);
    assert_eq!(tree.len(),6);

    let vec:Vec<u32> = (0u32..=12).into_iter().map(|x| tree.get_lee_idx(&x)).collect();
    assert_eq!(vec,vec![IDX_NOT_EXIST,6,6,5,5,4,4,4,3,3,2,2,1]);
}//}}}

#[test]
fn get_idxs_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![1,1,3,3,2,1,2,1,3,1,1,2,1,3,2,3,2,1,2,3,1,2,3,1,2,1,2,3,2,1,2,1,3,1]);
    assert_eq!(tree.len(),34);
    assert_eq!(tree.check_properties(),Ok(()));
    assert_eq!(tree.get_idxs(&1),vec![8,18,24,30,32,34,26,21,11,13,10,2,6,1]);
    assert_eq!(tree.get_idxs(&2),vec![5,17,22,27,29,31,25,19,12,15,7]);
    assert_eq!(tree.get_idxs(&3),vec![3,16,23,28,33,20,9,14,4]);
}//}}}

#[test]
fn iter_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![5,4,3,2,1]);
    let vec:Vec<u32> = tree.ord_iter().map(|x|*x).collect();
    assert_eq!(vec,vec![1,2,3,4,5]);

    let vec:Vec<u32> = tree.iter().map(|x|*x).collect();
    assert_eq!(vec,vec![5,4,3,2,1]);
}//}}}

#[test]
fn equal_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![0,1,2,3,4]);
    let mut tree1 = Tree::<u32>::new();

    let mut vec = vec![];
    for idx in 0..9 {
        tree1.insert(idx);
        vec.push(tree == tree1);
    }
    assert_eq!(vec,vec![false,false,false,false,true,false,false,false,false]);
    assert_eq!(Tree::<u32>::new(),Tree::<u32>::new());
}//}}}

#[test]
fn ord_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![0,1,2,3,4]);
    let mut tree1 = Tree::<u32>::new();

    let mut vec = vec![];
    for idx in 0..9 {
        tree1.insert(idx);
        vec.push(Ord::cmp(&tree,&tree1) as i8);
    }
    assert_eq!(vec,vec![1,1,1,1,0,-1,-1,-1,-1]);
}//}}}

#[test]
fn check_properties_t0()
{//{{{
    let mut tree = Tree::<u32>::new();
    for _ in 0..100000 {
        tree.insert(unsafe {rand()} as u32);
    }
    assert_eq!(tree.check_properties(),Ok(()));
}//}}}

}
