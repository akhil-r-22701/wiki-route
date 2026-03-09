use std::collections::{HashMap, VecDeque};

use crate::types::{Graph, LinkTargetId, PageId};

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

pub fn find_connection(
    start: PageId,
    target: PageId,
    graph: &Graph,
    reverse_graph: &Graph,
) -> Option<Vec<PageId>> {
    if start == target {
        return Some(vec![start]);
    }

    // page -> parent
    let mut forward_visited: HashMap<PageId, PageId> = HashMap::from([(start, start)]);
    let mut backward_visited: HashMap<PageId, PageId> = HashMap::from([(target, target)]);

    let mut forward_queue: VecDeque<PageId> = VecDeque::from([start]);
    let mut backward_queue: VecDeque<PageId> = VecDeque::from([target]);

    loop {
        if forward_queue.is_empty() && backward_queue.is_empty() {
            return None;
        }

        if let Some(cur) = forward_queue.pop_front() {
            for &neighbor in &graph[cur as usize] {
                if forward_visited.contains_key(&neighbor) {
                    continue;
                }

                forward_visited.insert(neighbor, cur);
                forward_queue.push_back(neighbor);

                if backward_visited.contains_key(&neighbor) {
                    return Some(reconstruct_path(
                        neighbor,
                        &forward_visited,
                        &backward_visited,
                    ));
                }
            }
        }

        if let Some(cur) = backward_queue.pop_front() {
            for &neighbor in &reverse_graph[cur as usize] {
                if backward_visited.contains_key(&neighbor) {
                    continue;
                }
                backward_visited.insert(neighbor, cur);
                backward_queue.push_back(neighbor);

                if forward_visited.contains_key(&neighbor) {
                    return Some(reconstruct_path(
                        neighbor,
                        &forward_visited,
                        &backward_visited,
                    ));
                }
            }
        }
    }
}

fn reconstruct_path(
    middle: PageId,
    forward_visited: &HashMap<PageId, PageId>,
    backward_visited: &HashMap<PageId, PageId>,
) -> Vec<PageId> {
    // Walk from meeting back to start via forward_visited
    let mut path = vec![middle];
    let mut cur = middle;

    while forward_visited[&cur] != cur {
        cur = forward_visited[&cur];
        path.push(cur);
    }

    path.reverse();

    // Walk from meeting back to target via backward_visited
    cur = middle;

    while backward_visited[&cur] != cur {
        cur = backward_visited[&cur];
        path.push(cur);
    }

    path
}
