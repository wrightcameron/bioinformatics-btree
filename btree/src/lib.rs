mod btree_cache;
mod pager;
pub mod btree_node;

use crate::pager::Pager;
use crate::btree_node::*;

pub struct BTree {
    degree: u32,
    number_of_nodes: u32,
    number_of_keys: u32,
    height: u32,
    pager: Pager,
}

impl BTree {
    pub fn new(sequence_length: u32, degree: u32, file_name: &str, use_cache: bool, cache_size: u32) -> BTree {
        let mut btree = BTree {
            degree,
            number_of_nodes: 1,
            number_of_keys: 0,
            height: 0,
            pager: Pager::new(&file_name, use_cache, cache_size, degree).unwrap(),
        };
        // Create Root node
        let mut root_node = Node::new();
        btree.pager.write_metadata(root_node.offset, degree);
        root_node.offset = btree.pager.file_cursor;
        btree.pager.write(&root_node);
        btree
    }

    pub fn btree_search_root(&mut self, key: TreeObject) -> Option<TreeObject> {
        let offset = self.pager.get_root_offset();
        let root_node = self.pager.read(offset);
        self.btree_search(root_node, key)
    }
    
    /// Searches the BTree for the TreeObject given as an argument
    pub fn btree_search(&mut self, given_root: Node, key: TreeObject) -> Option<TreeObject> {
        let mut index = 0;
        while index <= given_root.keys.len() && key > *given_root.keys.get(index).unwrap() {
            index += 1;
        }
        if index <= given_root.keys.len() && key == *given_root.keys.get(index).unwrap() {
            return Some(given_root.keys.get(index).unwrap().clone());
        } else if given_root.is_leaf() {
            return None
        } else {
            let child = self.pager.read(*given_root.children_ptrs.get(index).unwrap());
            return self.btree_search(child, key);
        }
    }

    // TODO Shouldn't this return child offsets of keys instead of keys, cause big enough btree this would take alot of memory.
    // TODO Also this might change once we add a cache, well anything with reference counters will change.
    pub fn btree_in_order_traversal(&mut self, node_offset_option: Option<u32>, sorted_keys: &mut Vec<TreeObject>) {
        if let Some(node_offset) = node_offset_option {
            let node = self.pager.read(node_offset);
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

    pub fn get_sorted_key_array(&mut self) -> Vec<u64> {
        let mut sorted_keys: Vec<TreeObject> = Vec::new();
        let root_offset = self.pager.get_root_offset() as u32;
        self.btree_in_order_traversal(Some(root_offset), &mut sorted_keys);
        sorted_keys.iter().map(| x | x.sequence ).collect()
    }

    /// Splits the tree when the degree of a node gets to size of degree
    pub fn btree_split_child(&mut self, given_root: &mut Node, index: u32) {
        let mut y: Node = self.pager.read(*given_root.children_ptrs.get(index as usize).unwrap());
        let mut z: Node = Node::new();
        z.offset = self.pager.file_cursor;
        z.is_leaf = y.is_leaf;
        z.number_of_keys = self.degree - 1;

        for i in 0..(self.degree - 1) {
            z.keys.insert(i as usize, y.keys.remove((self.degree) as usize));
            y.number_of_keys -= 1;
        }

        if ! y.is_leaf() {
            for i in 0..(self.degree) {
                z.children_ptrs.insert( i as usize, y.children_ptrs.remove((self.degree) as usize ))
            }
        }
        y.number_of_keys = self.degree - 1;
        given_root.number_of_keys = given_root.number_of_keys + 1;
        given_root.children_ptrs.insert(index as usize + 1, z.offset );
        given_root.keys.insert(index as usize, y.keys.remove(self.degree as usize - 1));
        self.pager.write(&y);
        self.pager.write(&z);
        self.pager.write(&given_root);
    }

    /// Inserts a node into the BTree
    pub fn btree_insert(&mut self, key: TreeObject){
        let (root_offset, degree) = self.pager.read_metadata();
        let mut root_node = self.pager.read(root_offset);
        if root_node.keys.len() as u32 == self.maximum_keys() {
            self.height += 1;
            let old_root = root_node;
            // file_cursor += node_size; this should be done in the pager
            let mut node = Node::new();
            node.is_leaf = false;
            node.number_of_keys = 0;
            node.add_child_ptr(old_root.offset);
            node.offset = self.pager.file_cursor;
            // TODO Redundent writes
            // Write above to file
            self.pager.write(&node);  // This needs to be written to move file cursor, or we move the file cursor some other way.
            // self.pager.write(&old_root);
            self.pager.write_metadata(node.offset, degree);
            self.number_of_nodes += 1;
            root_node = node;
            self.btree_split_child(&mut root_node, 0);
            self.btree_insert_non_full(&mut root_node, key);
        } else {        
            self.btree_insert_non_full(&mut root_node, key);
        }
    }

    /// Inserts an object into the BTree, when the BTree is not full.
    pub fn btree_insert_non_full(&mut self, given_root: &mut Node, key: TreeObject) {
        let mut index: isize = given_root.keys.len() as isize;
        if given_root.is_leaf() {
            while index > 0 && key < *given_root.keys.get(index as usize - 1).unwrap() {
                index -= 1;
            }
            if index > 0 && key == *given_root.keys.get(index as usize - 1).unwrap() {
                given_root.keys.get_mut(index as usize - 1).unwrap().increase_frequency();
            } else {
                given_root.keys.insert(index as usize, key);
                given_root.number_of_keys += 1;
                self.number_of_keys += 1;
            }
            self.pager.write(&given_root);
        } else {
            while index >= 1 && key < *given_root.keys.get(index as usize - 1).unwrap() {
                index -= 1;
            }
            index += 1;
            let mut child: Node = self.pager.read(*given_root.children_ptrs.get(index as usize - 1).unwrap());
            if child.keys.len() == (2 * self.degree as usize) - 1 {
                self.btree_split_child(given_root, index as u32 - 1);
                if key > *given_root.keys.get(index as usize - 1).unwrap() {
                    index += 1;
                }
                // Refresh the child node, which was changed from the split
                child = self.pager.read(*given_root.children_ptrs.get(index as usize - 1).unwrap());
            }
            self.btree_insert_non_full(&mut child, key);
            // if index > 0 && key == *given_root.keys.get(index - 1).unwrap() {
            //     given_root.keys.get_mut(index - 1).unwrap().increase_frequency();
            //     self.pager.write(&given_root);
            //     self.number_of_keys += 1;
            // } else {
            //     while index >= 0 && key < *given_root.keys.get(index).unwrap() {
            //         index -= 1;
            //     }
            //     index += 1;
            //     let mut child: Node = self.pager.read(*given_root.children_ptrs.get(index - 1).unwrap());
            //     if child.keys.len() == (2 * self.degree as usize) - 1 {
            //         self.btree_split_child(given_root, index
                        
            //              as u32);
            //         if key > *given_root.keys.get(index - 1).unwrap() {
            //             index += 1;
            //         }
            //     }
            //     self.btree_insert_non_full(&mut child, key)
            // }
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
        let sequence_length = 0;
        let use_cache = false;
        let cache_size = 0;
        BTree::new(sequence_length, degree, file_name, use_cache, cache_size)
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

    // /**
    //  * Ten keys (10 -> 1) inserted into a BTree of degree 2.
    //  */
    #[test]
    fn test_insert_10_keys_reverse_order() {
        let file_name = "test_insert_10_keys_reverse_order.tmp";
        let mut b: BTree = btree(2, file_name);
        let mut input = Vec::new();
        for i in (0..10).rev() {
            input.push(i);
            b.btree_insert(TreeObject {sequence: i as u64, frequency: 1 })
        }
        input.reverse();
        assert_eq!(10, b.get_size());
        assert_eq!(2, b.get_height());
        assert!(validate_btree_inserts(b, input))
    }

    // /**
    //  * Tests that adding duplicate key values to the tree doesn't create
    //  * duplicates within the tree.
    //  */
    #[test]
    fn test_insert_10_duplicates() {
        let file_name = "test_insert_10_duplicates.tmp";
        let mut b: BTree = btree(2, file_name);
        let input = vec![1,1,1,1,1,1,1,1,1,1];
        for _ in 0..10 {
            b.btree_insert(TreeObject {sequence: 1, frequency: 1 })
        }

        assert_eq!(1, b.get_size());
        assert_eq!(0, b.get_height());
        assert!(validate_btree_inserts(b, input))
    }

}
