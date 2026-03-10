use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use crate::types::{Graph, LinkTargetId, PageId};

pub fn build_graphs(
    pagelinks: &[(PageId, LinkTargetId)],
    linktargets: &HashMap<LinkTargetId, PageId>,
    max_page_id: PageId,
) -> (Graph, Graph) {
    let mut graph = vec![Vec::new(); max_page_id as usize + 1];
    let mut reverse_graph = vec![Vec::new(); max_page_id as usize + 1];

    for &(from_page_id, linktarget_id) in pagelinks {
        if let Some(&to_page_id) = linktargets.get(&linktarget_id) {
            graph[from_page_id as usize].push(to_page_id);
            reverse_graph[to_page_id as usize].push(from_page_id);
        }
    }

    (graph, reverse_graph)
}

pub fn save_graphs_bin(
    graph: &Graph,
    reverse_graph: &Graph,
    graph_path: &str,
    reverse_graph_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    bincode::serialize_into(BufWriter::new(File::create(graph_path)?), graph)?;
    bincode::serialize_into(
        BufWriter::new(File::create(reverse_graph_path)?),
        reverse_graph,
    )?;
    Ok(())
}

pub fn load_graphs_bin(
    graph_path: &str,
    reverse_graph_path: &str,
) -> Result<(Graph, Graph), Box<dyn std::error::Error>> {
    let graph: Graph = bincode::deserialize_from(BufReader::new(File::open(graph_path)?))?;
    let reverse_graph: Graph =
        bincode::deserialize_from(BufReader::new(File::open(reverse_graph_path)?))?;
    Ok((graph, reverse_graph))
}
