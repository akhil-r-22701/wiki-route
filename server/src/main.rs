mod bfs;
mod cli;
mod graph;
mod linktarget;
mod loader;
mod page;
mod pagelinks;
mod parser_utils;
mod server;
mod types;

use std::sync::Arc;

use clap::Parser;
use cli::Cli;
use server::ServerState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    let (graph, reverse_graph, title_maps) = match (cli.source.sql_dir, cli.source.data_dir) {
        (Some(sql_dir), None) => loader::load_from_sql(&sql_dir, cli.save_dir.as_deref())?,
        (None, Some(data_dir)) => loader::load_from_bin(&data_dir)?,
        _ => unreachable!(),
    };

    let state = ServerState {
        graph: Arc::new(graph),
        reverse_graph: Arc::new(reverse_graph),
        title_maps: Arc::new(title_maps),
    };

    server::run(&cli.socket, state)?;

    Ok(())
}
