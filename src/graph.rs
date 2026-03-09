use std::collections::HashMap;

use crate::types::{Graph, LinkTargetId, PageId};

pub fn build_graph(
    pagelinks: &[(PageId, LinkTargetId)],
    linktargets: &HashMap<LinkTargetId, PageId>,
    max_page_id: PageId,
) -> Graph {
    let mut graph = vec![Vec::new(); max_page_id as usize + 1];

    for &(from_page_id, linktarget_id) in pagelinks {
        if let Some(&to_page_id) = linktargets.get(&linktarget_id) {
            graph[from_page_id as usize].push(to_page_id);
        }
    }

    graph
}
