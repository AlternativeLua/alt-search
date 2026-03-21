use std::path::Path;
use clap::Parser;
use crate::cache::{Cache, FileEntry};
use crate::search::{Query, search};

#[derive(Parser)]
#[command(name = "altsearch")]
pub struct Cli {
    #[arg(short, long)]
    pub dir: String,

    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(short, long)]
    pub ext: Option<String>,

    #[arg(long)]
    pub min_size: Option<u64>,

    #[arg(long)]
    pub max_size: Option<u64>,

    #[arg(long)]
    pub dirs_only: bool,

    #[arg(long)]
    pub files_only: bool,
}

pub fn build_query(cli: &Cli) -> Query {
    Query {
        name_contains: cli.name.clone(),
        extension: cli.ext.clone(),
        min_size: cli.min_size,
        max_size: cli.max_size,
        is_dir: match (cli.dirs_only, cli.files_only) {
            (true, _) => Some(true),
            (_, true) => Some(false),
            _ => None,
        },
        ..Query::new()
    }
}

pub fn print_results(results: &[&FileEntry]) {
    if results.is_empty() {
        println!("No results found.");
        return;
    }
    
    println!("Found {} results.", results.len());
    
    for entry in results {
        println!("{}", entry.name);
    }
}

pub fn run(cli: &Cli) {
    let mut cache = Cache::new();
    cache.build(Path::new(&cli.dir), 30).unwrap();

    let query = build_query(cli);
    let results = search(&cache, & query);

    print_results(&results);
}