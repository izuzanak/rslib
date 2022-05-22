
#![allow(dead_code)]

const IDX_NOT_EXIST:u32 = std::u32::MAX;

struct TreeElement<T>
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
    data:Vec<TreeElement<T>>,
}//}}}

pub struct TreeIter<'a,T>
{//{{{
    tree:&'a Tree<T>,
    stack:Vec<u32>,
    idx:u32,
}//}}}

impl<T:std::default::Default + std::cmp::Ord> Tree<T>
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

        if gp_idx != IDX_NOT_EXIST
        {
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
        else
        {
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
        else
        {
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
                self.data.push(TreeElement{
                    parent_idx:IDX_NOT_EXIST,
                    left_idx:  IDX_NOT_EXIST,
                    right_idx: IDX_NOT_EXIST,
                    color:true,
                    valid:false,
                    value:T::default(),
                });
            }

            new_idx = self.data.len() as u32;
            self.data.push(TreeElement{
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
        else
        {
            let mut node_idx = self.root_idx;
            loop {
                let node = &mut self.data[node_idx as usize];

                let comp_result = std::cmp::Ord::cmp(value,&node.value);
                if comp_result == std::cmp::Ordering::Less {
                    if node.left_idx == self.leaf_idx {
                        node.left_idx = new_idx;
                        break;
                    }
                    node_idx = node.left_idx;
                }
                else {
                    if unique && comp_result == std::cmp::Ordering::Equal {
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
        let mut tree = Tree::<T>::new();

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

            let comp_result = std::cmp::Ord::cmp(value,&node.value);
            if comp_result == std::cmp::Ordering::Less {
                node_idx = node.left_idx;
            }
            else {
                if comp_result == std::cmp::Ordering::Equal {
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

          let comp_result = std::cmp::Ord::cmp(value,&node.value);
          if comp_result == std::cmp::Ordering::Less {
            node_idx = node.left_idx;
          }
          else {
            if comp_result == std::cmp::Ordering::Equal {
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

          let comp_result = std::cmp::Ord::cmp(value,&node.value);
          if comp_result == std::cmp::Ordering::Less {
            good_idx = node_idx;
            node_idx = node.left_idx;
          }
          else {
            if comp_result == std::cmp::Ordering::Equal {
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

          let comp_result = std::cmp::Ord::cmp(value,&node.value);
          if comp_result == std::cmp::Ordering::Less {
            node_idx = node.left_idx;
          }
          else {
            if comp_result  == std::cmp::Ordering::Equal {
              return node_idx;
            }

            good_idx = node_idx;
            node_idx = node.right_idx;
          }
        }

        good_idx
    }//}}}

    pub fn iter(&self) -> TreeIter<T>
    {//{{{
        let mut stack:Vec<u32> = vec![];
        let idx = if self.root_idx != IDX_NOT_EXIST {
            self.get_stack_min_value_idx(self.root_idx,&mut stack)
        }
        else {
            IDX_NOT_EXIST
        };

        TreeIter{
            tree:&self,
            stack:stack,
            idx:idx,
        }
    }//}}}
}//}}}

impl<'a,T:std::default::Default + std::cmp::Ord> Iterator for TreeIter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>
    {//{{{
        match self.idx {
            IDX_NOT_EXIST => None,
            _ => {
                let element = &self.tree.data[self.idx as usize];
                self.idx = self.tree.get_stack_next_idx(self.idx,&mut self.stack);
                Some(&element.value)
            }
        }
    }//}}}
}

impl<T:std::fmt::Display + std::default::Default + std::cmp::Ord> std::fmt::Display for Tree<T> {
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
}//}}}

#[test]
fn get_min_value_idx_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![0,1,2,3,4]);
    assert_eq!(tree.get_min_value_idx(tree.root_idx),1);

    let tree = Tree::<u32>::from(vec![5,4,3,2,1]);
    assert_eq!(tree.get_min_value_idx(tree.root_idx),5);
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
    let mut idx:u32 = 0;
    while idx < 5 {
        tree.insert(idx);
        idx += 1
    }
    assert_eq!(tree.len(),5);
    assert_eq!(format!("{}",tree),"[0,1,2,3,4]");
}//}}}

#[test]
fn unique_insert_t0()
{//{{{
    let mut tree = Tree::<u32>::new();
    let mut idx:u32 = 0;
    while idx < 20 {
        tree.unique_insert(idx % 5);
        idx += 1
    }
    assert_eq!(tree.len(),5);
    assert_eq!(format!("{}",tree),"[0,1,2,3,4]");
}//}}}

#[test]
fn remove_t0()
{//{{{
    let mut tree = Tree::<u32>::from(vec![0,1,2,3,4,5,6,7,8,9]);
    let mut idx:u32 = 4;
    while idx < 9 {
        tree.remove(idx);
        idx += 1
    }
    assert_eq!(tree.len(),5);
    assert_eq!(format!("{}",tree),"[0,1,2,8,9]");
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

    let vec:Vec<u32> = (1u32..=5).into_iter().rev().map(|x| tree.get_left_idx(&x)).collect();
    assert_eq!(vec,vec![1,2,5,9,11]);
}//}}}

#[test]
fn get_gre_idx_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![12,10,8,5,3,1]);
    assert_eq!(tree.len(),6);

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
fn iter_t0()
{//{{{
    let tree = Tree::<u32>::from(vec![5,4,3,2,1]);
    let mut vec = vec![];
    for value in tree.iter() {
        vec.push(*value);
    }
    assert_eq!(vec,vec![1,2,3,4,5]);

    let mut vec = vec![];
    for value in tree.data.iter() {
        if value.valid {
            vec.push(value.value);
        }
    }
    assert_eq!(vec,vec![5,4,3,2,1]);
}//}}}

}
