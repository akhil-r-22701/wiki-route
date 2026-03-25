mod cli;

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::process;

use clap::Parser;
use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut stream = UnixStream::connect(&cli.socket)?;

    writeln!(stream, "{}", cli.from)?;
    writeln!(stream, "{}", cli.to)?;
    stream.flush()?;

    let reader = BufReader::new(&stream);
    let mut lines = reader.lines();

    let status = match lines.next() {
        Some(Ok(line)) => line,
        _ => {
            eprintln!("Error: no response from server");
            process::exit(1);
        }
    };

    if let Some(msg) = status.strip_prefix("ERROR: ") {
        eprintln!("Error: {}", msg);
        process::exit(1);
    }

    if status == "NOT_FOUND" {
        eprintln!("No path found between '{}' and '{}'", cli.from, cli.to);
        process::exit(1);
    }

    // status == "OK", remaining lines are the path
    let path: Vec<String> = lines
        .map_while(|line| {
            let line = line.ok()?;
            if line.is_empty() { None } else { Some(line) }
        })
        .collect();

    println!("{}", path.join(" -> "));

    Ok(())
}
