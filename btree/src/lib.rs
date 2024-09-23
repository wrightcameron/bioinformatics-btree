mod btree_cache;
mod pager;
pub mod btree_node;

use std::cell::{RefCell, Ref};
use std::rc::Rc;
use crate::pager::Pager;
use crate::btree_node::*;
use crate::btree_cache::BTreeCache;

pub struct BTree {
    degree: u32,
    number_of_nodes: u32,
    number_of_keys: u32,
    height: u32,
    pager: Pager,
    cache: Option<BTreeCache>,
    root_node: Rc<RefCell<Node>>,
}

impl BTree {
    pub fn new(mut degree: u32, file_name: &str, use_cache: bool, cache_size: u32, truncate_file: bool) -> BTree {
        // If degree is 0, set degree to most optimal for 4096 bytes
        if degree == 0 {
            degree = 102;
        }
        // Retreave the root node if possible
        let mut pager = Pager::new(&file_name, degree, truncate_file).unwrap();
        
        // Create node and recreate pager file if it already exists.
        let mut node = Node::new();
        if truncate_file {
            node.offset = pager.recreate_file(file_name, degree, &node);
        } else {
            node = pager.read_root().unwrap();
        }
        
        // Add node to refcel
        let refcell_node = Rc::new(RefCell::new(node));

        // Create Cache - commented out till I can get Rc working again
        // let cache = if use_cache {
        //     // Add root node
        //     let mut cache = BTreeCache::new(cache_size);
        //     cache.add_object(refcell_node.clone());
        //     Some(cache)
        // } else {
        //     None
        // };

        let btree = BTree {
            degree,
            number_of_nodes: 1,
            number_of_keys: 0,
            height: 0,
            pager,
            cache: None,
            root_node: refcell_node,
        };
        btree
    }

    pub fn btree_search_root(&mut self, key: TreeObject) -> Option<TreeObject> {
        let root_node = self.read_root();
        self.btree_search(root_node, key)
    }           
    
    /// Searches the BTree for the TreeObject given as an argument
    pub fn btree_search(&mut self, given_root: Node, key: TreeObject) -> Option<TreeObject> {
        let mut index = 0;
        while index < given_root.keys.len() && key > *given_root.keys.get(index).unwrap() {
            index += 1;
        }
        if index < given_root.keys.len() && key == *given_root.keys.get(index).unwrap() {
            return Some(given_root.keys.get(index).unwrap().clone());
        } else if given_root.is_leaf() {
            return None
        } else {
            let child = self.read(*given_root.children_ptrs.get(index).unwrap());
            return self.btree_search(child, key);
        }
    }

    // TODO Shouldn't this return child offsets of keys instead of keys, cause big enough btree this would take alot of memory.
    // TODO Also this might change once we add a cache, well anything with reference counters will change.
    pub fn btree_in_order_traversal(&mut self, node_offset_option: Option<u32>, sorted_keys: &mut Vec<TreeObject>) {
        if let Some(node_offset) = node_offset_option {
            let node = self.read(node_offset);
            for i in 0..node.children_ptrs.len() {
                self.btree_in_order_traversal(node.children_ptrs.get(i).copied(), sorted_keys);
                if i < node.keys.len() {
                    sorted_keys.push(node.keys.get(i).copied().expect("Reason TODO"));
                }
            }
            if node.children_ptrs.len() == 0{
                for i in 0..node.keys.len() {
                    sorted_keys.push(node.keys.get(i).copied().expect("Reason TODO"));
                }
            }
        }
    }

    pub fn get_sorted_array(&mut self) -> Vec<TreeObject> {
        let mut sorted_keys: Vec<TreeObject> = Vec::new();
        // If no offset is found, due to empty file return empty Vec
        // TODO or return None it might be better.
        let root_offset = match self.pager.get_root_offset() {
            Ok(root_offset)  => root_offset,
            Err(_) => return vec![],
        };
        self.btree_in_order_traversal(Some(root_offset), &mut sorted_keys);
        sorted_keys
    }

    pub fn get_sorted_key_array(&mut self) -> Vec<u64> {
        let mut sorted_keys: Vec<TreeObject> = Vec::new();
        // If no offset is found, due to empty file return empty Vec
        // TODO or return None it might be better.
        let root_offset = match self.pager.get_root_offset() {
            Ok(root_offset)  => root_offset,
            Err(_) => return vec![],
        };
        self.btree_in_order_traversal(Some(root_offset), &mut sorted_keys);
        sorted_keys.iter().map(| x | x.sequence ).collect()
    }

    /// Splits the tree when the degree of a node gets to size of degree
    pub fn btree_split_child(&mut self, given_root: Rc<RefCell<Node>>, index: u32) {
        let mut borrowed_root = given_root.borrow_mut();
        let y: Rc<RefCell<Node>> = Rc::new(RefCell::new(self.read(*borrowed_root.children_ptrs.get(index as usize).unwrap())));
        let mut z: Node = Node::new();
        z.offset = self.pager.file_cursor;
        z.is_leaf = y.borrow().is_leaf;
        z.number_of_keys = self.degree - 1;

        for i in 0..(self.degree - 1) {
            z.keys.insert(i as usize, y.borrow_mut().keys.remove((self.degree) as usize));
            y.borrow_mut().number_of_keys -= 1;
        }

        if ! y.borrow().is_leaf() {
            for i in 0..(self.degree) {
                z.children_ptrs.insert( i as usize, y.borrow_mut().children_ptrs.remove((self.degree) as usize ))
            }
        }
        y.borrow_mut().number_of_keys = self.degree - 1;
        borrowed_root.number_of_keys = borrowed_root.number_of_keys + 1;
        borrowed_root.children_ptrs.insert(index as usize + 1, z.offset );
        borrowed_root.keys.insert(index as usize, y.borrow_mut().keys.remove(self.degree as usize - 1));
        self.write(&y.borrow());
        self.write(&z);
        self.write(&borrowed_root);
    }

    /// Inserts a node into the BTree
    pub fn btree_insert(&mut self, key: TreeObject){
        if self.root_node.borrow().keys.len() as u32 == self.maximum_keys() {
            self.height += 1;
            let old_root = self.root_node.replace(Node::new());
            self.root_node.borrow_mut().is_leaf = false;
            self.root_node.borrow_mut().number_of_keys = 0;
            self.root_node.borrow_mut().add_child_ptr(old_root.offset);
            self.root_node.borrow_mut().offset = self.pager.file_cursor;
            // Write above to file
            self.write(&self.root_node.clone().borrow() );  // This needs to be written to move file cursor, or we move the file cursor some other way.
            // self.pager.write(&old_root);
            self.pager.write_metadata(self.root_node.borrow().offset, self.degree);
            self.number_of_nodes += 1;
            self.btree_split_child(self.root_node.clone(), 0);
            self.btree_insert_non_full(self.root_node.clone(), key);
        } else {        
            self.btree_insert_non_full(self.root_node.clone(), key);
        }
    }

    /// Inserts an object into the BTree, when the BTree is not full.
    pub fn btree_insert_non_full(&mut self, given_root: Rc<RefCell<Node>>, key: TreeObject) {
        let mut index: isize = given_root.borrow().keys.len() as isize;
        if given_root.borrow().is_leaf() {
            while index > 0 && key < *given_root.borrow().keys.get(index as usize - 1).unwrap() {
                index -= 1;
            }
            if index > 0 && key == *given_root.borrow().keys.get(index as usize - 1).unwrap() {
                given_root.borrow_mut().keys.get_mut(index as usize - 1).unwrap().increase_frequency();
            } else {
                given_root.borrow_mut().keys.insert(index as usize, key);
                given_root.borrow_mut().number_of_keys += 1;
                self.number_of_keys += 1;
            }
            self.write(&given_root.borrow());
        } else {
            while index >= 1 && key < *given_root.borrow().keys.get(index as usize - 1).unwrap() {
                index -= 1;
            }
            if index >= 1 && key == *given_root.borrow().keys.get(index as usize - 1).unwrap() {
                given_root.borrow_mut().keys.get_mut(index as usize - 1).unwrap().increase_frequency();
                self.write(&given_root.borrow());
            } else {
                index += 1;
                let mut child: Rc<RefCell<Node>> = Rc::new(RefCell::new( self.read(*given_root.borrow().children_ptrs.get(index as usize - 1).unwrap()) ));
                if child.borrow().keys.len() == (2 * self.degree as usize) - 1 {
                    self.btree_split_child(given_root.clone(), index as u32 - 1);
                    if key > *given_root.borrow().keys.get(index as usize - 1).unwrap() {
                        index += 1;
                    }
                    // Refresh the child node, which was changed from the split
                    child = Rc::new(RefCell::new( self.read(*given_root.borrow().children_ptrs.get(index as usize - 1).unwrap()) ));
                }
                self.btree_insert_non_full(child, key);
            }

        }
    }

    // pub fn in_order_traversal(root, writer, sequence_length){

    // }

    pub fn maximum_keys(&self) -> u32 {
        2 * self.degree - 1
    }

    /**
     * @return Returns the number of keys in the BTree.
     */
    pub fn get_size(&self) -> u32 {
        self.number_of_keys
    }

    /**
     * @return The degree of the BTree.
     */
    pub fn get_degree(&self) -> u32 {
        self.degree
    }

    /**
     * @return Returns the number of nodes in the BTree.
     */
    pub fn get_number_of_nodes(&self) -> i32 {
        0
    }

    /**
     * @return The height of the BTree
     */
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /**
     * Deletes a key from the BTree. Not Implemented.
     *
     * @param key the key to be deleted
     */
    pub fn delete(key: i32) {}

    /**
     *
     * Insert a given sequence in the B-Tree. If the sequence already exists in the B-Tree,
     * the frequency count is incremented. Otherwise a new node is inserted
     * following the B-Tree insertion algorithm.
     *
     * @param obj
     *            A TreeObject representing a DNA string
     *
     */
    // void insert(TreeObject obj) throws IOException;
    pub fn insert<T>(&mut self, obj: T) {}

    // /**
    //  * Print out all objects in the given BTree in an inorder traversal to a file.
    //  *
    //  * @param out PrintWriter object representing output
    //  */
    // // void dumpToFile(PrintWriter out) throws IOException;
    // // pub fn dump_to_file(out: PrintWriter) {}

    // /**
    //  * Searches for a sequence in the given BTree.
    //  *
    //  * @param key
    //  *            The key value to search for.
    //  */
    // pub fn search<T>(key: i32) -> T {
    //     T
    // }

    // pub fn get_sorted_key_array(&self) -> Vec<i64>{
    //     let keys: Vec<TreeObject> = self.iter();
    //     Vec::new()
    // }

    // TODO Don't delete original code
    // fn write(&mut self, node: &Rc<RefCell<Node>>){
    //     if ! self.cache.is_none() {
    //         self.cache.as_mut().unwrap().add_object(node.clone());
    //     }
    //     self.pager.write(&node.borrow());
    // }

    fn write(&mut self, node: &Node){
        self.pager.write(&node);
    }

    // fn read(&mut self, offset: u32) -> Rc<RefCell<Node>> {
    //     if ! self.cache.is_none() {
    //         match self.cache.as_mut().unwrap().get_object(offset) { 
    //             Some(node) => node,
    //             None => Rc::new(RefCell::new(self.pager.read(offset))),
    //         }
    //     } else {
    //         Rc::new(RefCell::new(self.pager.read(offset)))
    //     }
    // }

    fn read(&mut self, offset: u32) -> Node {
        self.pager.read(offset)
    }

    // TODO DO we want to return a option or result?
    // fn read_root(&mut self) -> Rc<RefCell<Node>> {
    //     // If no offset is found, due to empty file return null
    //     let offset = self.pager.get_root_offset().expect("Root Offset couldn't be found!");
    //     self.read(offset)
    // }

    fn read_root(&mut self) -> Node {
        // If no offset is found, due to empty file return null
        let offset = self.pager.get_root_offset().expect("Root Offset couldn't be found!");
        self.read(offset)
    }

}

impl Iterator for BTree {
    type Item = TreeObject;
    
    fn next(&mut self) -> Option<Self::Item> {
        Some(TreeObject::new(1, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    fn delete_file(file: &str){
        std::fs::remove_file(file).ok();
    }

    /// BTree constructor used only for testing
    fn btree(degree: u32, file_name: &str) -> BTree {
        let use_cache = false;
        let cache_size = 0;
        BTree::new(degree, file_name, use_cache, cache_size, true)
    }

    fn validate_btree_inserts(mut b: BTree, input_keys: Vec<u64>) -> bool {
        let mut btree_keys = b.get_sorted_key_array();
        // input may be unsorted
        btree_keys.sort();
        // track input as a dynamic set to easily remove duplicates
        let mut input_no_duplicates: Vec<u64> = Vec::new();
        // Copy with exluding duplicates
        for i in 0..input_keys.len() {
            if i > 0 {
                // only add an element if it is different from the previous iteration
                if input_keys[i - 1] != input_keys[i] {
                    input_no_duplicates.push(input_keys[i]);
                }
            } else {
                input_no_duplicates.push(input_keys[i])
            }
        }

        if btree_keys.len() != input_no_duplicates.len() {
            return false;
        }

        let prev: u64 = btree_keys[0];

        for i in 0..btree_keys.len() {
            if btree_keys[i] != input_no_duplicates.get(i).unwrap().clone() {
                return false;
            }

            if i > 0 && prev > btree_keys[i] {
                return false;
            }
        }
        return true;
    }

    /// Test simple creation of an empty BTree.
    /// An empty BTree has 1 node with no keys and height of 0.
    #[test]
    fn test_btree_create() {
        let file_name = "test_btree_create.tmp";
        delete_file(file_name);
        let b: BTree = btree(1, file_name);
        assert_eq!(0, b.height);
        assert_eq!(0, b.get_size());    
        assert_eq!(1, b.number_of_nodes);
        delete_file(file_name);
    }

    /// Test constructing a BTree with custom degree.
    #[test]
    fn test_btree_create_degree() {
        let file_name = "test_btree_create_degree.tmp";
        delete_file(file_name);
        let b: BTree = btree(3, file_name);
        assert_eq!(3, b.get_degree());
        delete_file(file_name);
    }

    /// Test inserting a single key into an empty BTree.
    /// BTree size now reflects the single key.
    /// BTree structure is not validated in this test, as it would depend
    /// on searching the tree or examining private members of BTree.
    #[test]
    fn test_insert_one_key() {
        let file_name = "test_insert_one_key.tmp";
        delete_file(file_name);
        let mut b: BTree = btree(2, file_name);
        b.btree_insert(TreeObject { sequence: 1, frequency: 1 });
        assert_eq!(1, b.get_size());
        assert_eq!(0, b.get_height());
        delete_file(file_name);
    }

    #[test]
    fn test_5_key_split_root() {
        // https://youtu.be/K1a2Bk8NrYQ?t=285
        let file_name = "test_5_key_split_root.tmp";
        delete_file(file_name);
        let mut b: BTree = btree(3, file_name); // Chould split at 5 keys
        let input = vec![7, 23, 59, 67, 73, 97];
        b.btree_insert(TreeObject {sequence: 59 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 23 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 7 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 97 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 73 as u64, frequency: 1 });
        // split
        b.btree_insert(TreeObject {sequence: 67 as u64, frequency: 1 });
        assert_eq!(6, b.get_size());
        assert_eq!(1, b.get_height());
        assert!(validate_btree_inserts(b, input));
        delete_file(file_name);
    }


    #[test]
    fn test_10_key_split() {
        // https://youtu.be/K1a2Bk8NrYQ?t=363
        let file_name = "test_10_key_split.tmp";
        delete_file(file_name);
        let mut b: BTree = btree(3, file_name); // Chould split at 5 keys
        let input = vec![7, 19, 23, 41, 59, 61, 67, 73, 79, 97];
        b.btree_insert(TreeObject {sequence: 59 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 23 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 7 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 97 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 73 as u64, frequency: 1 });
        // split
        b.btree_insert(TreeObject {sequence: 67 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 19 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 79 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 61 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 41 as u64, frequency: 1 });
        assert_eq!(10, b.get_size());
        assert_eq!(1, b.get_height());
        assert!(validate_btree_inserts(b, input));
        delete_file(file_name);
    }


    #[test]
    fn test_11_key_3_split() {
        // https://youtu.be/K1a2Bk8NrYQ?t=363
        let file_name = "test_11_key_3_split.tmp";
        delete_file(file_name);
        let mut b: BTree = btree(3, file_name); // Chould split at 5 keys
        let input = vec![7, 19, 23, 41, 59, 61, 67, 73, 74, 79, 97];
        b.btree_insert(TreeObject {sequence: 59 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 23 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 7 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 97 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 73 as u64, frequency: 1 });
        // split
        b.btree_insert(TreeObject {sequence: 67 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 19 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 79 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 61 as u64, frequency: 1 });
        b.btree_insert(TreeObject {sequence: 41 as u64, frequency: 1 });
        // split
        b.btree_insert(TreeObject {sequence: 74 as u64, frequency: 1 });

        assert_eq!(11, b.get_size());
        assert_eq!(1, b.get_height());
        assert!(validate_btree_inserts(b, input));
        delete_file(file_name);
    }

    /// Ten Keys (0 -> 9) added to a tree of degree 2, ensuring full nodes will be split.
    #[test]
    fn test_insert_10_keys() {
        let file_name = "test_insert_10_keys.tmp";
        delete_file(file_name);
        let mut b: BTree = btree(2, file_name);
        let mut input = Vec::new();
        for i in 0..10 {
            input.push(i);
            b.btree_insert(TreeObject {sequence: i as u64, frequency: 1 })
        }
        assert_eq!(10, b.get_size());
        assert_eq!(2, b.get_height());
        assert!(validate_btree_inserts(b, input));
        delete_file(file_name);
    }

    //Ten keys (10 -> 1) inserted into a BTree of degree 2.
    #[test]
    fn test_insert_10_keys_reverse_order() {
        let file_name = "test_insert_10_keys_reverse_order.tmp";
        delete_file(file_name);
        let mut b: BTree = btree(2, file_name);
        let mut input = Vec::new();
        for i in (0..10).rev() {
            input.push(i);
            b.btree_insert(TreeObject {sequence: i as u64, frequency: 1 })
        }
        input.reverse();
        assert_eq!(10, b.get_size());
        assert_eq!(2, b.get_height());
        assert!(validate_btree_inserts(b, input));
        delete_file(file_name);
    }

    //Tests that adding duplicate key values to the tree doesn't create
    //duplicates within the tree.
    #[test]
    fn test_insert_10_duplicates() {
        let file_name = "test_insert_10_duplicates.tmp";
        delete_file(file_name);
        let mut b: BTree = btree(2, file_name);
        let input = vec![1,1,1,1,1,1,1,1,1,1];
        for _ in 0..10 {
            b.btree_insert(TreeObject {sequence: 1, frequency: 1 })
        }

        assert_eq!(1, b.get_size());
        assert_eq!(0, b.get_height());
        assert!(validate_btree_inserts(b, input));
        delete_file(file_name);
    }

}
