//! # Topology tests module
//! 
//! ## Description
//! This module contains [`TopologyTests`] trait and its  implementation  for  [`Graph`]
//! and [`ImmutableGraphContainer`].
//! 
//! [`Graph`]: crate::Graph
//! [`ImmutableGraphContainer`]: crate::ImmutableGraphContainer
use std::collections::{HashSet, VecDeque};
use crate::{attributes::AttributeCollection, BasicImmutableGraph, Graph, Id, Locale};





/// # Topology tests
/// 
/// ## Description
/// Topology tests are functions that check whether the  graph  has  certain  structural
/// properties or not. For this reason, all these functions return `bool`.
pub trait TopologyTests {
    /// # Check if graph is connected
    /// 
    /// ## Description
    /// Check if the given graph is connected or not.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `bool` - `true` is returned if the graph is  connected,  `false`  is  returned
    /// otherwise.
    /// 
    /// ## Details
    /// Uses breadth-first search to determine connectivity.
    /// 
    /// In     general,     `g.is_strongly_connected() == true`      always      implies
    /// `g.is_connected() == true`.
    /// 
    /// If     the      underlying      graph      `g`      is      [undirected][kinds],
    /// `g.is_connected() == g.is_strongly_connected()`.
    /// 
    /// Empty graphs are always connected.
    /// 
    /// [kinds]: crate::Graph#different-kinds-of-graphs
    fn is_connected(&self) -> bool;
}



// Graph::TopologyTests
impl<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType> TopologyTests for Graph<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>
where
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    LocaleType: Locale<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    fn is_connected(&self) -> bool {
        let mut unvisited_vertices = VecDeque::from(match self.iter_v().next() {
            Some(value) => [value],
            None => return true,
        });
        let mut visited_vertices = HashSet::with_capacity(self.count_v());
        while !unvisited_vertices.is_empty() {
            let curr_vertex_id = unvisited_vertices.pop_front().unwrap();
            visited_vertices.insert(curr_vertex_id.clone());
            unvisited_vertices.extend(self.edge_list.get(&curr_vertex_id).unwrap().iter_adjacent().filter(|x| !visited_vertices.contains(x)));
        }
        visited_vertices.len() == self.count_v()
    }
}





#[cfg(test)]
mod topology_tests_tests {
    use crate::*;
    use super::*;

    #[test]
    fn is_connected() {
        let mut g: graph!(X ---X--- X) = Graph::new();
        let id1 = g.add_v(None);
        let id2 = g.add_v(None);
        assert_eq!(g.is_connected(), false);
        g.add_e(&id1, &id2, true, None).unwrap();
        assert_eq!(g.is_connected(), true);
    }
}
