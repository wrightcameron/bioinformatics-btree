use std::fs::{File, remove_file};
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::io::SeekFrom;
use crate::btree_node::Node;
use crate::TreeObject;

const DISK_BLOCK_SIZE: u32 = 4096;
pub const STARTING_OFFSET: u32 = 8;

pub struct Pager {
    pub file_cursor: u32,
    file: File,
    degree: u32,
}

impl Pager {
    //TODO Remove truncate as we are hanlding in btree constructor now
    pub fn new(file_name: &str, degree: u32, truncate_file: bool) -> Result<Pager, std::io::Error> {
        let path = Path::new(file_name);
        // Delete file if truncate_file is set to true, cause rerunning tests with non deleted files results in incorrect outputs
        let file_cursor = 8;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path).unwrap();
        Ok(Pager { file_cursor, file, degree })
    }

    // TODO Need to return an offset, set the offset counter correctly.
    pub fn write_metadata(&mut self, mut root_offset: u32, degree: u32) {
        if root_offset == 0 {
            root_offset = 8;
            self.file_cursor += 8;
        }
        self.file.seek(SeekFrom::Start(0)).unwrap();
        self.file.write_all(&root_offset.to_be_bytes()).unwrap();
        self.file.write_all(&degree.to_be_bytes()).unwrap();
        self.file.flush().unwrap();
    }

    pub fn read_metadata(&mut self) -> Result<(u32, u32), std::io::Error >  {
        let mut buf = [0u8; 4];
        self.file.seek(SeekFrom::Start(0))?;
        // Root Offset
        self.file.read_exact(&mut buf)?;
        let root_offset = u32::from_be_bytes(buf);
        // degree
        self.file.read_exact(&mut buf)?;
        let degree = u32::from_be_bytes(buf);
        Ok((root_offset, degree))
    }

    pub fn write(&mut self, node: &Node) {
        let move_cursor = node.offset >= self.file_cursor;
        // Write node to disk
        // Offset
        self.file.seek(SeekFrom::Start(node.offset as u64)).unwrap();
        self.file.write_all(&node.offset.to_be_bytes()).unwrap();
        // is Leaf Node
        // Don't see a way to convert bool to u8, so this will do
        if node.is_leaf() {
            self.file.write_all(&[1;1]).unwrap();
        } else {
            self.file.write_all(&[0;1]).unwrap();
        }
        // Number of Keys
        self.file.write_all(&node.number_of_keys().to_be_bytes()).unwrap();
        self.file.write_all(&node.number_of_children().to_be_bytes()).unwrap();
        // Keys
        for i in 0..(2*self.degree-1) {
            if i < node.keys.len() as u32 {
                self.file.write_all(&node.keys.get(i as usize).unwrap().sequence.to_be_bytes()).unwrap();
                self.file.write_all(&node.keys.get(i as usize).unwrap().frequency.to_be_bytes()).unwrap();
            }
            else {
                self.file.write_all(&[0;16]).unwrap();
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
                self.file.write_all(offset).unwrap();
            }
            else {
                self.file.write_all(&[0;4]).unwrap();
            }
            if move_cursor{
                self.file_cursor += 4;
            }
        }

        if move_cursor {
            self.file_cursor += 13;
        }
        self.file.flush().unwrap();
    }

    pub fn read(&mut self, offset: u32) -> Node {
        let mut buf = [0u8; 4];
        self.file.seek(SeekFrom::Start(offset as u64)).unwrap();
        // Offset
        self.file.read_exact(&mut buf).unwrap();
        let found_offset = u32::from_be_bytes(buf);
        if found_offset != offset {
            panic!("Found offset ({found_offset}) doesn't match given offset ({offset}). Offset misaligned.")
        }
        // is Leaf Node
        self.file.read_exact(&mut buf[..1]).unwrap();
        let is_leaf: bool = u8::from_be_bytes(buf[..1].try_into().unwrap()) == 1;
        // Number of Keys
        self.file.read_exact(&mut buf).unwrap();
        let number_of_keys = u32::from_be_bytes(buf);
        // Number of Children Offsets
        self.file.read_exact(&mut buf).unwrap();
        let number_children_offsets = u32::from_be_bytes(buf);
        // Keys
        let mut key_buf = [0u8; 8];
        let mut keys: Vec<TreeObject> = Vec::new();
        for _ in 0..number_of_keys {
            self.file.read_exact(&mut key_buf).unwrap();
            let sequence = u64::from_be_bytes(key_buf);
            self.file.read_exact(&mut key_buf).unwrap();
            let frequency = u64::from_be_bytes(key_buf);
            keys.push(TreeObject {sequence, frequency});

        }
        let _new_offset = self.file.seek(SeekFrom::Current(((2*self.degree-1) as i64 - number_of_keys as i64) * 16)).unwrap();
        // Children Offsets
        let mut children_offsets: Vec<u32> = Vec::new();
        for _ in 0..number_children_offsets {
            self.file.read_exact(&mut buf).unwrap();
            children_offsets.push(u32::from_be_bytes(buf));
        }
        Node {keys,
            number_of_keys,  // TODO Why does a node care about max keys, couln't this be only known by the btree?
            is_leaf,
            children_ptrs: children_offsets,
            offset: found_offset
        }
    }

    pub fn get_root_offset(&mut self) -> Result<u32, std::io::Error> {
        let meta = match self.read_metadata() {
            Ok(meta)  => meta,
            Err(e) => return Err(e),
        };
        Ok(meta.0)
    }

    pub fn read_root(&mut self) -> Result<Node, std::io::Error> {
        let offset = self.get_root_offset()?;
        Ok(self.read(offset))
    }

    pub fn recreate_file(&mut self, file_name: &str, degree: u32, node: &Node) -> u32 {
        let path = Path::new(file_name);
        if Path::new(path).exists() {
            remove_file(path).expect("Unable to remove file.");
        }
        // Recreate file handler, otherwise writing into void
        self.file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path).unwrap();
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


    fn delete_file(file: &str){
        std::fs::remove_file(file).ok();
    }


    #[test]
    fn test_pager_metadata() {
        let file_name = "test_pager_metadata.tmp";
        delete_file(file_name);
        let mut pager = Pager::new(file_name, 1, true).unwrap();
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
            let mut pager = Pager::new(file_name, 1, true).unwrap();
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
            let mut pager = Pager::new(file_name, 1, true).unwrap();
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
            let mut pager = Pager::new(file_name, 1, true).unwrap();
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
            let (actual_offset, actual_degree ) = pager.read_metadata().unwrap();
            assert_eq!(8, actual_offset);
            assert_eq!(1, actual_degree);
            delete_file(file_name);
        }

}