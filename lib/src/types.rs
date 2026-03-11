pub type PageId = u32;
pub type LinkTargetId = u64;
pub type PageTitle = String;

// Graph adjacency list
// We use Vec instead of Hashmap to save time on hashing
// graph[page_id] = [neigh1_page_id, neigh2_page_id, ...]
pub type Graph = Vec<Vec<PageId>>;
