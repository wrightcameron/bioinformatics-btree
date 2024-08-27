use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufWriter, Read, Seek, Write};
use crate::btree_node::Node;
use crate::btree_cache::BTreeCache;

const DISK_BLOCK_SIZE: u32 = 4096;

pub struct Pager {
    file: File,
    file_cursor: u32,
    cache: Option<BTreeCache<u32>>,

}

impl Pager {
    pub fn new(file_name: &str, use_cache: bool, cache_size: u32) -> Result<Pager, std::io::Error> {
        let path = Path::new(&file_name);
        let file = File::create(path)?;
        let cache = if use_cache {
            Some(BTreeCache::new(cache_size) )
        } else {
            None
        };
        let pager = Pager { file, cache };
        Ok(pager)
    }

    pub fn write_metadata(&self, root_offset: u32, degree: u32){
        let mut write_buffer = BufWriter::new(&self.file);
        write_buffer.write(&root_offset.to_be_bytes());
        write_buffer.write(&degree.to_be_bytes());
    }

    pub fn write(&self, node: Node) {
        let mut write_buffer = BufWriter::new(&self.file);
        write_buffer.seek(std::io::SeekFrom::Start((node.offset as u64)));
        let mut remaining_block_space = DISK_BLOCK_SIZE;
        
    }

    pub fn write_root(&self, root_node: Node) {

    }

    pub fn write_node(&self, node: Node) {

    }

    pub fn rewrite_node(&self, node: Node) {

    }

    fn get_root_offset(&self) -> i32 {
        let mut read_buffer = BufReader::new(&self.file);
        let mut buf = [0;4];
        read_buffer.read(&mut buf);
        i32::from_ne_bytes(buf)
    }

}