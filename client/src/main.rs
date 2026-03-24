mod cli;

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;

use clap::Parser;
use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut stream = UnixStream::connect(&cli.socket)?;

    writeln!(stream, "{}", cli.from)?;
    writeln!(stream, "{}", cli.to)?;
    stream.flush()?;

    let reader = BufReader::new(&stream);
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        println!("{}", line);
    }

    Ok(())
}
