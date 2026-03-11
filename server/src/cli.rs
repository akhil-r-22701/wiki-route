use std::path::PathBuf;

use clap::{Args, Parser};

#[derive(Parser)]
#[command(name = "wiki-route-server")]
#[command(about = "Wikipedia path-finding server")]
pub struct Cli {
    #[command(flatten)]
    pub source: GraphSource,

    /// Unix socket path to listen on
    #[arg(long, default_value = "/tmp/wiki-route.sock")]
    pub socket: PathBuf,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct GraphSource {
    /// Directory containing page.sql, pagelinks.sql, linktarget.sql
    #[arg(long)]
    pub sql_dir: Option<PathBuf>,

    /// Directory containing graph.bin and reverse_graph.bin
    #[arg(long)]
    pub graphs_dir: Option<PathBuf>,
}
