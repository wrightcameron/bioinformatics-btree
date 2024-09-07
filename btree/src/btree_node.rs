use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

#[derive(Debug)]
pub struct Node{
    pub number_of_keys: u32,
    pub is_leaf: bool,
    pub offset: u32,
    pub keys: Vec<TreeObject>,
    pub children_ptrs: Vec<u32>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
        number_of_keys: 0,
        is_leaf: true,
        offset: 0,
        keys: Vec::new(),
        children_ptrs: Vec::new(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.number_of_keys == other.number_of_keys &&
        self.is_leaf == other.is_leaf &&
        self.offset == other.offset &&
        self.keys == other.keys &&
        self.children_ptrs == other.children_ptrs
    }
}

impl Node {
    pub fn new() -> Node {
        Node {..Default::default()}
    }

    pub fn add_child_ptr(&mut self, offset: u32) {
        self.children_ptrs.push(offset);
    }

    pub fn is_leaf(&self) -> bool {
        // self.children_ptrs.len() == 0
        self.is_leaf
    }

    pub fn number_of_keys(&self) -> u32 {
        // self.keys.len() as u32
        self.number_of_keys
    }

    pub fn number_of_children(&self) -> u32 {
        self.children_ptrs.len() as u32
    }
}

#[derive(Clone, Debug)]
pub struct TreeObject {
    pub sequence: u32,
    pub frequency: u32,
}

impl PartialEq for TreeObject {
    fn eq(&self, other: &Self) -> bool {
        self.sequence == other.sequence
    }
}

impl PartialOrd for TreeObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sequence.partial_cmp(&other.sequence)
    }
}

impl TreeObject{
    pub fn new(sequence: u32, frequency: u32) -> Self {
        TreeObject {sequence, frequency}
    }

    pub fn increase_frequency(&mut self) {
        self.frequency += 1;
    }
}