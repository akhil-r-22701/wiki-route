use std::fs::File;
use std::io::BufReader;

use wiki_route_lib::graph::build_graphs;
use wiki_route_lib::linktarget::{parse_linktargets, resolve_linktargets};
use wiki_route_lib::page::parse_pages;
use wiki_route_lib::pagelinks::parse_pagelinks;

const PAGELINKS_SQL_PATH: &str = "wiki-sql/simplewiki-latest-pagelinks.sql";
const PAGE_SQL_PATH: &str = "wiki-sql/simplewiki-latest-page.sql";
const LINKTARGET_SQL_PATH: &str = "wiki-sql/simplewiki-latest-linktarget.sql";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let page_file = File::open(PAGE_SQL_PATH)?;
    let page_reader = BufReader::new(page_file);

    let pagelinks_file = File::open(PAGELINKS_SQL_PATH)?;
    let pagelinks_reader = BufReader::new(pagelinks_file);

    let linktarget_file = File::open(LINKTARGET_SQL_PATH)?;
    let linktarget_reader = BufReader::new(linktarget_file);

    let pages = parse_pages(page_reader)?;
    println!("Parsed {} pages", pages.len());

    let pagelinks = parse_pagelinks(pagelinks_reader)?;
    println!("Parsed {} pagelinks", pagelinks.len());

    let linktargets = parse_linktargets(linktarget_reader)?;
    println!("Parsed {} linktargets", linktargets.len());
    let linktargets = resolve_linktargets(linktargets, &pages);
    println!("Resolved {} linktargets to page IDs", linktargets.len());

    let max_page_id = pages.values().copied().max().ok_or("TODO: Error")?;
    let (graph, reverse_graph) = build_graphs(&pagelinks, &linktargets, max_page_id);
    println!("Built graphs with {} pages", pages.len());

    Ok(())
}
