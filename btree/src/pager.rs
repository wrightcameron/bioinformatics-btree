use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufWriter, Read, Write};
use crate::btree_node::{Node};

pub struct Pager {
    file: File,

}

impl Pager {
    pub fn new(file_name: &str, use_cache: bool, cache_size: u32) -> Result<Pager, std::io::Error> {
        let path = Path::new(&file_name);
        let file = File::create(path)?;
        let pager = Pager { file };
        Ok(pager)
    }

    pub fn write_metadata(&self, root_offset: u32, degree: u32){
        let mut write_buffer = BufWriter::new(&self.file);
        write_buffer.write(&root_offset.to_be_bytes());
        write_buffer.write(&degree.to_be_bytes());
    }

    pub fn write_root(&self, root_node: Node) {

    }

    pub fn write_node(&self, node: Node) {

    }

    fn get_root_offset(&self) -> i32 {
        let mut read_buffer = BufReader::new(&self.file);
        let mut buf = [0;4];
        read_buffer.read(&mut buf);
        i32::from_ne_bytes(buf)
    }

}