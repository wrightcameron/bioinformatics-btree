// mod b_tree;

use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /*
    /// specifies whether the program should use cache (value 1) or no cache (value 0); if the value is 1, the <cache-size> has to be specified
    cache: i32,
    /// the degree to be used for the B-Tree. If the user specifies 0, then our program should choose the optimum degree based on a disk block size of 4096 bytes and the size of our B-Tree node on disk
    degree: i32,
    */
    /// input *.gbk file containing the input DNA sequences
    #[arg(short, long)]
    gbkfile: String,

    /// integer that must be between 1 and 31 (inclusive)
    #[arg(short, long)]
    length: i32,
    /*
    /// integer between 100 and 10000 (inclusive) that represents the maximum number of BTreeNode objects that can be stored in memory
    cachesize: Option<i32>,
    */

    /// Enable debugging messages, optional argument with a default value of zero
    #[arg(short, long)]
    debug: Option<i8>,
}

fn main() {
    // cli args
    let cli = Cli::parse();
    // let cache = cli.cache;
    // let degree = cli.degree;
    let gbk_file = cli.gbkfile;
    let sequence_length = cli.length;
    // let cache_size = cli.cachesize.unwrap_or(100);
    // let debug = cli.debug.unwrap_or(0);
    if cli.debug.unwrap_or(0) == 1 {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }
    // Check if gbk file exists, if it doesn't panic/exit
    if ! Path::new(&gbk_file).exists() {
        println!("{gbk_file} not found.");
        std::process::exit(1);
    }
    // TODO Combine with if file exists above cause function returns None if not found.
    // Scan gbk file
    let dna_sequences = parse_gbk(&gbk_file).expect("No Sequences found");
    let chunk_sequences: Vec<String> = dna_sequences.iter().map(| x | {
        x.chars().collect::<Vec<char>>()
            .chunks(sequence_length as usize)
            .map(| c | c.iter().collect::<String>())
            .collect::<Vec<String>>()
    }).collect::<Vec<Vec<String>>>().concat();
    log::debug!("{:?}", chunk_sequences);
    //TODO Create BTree Object
    //TODO Save btree object to disk
}

/// Parse the GBK file for DNA sequences
fn parse_gbk(gbk_file: &str) -> Option<Vec<String>> {
    let hay = fs::read_to_string(gbk_file).expect("Couldn't read file ({gbk_file})");
    let re = Regex::new(r"ORIGIN[^\/\/]*\/\/").unwrap();
    // let sequences = contents.match_indices(&re).collect::<Vec<_>>();
    let sequences: Vec<String> = re.find_iter(&hay.as_str()).map(|m| {
        let mut m = m.as_str().replace(&['\n', ' ', '/'][..], "");
        m = m.chars().filter(| c | !c.is_digit(10)).collect();
        m.replace("ORIGIN", "")
    }).collect();
    log::debug!("Sequences found {:?}", sequences);
    if sequences.len() == 0 { None } else { Some(sequences)}
}
