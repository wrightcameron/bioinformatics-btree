use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// input btreefile file containing the input DNA sequences in Btree
    #[arg(short, long)]
    btreefile: String,
    /// contains all the DNA strings of a specific subsequence length that we want to search for in the specified B-Tree file. The strings are one per line and they all must have the same length as the DNA subsequences in the B-Tree file. The DNA strings use A, C, T, and G (either lower or upper case)
    #[arg(short, long)]
    queryfile: String,
}

fn main() {
    // cli args
    let cli = Cli::parse();
    let btreefile = cli.btreefile;
    let queryfile = cli.queryfile;
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
    println!("Hello, world!");
}
