const PAGELINKS_SQL_PATH: &str = "../wiki-sql/simplewiki-latest-pagelinks.sql";
const PAGE_SQL_PATH: &str = "../wiki-sql/simplewiki-latest-page.sql";
const LINKTARGET_SQL_PATH: &str = "../wiki-sql/simplewiki-latest-linktarget.sql";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pagelinks_bytes = std::fs::read(PAGELINKS_SQL_PATH)?;
    let page_bytes = std::fs::read(PAGE_SQL_PATH)?;
    let linktarget_bytes = std::fs::read(LINKTARGET_SQL_PATH)?;

    Ok(())
}
