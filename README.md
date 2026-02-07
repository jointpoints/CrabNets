# CONNECTO.RS

![Version badge](https://img.shields.io/badge/version-0.1.0_pre--release-blue?style=for-the-badge)

## Welcome!
CONNECTO.RS is a fast Rust crate for graph manipulation with user-friendly interface.

## ⚡ Features
* **🕸 Different categories of graphs.** CONNECTO.RS allow you to create and process both
directed and undirected graphs that either allow or prohibit parallel edges.
* **🦺 Strong exception safety.** If a function in CONNECTO.RS returns an `Err(...)`,
CONNECTO.RS guarantee that every mutable argument of the said function will be in its initial
state.

## 🔍 Basic example
```rust
/* In this example, we'll compute the shortest path from A to D in the following directed
 * graph:
 *
 *      A         B
 *       ●---3---●
 *       ↑       |
 *       2       5
 *       |       ↓
 *       ●---1---●
 *      C         D
 *
 * The expected output is 8.
 */
use connecto_rs::essentials::*;

fn main() {
    // g is our graph.
    // [char|()] means that its vertices will have char IDs and weights of
    // type () (i.e. there'll be no vertex weights).
    // ---...--> means that the graph will be simple (no parallel edges) and
    // directed.
    // [usize|u8] means that the edges will have usize IDs and u8 weights.
    let mut g: graph!{ [char|()] ---[usize|u8]--> } = Graph::new();

    // List of all our vertices (vertex IDs and weights).
    let vertices = vec![
        (Some('A'), ()),
        (Some('B'), ()),
        (Some('C'), ()),
        (Some('D'), ()),
    ];

    // List of all our edges (edge IDs, IDs of the incident vertices,
    // directednesses and edge weights).
    let edges = vec![
        (Some(0), 'A', 'B', AbsoluteEdgeDirection::Undirected, 3),
        (Some(1), 'B', 'D', AbsoluteEdgeDirection::Directed, 5),
        (Some(2), 'D', 'C', AbsoluteEdgeDirection::Undirected, 1),
        (Some(3), 'C', 'A', AbsoluteEdgeDirection::Directed, 2),
    ];

    // Add all vertices and edges to the graph
    g.v_mut().insert_from_iter(vertices.into_iter()).unwrap();
    g.e_mut().insert_from_iter(edges.into_iter()).unwrap();
    .....
}
```
