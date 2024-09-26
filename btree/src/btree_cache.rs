use std::rc::Rc;
use std::cell::RefCell;
use crate::btree_node::Node;

// TODO Instead of turning cache into Node specific, could make it a interface and create methods in Node

/// BTreeCache is a struct encapsulating a Vec, for storing Reference Counters of Nodes in memory
pub struct BTreeCache {
    cache: Vec<Rc<RefCell<Node>>>,
    max_size: u32,
}

impl BTreeCache {
    /// Create BTree Cache struct, with set size
    pub fn new(max_size: u32) -> Self {
        // TODO  Handle if max size is set to 0 or less, should throw an error
        BTreeCache {
            cache: Vec::new(),
            max_size,
        }
    }

    /// Find Node within cache with matching offset, return node but place reference to node at front of cache
    pub fn get_object(&mut self, offset: u32) -> Option<Rc<RefCell<Node>>> {
        let index = self.cache.iter().position(|x| x.borrow().offset == offset)?;
        let res = self.cache.remove(index);
        //Move node to front of the cache
        self.cache.insert(0, res);
        self.cache.get(0).cloned()
        
    }

    /// Add node to cache, if cache is full pop off Node at end
    pub fn add_object(&mut self, obj: Rc<RefCell<Node>>) {
        // Check if obj already in vec
        if self.cache.contains(&obj)  {
            return
        }
        if self.cache.len() as u32 == self.max_size {
            self.cache.pop();
        }
        self.cache.insert(0, obj)
    }

    /// Remove object off end of cache, return node
    #[allow(dead_code)]
    pub fn remove_object(mut self) -> Option<Rc<RefCell<Node>>>{
        self.cache.pop()
    }

    /// Empty cache
    #[allow(dead_code)]
    pub fn clear_cache(mut self) {
        self.cache.clear();
    }
}
