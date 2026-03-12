use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::graph::{build_graphs, load_graphs_bin, save_graphs_bin};
use crate::linktarget::{parse_linktargets, resolve_linktargets};
use crate::page::parse_pages;
use crate::pagelinks::parse_pagelinks;
use crate::types::Graph;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn load_from_sql(sql_dir: &Path, save_dir: Option<&Path>) -> Result<(Graph, Graph)> {
    let page_path = sql_dir.join("page.sql");
    let linktarget_path = sql_dir.join("linktarget.sql");
    let pagelinks_path = sql_dir.join("pagelinks.sql");

    println!("[*] Parsing pages...");
    let pages = parse_pages(BufReader::new(File::open(&page_path)?))?;
    println!("[+] Parsed {} pages", pages.len());

    println!("[*] Parsing linktargets...");
    let linktargets = parse_linktargets(BufReader::new(File::open(&linktarget_path)?))?;
    let linktargets = resolve_linktargets(linktargets, &pages);
    drop(pages);
    println!("[+] Resolved {} linktargets", linktargets.len());

    println!("[*] Parsing pagelinks...");
    let pagelinks = parse_pagelinks(BufReader::new(File::open(&pagelinks_path)?))?;
    println!("[+] Parsed {} pagelinks", pagelinks.len());

    let max_page_id = linktargets
        .values()
        .copied()
        .max()
        .ok_or("no linktargets")?;

    println!("[*] Building graphs...");
    let (graph, reverse_graph) = build_graphs(&pagelinks, &linktargets, max_page_id);
    drop(pagelinks);
    drop(linktargets);
    println!("[+] Built graphs ({} nodes)", graph.len());

    if let Some(dir) = save_dir {
        let graph_path = dir.join("graph.bin");
        let reverse_graph_path = dir.join("reverse_graph.bin");
        save_graphs_bin(
            &graph,
            &reverse_graph,
            graph_path.to_str().ok_or("invalid path")?,
            reverse_graph_path.to_str().ok_or("invalid path")?,
        )?;
        println!("[+] Saved graphs to {}", dir.display());
    }

    Ok((graph, reverse_graph))
}

pub fn load_from_bin(graphs_dir: &Path) -> Result<(Graph, Graph)> {
    let graph_path = graphs_dir.join("graph.bin");
    let reverse_graph_path = graphs_dir.join("reverse_graph.bin");

    println!("[*] Loading precomputed graphs...");
    let (graph, reverse_graph) = load_graphs_bin(
        graph_path.to_str().ok_or("invalid path")?,
        reverse_graph_path.to_str().ok_or("invalid path")?,
    )?;
    println!("[+] Loaded graphs ({} nodes)", graph.len());

    Ok((graph, reverse_graph))
}
