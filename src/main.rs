use std::io::BufReader;
use std::fs::File;

use wiki_route::parser;

const PAGELINKS_SQL_PATH: &str = "../wiki-sql/simplewiki-latest-pagelinks.sql";
const PAGE_SQL_PATH: &str = "../wiki-sql/simplewiki-latest-page.sql";
const LINKTARGET_SQL_PATH: &str = "../wiki-sql/simplewiki-latest-linktarget.sql";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pagelinks_file = File::open(PAGELINKS_SQL_PATH)?;
    let pagelinks_reader = BufReader::new(pagelinks_file);

    let pagelinks = parser::parse_pagelinks(pagelinks_reader)?;
    println!("Parsed {} pagelink", pagelinks.len());

    Ok(())
}
