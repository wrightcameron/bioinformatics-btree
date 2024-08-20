use clap::Parser;
use std::path::Path;
use sqlite;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// input btreefile file containing the input DNA sequences in Btree
    #[arg(short, long)]
    database: String,
    /// contains all the DNA strings of a specific subsequence length that we want to search for in the specified B-Tree file. The strings are one per line and they all must have the same length as the DNA subsequences in the B-Tree file. The DNA strings use A, C, T, and G (either lower or upper case)
    #[arg(short, long)]
    queryfile: String,
    /// Enable debugging messages, optional argument with a default value of zero
    #[arg(short = 'v', long)]
    debug: Option<i8>,
}

fn main() {
    // cli args
    let cli = Cli::parse();
    let database = cli.database;
    let queryfile = cli.queryfile;
    if cli.debug.unwrap_or(0) == 1 {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }
    // Check if btree file exists, if it doesn't panic/exit
    if !Path::new(&database).exists() {
        println!("{database} not found.");
        std::process::exit(1);
    }
    log::debug!("GeneBank Database file found: {database}");
    // Check if queryfile file exists, if it doesn't panic/exit
    if !Path::new(&queryfile).exists() {
        println!("{queryfile} not found.");
        std::process::exit(1);
    }
    log::debug!("Query file found: {queryfile}");
    // Open Connection to Sqlite database
    let connection = sqlite::open(database).unwrap();
    let query = "
        SELECT * FROM gene_sequence LIMIT 10;
    ";
    let mut statement = connection.prepare(query).unwrap();
    while let Ok(sqlite::State::Row) = statement.next() {
        println!("{} {}", statement.read::<String, _>("sequence").unwrap(), statement.read::<i64, _>("frequency").unwrap());
    }
}
