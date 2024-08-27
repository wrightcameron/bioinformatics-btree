pub struct Node<'a>{
    pub number_keys: u32,
    pub keys: Vec<TreeObject>,
    pub children_ptrs: Vec<&'a Node<'a>>,
    pub is_leaf: bool,
    pub offset: u32,
}

impl Default for Node<'_> {
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

impl Node<'_> {
    pub fn new() -> Node<'static> {
        Node {..Default::default()}
    }
}

pub struct TreeObject {
    pub obj: u32,
}

impl TreeObject{
    pub fn new(obj: u32) -> Self {
        TreeObject {obj}
    }
}