use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use btree::BTree;
use btree::btree_node::TreeObject;
use gene;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// specifies whether the program should use cache (value 1) or no cache (value 0); if the value is 1, the <cache-size> has to be specified
    #[arg(short, long, default_value_t = 0)]
    cache: u32,
    /// the degree to be used for the B-Tree. If the user specifies 0, then our program should choose the optimum degree based on a disk block size of 4096 bytes and the size of our B-Tree node on disk
    #[arg(short, long, default_value_t = 3)]
    degree: u32,
    /// input *.gbk file containing the input DNA sequences
    #[arg(short, long)]
    gbkfile: String,
    /// integer that must be between 1 and 31 (inclusive)
    #[arg(short, long, default_value_t = 10)]
    length: u32,
    /// integer between 100 and 10000 (inclusive) that represents the maximum number of BTreeNode objects that can be stored in memory
    #[arg(short = 's', long)]
    cachesize: Option<u32>,
    /// Enable debugging messages, optional argument with a default value of zero
    #[arg(short = 'v', long)]
    debug: Option<u8>,
}

fn main() {
    // cli args
    let cli = Cli::parse();
    let cache = cli.cache;
    let degree = cli.degree;
    let gbk_file = cli.gbkfile;
    let sequence_length = cli.length;
    let cache_size = cli.cachesize.unwrap_or(100);
    if sequence_length < 1 || sequence_length > 31 {
        panic!("Sequence Length has to be between 1 - 31.")
    }
    // If debug true, the dump file will want to be created.
    let debug = cli.debug.unwrap_or(0) == 1;
    if debug {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }
    // Check if gbk file exists, if it doesn't panic/exit
    if ! Path::new(&gbk_file).exists() {
        println!("{gbk_file} not found.");
        std::process::exit(1);
    }
    // Scan gbk file
    let dna_sequences = parse_gbk(&gbk_file).expect("No Sequences found");
    // Get sequences into lengths
    let chunk_sequences: Vec<String> = dna_sequences.iter().map(| x | {
        x.chars().collect::<Vec<char>>()
            .windows(sequence_length as usize)
            .map(| c | c.iter().collect::<String>())
            .collect::<Vec<String>>()
    }).collect::<Vec<Vec<String>>>().concat();
    // Create's too much noise in log if printing vec.
    log::debug!("All Moving Window slices found {:?}", chunk_sequences.len());
    let use_cache = cache == 0;
    let output_file = format!("{gbk_file}.btree.data.{sequence_length}.{degree}");
    //Create BTree Object
    let mut btree = BTree::new(degree, &output_file, use_cache, cache_size, true);
    for i in chunk_sequences.iter() {
        // Change sequence of gene's to binary.
        let bin_sequence = gene::sequence_to_bin(i);
        let obj: TreeObject = TreeObject { sequence: bin_sequence, frequency: 1};
        btree.btree_insert(obj);
    }
    // If debug True, create dump file with gene sequences and frequencies
    if debug {
        let key_array = btree.get_sorted_array();
        let mut file = File::create("dump").unwrap();
        for key in key_array.iter() {
            let line = format!("{} {}\n", gene::sequence_from_bin(key.sequence, sequence_length as u8), key.frequency);
            file.write(line.as_bytes()).unwrap();
        }
    }
}

/// Parse the GBK file for DNA sequences
fn parse_gbk(gbk_file: &str) -> Option<Vec<String>> {
    let hay = fs::read_to_string(gbk_file).expect("Couldn't read file ({gbk_file})");
    let re = Regex::new(r"ORIGIN[^\/\/]*\/\/").unwrap();
    let sequence_break_re = Regex::new(r"n+").unwrap();
    // let sequences = contents.match_indices(&re).collect::<Vec<_>>();
    let sequences: Vec<String> = re.find_iter(&hay.as_str()).map(|m| {
        // Remove whitespace
        let mut m = m.as_str().replace(&['\n', ' ', '/'][..], "");
        // Remove line numbers
        m = m.chars().filter(| c | !c.is_digit(10)).collect();
        // Condence sequence breaks down to one n
        m = sequence_break_re.replace_all(&m, "x").as_ref().to_string();
        // Remove starting ORIGIN
        m = m.replace("ORIGIN", "");
        // Split sequence breaks 
        let res: Vec<String> = if m.contains('x') {
            m.split('x').map(| y | y.to_string() ).collect()
        } else {
            vec![m.to_string()]
        };
        res
    }).flatten().collect();
    log::debug!("Sequences found {:?}", sequences.len());
    if sequences.len() == 0 { None } else { Some(sequences)}
}
