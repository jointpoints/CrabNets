//! <p style="text-align: center;">
//!     <i><b>Fast Rust crate for graph manipulation with user-friendly interface</b></i>
//! </p>
//!
//! ```ignore
//! cargo add connecto_rs
//! ```
//!
//! ## ⚡ Features
//! * **🕸 Different categories of graphs.** CONNECTO.RS allow you to create and process both
//! directed and undirected graphs that either allow or prohibit parallel edges.
//! * **🦺 Strong exception safety.** If a function in CONNECTO.RS returns an `Err(...)`,
//! CONNECTO.RS guarantee that every mutable argument of the said function will be in its initial
//! state.
//!
//! ## 🔍 Basic example
//! ```ignore
//! /* In this example, we'll compute the shortest path from A to D in the following directed
//!  * graph:
//!  *
//!  *      A         B
//!  *       ●---3---●
//!  *       ↑       |
//!  *       2       5
//!  *       |       ↓
//!  *       ●---1---●
//!  *      C         D
//!  *
//!  * The expected output is 8.
//!  */
//! use connecto_rs::essentials::*;
//!
//! fn main() {
//!     // g is our graph.
//!     // [char|()] means that its vertices will have char IDs and weights of
//!     // type () (i.e. there'll be no vertex weights).
//!     // ---...--> means that the graph will be simple (no parallel edges) and
//!     // directed.
//!     // [usize|u8] means that the edges will have usize IDs and u8 weights.
//!     let mut g: graph!{ [char|()] ---[usize|u8]--> } = Graph::new();
//!
//!     // List of all our vertices (vertex IDs and weights).
//!     let vertices = vec![
//!         (Some('A'), ()),
//!         (Some('B'), ()),
//!         (Some('C'), ()),
//!         (Some('D'), ()),
//!     ];
//!
//!     // List of all our edges (edge IDs, IDs of the incident vertices,
//!     // directednesses and edge weights).
//!     let edges = vec![
//!         (Some(0), 'A', 'B', AbsoluteEdgeDirection::Undirected, 3),
//!         (Some(1), 'B', 'D', AbsoluteEdgeDirection::Directed, 5),
//!         (Some(2), 'D', 'C', AbsoluteEdgeDirection::Undirected, 1),
//!         (Some(3), 'C', 'A', AbsoluteEdgeDirection::Directed, 2),
//!     ];
//!
//!     // Add all vertices and edges to the graph
//!     g.v_mut().insert_from_iter(vertices.into_iter()).unwrap();
//!     g.e_mut().insert_from_iter(edges.into_iter()).unwrap();
//!     .....
//! }
//! ```
//!
//! ## 📚 Learn CONNECTO.RS quickly, step by step
//! 1. [Get acquainted with `Graph`, the main struct of CONNECTO.RS.][graph]
//! 2. [Specify a graph type using the `graph!` macro.][graphmacro]
//! 3. [Explore different kinds of operations that CONNECTO.RS enable you to do on your graphs out of the box.][util]
//! 4. todo!
//! 5. [Load and save your graphs.][io]
//!
//! [graph]: Graph
//! [graphmacro]: graph
//! [util]: crate::ops
//! [io]: todo!
pub mod locales;
pub mod essentials;
pub mod iter;

/// # Operations with your graphs
///
/// CONNECTO.RS implements a wide range of functions to edit, query or analyse your graphs as well
/// as solvers for various standard problems on them.
/// All these functions are grouped into the following modules:
/// * [`connecto_rs::optim`][opt] -- Functions that solve optimisation problems on graphs.
/// * [`connecto_rs::stat`][stat] -- Functions that compute statistics on graphs.
/// * [`connecto_rs::trans`][trans] -- Functions that perform transformations on graphs.
///
/// [opt]: crate::optim
/// [stat]: crate::stat
/// [trans]: crate::trans
pub mod ops;

use std::{collections::HashMap, fmt::Debug, hash::Hash, marker::PhantomData};
use anyhow::{Ok, Result, anyhow};

use crate::{iter::{AdjacentVerticesIter, DFSPreorderIter}, locales::Locale};
pub use crate::ops::*;










pub trait Id
where
Self: Copy + Clone + Debug + Default + Eq + Hash + PartialOrd,
{
    fn increment(&self) -> Self;
}



macro_rules! impl_id_for_numeric_type {
    ($num_type:ty) => {
        impl Id for $num_type {
            fn increment(&self) -> Self {
                self + 1
            }
        }
    };
}
impl_id_for_numeric_type!(i8);
impl_id_for_numeric_type!(i16);
impl_id_for_numeric_type!(i32);
impl_id_for_numeric_type!(i64);
impl_id_for_numeric_type!(i128);
impl_id_for_numeric_type!(isize);
impl_id_for_numeric_type!(u8);
impl_id_for_numeric_type!(u16);
impl_id_for_numeric_type!(u32);
impl_id_for_numeric_type!(u64);
impl_id_for_numeric_type!(u128);
impl_id_for_numeric_type!(usize);



impl Id for char {
    fn increment(&self) -> Self {
        const UNICODE_SCALAR_VALUE_RANGE_1_LOWER: u32 = 0;
        const UNICODE_SCALAR_VALUE_RANGE_1_UPPER: u32 = 0xD7FF;
        const UNICODE_SCALAR_VALUE_RANGE_2_LOWER: u32 = 0xE000;
        const UNICODE_SCALAR_VALUE_RANGE_2_UPPER: u32 = 0x10FFFF;
        let curr_value = *self as u32; // guaranteed to be in one of the Unicode scalar value ranges
        let new_value = curr_value + 1;
        char::from_u32(if new_value == UNICODE_SCALAR_VALUE_RANGE_1_UPPER + 1 {
            UNICODE_SCALAR_VALUE_RANGE_2_LOWER
        } else if new_value == UNICODE_SCALAR_VALUE_RANGE_2_UPPER + 1 {
            UNICODE_SCALAR_VALUE_RANGE_1_LOWER
        } else {
            new_value
        }).unwrap()
    }
}










#[derive(PartialEq, Eq)]
pub enum AbsoluteEdgeDirection {
    Directed,
    Undirected,
}



pub enum RelativeEdgeDirection {
    DirectedFrom,
    DirectedTo,
    Undirected,
}



pub enum AbsoluteEdgeDirectionSelector {
    Any,
    Directed,
    Undirected,
}



#[derive(PartialEq, Eq)]
pub enum RelativeEdgeDirectionSelector {
    Any,
    AnyDirected,
    DirectedFrom,
    DirectedTo,
    Undirected,
}










#[derive(Clone)]
pub struct Edge<VertexIdType, EdgeWeightType>
where
VertexIdType: Id,
{
    vid1: VertexIdType,
    vid2: VertexIdType,
    weight: EdgeWeightType,
}










#[derive(Clone)]
pub struct Vertex<EdgeIdType, LocaleType, VertexWeightType>
where
EdgeIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    locale: LocaleType,
    weight: VertexWeightType,
    _phantom: PhantomData<EdgeIdType>,
}










#[derive(Clone)]
pub struct Graph<EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    edges_dir: HashMap<EdgeIdType, Edge<VertexIdType, EdgeWeightType>>,
    edges_undir: HashMap<EdgeIdType, Edge<VertexIdType, EdgeWeightType>>,
    vertices: HashMap<VertexIdType, Vertex<EdgeIdType, LocaleType, VertexWeightType>>,
    next_free_edge_id: EdgeIdType,
    next_free_vertex_id: VertexIdType,
}



impl<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> Graph<EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    /// # Set of edges (immutable)
    ///
    /// Allows to interact with the edges of the graph in the _immutable_ mode.
    #[inline(always)]
    pub fn e(&'a self) -> ImmutableEdgeSet<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> {
        ImmutableEdgeSet { g: self }
    }

    /// # Set of edges (mutable)
    ///
    /// Allows to interact with the edges of the graph in the _mutable_ mode.
    #[inline(always)]
    pub fn e_mut(&'a mut self) -> MutableEdgeSet<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> {
        MutableEdgeSet { g: self }
    }

    /// # Set of vertices (immutable)
    ///
    /// Allows to interact with the vertices of the graph in the _immutable_ mode.
    #[inline(always)]
    pub fn v(&'a self) -> ImmutableVertexSet<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> {
        ImmutableVertexSet { g: self }
    }

    /// # Set of vertices (mutable)
    ///
    /// Allows to interact with the vertices of the graph in the _mutable_ mode.
    #[inline(always)]
    pub fn v_mut(&'a mut self) -> MutableVertexSet<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> {
        MutableVertexSet { g: self }
    }

    /// # Create a new graph
    ///
    /// Alias to [`Graph::null`].
    #[inline(always)]
    pub fn new() -> Self {
        Self::null()
    }

    /// # Create a null graph
    ///
    /// Creates a graph with no vertices and no edges.
    #[inline(always)]
    pub fn null() -> Self {
        Graph {
            edges_dir: HashMap::new(),
            edges_undir: HashMap::new(),
            vertices: HashMap::new(),
            next_free_edge_id: EdgeIdType::default(),
            next_free_vertex_id: VertexIdType::default()
        }
    }
}



/// # Macro to specify a graph
///
/// Graphs in CONNECTO.RS are _generic_ and must be specified when a graph is instantiated.
/// This macro enables users to set specific values for the four generic type parameters of
/// [`Graph`], namely:
/// * `EdgeIdType` -- this type is used to uniquely identify the edges of a graph.
/// * `EdgeWeightType` -- this type is used as a weight of each edge. This is not necessarily a
/// number or even a primitive type: it can a struct, an enum, and many other things.
/// * `VertexIdType` -- this type is used to uniquely identify the vertices of a graph.
/// * `VertexWeightType` -- this type is used as a weight of each vertex. Just as `EdgeWeightType`,
/// this is not necessarily a number or even a primitive type: it can a struct, an enum, and many
/// other things.
///
/// The remaining generic type parameter of [`Graph`], `LocaleType`, is used to determine the
/// category of the graph and is encoded graphically to make your code shorter and easier to read.
/// Specifically, use the following patterns for this macro for each category:
/// * Undirected simple graph
/// ```ignore
/// graph!{ [VertexIdType|VertexWeightType] ---[EdgeIdType|EdgeWeightType]--- }
/// ```
/// * Directed simple graph
/// ```ignore
/// graph!{ [VertexIdType|VertexWeightType] ---[EdgeIdType|EdgeWeightType]--> }
/// ```
/// * Undirected multi-graph
/// ```ignore
/// graph!{ [VertexIdType|VertexWeightType] ===[EdgeIdType|EdgeWeightType]=== }
/// ```
/// * Directed multi-graph
/// ```ignore
/// graph!{ [VertexIdType|VertexWeightType] ===[EdgeIdType|EdgeWeightType]==> }
/// ```
///
/// With this macro, we aimed to create a clear and easily interpretable visual representation of a
/// graph.
/// The left block `[...|...]` represents a vertex, whereas the right block `---[...|...]---`,
/// `---[...|...]-->`, `===[...|...]===` or `===[...|...]==>` represents edges.
/// The symbol `>` on the right denotes an arrowhead, which appears only on directed edges, hence,
/// all patterns with an arrowhead denote directed graph categories.
/// The equality signs `=`, on the other hand, indicate parallel edges, a defining characteristic
/// of multigraphs, hence, all patterns with equality signs denote multigraphs.
#[macro_export]
macro_rules! graph {
    ([$vertex_id_type:ty|$vertex_weight_type:ty] ---[$edge_id_type:ty|$edge_weight_type:ty]---) => {
        Graph<$edge_id_type, $vertex_id_type, UndirectedSimpleLocale<$edge_id_type>, $edge_weight_type, $vertex_weight_type>
    };
}










pub struct ImmutableEdgeSet<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    g: &'a Graph<EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>,
}



impl<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> ImmutableEdgeSet<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    /// # Check if an edge exists
    ///
    /// Checks whether the edge with ID `eid` exists in the graph.
    #[inline(always)]
    pub fn contains(&self, eid: EdgeIdType) -> bool {
        self.g.edges_dir.contains_key(&eid) || self.g.edges_undir.contains_key(&eid)
    }

    /// # Count edges
    ///
    /// Returns the number of edges with the certain directedness in the graph.
    #[inline(always)]
    pub fn count(&self, dirsel: AbsoluteEdgeDirectionSelector) -> usize {
        match dirsel {
            AbsoluteEdgeDirectionSelector::Any => self.g.edges_dir.len() + self.g.edges_undir.len(),
            AbsoluteEdgeDirectionSelector::Directed => self.g.edges_dir.len(),
            AbsoluteEdgeDirectionSelector::Undirected => self.g.edges_undir.len(),
        }
    }

    /// # Count edges between specific vertices
    ///
    /// Returns the number of edges between the vertices with IDs `vid1` and `vid2`.
    /// See [`RelativeEdgeDirectionSelector`] for the description of `reldirsel`.
    /// Returns `Ok(num)` if successful, where `num` is the requested number of edges.
    /// Returns `Err(anyhow::Error(...))` if the vertex with ID `vid1` or `vid2` doesn't exist.
    pub fn count_between(&self, vid1: VertexIdType, vid2: VertexIdType, reldirsel: RelativeEdgeDirectionSelector) -> Result<usize> {
        if !self.g.vertices.contains_key(&vid1) {
            return Err(anyhow!("The number of edges between the vertices with IDs {:?} and {:?} can't be counted because the vertex with ID {:?} doesn't exist.", vid1, vid2, vid1));
        }
        if !self.g.vertices.contains_key(&vid2) {
            return Err(anyhow!("The number of edges between the vertices with IDs {:?} and {:?} can't be counted because the vertex with ID {:?} doesn't exist.", vid1, vid2, vid2));
        }
        let mut num;
        match reldirsel {
            RelativeEdgeDirectionSelector::Any => {
                num = self.g.edges_dir.iter().filter(|(_, e)|
                    (*e).vid1 == vid1 && (*e).vid2 == vid2 || (*e).vid1 == vid2 && (*e).vid2 == vid1
                ).count();
                let (vid1, vid2) = if vid1 <= vid2 { (vid1, vid2) } else { (vid2, vid1) };
                num += self.g.edges_undir.iter().filter(|(_, e)|
                    (*e).vid1 == vid1 && (*e).vid2 == vid2
                ).count();
            },
            RelativeEdgeDirectionSelector::AnyDirected => {
                num = self.g.edges_dir.iter().filter(|(_, e)|
                    (*e).vid1 == vid1 && (*e).vid2 == vid2 || (*e).vid1 == vid2 && (*e).vid2 == vid1
                ).count();
            },
            RelativeEdgeDirectionSelector::DirectedFrom => {
                num = self.g.edges_dir.iter().filter(|(_, e)|
                    (*e).vid1 == vid1 && (*e).vid2 == vid2
                ).count();
            },
            RelativeEdgeDirectionSelector::DirectedTo => {
                num = self.g.edges_dir.iter().filter(|(_, e)|
                    (*e).vid1 == vid2 && (*e).vid2 == vid1
                ).count();
            },
            RelativeEdgeDirectionSelector::Undirected => {
                let (vid1, vid2) = if vid1 <= vid2 { (vid1, vid2) } else { (vid2, vid1) };
                num = self.g.edges_undir.iter().filter(|(_, e)|
                    (*e).vid1 == vid1 && (*e).vid2 == vid2
                ).count();
            },
        }
        Ok(num)
    }
}










pub struct MutableEdgeSet<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    g: &'a mut Graph<EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>,
}



impl<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> MutableEdgeSet<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    /// # Insert a new edge
    ///
    /// Adds a new edge with weight `eweight` between vertices with IDs `vid1` and `vid2`.
    /// If `eid_option == Some(eid)`, then the new edge will have the ID `eid`.
    /// If `eid_option == None`, the ID of the new edge will be selected automatically.
    /// If the graph is directed and `dir == AbsoluteEdgeDirection::Directed`, then the new edge
    /// will be directed from `vid1` to `vid2`.
    /// If the graph is undirected, the value of `dir` is ignored and the new edge will always
    /// be undirected.
    /// Returns `Ok(eid)` if successful, where `eid` is the ID of the new edge.
    /// Returns `Err(anyhow::Error(...))` if the addition of the new edge would lead to any of the
    /// following:
    /// * _Duplicate edge IDs._ `eid_option == Some(eid)` **and** there's already the edge with ID
    /// `eid` in the graph.
    /// * _Incidence on a non-existent vertex._ Vertex with ID `vid1` or `vid2` doesn't exist.
    /// * _Parallel edge in a simple graph (1)._ The graph is undirected **and** the graph is
    /// simple **and** there's already an edge between `vid1` and `vid2`.
    /// * _Parallel edge in a simple graph (2)._ The graph is directed **and** the graph is simple
    /// **and** there's already an edge between `vid1` to `vid2` of the same orientation as the new
    /// edge.
    pub fn insert(&mut self, eid_option: Option<EdgeIdType>, vid1: VertexIdType, vid2: VertexIdType, dir: AbsoluteEdgeDirection, eweight: EdgeWeightType) -> Result<EdgeIdType> {
        if let Some(eid) = eid_option && (self.g.edges_dir.contains_key(&eid) || self.g.edges_undir.contains_key(&eid)) {
            return Err(anyhow!("The edge with ID {:?} can't be added because the edge with this ID already exists in the graph.", eid));
        }
        if !self.g.vertices.contains_key(&vid1) {
            return Err(anyhow!("A new edge between the vertices with IDs {:?} and {:?} can't be added because the vertex with ID {:?} doesn't exist.", vid1, vid2, vid1));
        }
        if !self.g.vertices.contains_key(&vid2) {
            return Err(anyhow!("A new edge between the vertices with IDs {:?} and {:?} can't be added because the vertex with ID {:?} doesn't exist.", vid1, vid2, vid2));
        }
        let assertion_reldirsel = match dir {
            AbsoluteEdgeDirection::Directed => if LocaleType::IS_DIRECTED {
                RelativeEdgeDirectionSelector::DirectedFrom
            } else {
                RelativeEdgeDirectionSelector::Undirected
            },
            AbsoluteEdgeDirection::Undirected => RelativeEdgeDirectionSelector::Undirected,
        };
        if LocaleType::IS_SIMPLE && self.g.e().count_between(vid1, vid2, assertion_reldirsel).unwrap() > 0 {
            return Err(anyhow!("A new edge between the vertices with IDs {:?} and {:?} can't be added because an edge between them going in the same direction already exists and the graph is simple.", vid1, vid2));
        }
        let eid = match eid_option {
            Some(value) => value,
            None => {
                let value = self.g.next_free_edge_id;
                self.g.next_free_edge_id = self.g.next_free_edge_id.increment();
                while self.g.e().contains(value) {
                    self.g.next_free_edge_id = self.g.next_free_edge_id.increment();
                }
                value
            },
        };
        let reldir = match dir {
            AbsoluteEdgeDirection::Directed => RelativeEdgeDirection::DirectedFrom,
            AbsoluteEdgeDirection::Undirected => RelativeEdgeDirection::Undirected,
        };
        self.g.vertices.get_mut(&vid1).unwrap().locale.insert(eid, reldir);
        let reldir = match dir {
            AbsoluteEdgeDirection::Directed => RelativeEdgeDirection::DirectedTo,
            AbsoluteEdgeDirection::Undirected => RelativeEdgeDirection::Undirected,
        };
        self.g.vertices.get_mut(&vid2).unwrap().locale.insert(eid, reldir);
        match dir {
            AbsoluteEdgeDirection::Directed => self.g.edges_dir.insert(eid, Edge { vid1, vid2, weight: eweight }),
            AbsoluteEdgeDirection::Undirected => {
                let (vid1, vid2) = if vid1 <= vid2 { (vid1, vid2) } else { (vid2, vid1) };
                self.g.edges_undir.insert(eid, Edge { vid1, vid2, weight: eweight })
            },
        };
        Ok(eid)
    }

    /// # Remove an existing edge
    ///
    /// Deletes the edge with ID `eid` from the graph.
    /// Returns `Some((vid1, vid2, dir, eweight))` if the edge existed in the graph, where `vid1`
    /// and `vid2` are the IDs of the vertices that the edge was incident on, `dir` is the
    /// directedness of the edge, `eweight` is its weight.
    /// If `dir == AbsoluteEdgeDirection::Directed`, then the edge was directed from `vid1` to
    /// `vid2`.
    /// Returns `None` otherwise.
    pub fn remove(&mut self, eid: EdgeIdType) -> Option<(VertexIdType, VertexIdType, AbsoluteEdgeDirection, EdgeWeightType)> {
        let (removed_edge, dir) = if self.g.edges_dir.get(&eid).is_some() {
            (self.g.edges_dir.remove(&eid).unwrap(), AbsoluteEdgeDirection::Directed)
        } else if self.g.edges_undir.get(&eid).is_some() {
            (self.g.edges_undir.remove(&eid).unwrap(), AbsoluteEdgeDirection::Undirected)
        } else {
            return None;
        };
        self.g.vertices.get_mut(&removed_edge.vid1).unwrap().locale.remove(&eid);
        self.g.vertices.get_mut(&removed_edge.vid2).unwrap().locale.remove(&eid);
        self.g.next_free_edge_id = if eid < self.g.next_free_edge_id { eid } else { self.g.next_free_edge_id };
        Some((removed_edge.vid1, removed_edge.vid2, dir, removed_edge.weight))
    }
}










pub struct ImmutableVertexSet<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    g: &'a Graph<EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>,
}



impl<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> ImmutableVertexSet<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    /// # Check if a vertex exists
    ///
    /// Checks whether the vertex with ID `vid` exists in the graph.
    #[inline(always)]
    pub fn contains(&self, vid: VertexIdType) -> bool {
        self.g.vertices.contains_key(&vid)
    }

    /// # Count vertices
    ///
    /// Returns the number of vertices in the graph.
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.g.vertices.len()
    }

    /// # Get vertex weight
    ///
    /// Returns `Some(&vweight)`, where `vweight` is the weight of the vertex with ID `vid`.
    /// Returns `None` if vertex `vid` doesn't exist.
    #[inline(always)]
    pub fn get(&self, vid: VertexIdType) -> Option<&'a VertexWeightType> {
        Some(&self.g.vertices.get(&vid)?.weight)
    }

    /// # Iterate over vertices
    ///
    /// Returns an iterator over all vertices in the graph.
    /// The iterator produces items `(vid, &vweight)`, where `vid` is the ID of a vertex in the
    /// graph and `vweight` is its weight.
    ///
    /// > **⚠ Lack-of-guarantee warning**
    /// >
    /// > This function does **not** guarantee any specific order of vertices.
    ///
    /// > **⚠ Repeatability warning**
    /// >
    /// > This function is **not** repeatable, i.e. if you run your program on the same machine
    /// > multiple times, corresponding calls to this function may return iterators that produce
    /// > different sequences of vertices.
    #[inline(always)]
    pub fn iter(&self) -> Box<dyn Iterator<Item = (VertexIdType, &'a VertexWeightType)> + 'a> {
        Box::new(self.g.vertices.iter().map(|(vid, v)| (*vid, &v.weight)))
    }

    /// # Iterate over vertices adjacent to the given vertex
    ///
    /// Constructs an iterator over all vertices adjacent to the vertex with ID `vid`.
    /// Depending on the value of `reldirsel`, the iterator will iterate over all vertices `adjvid`
    /// that are connected with `vid` via at least one edge of the specified relative direction:
    /// * `reldirsel == RelativeEdgeDirectionSelector::Any` -- iterator will iterate over all
    /// vertices that are connected with `vid` via any edge.
    /// * `reldirsel == RelativeEdgeDirectionSelector::AnyDirected` -- iterator will iterate over
    /// all vertices that are connected with `vid` via at least one directed edge,
    /// whether this edge is directed from `vid` to `adjvid` or from `adjvid` to `vid`.
    /// * `reldirsel == RelativeEdgeDirectionSelector::DirectedFrom` -- iterator will iterate over
    /// all vertices that are connected with `vid` via at least one edge directed from
    /// `vid` to `adjvid`.
    /// * `reldirsel == RelativeEdgeDirectionSelector::DirectedTo` -- iterator will iterate over
    /// all vertices that are connected with `vid` via at least one edge directed from
    /// `adjvid` to `vid`.
    /// * `reldirsel == RelativeEdgeDirectionSelector::Undirected` -- iterator will iterate over
    /// all vertices that are connected with `vid` via at least one undirected edge.
    ///
    /// Returns `Ok(it)`, if successful.
    /// Here, `it` is an iterator that produces items `(adjvid, &adjvweight)`, where `adjvid` is
    /// the ID of an adjacent vertex and `adjvweight` is its weight.
    /// Returns `Err(anyhow::Error(...))` if the vertex with ID `vid` doesn't exist.
    ///
    /// > **⚠ Lack-of-guarantee warning**
    /// >
    /// > This function does **not** guarantee any specific order of vertices.
    ///
    /// > **⚠ Repeatability warning**
    /// >
    /// > This function is **not** repeatable, i.e. if you run your program on the same machine
    /// > multiple times, corresponding calls to this function may return iterators that produce
    /// > different sequences of vertices.
    pub fn iter_adjacent(&self, vid: VertexIdType, reldirsel: RelativeEdgeDirectionSelector) -> Result<AdjacentVerticesIter<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>> {
        AdjacentVerticesIter::new(self.g, vid, reldirsel)
    }

    /// # Iterate over vertices in a DFS preorder
    ///
    /// Constructs an iterator over vertices reachable from the vertex with ID `vid` via a
    /// depth-first search.
    /// The iterator will visit the vertices in the DFS preorder.
    /// This is useful for systematic graph traversals.
    /// For example, for the following graph:
    /// ```ignore
    ///     A    B    C
    ///      ●---●-->●
    ///      |       ↑
    ///    D ●-------● E
    ///      |       |
    ///      ●<------●
    ///     F         G
    /// ```
    /// a possible DFS preorder of vertices with `vid = 'A'` is `A`, `D`, `F`, `E`, `C`, `G`, `B`.
    /// On the other hand, with `vid = 'C'`, the only possible DFS preorder is `C` because all
    /// other vertices are not reachable from `C` via a DFS.
    ///
    /// Returns `Ok(it)`, if successful.
    /// Here, `it` is an iterator that produces items `(vid, &vweight)`, where `vid` is the ID of a
    /// vertex and `vweight` is its weight.
    /// Returns `Err(anyhow::Error(...))` if the vertex with ID `vid` doesn't exist.
    ///
    /// > **⚠ Repeatability warning**
    /// >
    /// > This function is **not** repeatable, i.e. if you run your program on the same machine
    /// > multiple times, corresponding calls to this function may return iterators that produce
    /// > different sequences of vertices.
    pub fn iter_in_dfs_preorder(&self, vid: VertexIdType) -> Result<DFSPreorderIter<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>> {
        DFSPreorderIter::new(self.g, vid)
    }
}










pub struct MutableVertexSet<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    g: &'a mut Graph<EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>,
}



impl<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> MutableVertexSet<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    /// # Insert a new vertex
    ///
    /// Adds a new vertex with weight `vweight` to the graph.
    /// If `vid_option == Some(vid)`, then the new vertex will have the ID `vid`.
    /// If `vid_option == None`, the ID of the new vertex will be selected automatically.
    /// Returns `Ok(vid)` if successful, where `vid` is the ID of the new vertex.
    /// Returns `Err(anyhow::Err(...))` if the vertex with ID `vid` already exists.
    pub fn insert(&mut self, vid_option: Option<VertexIdType>, vweight: VertexWeightType) -> Result<VertexIdType> {
        if let Some(vid) = vid_option && self.g.vertices.contains_key(&vid) {
            return Err(anyhow!("The vertex with ID {:?} can't be added because the vertex with this ID already exists in the graph.", vid));
        }
        let vid = match vid_option {
            Some(value) => value,
            None => {
                let value = self.g.next_free_vertex_id;
                self.g.next_free_vertex_id = self.g.next_free_vertex_id.increment();
                while self.g.vertices.contains_key(&value) {
                    self.g.next_free_vertex_id = self.g.next_free_vertex_id.increment();
                }
                value
            },
        };
        self.g.vertices.insert(vid, Vertex { locale: Locale::new(), weight: vweight, _phantom: PhantomData });
        Ok(vid)
    }

    /// # Remove an existing vertex
    ///
    /// Deletes the vertex with ID `vid` from the graph and all the edges incident on it.
    /// Returns `Some(vweight, es)` if the vertex existed in the graph, where `vweight` is the
    /// weight of the deleted vertex, `es` is the vector of tuples `(vid1, vid2, dir, eweight)`
    /// describing the deleted incident edges
    /// (see [MutableEdgeSet::remove] for the description of `vid1`, `vid2`, `dir` and `eweight`).
    /// Returns `None` otherwise.
    pub fn remove(&mut self, vid: VertexIdType) -> Option<(VertexWeightType, Vec<(VertexIdType, VertexIdType, AbsoluteEdgeDirection, EdgeWeightType)>)> {
        if !self.g.vertices.contains_key(&vid) {
            return None;
        }
        let mut es = Vec::with_capacity(self.g.vertices[&vid].locale.count_all());
        for (eid, _) in self.g.vertices[&vid].locale.iter_all().collect::<Vec<_>>() {
            es.push(self.g.e_mut().remove(eid).unwrap());
        }
        Some((self.g.vertices.remove(&vid).unwrap().weight, es))
    }
}
