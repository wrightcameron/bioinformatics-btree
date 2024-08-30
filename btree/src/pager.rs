use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufWriter, Read, Seek, Write};
use std::io::SeekFrom;
use crate::btree_node::Node;
use crate::btree_cache::BTreeCache;
use crate::TreeObject;

const DISK_BLOCK_SIZE: u32 = 4096;

pub struct Pager {
    file_name: String,
    pub file_cursor: u32,
    cache: Option<BTreeCache<u32>>,

}

impl Pager {
    pub fn new(file_name: &str, use_cache: bool, cache_size: u32) -> Result<Pager, std::io::Error> {
        let cache = if use_cache {
            Some(BTreeCache::new(cache_size) )
        } else {
            None
        };
        let file_cursor = 0;
        let pager = Pager { file_name: file_name.to_string(), file_cursor, cache };
        Ok(pager)
    }

    // TODO Need to return an offset, set the offset counter correctly.
    pub fn write_metadata(&mut self, root_offset: u32, degree: u32) {
        let path = Path::new(&self.file_name);
        let file = File::create(path).unwrap();
        let mut write_buffer = BufWriter::new(file);
        write_buffer.write(&root_offset.to_be_bytes()).unwrap();
        write_buffer.write(&degree.to_be_bytes()).unwrap();
        self.file_cursor += 8;
        write_buffer.flush().unwrap();
    }

    pub fn read_metadata(&mut self) -> (u32, u32) {
        let path = Path::new(&self.file_name);
        let mut file = File::open(path).unwrap();
        let mut buf = [0u8; 4];
        file.seek(SeekFrom::Start(0)).unwrap();
        // Root Offset
        file.read_exact(&mut buf).unwrap();
        let root_offset = u32::from_be_bytes(buf);
        // degree
        file.read_exact(&mut buf).unwrap();
        let degree = u32::from_be_bytes(buf);
        (root_offset, degree)
    }

    pub fn write(&mut self, node: &Node) {
        let path = Path::new(&self.file_name);
        let file = File::create(path).unwrap();
        let remaining_block_space = DISK_BLOCK_SIZE;
        let mut write_buffer = BufWriter::new(file);
        // Write node to disk
        // Offset
        write_buffer.seek(SeekFrom::Start(node.offset as u64)).unwrap();
        write_buffer.write(&node.offset.to_be_bytes()).unwrap();
        // is Leaf Node
        // Don't see a way to convert bool to u8, so this will do
        if node.is_leaf {
            write_buffer.write(&[1;1]).unwrap();
        } else {
            write_buffer.write(&[0;1]).unwrap();
        }
        // Number of Keys
        write_buffer.write(&node.number_keys.to_be_bytes()).unwrap();
        // Keys
        for i in &node.keys{
            write_buffer.write(&i.sequence.to_be_bytes()).unwrap();
            write_buffer.write(&i.frequency.to_be_bytes()).unwrap();
            self.file_cursor += 8;
        }
        // Children Offsets
        for i in &node.children_ptrs {
            write_buffer.write(&i.to_be_bytes()).unwrap();
            self.file_cursor += 4;
        }
        self.file_cursor += 9;
        write_buffer.flush().unwrap();
    }

    pub fn read(&mut self, offset: u32) -> Node {
        let path = Path::new(&self.file_name);
        let mut file = File::open(path).unwrap();
        let mut buf = [0u8; 4];
        file.seek(SeekFrom::Start(offset as u64));
        // Offset
        file.read_exact(&mut buf).unwrap();
        let offset = u32::from_be_bytes(buf);
        // is Leaf Node
        file.read_exact(&mut buf[..1]).unwrap();
        let is_leaf: bool = u8::from_be_bytes(buf[..1].try_into().unwrap()) == 1;
        // Number of Keys
        file.read_exact(&mut buf).unwrap();
        let number_of_keys = u32::from_be_bytes(buf);
        // Keys
        let mut keys: Vec<TreeObject> = Vec::new();
        for _ in 0..number_of_keys {
            file.read_exact(&mut buf).unwrap();
            let sequence = u32::from_be_bytes(buf);
            file.read_exact(&mut buf).unwrap();
            let frequency = u32::from_be_bytes(buf);
            keys.push(TreeObject {sequence, frequency});

        }
        // Children Offsets
        let mut children_offsets: Vec<u32> = Vec::new();
        for i in 0..number_of_keys {
            file.read_exact(&mut buf).unwrap();
            children_offsets.push(u32::from_be_bytes(buf));
        }
        Node {number_keys: number_of_keys, 
            keys, 
            children_ptrs: children_offsets, 
            is_leaf, 
            offset
        }
    }

    pub fn write_root(&self, root_node: Node) {

    }

    pub fn write_node(&self, node: Node) {

    }

    pub fn rewrite_node(&self, node: Node) {

    }

    fn get_root_offset(&self) -> i32 {
        let path = Path::new(&self.file_name);
        let file = File::open(path).unwrap();
        let mut read_buffer = BufReader::new(file);
        let mut buf = [0;4];
        read_buffer.read(&mut buf);
        i32::from_ne_bytes(buf)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_NAME: &str = "Test_BTree.tmp";
    
    fn setup(file: &str){
        std::fs::remove_file(file).ok();
    }

    fn teardown(){
        std::fs::remove_file(TEST_FILE_NAME).ok();
    }

    #[test]
    fn test_pager_metadata() {
        let file_name = "test_pager_metadata.tmp";
        setup(file_name);
        let mut pager = Pager::new(file_name, false, 0).unwrap();
        let expected_root_offset = 10;
        let expected_degree = 10;
        pager.write_metadata(expected_root_offset, expected_degree);
        let (actual_root_offset, actual_degree) = pager.read_metadata();
        assert_eq!(expected_root_offset, actual_root_offset);
        assert_eq!(expected_degree, actual_degree);
        setup(file_name);
    }

        #[test]
        fn test_pager_write_read_1_node() {
            let file_name = "test_pager_write_read_1_node.tmp";
            setup(file_name);
            let mut pager = Pager::new(file_name, false, 0).unwrap();
            let mut expected_node = Node::new();
            pager.write(&expected_node);
            let actual_node = pager.read(expected_node.offset);
            assert_eq!(expected_node.number_keys, actual_node.number_keys);
            assert_eq!(expected_node.is_leaf, actual_node.is_leaf);
            assert_eq!(expected_node.offset, actual_node.offset);
            setup(file_name);
        }

}