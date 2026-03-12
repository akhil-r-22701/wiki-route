use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::Arc;

use log::{error, info};

use crate::types::Graph;

pub struct ServerState {
    pub graph: Arc<Graph>,
    pub reverse_graph: Arc<Graph>,
}

pub fn run(socket: &Path, state: ServerState) -> Result<(), Box<dyn std::error::Error>> {
    if socket.exists() {
        std::fs::remove_file(socket)?;
    }

    let listener = UnixListener::bind(socket)?;
    info!("Ready. Listening on {}", socket.display());

    for stream in listener.incoming() {
        let stream = stream?;
        let graph = Arc::clone(&state.graph);
        let reverse_graph = Arc::clone(&state.reverse_graph);

        std::thread::spawn(move || {
            if let Err(e) = handle_connection(stream, &graph, &reverse_graph) {
                error!("Connection error: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_connection(
    stream: std::os::unix::net::UnixStream,
    _graph: &Graph,
    _reverse_graph: &Graph,
) -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;
    let mut lines = reader.lines();

    loop {
        let from = match lines.next() {
            Some(Ok(line)) => line,
            _ => break,
        };
        let to = match lines.next() {
            Some(Ok(line)) => line,
            _ => break,
        };

        info!("Query: {} -> {}", from, to);

        // TODO: resolve titles to PageIds, run BFS, respond with path
        writer.write_all(b"TODO\n\n")?;
        writer.flush()?;
    }

    Ok(())
}
