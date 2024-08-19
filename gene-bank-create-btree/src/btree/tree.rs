pub struct BTree {
    root_node: Option<Node>,
    degree: i32,
}

impl BTree {
    pub fn new() -> BTree {
        //TODO Handle degree, fileName, useCache, cacheSize
        btree = BTree { ..Default::default() };
        btree.degree = 0;
        // Create Root node
        root_node: Node = Node:new();
        root_node.is_leaf(true);
        root_node.number_keys = 0;
        // Need to encapsulate the node in an option
        btree.root_node = root_node
        // TODO Do we need some offset here?
        btre
        

    pub fn btree_search(given_root, key) -> Option<TreeObject> {

    }

    pub fn btree_split_child(given_root, index) {

    }

    pub fn btree_insert(key: TreeObject){

    }

    pub fn btree_insert_non_full(given_root, key) {

    }

    pub fn in_order_traversal(root, writer, sequence_length){

    }

    /**
     * @return Returns the number of keys in the BTree.
     */
    pub fn get_size() -> i32 {
        0
    }

    /**
     * @return The degree of the BTree.
     */
    pub fn get_degree() -> i32 {
        0
    }

    /**
     * @return Returns the number of nodes in the BTree.
     */
    pub fn get_number_of_nodes() -> i32 {
        0
    }

    /**
     * @return The height of the BTree
     */
    pub fn get_height() -> i32 {
        0
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
    pub fn insert<T>(obj: T) {}

    /**
     * Print out all objects in the given BTree in an inorder traversal to a file.
     *
     * @param out PrintWriter object representing output
     */
    // void dumpToFile(PrintWriter out) throws IOException;
    pub fn dump_to_file(out: PrintWriter) {}

    /**
     * Searches for a sequence in the given BTree.
     *
     * @param key
     *            The key value to search for.
     */
    // TreeObject search(long key) throws IOException;
    pub fn search<T>(key: i32) -> T {
        T
    }
}

impl Default for BTree {
    fn default() -> BTree {
        BTree {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

}
