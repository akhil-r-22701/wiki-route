mod cli;
mod loader;

use clap::Parser;
use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let (_graph, _reverse_graph) = match (cli.source.sql_dir, cli.source.graphs_dir) {
        (Some(sql_dir), None) => loader::load_from_sql(&sql_dir, cli.save_dir.as_deref())?,
        (None, Some(graphs_dir)) => loader::load_from_bin(&graphs_dir)?,
        _ => unreachable!(),
    };

    println!("[+] Ready. Listening on {}", cli.socket.display());

    Ok(())
}
