use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;
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
    degree: u32,

}

impl Pager {
    pub fn new(file_name: &str, use_cache: bool, cache_size: u32, degree: u32) -> Result<Pager, std::io::Error> {
        let cache = if use_cache {
            Some(BTreeCache::new(cache_size) )
        } else {
            None
        };
        let file_cursor = 0;
        let pager = Pager { file_name: file_name.to_string(), file_cursor, cache, degree };
        Ok(pager)
    }

    // TODO Need to return an offset, set the offset counter correctly.
    pub fn write_metadata(&mut self, mut root_offset: u32, degree: u32) {
        if root_offset == 0 {
            root_offset = 8;
            self.file_cursor += 8;
        }
        let path = Path::new(&self.file_name);
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(&root_offset.to_be_bytes()).unwrap();
        file.write_all(&degree.to_be_bytes()).unwrap();
        
        file.flush().unwrap();
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
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path).unwrap();
        // OpenOptions::new().append(true)
        // let mut write_buffer = BufWriter::new(file);
        let move_cursor = node.offset >= self.file_cursor;
        // Write node to disk
        // Offset
        file.seek(SeekFrom::Start(node.offset as u64)).unwrap();
        file.write_all(&node.offset.to_be_bytes()).unwrap();
        // is Leaf Node
        // Don't see a way to convert bool to u8, so this will do
        if node.is_leaf() {
            file.write_all(&[1;1]).unwrap();
        } else {
            file.write_all(&[0;1]).unwrap();
        }
        // Number of Keys
        file.write_all(&node.number_of_keys().to_be_bytes()).unwrap();
        file.write_all(&node.number_of_children().to_be_bytes()).unwrap();
        // Keys
        // for i in &node.keys{
        //     file.write_all(&i.sequence.to_be_bytes()).unwrap();
        //     file.write_all(&i.frequency.to_be_bytes()).unwrap();
        //     if move_cursor{
        //         self.file_cursor += 8;
        //     }
        // }
        for i in 0..(2*self.degree-1) {
            if i < node.keys.len() as u32 {
                file.write_all(&node.keys.get(i as usize).unwrap().sequence.to_be_bytes()).unwrap();
                file.write_all(&node.keys.get(i as usize).unwrap().frequency.to_be_bytes()).unwrap();
            }
            else {
                file.write_all(&[0;8]).unwrap();
            }
            if move_cursor{
                self.file_cursor += 8;
            }
        }
        // Children Offsets
        // for i in &node.children_ptrs {
        //     file.write_all(&i.to_be_bytes()).unwrap();
        //     if move_cursor {
        //         self.file_cursor += 4;
        //     }
        // }
        for i in 0..(2*self.degree) {
            if i < node.children_ptrs.len() as u32 {
                let offset = &node.children_ptrs.get(i as usize).unwrap().to_be_bytes();
                file.write_all(offset).unwrap();
            }
            else {
                file.write_all(&[0;4]).unwrap();
            }
            if move_cursor{
                self.file_cursor += 4;
            }
        }

        if move_cursor {
            self.file_cursor += 13;
        }
        file.flush().unwrap();
    }

    pub fn read(&mut self, offset: u32) -> Node {
        let path = Path::new(&self.file_name);
        let mut file = File::open(path).unwrap();
        let mut buf = [0u8; 4];
        file.seek(SeekFrom::Start(offset as u64)).unwrap();
        // Offset
        file.read_exact(&mut buf).unwrap();
        let found_offset = u32::from_be_bytes(buf);
        if found_offset != offset {
            panic!("Found offset doesn't match given offset. Offset misaligned.")
        }
        // is Leaf Node
        file.read_exact(&mut buf[..1]).unwrap();
        let is_leaf: bool = u8::from_be_bytes(buf[..1].try_into().unwrap()) == 1;
        // Number of Keys
        file.read_exact(&mut buf).unwrap();
        let number_of_keys = u32::from_be_bytes(buf);
        // Number of Children Offsets
        file.read_exact(&mut buf).unwrap();
        let number_children_offsets = u32::from_be_bytes(buf);
        // Keys
        let mut keys: Vec<TreeObject> = Vec::new();
        for _ in 0..number_of_keys {
            file.read_exact(&mut buf).unwrap();
            let sequence = u32::from_be_bytes(buf);
            file.read_exact(&mut buf).unwrap();
            let frequency = u32::from_be_bytes(buf);
            keys.push(TreeObject {sequence, frequency});

        }
        let _new_offset = file.seek(SeekFrom::Current(((2*self.degree-1) as i64 - number_of_keys as i64) * 8)).unwrap();
        // Children Offsets
        let mut children_offsets: Vec<u32> = Vec::new();
        for _ in 0..number_children_offsets {
            file.read_exact(&mut buf).unwrap();
            children_offsets.push(u32::from_be_bytes(buf));
        }
        Node {keys,
            number_of_keys,  // TODO Why does a node care about max keys, couln't this be only known by the btree?
            is_leaf,
            children_ptrs: children_offsets,
            offset: found_offset
        }
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
    use rand::Rng;
    use super::*;

    const TEST_FILE_NAME: &str = "Test_BTree.tmp";
    
    fn delete_file(file: &str){
        std::fs::remove_file(file).ok();
    }

    // fn gen_random_node() -> Node {
    //     Node {}
    // }

    #[test]
    fn test_pager_metadata() {
        let file_name = "test_pager_metadata.tmp";
        delete_file(file_name);
        let mut pager = Pager::new(file_name, false, 0, 1).unwrap();
        let expected_root_offset = 10;
        let expected_degree = 10;
        pager.write_metadata(expected_root_offset, expected_degree);
        let (actual_root_offset, actual_degree) = pager.read_metadata();
        assert_eq!(expected_root_offset, actual_root_offset);
        assert_eq!(expected_degree, actual_degree);
        delete_file(file_name);
    }

        #[test]
        fn test_pager_write_read_1_node() {
            let file_name = "test_pager_write_read_1_node.tmp";
            delete_file(file_name);
            let mut pager = Pager::new(file_name, false, 0, 1).unwrap();
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
            let mut pager = Pager::new(file_name, false, 0, 1).unwrap();
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
            let mut pager = Pager::new(file_name, false, 0, 1).unwrap();
            pager.file_cursor = 8;
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
            let (actual_offset, actual_degree ) = pager.read_metadata();
            assert_eq!(8, actual_offset);
            assert_eq!(1, actual_degree);
            delete_file(file_name);
        }

}