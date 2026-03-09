use std::collections::HashMap;
use std::io::BufRead;

use crate::parser_utils::extract_quoted;
use crate::types::{PageId, PageTitle};

const INSERT_PREFIX: &str = "INSERT INTO `page` VALUES ";

pub fn parse_pages<R: BufRead>(reader: R) -> Result<HashMap<PageTitle, PageId>, std::io::Error> {
    let mut pages = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        let Some(rest) = line.strip_prefix(INSERT_PREFIX) else {
            continue;
        };
        let rest = rest.strip_suffix(';').unwrap_or(rest);

        for tuple in rest.split("),(") {
            let tuple = tuple.trim_matches(|c| c == '(' || c == ')');

            if let Some((page_id, title)) = parse_tuple(tuple) {
                pages.insert(title, page_id);
            }
        }
    }

    Ok(pages)
}

fn parse_tuple(tuple: &str) -> Option<(PageId, PageTitle)> {
    let mut parts = tuple.splitn(3, ',');
    let page_id: u32 = parts.next()?.parse().ok()?;
    let namespace: u32 = parts.next()?.parse().ok()?;

    if namespace != 0 {
        return None;
    }

    // rest = "'Title',is_redirect,..."
    let rest = parts.next()?;
    let rest = rest.strip_prefix('\'')?;
    let (title, after_title) = extract_quoted(rest)?;

    // after_title = ",is_redirect,..."
    let is_redirect = after_title.strip_prefix(',')?.starts_with('1');
    if is_redirect {
        return None;
    }

    Some((PageId(page_id), PageTitle(title)))
}
