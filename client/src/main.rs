mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    // TODO: connect to cli.socket, send query, print result
    println!("Connecting to {}...", cli.socket.display());
    todo!()
}
