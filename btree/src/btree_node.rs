#[derive(Debug)]
pub struct Node{
    pub number_keys: u32,
    pub keys: Vec<TreeObject>,
    pub children_ptrs: Vec<u32>,
    pub is_leaf: bool,
    pub offset: u32,
}

impl Default for Node {
    fn default() -> Self {
        Node { 
        number_keys: 0,
        keys: Vec::new(),
        children_ptrs: Vec::new(),
        is_leaf: true,
        offset: 0
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.number_keys == other.number_keys &&
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
}

#[derive(Debug)]
pub struct TreeObject {
    pub sequence: u32,
    pub frequency: u32,
}

impl PartialEq for TreeObject {
    fn eq(&self, other: &Self) -> bool {
        self.sequence == other.sequence &&
        self.frequency == other.frequency
    }
}

impl TreeObject{
    pub fn new(sequence: u32, frequency: u32) -> Self {
        TreeObject {sequence, frequency}
    }
}