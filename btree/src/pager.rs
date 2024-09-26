use std::fs::{File, remove_file};
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::io::SeekFrom;
use std::io::{BufWriter, BufReader};
use crate::btree_node::Node;
use crate::TreeObject;

/// Represents numeric amount of bytes in sequence.
type Bytes = u32;

const _DISK_BLOCK_SIZE: Bytes = 4096;
pub const STARTING_OFFSET: Bytes = 8;

/// Pager Struct representing reading and writing Btree's gene sequence to file.
/// Pager is specifically designed for Node structs.
pub struct Pager {
    pub file_cursor: Bytes,
    buf_write: BufWriter<File>,
    buf_read: BufReader<File>,
    degree: u32,
}

impl Pager {
    /// Pager Constructor
    pub fn new(file_name: &str, degree: u32) -> Result<Pager, std::io::Error> {
        let path = Path::new(file_name);
        // Delete file if truncate_file is set to true, cause rerunning tests with non deleted files results in incorrect outputs
        let file_cursor = STARTING_OFFSET;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path).unwrap();
        let buf_write = BufWriter::new(file.try_clone().unwrap());
        let buf_read = BufReader::new(file.try_clone().unwrap());
        Ok(Pager { file_cursor, buf_write, buf_read, degree })
    }

    // TODO Need to return an offset, set the offset counter correctly.
    /// Write btree metadata to start of file, first 8 bytes.
    /// Meta data is u32 root offset, followed by u32 degree
    pub fn write_metadata(&mut self, mut root_offset: u32, degree: u32) {
        if root_offset == 0 {
            root_offset = STARTING_OFFSET;
            self.file_cursor += STARTING_OFFSET;
        }
        self.buf_write.seek(SeekFrom::Start(0)).unwrap();
        self.buf_write.write_all(&root_offset.to_be_bytes()).unwrap();
        self.buf_write.write_all(&degree.to_be_bytes()).unwrap();
        self.buf_write.flush().unwrap();
    }

    /// Read metadata at start of file, throwing error if byte sequence not found.
    /// Return u32 offset followed by u32 degree in tuple
    pub fn read_metadata(&mut self) -> Result<(u32, u32), std::io::Error >  {
        let mut buf = [0u8; 4];
        self.buf_read.seek(SeekFrom::Start(0))?;
        // Root Offset
        self.buf_read.read_exact(&mut buf)?;
        let root_offset = u32::from_be_bytes(buf);
        // degree
        self.buf_read.read_exact(&mut buf)?;
        let degree = u32::from_be_bytes(buf);
        Ok((root_offset, degree))
    }

    /// Write BTree Node to file, goes through all parts of Node's values and writes their
    /// byte sequence to disk.  If Node doesn't have keys or child ptrs, write their max 
    /// possible ammount to give buffer between this node and next in file.
    pub fn write(&mut self, node: &Node) {
        // Don't move file cursor for updating existing nodes
        let move_cursor = node.offset >= self.file_cursor;
        // Write node to disk
        // Offset
        self.buf_write.seek(SeekFrom::Start(node.offset as u64)).unwrap();
        self.buf_write.write_all(&node.offset.to_be_bytes()).unwrap();
        // is Leaf Node
        // Don't see a way to convert bool to u8, so this will do
        if node.is_leaf() {
            self.buf_write.write_all(&[1;1]).unwrap();
        } else {
            self.buf_write.write_all(&[0;1]).unwrap();
        }
        // Number of Keys
        self.buf_write.write_all(&node.number_of_keys().to_be_bytes()).unwrap();
        self.buf_write.write_all(&node.number_of_children().to_be_bytes()).unwrap();
        // Keys
        for i in 0..(2*self.degree-1) {
            if i < node.keys.len() as u32 {
                self.buf_write.write_all(&node.keys.get(i as usize).unwrap().sequence.to_be_bytes()).unwrap();
                self.buf_write.write_all(&node.keys.get(i as usize).unwrap().frequency.to_be_bytes()).unwrap();
            }
            else {
                self.buf_write.write_all(&[0;16]).unwrap();
            }
            if move_cursor{
                // 2 u64s is 16 bytes (8 bits)
                self.file_cursor += 16;
            }
        }
        // Children Offsets
        for i in 0..(2*self.degree) {
            if i < node.children_ptrs.len() as u32 {
                let offset = &node.children_ptrs.get(i as usize).unwrap().to_be_bytes();
                self.buf_write.write_all(offset).unwrap();
            }
            else {
                self.buf_write.write_all(&[0;4]).unwrap();
            }
            if move_cursor{
                self.file_cursor += 4;
            }
        }

        if move_cursor {
            self.file_cursor += 13;
        }
        self.buf_write.flush().unwrap();
    }

    /// Read Node Struct from file, with given byte offset.
    pub fn read(&mut self, offset: u32) -> Node {
        let mut buf = [0u8; 4];
        self.buf_read.seek(SeekFrom::Start(offset as u64)).unwrap();
        // Offset
        self.buf_read.read_exact(&mut buf).unwrap();
        let found_offset = u32::from_be_bytes(buf);
        if found_offset != offset {
            panic!("Found offset ({found_offset}) doesn't match given offset ({offset}). Offset misaligned.")
        }
        // is Leaf Node
        self.buf_read.read_exact(&mut buf[..1]).unwrap();
        let is_leaf: bool = u8::from_be_bytes(buf[..1].try_into().unwrap()) == 1;
        // Number of Keys
        self.buf_read.read_exact(&mut buf).unwrap();
        let number_of_keys = u32::from_be_bytes(buf);
        // Number of Children Offsets
        self.buf_read.read_exact(&mut buf).unwrap();
        let number_children_offsets = u32::from_be_bytes(buf);
        // Keys
        let mut key_buf = [0u8; 8];
        let mut keys: Vec<TreeObject> = Vec::new();
        for _ in 0..number_of_keys {
            self.buf_read.read_exact(&mut key_buf).unwrap();
            let sequence = u64::from_be_bytes(key_buf);
            self.buf_read.read_exact(&mut key_buf).unwrap();
            let frequency = u64::from_be_bytes(key_buf);
            keys.push(TreeObject {sequence, frequency});

        }
        let _new_offset = self.buf_read.seek(SeekFrom::Current(((2*self.degree-1) as i64 - number_of_keys as i64) * 16)).unwrap();
        // Children Offsets
        let mut children_offsets: Vec<u32> = Vec::new();
        for _ in 0..number_children_offsets {
            self.buf_read.read_exact(&mut buf).unwrap();
            children_offsets.push(u32::from_be_bytes(buf));
        }
        Node {keys,
            number_of_keys,  // TODO Why does a node care about max keys, couln't this be only known by the btree?
            is_leaf,
            children_ptrs: children_offsets,
            offset: found_offset
        }
    }

    /// Get root offset from metadata
    pub fn get_root_offset(&mut self) -> Result<u32, std::io::Error> {
        let meta = match self.read_metadata() {
            Ok(meta)  => meta,
            Err(e) => return Err(e),
        };
        Ok(meta.0)
    }

    /// Return the Node Struct, by finding where it is from the metadata
    pub fn read_root(&mut self) -> Result<Node, std::io::Error> {
        let offset = self.get_root_offset()?;
        Ok(self.read(offset))
    }

    /// Drop an existing btree file, and recreate the file along with its metadata and first node
    pub fn recreate_file(&mut self, file_name: &str, degree: u32, node: &Node) -> u32 {
        let path = Path::new(file_name);
        if Path::new(path).exists() {
            remove_file(path).expect("Unable to remove file.");
        }
        // Recreate file handler, otherwise writing into void
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path).unwrap();
        self.buf_write = BufWriter::new(file.try_clone().unwrap());
        self.buf_read = BufReader::new(file.try_clone().unwrap());

        self.write_metadata(STARTING_OFFSET, degree);
        //TODO Node here isn't set correctly to offset of 8, should fix - temp fix is setting it in node struct temp
        assert!(self.file_cursor == STARTING_OFFSET);
        self.write(node);
        STARTING_OFFSET
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function, delete files for test cleanup
    fn delete_file(file: &str){
        std::fs::remove_file(file).ok();
    }


    #[test]
    fn test_pager_metadata() {
        let file_name = "test_pager_metadata.tmp";
        delete_file(file_name);
        let mut pager = Pager::new(file_name, 1).unwrap();
        let expected_root_offset = 10;
        let expected_degree = 10;
        pager.write_metadata(expected_root_offset, expected_degree);
        let (actual_root_offset, actual_degree) = pager.read_metadata().unwrap();
        assert_eq!(expected_root_offset, actual_root_offset);
        assert_eq!(expected_degree, actual_degree);
        delete_file(file_name);
    }

        #[test]
        fn test_pager_write_read_1_node() {
            let file_name = "test_pager_write_read_1_node.tmp";
            delete_file(file_name);
            let mut pager = Pager::new(file_name, 1).unwrap();
            let expected_node = Node::new();
            pager.write(&expected_node);
            let actual_node = pager.read(expected_node.offset);
            assert_eq!(expected_node, actual_node);
            delete_file(file_name);
        }

        #[test]
        fn test_pager_write_read_2_node() {
            let file_name = "test_pager_write_read_2_node.tmp";
            delete_file(file_name);
            let mut pager = Pager::new(file_name, 1).unwrap();
            let mut nodes: Vec<Node> = Vec::new();
            for _ in 0..2 {
                let mut node = Node::new();
                node.offset = pager.file_cursor;
                pager.write(&node);
                nodes.push(node)
            }
            for expected_node in nodes{
                let actual_node = pager.read(expected_node.offset);
                assert_eq!(expected_node, actual_node);
            }
            delete_file(file_name);
        }

        #[test]
        fn test_pager_metadata_10_nodes(){
            let file_name = "test_pager_metadata_10_nodes.tmp";
            delete_file(file_name);
            let mut pager = Pager::new(file_name, 1).unwrap();
            pager.file_cursor = STARTING_OFFSET;
            pager.write_metadata(8, 1);
            let mut nodes: Vec<Node> = Vec::new();
            for _ in 0..10 {
                let mut node = Node::new();
                node.offset = pager.file_cursor;
                pager.write(&node);
                nodes.push(node)
            }
            for expected_node in nodes{
                let actual_node = pager.read(expected_node.offset);
                assert_eq!(expected_node, actual_node);
            }
            let (actual_offset, actual_degree ) = pager.read_metadata().unwrap();
            assert_eq!(8, actual_offset);
            assert_eq!(1, actual_degree);
            delete_file(file_name);
        }

}