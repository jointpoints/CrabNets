//! # Topology tests module
//! 
//! ## Description
//! This module contains [`TopologyTests`] trait and its  implementation  for  [`Graph`]
//! and [`ImmutableGraphContainer`].
//! 
//! [`Graph`]: crate::Graph
//! [`ImmutableGraphContainer`]: crate::ImmutableGraphContainer
use std::collections::{HashMap, HashSet, VecDeque};
use crate::{attributes::AttributeCollection, BasicImmutableGraph, Graph, Hints, Id, Locale};





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * AUXILIARY FUNCTIONS                                                               *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



// This function must only be used for graphs with small number of vertices (< 256)
fn has_hamiltonian_path_bruteforce<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>(g: &Graph<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>) -> bool
where
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    LocaleType: Locale<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    for start_vertex_id in g.iter_v() {
        let mut dfs_stack = VecDeque::from([(start_vertex_id.clone(), Vec::from([start_vertex_id]))]);
        while let Some((curr_id, curr_visited)) = dfs_stack.pop_back() {
            if curr_visited.len() == g.count_v() {
                return true;
            }
            for neighbour_id in g.iter_adjacent_undir(&curr_id).unwrap().chain(g.iter_adjacent_out(&curr_id).unwrap()) {
                // The neighbour must not be included in the path
                if let Err(index) = curr_visited.binary_search(&neighbour_id) {
                    let mut neighbour_visited = curr_visited.clone();
                    neighbour_visited.insert(index, neighbour_id.clone());
                    dfs_stack.push_back((neighbour_id.clone(), neighbour_visited));
                }
            }
        }
    }
    false
}

fn has_hamiltonian_path_dynamic<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>(g: &Graph<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>) -> bool
where
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    LocaleType: Locale<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    unimplemented!();
}

fn has_hamiltonian_path_low_treewidth<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>(g: &Graph<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>) -> bool
where
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    LocaleType: Locale<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    unimplemented!();
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * TOPOLOGY TESTS                                                                    *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



/// # Topology tests
/// 
/// ## Description
/// Topology tests are functions that check whether the  graph  has  certain  structural
/// properties or not. For this reason, all these functions return `bool`.
pub trait TopologyTests {
    /// # Check if graph has Hamiltonian path
    /// 
    /// ## Description
    /// Check if the given graph has a path that visits all vertices exactly once.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// * `given_hints` : `Hints` - hints that may lead the algorithm to choose a faster
    /// approach for the given graph.
    /// * `that_are_forced` : `bool` - shows whether the hints given by the user  should
    /// be forcefully followed (`true`) or verified by the algorithm (`false`).
    /// 
    /// ## Returns
    /// * `bool` - `true` is returned if the graph has a Hamiltonian  path,  `false`  is
    /// returned otherwise.
    /// 
    /// ## Details
    /// <div class="warning">
    /// 
    /// 🧠 **Adaptive algorithm**
    /// 
    /// This function  implements  an  adaptive  algorithm  that  chooses  the  approach
    /// depending on the properties of the given  graph.  These  properties  are  either
    /// checked automatically at runtime (if the  computation  time  required  to  check
    /// these properties is negligible) or provided by the user in the form of  [hints].
    /// If the hints given are wrong but they're forced to be  followed,  the  algorithm
    /// _may produce a wrong answer_! If the hints  given  are  wrong  but  they're  not
    /// forced to be followed, the algorithm will still  produce  correct  results,  but
    /// potentially much slower.
    /// 
    /// On the other hand, if the  hints  are  correct,  enforcing  them  may  introduce
    /// further speed-up as the developer _guarantees_  their  correctness  and,  hence,
    /// they won't be verified.
    /// 
    /// </div>
    /// 
    /// If the underlying graph is [directed][kinds], the  algorithm  will  look  for  a
    /// Hamiltonian path traversable in at least one direction.
    /// 
    /// Empty graphs always have Hamiltonian paths.
    /// 
    /// [hints]: crate::Hints
    /// [kinds]: crate::Graph#different-kinds-of-graphs
    fn has_hamiltonian_path(&self, given_hints: Hints, that_are_forced: bool) -> bool;
    /// # Check if graph is connected
    /// 
    /// ## Description
    /// Check if the given graph is connected.
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
    /// Prefer          calling          `g.is_connected()`          instead          of
    /// `g.iter_connected_components().len() == 1`.
    /// 
    /// ## Complexity
    /// Time: O(|V| + |E|).
    /// 
    /// Space: O(|V|).
    /// 
    /// [kinds]: crate::Graph#different-kinds-of-graphs
    fn is_connected(&self) -> bool;
    /// # Check if graph is strongly connected
    /// 
    /// ## Description
    /// Check if the given graph is strongly connected.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `bool` - `true` is returned if the graph is  strongly  connected,  `false`  is
    /// returned otherwise.
    /// 
    /// ## Details
    /// Uses  an  optimised  variant  of  Tarjan's  depth-first-search-based   algorithm
    /// [[source for the original algorithm](https://doi.org/10.1137/0201010)]. Tarjan's
    /// algorithm enumerates all strongly connected components, however,  this  function
    /// solves the decision  problem:  'Does  this  graph  contain  exactly  1  strongly
    /// connected component?' Therefore, several simplifications have  been  made  (such
    /// as, for example, early stopping) to improve the performance.
    /// 
    /// In     general,     `g.is_strongly_connected() == true`      always      implies
    /// `g.is_connected() == true`.
    /// 
    /// If     the      underlying      graph      `g`      is      [undirected][kinds],
    /// `g.is_connected() == g.is_strongly_connected()`.
    /// 
    /// Undirected edge is equivalent to a pair of edges directed in opposite ways.
    /// 
    /// Empty graphs are always strongly connected.
    /// 
    /// Prefer       calling        `g.is_strongly_connected()`        instead        of
    /// `g.iter_strongly_connected_components().len() == 1`.
    /// 
    /// Prefer calling `g.is_connected()`  instead  of  `g.is_strongly_connected()`  for
    /// [undirected][kinds] graphs.
    /// 
    /// ## Complexity
    /// Time: O(|V| + |E|).
    /// 
    /// Space: O(|V|).
    /// 
    /// [kinds]: crate::Graph#different-kinds-of-graphs
    fn is_strongly_connected(&self) -> bool;
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
    fn has_hamiltonian_path(&self, given_hints: Hints, that_are_forced: bool) -> bool {
        if self.count_v() == 0 {
            return true;
        }
        if self.count_v() <= 10 {
            return has_hamiltonian_path_bruteforce(self);
        }
        if given_hints.contains(Hints::LOW_TREEWIDTH) {
            if that_are_forced || false {
                return has_hamiltonian_path_low_treewidth(self);
            }
        }
        has_hamiltonian_path_dynamic(self)
    }

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

    fn is_strongly_connected(&self) -> bool {
        let mut dfs_stack = VecDeque::from(match self.iter_v().next() {
            Some(value) => [value],
            None => return true,
        });
        let mut curr_index = 0usize;
        let mut indices = HashMap::with_capacity(self.count_v());
        let mut back_links: HashMap<VertexIdType, usize> = HashMap::with_capacity(self.count_v());
        let mut visited_vertices = HashSet::with_capacity(self.count_v());
        while !dfs_stack.is_empty() {
            let curr_vertex_id = dfs_stack.back().unwrap().clone();
            if visited_vertices.contains(&curr_vertex_id) {
                dfs_stack.pop_back().unwrap();
                for adjacent_id in self.iter_adjacent_out(&curr_vertex_id).unwrap().chain(self.iter_adjacent_undir(&curr_vertex_id).unwrap()) {
                    let adjacent_back_link = back_links[&adjacent_id];
                    let curr_back_link = back_links.get_mut(&curr_vertex_id).unwrap();
                    *curr_back_link = adjacent_back_link.min(*curr_back_link);
                }
                if back_links[&curr_vertex_id] == indices[&curr_vertex_id] && !dfs_stack.is_empty() {
                    return false;
                }
            } else {
                visited_vertices.insert(curr_vertex_id.clone());
                indices.insert(curr_vertex_id.clone(), curr_index);
                back_links.insert(curr_vertex_id.clone(), curr_index);
                curr_index += 1;
                for adjacent_id in self.iter_adjacent_out(&curr_vertex_id).unwrap().chain(self.iter_adjacent_undir(&curr_vertex_id).unwrap()) {
                    if visited_vertices.contains(&adjacent_id) {
                        let curr_back_link = back_links.get_mut(&curr_vertex_id).unwrap();
                        *curr_back_link = indices[&curr_vertex_id].min(indices[&adjacent_id]);
                    } else {
                        dfs_stack.push_back(adjacent_id);
                    }
                }
            }
        }
        visited_vertices.len() == self.count_v()
    }
}





#[cfg(test)]
mod topology_tests_tests {
    use crate::*;
    use super::*;

    #[test]
    fn has_hamiltonian_path() {
        // Brute force
        let mut g: graph!(X ---X--> X) = Graph::new();
        assert_eq!(g.has_hamiltonian_path(Hints::empty(), false), true);
        let id1 = g.add_v(None);
        let id2 = g.add_v(None);
        let id3 = g.add_v(None);
        g.add_e(&id2, &id1, true, None).unwrap();
        g.add_e(&id3, &id1, true, None).unwrap();
        assert_eq!(g.has_hamiltonian_path(Hints::empty(), false), false);
        assert_eq!(g.has_hamiltonian_path(Hints::empty(), true), false);
        g.remove_e(&id3, &id1, &0).unwrap();
        g.add_e(&id3, &id2, true, None).unwrap();
        assert_eq!(g.has_hamiltonian_path(Hints::empty(), false), true);
        assert_eq!(g.has_hamiltonian_path(Hints::empty(), true), true);
    }

    #[test]
    fn is_connected() {
        let mut g: graph!(X ---X--- X) = Graph::new();
        assert_eq!(g.is_connected(), true);
        let id1 = g.add_v(None);
        let id2 = g.add_v(None);
        assert_eq!(g.is_connected(), false);
        g.add_e(&id1, &id2, true, None).unwrap();
        assert_eq!(g.is_connected(), true);
    }

    #[test]
    fn is_strongly_connected() {
        let mut g: graph!(X ---X--> X) = Graph::new();
        assert_eq!(g.is_strongly_connected(), true);
        let id1 = g.add_v(None);
        let id2 = g.add_v(None);
        assert_eq!(g.is_strongly_connected(), false);
        g.add_e(&id1, &id2, true, None).unwrap();
        assert_eq!(g.is_strongly_connected(), false);
        assert_eq!(g.is_connected(), true);
        let id3 = g.add_v(None);
        let id4 = g.add_v(None);
        let id5 = g.add_v(None);
        let id6 = g.add_v(None);
        g.add_e(&id2, &id3, true, None).unwrap();
        g.add_e(&id3, &id1, true, None).unwrap();
        g.add_e(&id3, &id4, true, None).unwrap();
        g.add_e(&id3, &id5, true, None).unwrap();
        g.add_e(&id5, &id6, true, None).unwrap();
        g.add_e(&id6, &id3, true, None).unwrap();
        assert_eq!(g.is_strongly_connected(), false);
        g.remove_e(&id3, &id4, &0).unwrap();
        g.add_e(&id3, &id4, false, None).unwrap();
        assert_eq!(g.is_strongly_connected(), true);
        g.remove_v(&id4);
        assert_eq!(g.is_strongly_connected(), true);
    }
}
