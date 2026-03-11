mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match (cli.source.sql_dir, cli.source.graphs_dir) {
        (Some(dir), None) => println!("Building graphs from SQL in: {}", dir.display()),
        (None, Some(dir)) => println!("Loading precomputed graphs from: {}", dir.display()),
        _ => unreachable!(),
    }

    println!("Socket: {}", cli.socket.display());
}
