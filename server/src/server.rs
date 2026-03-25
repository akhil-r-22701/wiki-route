use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::Arc;

use log::{error, info};

use crate::bfs::find_connection;
use crate::graph::TitleMaps;
use crate::types::Graph;

pub struct ServerState {
    pub graph: Arc<Graph>,
    pub reverse_graph: Arc<Graph>,
    pub title_maps: Arc<TitleMaps>,
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
        let title_maps = Arc::clone(&state.title_maps);

        std::thread::spawn(move || {
            if let Err(e) = handle_connection(stream, &graph, &reverse_graph, &title_maps) {
                error!("Connection error: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_connection(
    stream: std::os::unix::net::UnixStream,
    graph: &Graph,
    reverse_graph: &Graph,
    title_maps: &TitleMaps,
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

        let (Some(&from_id), Some(&to_id)) = (
            title_maps.title_to_id.get(&from),
            title_maps.title_to_id.get(&to),
        ) else {
            let unknown = if !title_maps.title_to_id.contains_key(&from) {
                &from
            } else {
                &to
            };
            writer.write_all(format!("ERROR: unknown page '{}'\n\n", unknown).as_bytes())?;
            writer.flush()?;
            continue;
        };

        let response = match find_connection(from_id, to_id, graph, reverse_graph) {
            Some(path) => {
                let titles: Vec<&str> = path
                    .iter()
                    .map(|id| {
                        title_maps
                            .id_to_title
                            .get(id)
                            .map(|s| s.as_str())
                            .unwrap_or("?")
                    })
                    .collect();
                format!("OK\n{}\n\n", titles.join("\n"))
            }
            None => "NOT_FOUND\n\n".to_string(),
        };

        writer.write_all(response.as_bytes())?;
        writer.flush()?;
    }

    Ok(())
}
