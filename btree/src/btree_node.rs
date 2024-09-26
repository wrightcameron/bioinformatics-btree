use std::cmp::Ordering;

/// Node Struct, representing every node within btree
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
        offset: 8,
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
    /// Node constructor
    pub fn new() -> Node {
        Node {..Default::default()}
    }

    /// Add child pointer, represented as node file offset.
    pub fn add_child_ptr(&mut self, offset: u32) {
        self.children_ptrs.push(offset);
    }

    /// boolean response if node is a leaf node or has children
    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }

    /// Get number of keys within node
    pub fn number_of_keys(&self) -> u32 {
        self.number_of_keys
    }

    /// Get number of children offset pointers within node
    pub fn number_of_children(&self) -> u32 {
        self.children_ptrs.len() as u32
    }
}

/// TreeObject represents the Key Value pair stored within Btree.
/// Both Key and Value are unfortunetly coupled together.
#[derive(Clone, Copy, Debug, Eq)]
pub struct TreeObject {
    pub sequence: u64,
    pub frequency: u64,
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

impl Ord for TreeObject {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sequence.cmp(&other.sequence)
    }
}

impl TreeObject{
    /// Constructor for TreeObject, return Treeobject
    pub fn new(sequence: u64, frequency: u64) -> Self {
        TreeObject {sequence, frequency}
    }

    /// Increment the frequency up by one
    pub fn increase_frequency(&mut self) {
        self.frequency += 1;
    }
}