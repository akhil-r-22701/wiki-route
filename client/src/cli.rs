use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "wiki-route")]
#[command(about = "Wikipedia shortest-path client")]
pub struct Cli {
    /// Unix socket path to connect to
    #[arg(long, default_value = "/tmp/wiki-route.sock")]
    pub socket: PathBuf,
}
