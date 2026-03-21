use clap::Parser;
use crate::cache::Cache;
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
    todo!()
}

pub fn print_results(results: &[&FileEntry]) {
    todo!()
}

pub fn run(cli: &Cli) {
    todo!()
}