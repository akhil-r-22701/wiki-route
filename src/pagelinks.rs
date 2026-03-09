use std::io::BufRead;

use crate::types::{LinkTargetId, PageId};

const INSERT_PREFIX: &str = "INSERT INTO `pagelinks` VALUES ";

pub fn parse_pagelinks<R: BufRead>(
    reader: R,
) -> Result<Vec<(PageId, LinkTargetId)>, std::io::Error> {
    let mut pagelinks = Vec::new();

    for line in reader.lines() {
        let line = line?;

        // Skip lines not starting with INSERT_PREFIX
        let Some(rest) = line.strip_prefix(INSERT_PREFIX) else {
            continue;
        };

        let rest = rest.strip_suffix(';').unwrap_or(rest);

        for tuple in rest.split("),(") {
            let tuple = tuple.trim_matches(|c| c == '(' || c == ')');
            let mut parts = tuple.splitn(3, ',');

            let (Some(from), Some(from_ns), Some(target_id)) =
                (parts.next(), parts.next(), parts.next())
            else {
                continue;
            };

            if from_ns != "0" {
                continue;
            }

            let (Ok(from), Ok(target_id)) = (from.parse::<u32>(), target_id.parse::<u64>()) else {
                continue;
            };

            pagelinks.push((PageId(from), LinkTargetId(target_id)));
        }
    }

    Ok(pagelinks)
}
