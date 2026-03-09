use std::collections::HashMap;
use std::io::BufRead;

use crate::parser_utils::extract_quoted;
use crate::types::{LinkTargetId, PageTitle};

const INSERT_PREFIX: &str = "INSERT INTO `linktarget` VALUES ";

pub fn parse_linktargets<R: BufRead>(
    reader: R,
) -> Result<HashMap<LinkTargetId, PageTitle>, std::io::Error> {
    let mut linktargets = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        let Some(rest) = line.strip_prefix(INSERT_PREFIX) else {
            continue;
        };

        let rest = rest.strip_suffix(';').unwrap_or(rest);

        for tuple in rest.split("),(") {
            let tuple = tuple.trim_matches(|c| c == '(' || c == ')');

            if let Some((lt_id, title)) = parse_tuple(tuple) {
                linktargets.insert(lt_id, title);
            }
        }
    }

    Ok(linktargets)
}

fn parse_tuple(tuple: &str) -> Option<(LinkTargetId, PageTitle)> {
    let mut parts = tuple.splitn(3, ',');
    let lt_id: u64 = parts.next()?.parse().ok()?;
    let namespace: u32 = parts.next()?.parse().ok()?;

    if namespace != 0 {
        return None;
    }

    let rest = parts.next()?.strip_prefix('\'')?;
    let (title, _) = extract_quoted(rest)?;

    Some((lt_id, title))
}
