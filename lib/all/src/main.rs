#![allow(dead_code)]

use std::collections::LinkedList;
use rbtree::RBTree;

const CYCLE_COUNT:u32 = 1_000_000;

fn list_test()
{//{{{
    let mut list = list::List::<u32>::new();

    let mut idx = 0;
    while idx < CYCLE_COUNT {
        list.prepend(idx);
        idx += 1;
    }
    
    let mut sum:u64 = 0;
    for value in &mut list.iter() {
        sum += *value as u64;
        //println!("value: {}",value);
    }

    println!("sum: {}\n",sum)
}//}}}

fn tree_test()
{//{{{
    let mut tree = tree::Tree::<u32>::new();

    let mut idx = 0;
    while idx < CYCLE_COUNT {
        tree.insert(idx);
        idx += 1;
    }
    
    //let mut sum:u64 = 0;
    //for value in &mut tree.iter() {
    //    sum += *value as u64;
    //    //println!("value: {}",value);
    //}

    //println!("sum: {}\n",sum)
}//}}}

fn tree_test_1()
{//{{{
    let mut tree = tree::Tree::<(u32,u32)>::new();

    let mut idx = 0;
    while idx < CYCLE_COUNT {
        tree.insert((idx,0));
        idx += 1;
    }
    
    //let mut sum:u64 = 0;
    //for value in &mut tree.iter() {
    //    sum += *value as u64;
    //    //println!("value: {}",value);
    //}

    //println!("sum: {}\n",sum)
}//}}}

fn rust_vec_test()
{//{{{
    let mut vec:Vec<u32> = vec![];

    let mut idx = 0;
    while idx < CYCLE_COUNT {
        vec.push(idx);
        idx += 1;
    }
    
    let mut sum:u64 = 0;
    for value in &mut vec.iter() {
        sum += *value as u64;
        //println!("value: {}",value);
    }

    println!("sum: {}\n",sum)
}//}}}

fn rust_list_test()
{//{{{
    let mut list = LinkedList::<u32>::new();

    let mut idx = 0;
    while idx < CYCLE_COUNT {
        list.push_front(idx);
        idx += 1;
    }
    
    let mut sum:u64 = 0;
    for value in &mut list.iter() {
        sum += *value as u64;
        //println!("value: {}",value);
    }

    println!("sum: {}\n",sum)
}//}}}

fn rust_tree_test()
{//{{{
    let mut tree = RBTree::<u32,u32>::new();

    let mut idx = 0;
    while idx < CYCLE_COUNT {
        tree.insert(idx,0);
        idx += 1;
    }
    
    //let mut sum:u64 = 0;
    //for value in &mut list.iter() {
    //    sum += *value as u64;
    //    //println!("value: {}",value);
    //}

    //println!("sum: {}\n",sum)
}//}}}

fn main() {
    println!("Hello, world!");
    println!("u32 size: {}",std::mem::size_of::<u32>());
    println!("usize size: {}",std::mem::size_of::<usize>());
    println!("Option<usize> size: {}",std::mem::size_of::<Option<usize>>());

    //list_test();
    //tree_test();
    tree_test_1();
    //rust_vec_test();
    //rust_list_test();
    //rust_tree_test();
}
