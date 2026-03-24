use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use serde::{Deserialize, Serialize};

use crate::types::{Graph, LinkTargetId, PageId, PageTitle};

#[derive(Serialize, Deserialize)]
pub struct TitleMaps {
    pub title_to_id: HashMap<PageTitle, PageId>,
    pub id_to_title: HashMap<PageId, PageTitle>,
}

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

pub fn save_titles_bin(maps: &TitleMaps, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    bincode::serialize_into(BufWriter::new(File::create(path)?), maps)?;
    Ok(())
}

pub fn load_titles_bin(path: &str) -> Result<TitleMaps, Box<dyn std::error::Error>> {
    let maps: TitleMaps = bincode::deserialize_from(BufReader::new(File::open(path)?))?;
    Ok(maps)
}
