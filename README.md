# wiki-route

Find the shortest path between any two Wikipedia articles. The Wikipedia game, solved.


https://github.com/user-attachments/assets/f6e32802-3450-4304-a3b8-b81f2bf2391b


> Note: The demo uses **Simple English Wikipedia** rather than full English Wikipedia for faster loading times.

## How it works

### Algorithm

Wikipedia articles form a directed graph: each article is a node, and each hyperlink to another article is an edge. Finding the shortest path between two articles is a classic graph search problem.

wiki-route uses **bidirectional BFS** (breadth-first search). It searches forward from the source article and backward from the target article simultaneously, expanding one level at a time. When the two search frontiers meet, the shortest path is guaranteed. This is significantly faster than a one-directional BFS, since the search space grows exponentially with depth.

### Architecture

The main bottleneck is loading the graph into memory. Parsing the raw SQL dumps for English Wikipedia takes significant time and RAM. To avoid repeating this cost on every query, wiki-route uses a **client/server architecture** over a Unix domain socket:

- **Server** (`wiki-route-server`): Parses MediaWiki SQL dump files (or loads precomputed binary data), builds the in-memory graph, and listens for queries on a Unix socket.
- **Client** (`wiki-route`): A lightweight CLI that connects to the server, sends a query, and prints the result. Each query is near-instant since the graph is already loaded.

The server can precompute and save the graph to `.bin` files, making subsequent startups much faster by loading binary instead of re-parsing SQL.

## Getting the data

wiki-route parses three SQL dump files from the [Wikimedia database dumps](https://dumps.wikimedia.org/):

| File | Description |
|------|-------------|
| `page.sql` | Page metadata (titles, namespaces) |
| `pagelinks.sql` | Links between pages |
| `linktarget.sql` | Link target resolution table |

**English Wikipedia (~7M articles):**
- https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-page.sql.gz
- https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-pagelinks.sql.gz
- https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-linktarget.sql.gz

**Simple English Wikipedia (~280K articles):**
- https://dumps.wikimedia.org/simplewiki/latest/simplewiki-latest-page.sql.gz
- https://dumps.wikimedia.org/simplewiki/latest/simplewiki-latest-pagelinks.sql.gz
- https://dumps.wikimedia.org/simplewiki/latest/simplewiki-latest-linktarget.sql.gz

Download and decompress the `.gz` files into a directory (e.g. `sql/`). Rename them to `page.sql`, `pagelinks.sql`, and `linktarget.sql`.

## Installation

From [crates.io](https://crates.io/):
```sh
cargo install wiki-route wiki-route-server
```

Or build from source:
```sh
git clone https://github.com/michal-pielka/wiki-route.git
cd wiki-route
cargo build --release
```

## Usage

### Start the server

From SQL dumps (first time):
```sh
wiki-route-server -v --sql-dir sql/ --save-dir data/
```

From precomputed binary data (fast startup):
```sh
wiki-route-server -v --data-dir data/
```

### Query

```sh
wiki-route United_States France
```

```
United_States -> NATO -> France
```

Page titles use underscores in place of spaces, matching the format used in Wikipedia URLs (e.g. `United_States`, not `United States`).

### Server options

```
wiki-route-server [OPTIONS] <--sql-dir <SQL_DIR>|--data-dir <DATA_DIR>>

Options:
    --sql-dir <SQL_DIR>    Directory containing page.sql, pagelinks.sql, linktarget.sql
    --data-dir <DATA_DIR>  Directory containing precomputed .bin files
    --save-dir <SAVE_DIR>  Save computed data to this directory (only valid with --sql-dir)
    --socket <SOCKET>      Unix socket path [default: /tmp/wiki-route.sock]
    -v, --verbose...       Increase verbosity (-v info, -vv debug, -vvv trace)
```

### Client options

```
wiki-route [OPTIONS] <FROM> <TO>

Options:
    --socket <SOCKET>  Unix socket path [default: /tmp/wiki-route.sock]
```

## License

[MIT](LICENSE)
