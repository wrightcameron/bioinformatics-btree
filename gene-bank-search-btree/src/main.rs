use clap::Parser;
use std::path::Path;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// specifies whether the program should use cache (value 1) or no cache (value 0); if the value is 1, the <cache-size> has to be specified
    #[arg(short, long, default_value_t = 0)]
    cache: i32,
    /// the degree to be used for the B-Tree. If the user specifies 0, then our program should choose the optimum degree based on a disk block size of 4096 bytes and the size of our B-Tree node on disk
    #[arg(short, long, default_value_t = 0)]
    degree: i32,
    
    /// input btreefile file containing the input DNA sequences in Btree
    #[arg(short, long)]
    btreefile: String,
    /// integer that must be between 1 and 31 (inclusive)
    #[arg(short, long, default_value_t = 10)]
    length: i32,
    /// contains all the DNA strings of a specific subsequence length that we want to search for in the specified B-Tree file. The strings are one per line and they all must have the same length as the DNA subsequences in the B-Tree file. The DNA strings use A, C, T, and G (either lower or upper case)
    #[arg(short, long)]
    queryfile: String,
    /// integer between 100 and 10000 (inclusive) that represents the maximum number of BTreeNode objects that can be stored in memory
    #[arg(short = 's', long)]
    cachesize: Option<i32>,
    /// Enable debugging messages, optional argument with a default value of zero
    #[arg(short = 'v', long)]
    debug: Option<i8>,
}

fn main() {
    let cli = Cli::parse();
    //TODO Handle flags
    let btreefile = cli.btreefile;
    let queryfile = cli.queryfile;
    let debug = cli.debug.unwrap_or(0);

    if cli.debug.unwrap_or(0) == 1 {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }
    // Check if btree file exists, if it doesn't panic/exit
    if ! Path::new(&btreefile).exists() {
        println!("{btreefile} not found.");
        std::process::exit(1);
    }
    // Check if queryfile file exists, if it doesn't panic/exit
    if ! Path::new(&queryfile).exists() {
        println!("{queryfile} not found.");
        std::process::exit(1);
    }
    todo!();
    println!("Hello, world!");
}
