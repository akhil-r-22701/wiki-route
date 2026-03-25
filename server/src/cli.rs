use std::path::PathBuf;

use clap::{ArgAction, Args, Parser};

#[derive(Parser)]
#[command(name = "wiki-route-server")]
#[command(about = "Wikipedia path-finding server")]
pub struct Cli {
    #[command(flatten)]
    pub source: GraphSource,

    /// Save computed data to this directory (only valid with --sql-dir)
    #[arg(long, conflicts_with = "data_dir")]
    pub save_dir: Option<PathBuf>,

    /// Unix socket path to listen on
    #[arg(long, default_value = "/tmp/wiki-route.sock")]
    pub socket: PathBuf,

    /// Increase verbosity (-v info, -vv debug, -vvv trace)
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct GraphSource {
    /// Directory containing page.sql, pagelinks.sql, linktarget.sql
    #[arg(long)]
    pub sql_dir: Option<PathBuf>,

    /// Directory containing precomputed .bin files (graph.bin, reverse_graph.bin, titles.bin)
    #[arg(long)]
    pub data_dir: Option<PathBuf>,
}
