use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use log::info;

use crate::graph::{
    TitleMaps, build_graphs, load_graphs_bin, load_titles_bin, save_graphs_bin, save_titles_bin,
};
use crate::linktarget::{parse_linktargets, resolve_linktargets};
use crate::page::{parse_id_to_title, parse_pages};
use crate::pagelinks::parse_pagelinks;
use crate::types::Graph;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn load_from_sql(sql_dir: &Path, save_dir: Option<&Path>) -> Result<(Graph, Graph, TitleMaps)> {
    let page_path = sql_dir.join("page.sql");
    let linktarget_path = sql_dir.join("linktarget.sql");
    let pagelinks_path = sql_dir.join("pagelinks.sql");

    info!("Parsing pages...");
    let title_to_id = parse_pages(BufReader::new(File::open(&page_path)?))?;
    info!("Parsed {} pages", title_to_id.len());

    info!("Parsing id_to_title...");
    let id_to_title = parse_id_to_title(BufReader::new(File::open(&page_path)?))?;
    info!("Parsed {} id_to_title entries", id_to_title.len());

    info!("Parsing linktargets...");
    let linktargets = parse_linktargets(BufReader::new(File::open(&linktarget_path)?))?;
    let linktargets = resolve_linktargets(linktargets, &title_to_id);
    info!("Resolved {} linktargets", linktargets.len());

    let title_maps = TitleMaps {
        title_to_id,
        id_to_title,
    };

    info!("Parsing pagelinks...");
    let pagelinks = parse_pagelinks(BufReader::new(File::open(&pagelinks_path)?))?;
    info!("Parsed {} pagelinks", pagelinks.len());

    let max_page_id = linktargets
        .values()
        .copied()
        .max()
        .ok_or("no linktargets")?;

    info!("Building graphs...");
    let (graph, reverse_graph) = build_graphs(&pagelinks, &linktargets, max_page_id);
    drop(pagelinks);
    drop(linktargets);
    info!("Built graphs ({} nodes)", graph.len());

    if let Some(dir) = save_dir {
        std::fs::create_dir_all(dir)?;
        let graph_path = dir.join("graph.bin");
        let reverse_graph_path = dir.join("reverse_graph.bin");
        let titles_path = dir.join("titles.bin");
        save_graphs_bin(
            &graph,
            &reverse_graph,
            graph_path.to_str().ok_or("invalid path")?,
            reverse_graph_path.to_str().ok_or("invalid path")?,
        )?;
        save_titles_bin(&title_maps, titles_path.to_str().ok_or("invalid path")?)?;
        info!("Saved graphs and titles to {}", dir.display());
    }

    Ok((graph, reverse_graph, title_maps))
}

pub fn load_from_bin(graphs_dir: &Path) -> Result<(Graph, Graph, TitleMaps)> {
    let graph_path = graphs_dir.join("graph.bin");
    let reverse_graph_path = graphs_dir.join("reverse_graph.bin");
    let titles_path = graphs_dir.join("titles.bin");

    info!("Loading precomputed graphs...");
    let (graph, reverse_graph) = load_graphs_bin(
        graph_path.to_str().ok_or("invalid path")?,
        reverse_graph_path.to_str().ok_or("invalid path")?,
    )?;
    info!("Loaded graphs ({} nodes)", graph.len());

    info!("Loading title maps...");
    let title_maps = load_titles_bin(titles_path.to_str().ok_or("invalid path")?)?;
    info!(
        "Loaded {} title_to_id, {} id_to_title",
        title_maps.title_to_id.len(),
        title_maps.id_to_title.len()
    );

    Ok((graph, reverse_graph, title_maps))
}
