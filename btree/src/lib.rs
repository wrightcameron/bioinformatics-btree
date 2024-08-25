mod btree_cache;
mod pager;
mod btree_node;

use crate::btree_node::*;

// enum GeneBase {
//     A,
//     T,
//     C,
//     G,
// }

pub struct BTree<'a> {
    root_node: Option<Node<'a>>,
    degree: u32,
    number_of_nodes: u32,
    height: u32,
}

impl Default for BTree<'_> {
    fn default() -> Self { 
        BTree {
            root_node: None,
            degree: 0,
            number_of_nodes: 0,
            height: 0,
        }
    }
}

impl BTree<'_> {
    pub fn new(sequence_length: u32, degree: u32, file_name: &str, use_cache: bool, cache_size: u32) -> BTree<'static> {
        let mut btree = BTree { ..Default::default() };
        btree.degree = degree;
        // Create Root node
        let mut root_node: Node = Node::new();
        root_node.is_leaf = true;
        root_node.number_keys = 0;
        root_node.offset = 12;
        // Need to encapsulate the node in an option
        btree.root_node = Some(root_node);
        btree.number_of_nodes = 1;
        // TODO Do we need some offset here?

        let output_file = format!("{file_name}.btree.data.{sequence_length}.{degree}");
        let pager = pager::Pager::new(file_name, use_cache, cache_size);
        // pager.write_metadata(offset, degree);
        // pager.write_node(&root_node);
        btree
    }

    // pub fn btree_search(given_root, key) -> Option<TreeObject> {

    // }

    // pub fn btree_split_child(given_root, index) {

    // }

    pub fn btree_insert(self, key: TreeObject){

    }

    // pub fn btree_insert_non_full(given_root, key) {

    // }

    // pub fn in_order_traversal(root, writer, sequence_length){

    // }

    /**
     * @return Returns the number of keys in the BTree.
     */
    pub fn get_size(&self) -> u32 {
        self.number_of_nodes - 1
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

    pub fn get_sorted_key_array(&self) -> Vec<i64>{
        Vec::new()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_NAME: &str = "Test_BTree.tmp";

    fn setup(){
        todo!();
    }

    fn teardown(){
        std::fs::remove_file(TEST_FILE_NAME).unwrap();
    }

    /// BTree constructor used only for testing
    fn btree(degree: u32, file_name: &str) -> BTree {
        let sequence_length = 0;
        let use_cache = false;
        let cache_size = 0;
        BTree::new(sequence_length, degree, file_name, use_cache, cache_size)
    }

    fn validate_btree_inserts(b: BTree, input_keys: Vec<i64>) -> bool {
        let mut btree_keys = b.get_sorted_key_array();
        // input may be unsorted
        btree_keys.sort();
        // track input as a dynamic set to easily remove duplicates
        let mut input_no_duplicates: Vec<i64> = Vec::new();
        // Copy with exluding duplicates
        for i in 1..input_keys.len() {
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

        let prev: i64 = btree_keys[0];

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
        let b: BTree = btree(1, TEST_FILE_NAME);
        assert_eq!(0, b.height);
        assert_eq!(0, b.get_size());
        assert_eq!(1, b.number_of_nodes);
        teardown();
    }

    /// Test constructing a BTree with custom degree.
    #[test]
    fn test_btree_create_degree() {
        let b: BTree = btree(3, TEST_FILE_NAME);
        assert_eq!(3, b.get_degree());
    }

    /// Test inserting a single key into an empty BTree.
    /// BTree size now reflects the single key.
    /// BTree structure is not validated in this test, as it would depend
    /// on searching the tree or examining private members of BTree.
    #[test]
    fn test_insert_one_key() {
        let mut b: BTree = btree(2, TEST_FILE_NAME);
        b.insert(TreeObject { obj: 1 });

        assert_eq!(1, b.get_size());
        assert_eq!(0, b.get_height());
        // assert!()
    }

    /**
     * Ten Keys (0 -> 9) added to a tree of degree 2, ensuring full nodes will be split.
     *
     */
    #[test]
    fn test_insert_10_keys() {
        let mut b: BTree = btree(2, TEST_FILE_NAME);
        //TODO Change this to array, instead of vector
        let mut input: Vec<i64> = Vec::new();
        for i in 0..10 {
            input[i] = i as i64;
            b.insert(TreeObject {obj: i as u32})
        }

        assert_eq!(10, b.get_size());
        assert_eq!(2, b.get_height());
        assert!(validate_btree_inserts(b, input))
    }

    /**
     * Ten keys (10 -> 1) inserted into a BTree of degree 2.
     */
    #[test]
    fn test_insert_10_keys_reverse_order() {
        let mut b: BTree = btree(2, TEST_FILE_NAME);
        //TODO Change this to array, instead of vector
        let mut input: Vec<i64> = Vec::new();
        for i in (0..10).rev() {
            input[i] = i as i64;
            b.insert(TreeObject {obj: i as u32})
        }

        assert_eq!(10, b.get_size());
        assert_eq!(2, b.get_height());
        assert!(validate_btree_inserts(b, input))
    }

    /**
     * Tests that adding duplicate key values to the tree doesn't create
     * duplicates within the tree.
     */
    #[test]
    fn test_insert_10_duplicates() {
        let mut b: BTree = btree(2, TEST_FILE_NAME);
        //TODO Change this to array, instead of vector
        let input = vec![1,1,1,1,1,1,1,1,1,1];
        for _ in 0..10 {
            b.insert(TreeObject {obj: 1})
        }

        assert_eq!(10, b.get_size());
        assert_eq!(2, b.get_height());
        assert!(validate_btree_inserts(b, input))
    }

}
