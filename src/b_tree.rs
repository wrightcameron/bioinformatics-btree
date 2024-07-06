pub struct BTree {}

impl BTree {
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
    pub fn insert(obj: TreeObj) {}

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
    pub fn search(key: i32) -> TreeObj {}
}
