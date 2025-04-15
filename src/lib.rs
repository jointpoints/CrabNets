//! # Connecto.rs
//! 
//! <div style="text-align: center; font-weight: bold; font-style: italic;">
//! Fast and flexible Rust crate for graph manipulation
//! </div>
//! 
//! ```
//! cargo add connecto_rs
//! ```
//! Run this command to add Connecto.rs to your Rust project!
//! 
//! ## Basic example
//! ```
//! /* In this example, we create the following directed graph with u8 edge weights:
//!  * 
//!  *     A           B
//!  *      ●────5────●
//!  *      ↑         │
//!  *      2         4
//!  *      │         ↓
//!  *      ●────3────●
//!  *     C           D
//!  * 
//!  * Then, we find the shortest path from A to C.
//!  */
//! use connecto_rs::essentials::*;
//! 
//! fn main() {
//!     // A new graph with directed edges
//!     let mut g: graph!{[&str] ---[u8]-->} = Graph::null();
//!     // Add 4 vertices with IDs A, B, C, D
//!     g.v_mut().add_from_iter(["A", "B", "C", "D"].into_iter());
//!     assert_eq!(g.v().count(), 4);
//!     // Add edges
//!     g.e_mut().add_from_iter([
//!         (0, 1, 5, false),
//!         (2, 0, 2, true),
//!         (1, 3, 4, true),
//!         (2, 3, 3, false)
//!     ].into_iter());
//!     assert_eq!(g.e().count(), 4);
//!     // Find the shortest path
//! }
//! ```
//! 
//! ## Getting started
//! Follow these steps to learn the basics of Connecto.rs:
//! 1. [`Graph`] -- the king structure of Connecto.rs that you will always interact with.
//! Represents a graph.
//! 2. [`graph!`][step2] -- the easy way to succinctly declare different graph types.
//! 3. <span style="color: red;">Something about I/O?</span>
//! 
//! [step2]: graph!
pub mod locales;

use std::{collections::HashMap, error::Error, fmt::Display, marker::PhantomData};
use anyhow::{Ok, Result};










#[derive(Debug)]
pub struct ConnectorsError {
    message: String,
}



impl ConnectorsError {
    fn new(with_message: String) -> ConnectorsError {
        ConnectorsError { message: with_message }
    }
}



impl Display for ConnectorsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}



impl Error for ConnectorsError {}



#[macro_export]
macro_rules! connecto_rs_error {
    ($content: expr) => {
        Err(ConnectorsError::new($content).into())
    };
}










pub type Id = usize;










/// # Locale trait
/// 
/// ## Description
/// **Locale** stores data about a specific vertex and metadata about its 'surroundings', e.g. the
/// IDs of incident edges, the IDs of adjacent vertices, etc.
/// 
/// The properties of a graph, such as whether it can have parallel edges or directed edges, are
/// determined by the characteristics of its locales.
/// Thus, if you wish to develop your own type of graphs, e.g. hypergraphs, perhaps, it's going to
/// suffice to simply implement a new Connecto.rs locale type.
/// 
/// ## Connecto.rs standard locale types
/// See the [`locales`] module for the implementations of several standard locale types.
pub trait Locale<VertexType>
{
    /// # Deregister edge
    /// 
    /// ## Description
    /// Deregisters edge with the given ID from the locale if it is registered in it.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.e_mut().remove()`][remove]
    /// * [`g.e_mut().remove_from_iter()`][removefromiter]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [remove]: MutableEdgeView::remove
    /// [removefromiter]: MutableEdgeView::remove_from_iter
    fn deregister_edge(&mut self, id: Id);



    /// # Expose immutable reference to vertex
    /// 
    /// ## Description
    /// Returns an immutable reference to the vertex stored inside the locale.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.v().get()`][get]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [get]: ImmutableVertexView::get
    fn expose(&self) -> &VertexType;



    /// # Expose mutable reference to vertex
    /// 
    /// ## Description
    /// Returns a mutable reference to the vertex stored inside the locale.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.v_mut().get()`][get]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [get]: MutableVertexView::get
    fn expose_mut(&mut self) -> &mut VertexType;



    /// # Is edge registered?
    /// 
    /// ## Description
    /// Checks if an edge with the given ID is registered in the locale.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.e().is_incident()`][isinc]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [isinc]: ImmutableEdgeView::is_incident
    fn is_registered(&self, eid: Id) -> bool;



    /// # Is edge registered as ingoing?
    /// 
    /// ## Description
    /// Checks if an edge with the given ID is registered in the locale as ingoing.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.e().is_incident_in()`][isincin]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [isincin]: ImmutableEdgeView::is_incident_in
    fn is_registered_in(&self, eid: Id) -> bool;



    /// # Is edge registered as outgoing?
    /// 
    /// ## Description
    /// Checks if an edge with the given ID is registered in the locale as outgoing.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.e().is_incident_out()`][isincout]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [isincout]: ImmutableEdgeView::is_incident_out
    fn is_registered_out(&self, eid: Id) -> bool;



    /// # Iterate over incident edges
    /// 
    /// ## Description
    /// Returns an iterator over tuples (adjacent vertex ID, corresponding edge ID).
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.e().iter_incident()`][iterinc]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [iterinc]: ImmutableEdgeView::iter_incident
    fn iter_incident<'a>(&'a self) -> Box<dyn Iterator<Item = (Id, Id)> + 'a>;



    /// # Iterate over ingoing incident edges
    /// 
    /// ## Description
    /// Returns an iterator over tuples (adjacent vertex ID, corresponding edge ID) where each edge
    /// produced by an iterator is directed and the vertex from this locale is their target vertex.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.e().iter_incident_in()`][iterincin]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [iterincin]: ImmutableEdgeView::iter_incident_in
    fn iter_incident_in<'a>(&'a self) -> Box<dyn Iterator<Item = (Id, Id)> + 'a>;



    /// # Iterate over outgoing incident edges
    /// 
    /// ## Description
    /// Returns an iterator over tuples (adjacent vertex ID, corresponding edge ID) where each edge
    /// produced by an iterator is directed and the vertex from this locale is their source vertex.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.e().iter_incident_out()`][iterincout]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [iterincout]: ImmutableEdgeView::iter_incident_out
    fn iter_incident_out<'a>(&'a self) -> Box<dyn Iterator<Item = (Id, Id)> + 'a>;



    /// # Iterate over undirected incident edges
    /// 
    /// ## Description
    /// Returns an iterator over tuples (adjacent vertex ID, corresponding edge ID) where each edge
    /// produced by an iterator is undirected.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.e().iter_incident_undir()`][iterincundir]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [iterincundir]: ImmutableEdgeView::iter_incident_undir
    fn iter_incident_undir<'a>(&'a self) -> Box<dyn Iterator<Item = (Id, Id)> + 'a>;



    /// # Leak vertex
    /// 
    /// ## Description
    /// Consumes the locale and returns the vertex stored in it.
    /// The locale is destroyed after the execution of this function.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    fn leak(self) -> VertexType;



    /// # Register new edge
    /// 
    /// ## Description
    /// Registers information about a new edge in the locale.
    /// If the addition of a new edge would violate the policy of a specific locate type, returns
    /// `Err(ConnectorsError)`.
    /// Otherwise, returns `Ok(())`.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.e_mut().add()`][add]
    /// * [`g.e_mut().add_from_iter()`][addfromiter]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [add]: MutableEdgeView::add
    /// [addfromiter]: MutableEdgeView::add_from_iter
    fn register_edge(&mut self, eid: Id, vid: Id, directed: bool) -> Result<()>;



    /// # Replace vertex
    /// 
    /// ## Description
    /// Replaces the vertex stored in the locale with a new one.
    /// Returns the old vertex that is being replaced.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.v_mut().add()`][add]
    /// * [`g.v_mut().add_from_iter()`][addfromiter]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [add]: MutableVertexView::add
    /// [addfromiter]: MutableVertexView::add_from_iter
    fn replace_vertex(&mut self, new_vertex: VertexType) -> VertexType;



    /// # Create new locale with given vertex
    /// 
    /// ## Description
    /// Creates a new locale for the given vertex.
    /// The ownership over the vertex is passed to the new locale.
    /// 
    /// ## ⚠ This is an under-the-hood function
    /// This function is **not** supposed to be called directly by the end user.
    /// It's typically called from the following user-level funtions:
    /// * [`g.v_mut().add()`][add]
    /// * [`g.v_mut().add_from_iter()`][addfromiter]
    /// 
    /// Unless you try to implement your own locale type, consider calling one of the
    /// functions above.
    /// 
    /// [add]: MutableVertexView::add
    /// [addfromiter]: MutableVertexView::add_from_iter
    fn with_vertex(vertex: VertexType) -> Self;
}










/// # Vertex view (immutable)
/// 
/// ## Description
/// This structure provides you with an interface to interact with the vertices of a graph.
/// Note that it only provides methods that do not require the mutability of the graph.
/// They include, e.g. look-up and counting vertices.
/// 
/// An immutable vertex view of a graph is typically accessed with method [`g.v()`][v].
/// 
/// ## See also
/// * [`MutableVertexView`] -- Access this structure for a vertex interface with mutable
/// methods.
/// 
/// [v]: Graph::v
pub struct ImmutableVertexView<'a, EdgeType, LocaleType, VertexType>
where
    LocaleType: Locale<VertexType>,
{
    g: &'a Graph<EdgeType, LocaleType, VertexType>,
}



impl<'a, EdgeType, LocaleType, VertexType> ImmutableVertexView<'a, EdgeType, LocaleType, VertexType>
where
    LocaleType: Locale<VertexType>,
{
    /// # Check whether vertex with given ID exists
    /// 
    /// ## Description
    /// Returns `true` if a vertex with the given ID is contained in the graph, or `false`
    /// otherwise.
    #[inline]
    pub fn contains(&self, id: Id) -> bool {
        self.g.locales.contains_key(&id)
    }



    /// # Number of vertices
    /// 
    /// ## Description
    /// Returns the number of vertices in the graph.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[u8] ---[()]---} = Graph::null();
    /// g.v_mut().add_from_iter([1, 2, 4].into_iter());
    /// assert_eq!(g.v().count(), 3);
    /// ```
    #[inline]
    pub fn count(&self) -> usize {
        self.g.locales.len()
    }



    /// # Get immutable vertex reference by ID
    /// 
    /// ## Description
    /// Returns `Some(&vertex)` if a vertex with the given ID exists, or `None` otherwise.
    #[inline]
    pub fn get(&self, id: Id) -> Option<&VertexType> {
        match self.g.locales.get(&id) {
            Some(locale) => Some(locale.expose()),
            None => None,
        }
    }
}





/// # Vertex view (mutable)
/// 
/// ## Description
/// This structure provides you with an interface to interact with the vertices of a graph.
/// Note that it only provides methods that require the mutability of the graph.
/// They include, e.g. addition or removal of vertices.
/// 
/// A mutable vertex view of a graph is typically accessed with method [`g.v_mut()`][vmut].
/// 
/// ## See also
/// * [`ImmutableVertexView`] -- Access this structure for a vertex interface with immutable
/// methods.
/// 
/// [vmut]: Graph::v_mut
pub struct MutableVertexView<'a, EdgeType, LocaleType, VertexType>
where
    LocaleType: Locale<VertexType>,
{
    g: &'a mut Graph<EdgeType, LocaleType, VertexType>,
}



impl<'a, EdgeType, LocaleType, VertexType> MutableVertexView<'a, EdgeType, LocaleType, VertexType>
where
    LocaleType: Locale<VertexType>,
{
    /// # Add new vertex
    /// 
    /// ## Description
    /// Adds a new vertex to the graph and returns its ID.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[String] ---[()]---} = Graph::null();
    /// assert_eq!(g.v_mut().add("test vertex 1".to_string()), 0);
    /// assert_eq!(g.v_mut().add("test vertex 2".to_string()), 1);
    /// ```
    /// 
    /// ## See also
    /// * [`g.v_mut().add_from_iter()`][addfromiter] -- Add vertices from a vertex iterator.
    /// 
    /// [addfromiter]: MutableVertexView::add_from_iter
    pub fn add(&mut self, new_vertex: VertexType) -> Id {
        let new_vertex_id= self.g.next_vertex_id;
        match self.g.locales.get_mut(&new_vertex_id) {
            Some(locale) => {locale.replace_vertex(new_vertex);},
            None => {self.g.locales.insert(new_vertex_id, Locale::with_vertex(new_vertex));},
        }
        while self.g.locales.contains_key(&self.g.next_vertex_id) {
            self.g.next_vertex_id += 1;
        }
        return new_vertex_id;
    }



    /// # Add new vertices from iterator
    /// 
    /// ## Description
    /// Adds new vertices from the given data iterator.
    /// The iterator must produce objects of `VertexType`.
    /// Returns a vector of IDs of the added vertices.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[bool] ---[()]---} = Graph::null();
    /// let my_vertices = [true, true, false, false];
    /// g.v_mut().add_from_iter(my_vertices.into_iter());
    /// assert_eq!(g.v().count(), 4);
    /// ```
    /// 
    /// ## See also
    /// * [`g.v_mut().add()`][add] -- Add a single new vertex.
    /// 
    /// [add]: MutableVertexView::add
    pub fn add_from_iter<VertexDataIter: Iterator<Item = VertexType>>(&mut self, data: VertexDataIter) -> Vec<Id> {
        let new_vertices: Vec<_> = data.collect();
        let mut new_vertices_ids = Vec::with_capacity(new_vertices.len());
        self.g.locales.reserve(new_vertices.len());
        for new_vertex in new_vertices {
            new_vertices_ids.push(self.add(new_vertex));
        }
        new_vertices_ids
    }



    /// # Get mutable vertex reference by ID
    /// 
    /// ## Description
    /// Returns `Some(&mut vertex)` if a vertex with the given ID exists, or `None` otherwise.
    pub fn get(&mut self, vid: Id) -> Option<&mut VertexType> {
        match self.g.locales.get_mut(&vid) {
            Some(locale) => Some(locale.expose_mut()),
            None => None,
        }
    }



    /// # Merge vertices
    /// 
    /// ## Description
    /// Merges vertex with ID `vid2` with the vertex with ID `vid1`.
    /// 
    /// Vertex `vid2` will be removed from the graph and all edges that are incident on vertex
    /// `vid2` will become incident on vertex `vid1`.
    /// If both `vid1` and `vid2` are adjacent to some vertex $i$, then:
    /// * If the graph is a multigraph, all edges that are incident on `vid2` will become incident
    /// on `vid1`.
    /// * If the graph is simple, edges incident on `vid1` will have priority, i.e. the edge
    /// between `vid1` and $i$ will be kept, whereas the edge between `vid2` and $i$ will be
    /// removed.
    /// 
    /// If `keep_edges_as_loops` is set to `true`, all edges between `vid1` and `vid2` will become
    /// loops for `vid1`.
    /// If it's set to `false`, these edges will be removed.
    /// 
    /// Returns `Err(ConnectorsError)` if at least one of the vertices doesn't exist.
    /// Otherwise, returns `Ok(())`.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[()] ===[u8]==>} = Graph::null();
    /// g.v_mut().add_from_iter([(), ()].into_iter());
    /// g.e_mut().add_from_iter([
    ///     (0, 1, 10, true),
    ///     (1, 0, 20, true),
    /// ].into_iter());
    /// assert!(g.v_mut().merge(0, 1, true).is_ok());
    /// assert_eq!(g.v().count(), 1);
    /// assert_eq!(g.e().count(), 2);
    /// ```
    pub fn merge(&mut self, vid1: Id, vid2: Id, keep_edges_as_loops: bool) -> Result<()> {
        for vid in [vid1, vid2] {
            if !self.g.v().contains(vid) {
                return connecto_rs_error!(format!("Cannot merge vertices {} and {} because vertex {} doesn't exist.", vid1, vid2, vid));
            }
        }
        todo!();
        Ok(())
    }



    /// # Remove vertex
    /// 
    /// ## Description
    /// Removes a vertex with the given ID.
    /// If the vertex exists, returns `Some(deleted_vertex)`.
    /// If the vertex doesn't exist, returns `None`.
    /// All edges incident on the deleted vertex will be silently deleted.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[()] ---[()]---} = Graph::null();
    /// g.v_mut().add(());
    /// g.v_mut().add(());
    /// g.e_mut().add(0, 1, (), ());
    /// assert!(g.v_mut().remove(0).is_some());
    /// assert!(g.v_mut().remove(2).is_none());
    /// assert_eq!(g.v().count(), 1);
    /// assert_eq!(g.e().count(), 0);
    /// ```
    /// 
    /// ## See also
    /// * [`g.v_mut().remove_from_iter`][removefromiter] -- Removes vertices from a vertex ID
    /// iterator.
    /// 
    /// [removefromiter]: MutableVertexView::remove_from_iter
    pub fn remove(&mut self, id: Id) -> Option<VertexType> {
        match self.g.locales.remove(&id) {
            Some(locale) => {
                self.g.next_vertex_id = id;
                for (_, eid) in locale.iter_incident() {
                    self.g.edges.remove(&eid);
                }
                Some(locale.leak())
            },
            None => None
        }
    }



    /// # Remove vertices from iterator
    /// 
    /// ## Description
    /// Removes vertices with IDs from the given data iterator.
    /// The iterator must produce vertex IDs.
    /// Returns a vector of `Option<VertexType>`, where each value is either `Some(old_vertex)` if
    /// a vetrex with the corresponding ID exists, or `None` otherwise.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[()] ---[()]---} = Graph::null();
    /// g.v_mut().add_from_iter([(), (), ()].into_iter());
    /// g.v_mut().remove_from_iter([0, 1, 4].into_iter());
    /// assert_eq!(g.v().count(), 1);
    /// ```
    /// 
    /// ## See also
    /// * [`g.v_mut().remove`][remove] -- Removes a single vertex.
    /// 
    /// [remove]: MutableVertexView::remove
    pub fn remove_from_iter<VertexDataIter: Iterator<Item = Id>>(&mut self, data: VertexDataIter) {
        for id in data {
            self.remove(id);
        }
    }
}










/// # Edge view (immutable)
/// 
/// ## Description
/// This structure provides you with an interface to interact with the edges of a graph.
/// Note that it only provides methods that do not require the mutability of the graph.
/// They include, e.g. look-up and counting edges.
/// 
/// An immutable edge view of a graph is typically accessed with method [`g.e()`][e].
/// 
/// ## See also
/// * [`MutableEdgeView`] -- Access this structure for an edge interface with mutable
/// methods.
/// 
/// [e]: Graph::e
pub struct ImmutableEdgeView<'a, EdgeType, LocaleType, VertexType>
where
    LocaleType: Locale<VertexType>,
{
    g: &'a Graph<EdgeType, LocaleType, VertexType>,
}



impl<'a, EdgeType, LocaleType, VertexType> ImmutableEdgeView<'a, EdgeType, LocaleType, VertexType>
where
    LocaleType: Locale<VertexType>,
{
    /// # Number of edges
    /// 
    /// ## Description
    /// Returns the number of edges in the graph.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[u8] ---[bool]---} = Graph::null();
    /// g.v_mut().add_from_iter([1, 2, 4].into_iter());
    /// g.e_mut().add_from_iter([
    ///     (0, 1, true, false),
    ///     (1, 2, false, false),
    ///     (2, 0, true, false),
    /// ].into_iter());
    /// assert_eq!(g.e().count(), 3);
    /// ```
    #[inline]
    pub fn count(&self) -> usize {
        self.g.edges.len()
    }



    /// # Iterate over edges
    /// 
    /// ## Description
    /// Iterates over all edges in the graph.
    /// Returns an iterator producing tuples (edge ID, vertex ID 1, vertex ID 2, &edge).
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[u8] ---[bool]---} = Graph::null();
    /// g.v_mut().add_from_iter([1, 2, 4].into_iter());
    /// g.e_mut().add_from_iter([
    ///     (0, 1, true, false),
    ///     (1, 2, false, false),
    ///     (2, 0, true, false),
    /// ].into_iter());
    /// let mut edges: Vec<_> = g.e().iter().collect::<Vec<_>>();
    /// edges.sort_by_key(|(eid, _, _, _)| *eid);
    /// assert_eq!(edges, vec![(0, 0, 1, &true), (1, 1, 2, &false), (2, 2, 0, &true)]);
    /// ```
    /// 
    /// ## See also
    /// * [`g.e_mut().iter()`][mutiter] -- Same as this fucntion but with mutable edge references.
    /// * [`g.e().iter_dir()`][iterdir] -- Iterates over all directed edges in the graph.
    /// * [`g.e().iter_undir()`][iterundir] -- Iterates over all undirected edges in the graph.
    /// * [`g.e().iter_incident()`][iterinc] -- Iterates over all edges incident on a specific
    /// vertex.
    /// * [`g.e().iter_incident_in()`][iterincin] -- Iterates over all directed edges going in a
    /// specific vertex.
    /// * [`g.e().iter_incident_out()`][iterincout] -- Iterates over all directed edges going out
    /// of a specific vertex.
    /// * [`g.e().iter_incident_undir()`][iterincundir] -- Iterates over all undirected edges
    /// incident on a specific vertex.
    /// 
    /// [mutiter]: MutableEdgeView::iter
    /// [iterdir]: ImmutableEdgeView::iter_dir
    /// [iterundir]: ImmutableEdgeView::iter_undir
    /// [iterinc]: ImmutableEdgeView::iter_incident
    /// [iterincin]: ImmutableEdgeView::iter_incident_in
    /// [iterincout]: ImmutableEdgeView::iter_incident_out
    /// [iterincundir]: ImmutableEdgeView::iter_incident_undir
    pub fn iter(&self) -> Box<dyn Iterator<Item = (Id, Id, Id, &'a EdgeType)> + 'a> {
        Box::new(self.g.edges.iter().map(|(&eid, (vid1, vid2, edge))| (eid, *vid1, *vid2, edge)))
    }



    /// # Iterate over directed edges
    /// 
    /// ## Description
    /// Iterates over all directed edges in the graph.
    /// Returns an iterator producing tuples (edge ID, vertex ID 1, vertex ID 2, &edge).
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[u8] ---[bool]-->} = Graph::null();
    /// g.v_mut().add_from_iter([1, 2, 4].into_iter());
    /// g.e_mut().add_from_iter([
    ///     (0, 1, true, false),
    ///     (1, 2, false, true),
    ///     (2, 0, true, false),
    /// ].into_iter());
    /// let mut edges: Vec<_> = g.e().iter_dir().collect::<Vec<_>>();
    /// edges.sort_by_key(|(eid, _, _, _)| *eid);
    /// assert_eq!(edges, vec![(1, 1, 2, &false)]);
    /// ```
    /// 
    /// ## See also
    /// * [`g.e_mut().iter_dir()`][mutiterdir] -- Same as this fucntion but with mutable edge
    /// references.
    /// * [`g.e().iter()`][iter] -- Iterates over all edges in the graph.
    /// * [`g.e().iter_undir()`][iterundir] -- Iterates over all undirected edges in the graph.
    /// * [`g.e().iter_incident()`][iterinc] -- Iterates over all edges incident on a specific
    /// vertex.
    /// * [`g.e().iter_incident_in()`][iterincin] -- Iterates over all directed edges going in a
    /// specific vertex.
    /// * [`g.e().iter_incident_out()`][iterincout] -- Iterates over all directed edges going out
    /// of a specific vertex.
    /// * [`g.e().iter_incident_undir()`][iterincundir] -- Iterates over all undirected edges
    /// incident on a specific vertex.
    /// 
    /// [mutiterdir]: MutableEdgeView::iter_dir
    /// [iter]: ImmutableEdgeView::iter
    /// [iterundir]: ImmutableEdgeView::iter_undir
    /// [iterinc]: ImmutableEdgeView::iter_incident
    /// [iterincin]: ImmutableEdgeView::iter_incident_in
    /// [iterincout]: ImmutableEdgeView::iter_incident_out
    /// [iterincundir]: ImmutableEdgeView::iter_incident_undir
    pub fn iter_dir(&self) -> Box<dyn Iterator<Item = (Id, Id, Id, &'a EdgeType)> + 'a> {
        Box::new(self.g.edges
            .iter()
            .filter(|(eid, (vid1, _, _))| self.g.locales[vid1].is_registered_out(**eid))
            .map(|(eid, (vid1, vid2, edge))| (*eid, *vid1, *vid2, edge))
        )
    }



    /// # Iterate edges incident on vertex
    /// 
    /// ## Description
    /// Iterates over all edges incident on the given vertex.
    /// 
    /// Returns `Ok(iterator)` if a vertex with the given ID exists.
    /// In this case, `iterator` produces tuples (edge ID, adjacent vertex ID, &edge).
    /// Otherwise, returns `Err(ConnectorsError)`.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[u8] ---[bool]---} = Graph::null();
    /// g.v_mut().add_from_iter([1, 2, 4].into_iter());
    /// g.e_mut().add_from_iter([
    ///     (0, 1, true, false),
    ///     (1, 2, false, false),
    ///     (2, 0, true, false),
    /// ].into_iter());
    /// let mut edges = g.e().iter_incident(1).unwrap().collect::<Vec<_>>();
    /// edges.sort_by_key(|(eid, _, _)| *eid);
    /// assert_eq!(edges, vec![(0, 0, &true), (1, 2, &false)]);
    /// ```
    /// 
    /// ## See also
    /// * [`g.e_mut().iter_incident()`][mutiterinc] -- Same as this fucntion but with mutable edge
    /// references.
    /// * [`g.e().iter()`][iter] -- Iterates over all edges in the graph.
    /// * [`g.e().iter_dir()`][iterdir] -- Iterates over all directed edges in the graph.
    /// * [`g.e().iter_undir()`][iterundir] -- Iterates over all undirected edges in the graph.
    /// * [`g.e().iter_incident_in()`][iterincin] -- Iterates over all directed edges going in a
    /// specific vertex.
    /// * [`g.e().iter_incident_out()`][iterincout] -- Iterates over all directed edges going out
    /// of a specific vertex.
    /// * [`g.e().iter_incident_undir()`][iterincundir] -- Iterates over all undirected edges
    /// incident on a specific vertex.
    /// 
    /// [mutiterinc]: MutableEdgeView::iter_incident
    /// [iter]: ImmutableEdgeView::iter
    /// [iterdir]: ImmutableEdgeView::iter_dir
    /// [iterundir]: ImmutableEdgeView::iter_undir
    /// [iterincin]: ImmutableEdgeView::iter_incident_in
    /// [iterincout]: ImmutableEdgeView::iter_incident_out
    /// [iterincundir]: ImmutableEdgeView::iter_incident_undir
    pub fn iter_incident(&self, vid: Id) -> Result<Box<dyn Iterator<Item = (Id, Id, &'a EdgeType)> + 'a>> {
        if self.g.locales.get(&vid).is_none() {
            return connecto_rs_error!(format!("Cannot iterate over edges incident on vertex {} because it doesn't exist.", vid));
        }
        let locale = self.g.locales.get(&vid).unwrap();
        Ok(Box::new(locale.iter_incident().map(|(vid, eid)| {
            let (_, _, edge) = self.g.edges.get(&eid).unwrap();
            (eid, vid, edge)
        })))
    }



    /// # Iterate directed edges going in vertex
    /// 
    /// ## Description
    /// Iterates over all directed edges for which the given vertex is the target vertex.
    /// 
    /// Returns `Ok(iterator)` if a vertex with the given ID exists.
    /// In this case, `iterator` produces tuples (edge ID, adjacent vertex ID, &edge).
    /// Otherwise, returns `Err(ConnectorsError)`.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[u8] ---[bool]-->} = Graph::null();
    /// g.v_mut().add_from_iter([1, 2, 4].into_iter());
    /// g.e_mut().add_from_iter([
    ///     (0, 1, true, false),
    ///     (1, 2, false, true),
    ///     (2, 0, true, false),
    /// ].into_iter());
    /// let mut edges = g.e().iter_incident_in(1).unwrap().collect::<Vec<_>>();
    /// edges.sort_by_key(|(eid, _, _)| *eid);
    /// assert_eq!(edges, vec![]);
    /// ```
    /// 
    /// ## See also
    /// * [`g.e_mut().iter_incident_in()`][mutiterincin] -- Same as this fucntion but with mutable
    /// edge references.
    /// * [`g.e().iter()`][iter] -- Iterates over all edges in the graph.
    /// * [`g.e().iter_dir()`][iterdir] -- Iterates over all directed edges in the graph.
    /// * [`g.e().iter_undir()`][iterundir] -- Iterates over all undirected edges in the graph.
    /// * [`g.e().iter_incident()`][iterinc] -- Iterates over all edges incident on a specific
    /// vertex.
    /// * [`g.e().iter_incident_out()`][iterincout] -- Iterates over all directed edges going out
    /// of a specific vertex.
    /// * [`g.e().iter_incident_undir()`][iterincundir] -- Iterates over all undirected edges
    /// incident on a specific vertex.
    /// 
    /// [mutiterincin]: MutableEdgeView::iter_incident_in
    /// [iter]: ImmutableEdgeView::iter
    /// [iterdir]: ImmutableEdgeView::iter_dir
    /// [iterundir]: ImmutableEdgeView::iter_undir
    /// [iterinc]: ImmutableEdgeView::iter_incident
    /// [iterincout]: ImmutableEdgeView::iter_incident_out
    /// [iterincundir]: ImmutableEdgeView::iter_incident_undir
    pub fn iter_incident_in(&self, vid: Id) -> Result<Box<dyn Iterator<Item = (Id, Id, &'a EdgeType)> + 'a>> {
        if self.g.locales.get(&vid).is_none() {
            return connecto_rs_error!(format!("Cannot iterate over edges going in vertex {} because it doesn't exist.", vid));
        }
        let locale = self.g.locales.get(&vid).unwrap();
        Ok(Box::new(locale.iter_incident_in().map(|(vid, eid)| {
            let (_, _, edge) = self.g.edges.get(&eid).unwrap();
            (eid, vid, edge)
        })))
    }



    /// # Iterate directed edges going out of vertex
    /// 
    /// ## Description
    /// Iterates over all directed edges for which the given vertex is the source vertex.
    /// 
    /// Returns `Ok(iterator)` if a vertex with the given ID exists.
    /// In this case, `iterator` produces tuples (edge ID, adjacent vertex ID, &edge).
    /// Otherwise, returns `Err(ConnectorsError)`.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[u8] ---[bool]-->} = Graph::null();
    /// g.v_mut().add_from_iter([1, 2, 4].into_iter());
    /// g.e_mut().add_from_iter([
    ///     (0, 1, true, false),
    ///     (1, 2, false, true),
    ///     (2, 0, true, false),
    /// ].into_iter());
    /// let mut edges = g.e().iter_incident_out(1).unwrap().collect::<Vec<_>>();
    /// edges.sort_by_key(|(eid, _, _)| *eid);
    /// assert_eq!(edges, vec![(1, 2, &false)]);
    /// ```
    /// 
    /// ## See also
    /// * [`g.e_mut().iter_incident_out()`][mutiterincout] -- Same as this fucntion but with
    /// mutable edge references.
    /// * [`g.e().iter()`][iter] -- Iterates over all edges in the graph.
    /// * [`g.e().iter_dir()`][iterdir] -- Iterates over all directed edges in the graph.
    /// * [`g.e().iter_undir()`][iterundir] -- Iterates over all undirected edges in the graph.
    /// * [`g.e().iter_incident()`][iterinc] -- Iterates over all edges incident on a specific
    /// vertex.
    /// * [`g.e().iter_incident_in()`][iterincin] -- Iterates over all directed edges going in a
    /// specific vertex.
    /// * [`g.e().iter_incident_undir()`][iterincundir] -- Iterates over all undirected edges
    /// incident on a specific vertex.
    /// 
    /// [mutiterincout]: MutableEdgeView::iter_incident_out
    /// [iter]: ImmutableEdgeView::iter
    /// [iterdir]: ImmutableEdgeView::iter_dir
    /// [iterundir]: ImmutableEdgeView::iter_undir
    /// [iterinc]: ImmutableEdgeView::iter_incident
    /// [iterincin]: ImmutableEdgeView::iter_incident_in
    /// [iterincundir]: ImmutableEdgeView::iter_incident_undir
    pub fn iter_incident_out(&self, vid: Id) -> Result<Box<dyn Iterator<Item = (Id, Id, &'a EdgeType)> + 'a>> {
        if self.g.locales.get(&vid).is_none() {
            return connecto_rs_error!(format!("Cannot iterate over edges going out of vertex {} because it doesn't exist.", vid));
        }
        let locale = self.g.locales.get(&vid).unwrap();
        Ok(Box::new(locale.iter_incident_out().map(|(vid, eid)| {
            let (_, _, edge) = self.g.edges.get(&eid).unwrap();
            (eid, vid, edge)
        })))
    }



    /// # Iterate undirected edges incident on vertex
    /// 
    /// ## Description
    /// Iterates over all undirected edges incident on the given vertex.
    /// 
    /// Returns `Ok(iterator)` if a vertex with the given ID exists.
    /// In this case, `iterator` produces tuples (edge ID, adjacent vertex ID, &edge).
    /// Otherwise, returns `Err(ConnectorsError)`.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[u8] ---[bool]-->} = Graph::null();
    /// g.v_mut().add_from_iter([1, 2, 4].into_iter());
    /// g.e_mut().add_from_iter([
    ///     (0, 1, true, false),
    ///     (1, 2, false, true),
    ///     (2, 0, true, false),
    /// ].into_iter());
    /// let mut edges = g.e().iter_incident_undir(1).unwrap().collect::<Vec<_>>();
    /// edges.sort_by_key(|(eid, _, _)| *eid);
    /// assert_eq!(edges, vec![(0, 0, &true)]);
    /// ```
    /// 
    /// ## See also
    /// * [`g.e_mut().iter_incident_undir()`][mutiterincundir] -- Same as this fucntion but with
    /// mutable edge references.
    /// * [`g.e().iter()`][iter] -- Iterates over all edges in the graph.
    /// * [`g.e().iter_dir()`][iterdir] -- Iterates over all directed edges in the graph.
    /// * [`g.e().iter_undir()`][iterundir] -- Iterates over all undirected edges in the graph.
    /// * [`g.e().iter_incident()`][iterinc] -- Iterates over all edges incident on a specific
    /// vertex.
    /// * [`g.e().iter_incident_in()`][iterincin] -- Iterates over all directed edges going in a
    /// specific vertex.
    /// * [`g.e().iter_incident_out()`][iterincout] -- Iterates over all directed edges going out
    /// of a specific vertex.
    /// 
    /// [mutiterincundir]: MutableEdgeView::iter_incident_undir
    /// [iter]: ImmutableEdgeView::iter
    /// [iterdir]: ImmutableEdgeView::iter_dir
    /// [iterundir]: ImmutableEdgeView::iter_undir
    /// [iterinc]: ImmutableEdgeView::iter_incident
    /// [iterincin]: ImmutableEdgeView::iter_incident_in
    /// [iterincout]: ImmutableEdgeView::iter_incident_out
    pub fn iter_incident_undir(&self, vid: Id) -> Result<Box<dyn Iterator<Item = (Id, Id, &'a EdgeType)> + 'a>> {
        if self.g.locales.get(&vid).is_none() {
            return connecto_rs_error!(format!("Cannot iterate over undirected edges incident on vertex {} because it doesn't exist.", vid));
        }
        let locale = self.g.locales.get(&vid).unwrap();
        Ok(Box::new(locale.iter_incident_undir().map(|(vid, eid)| {
            let (_, _, edge) = self.g.edges.get(&eid).unwrap();
            (eid, vid, edge)
        })))
    }
}





/// # Edge view (mutable)
/// 
/// ## Description
/// This structure provides you with an interface to interact with the edges of a graph.
/// Note that it only provides methods that require the mutability of the graph.
/// They include, e.g. addition or removal of edges.
/// 
/// A mutable edge view of a graph is typically accessed with method [`g.e_mut()`][emut].
/// 
/// ## See also
/// * [`ImmutableEdgeView`] -- Access this structure for an edge interface with immutable
/// methods.
/// 
/// [emut]: Graph::e_mut
pub struct MutableEdgeView<'a, EdgeType, LocaleType, VertexType>
where
    LocaleType: Locale<VertexType>,
{
    g: &'a mut Graph<EdgeType, LocaleType, VertexType>,
}



impl<'a, EdgeType, LocaleType, VertexType> MutableEdgeView<'a, EdgeType, LocaleType, VertexType>
where
    LocaleType: Locale<VertexType>,
{
    /// # Add new edge
    /// 
    /// ## Description
    /// Adds a new edge incident on vertices with IDs `vid1` and `vid2`.
    /// 
    /// If the graph is directed and `directed` is set to `true`, the edge will be directed from
    /// `vid1` to `vid2`.
    /// If it's set to `false`, the edge will be undirected.
    /// If the graph is undirected, the value of `directed` is ignored and the edge will be
    /// undirected in either case.
    /// 
    /// If the graph is simple and an edge between `vid1` and `vid2` already exists, returns
    /// `Err(ConnectorsError)`.
    /// If a vertex with ID `vid1` or `vid2` doesn't exist, returns `Err(ConnectorsError)`.
    /// Otherwise, returns `Ok(new_edge_id)`.
    /// 
    /// ## Safety
    /// This function is _safe_, i.e. if it returns an error, the graph is guaranteed to be
    /// unaltered.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[()] ---[()]---} = Graph::null();
    /// g.v_mut().add(()); // Adds a vertex with ID 0
    /// g.v_mut().add(()); // Adds a vertex with ID 1
    /// assert!(g.e_mut().add(0, 1, (), false).is_ok()); // Adds an edge with ID 0
    /// assert!(g.e_mut().add(0, 1, (), false).is_err()); // An edge between 0 and 1 already exists
    /// assert!(g.e_mut().add(1, 2, (), false).is_err()); // There's no vertex with ID 2
    /// ```
    /// 
    /// ## See also
    /// * [`g.e_mut().add_from_iter()`][addfromiter] -- Adds edges from a (vertex ID 1,
    /// vertex ID 2, edge, directed) iterator.
    /// 
    /// [addfromiter]: MutableEdgeView::add_from_iter
    pub fn add(&mut self, vid1: Id, vid2: Id, new_edge: EdgeType, directed: bool) -> Result<Id> {
        for vid in [vid1, vid2] {
            if !self.g.v().contains(vid) {
                return connecto_rs_error!(format!("Cannot add an edge incident on vertices {} and {} because vertex {} doesn't exist.", vid1, vid2, vid));
            }
        }
        let new_edge_id = self.g.next_edge_id;
        if self.g.locales.get_mut(&vid1).unwrap().register_edge(new_edge_id, vid2, directed).is_err() {
            return connecto_rs_error!(format!("Cannot add an edge incident on vertices {} and {} because the graph is simple and an edge between these vertices already exists.", vid1, vid2));
        }
        self.g.locales.get_mut(&vid2).unwrap().register_edge(new_edge_id, vid1, directed).unwrap();
        self.g.edges.insert(new_edge_id, (vid1, vid2, new_edge));
        while self.g.edges.contains_key(&self.g.next_edge_id) {
            self.g.next_edge_id += 1;
        }
        Ok(new_edge_id)
    }



    /// # Add new edges from iterator
    /// 
    /// ## Description
    /// Adds new edges from the given data iterator.
    /// The iterator must produce tuples (vertex ID 1, vertex ID 2, edge, directed).
    /// See the documentation for [`g.e_mut().add()`][add] to learn the meaning of these arguments.
    /// 
    /// It is recommended to use this method when you want to add multiple edges in a row because
    /// it allocates necessary memory in one chunck.
    /// 
    /// Returns `Vec<Result>`, where each element is the return value of [`g.e_mut().add()`][add]
    /// for the corresponding new edge.
    /// 
    /// ## Safety
    /// This function is _safe_, i.e. if it returns an error, the graph is guaranteed to be
    /// unaltered.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[()] ---[()]---} = Graph::null();
    /// g.v_mut().add_from_iter([(), (), ()].into_iter()); // Adds vertices with IDs 0, 1, 2
    /// let my_edges = [(0, 1, (), false), (1, 2, (), false)];
    /// let results = g.e_mut().add_from_iter(my_edges.into_iter());
    /// assert!(results[0].is_ok());
    /// assert!(results[1].is_ok());
    /// let my_edges = [(0, 2, (), false), (0, 1, (), false)];
    /// let results = g.e_mut().add_from_iter(my_edges.into_iter());
    /// assert!(results[0].is_ok());
    /// assert!(results[1].is_err()); // An edge between 0 and 1 already exists
    /// assert_eq!(g.e().count(), 3);
    /// ```
    /// 
    /// ## See also
    /// * [`g.e_mut().add()`][add] -- Adds a single edge.
    /// 
    /// [add]: MutableEdgeView::add
    pub fn add_from_iter<EdgeDataIter: Iterator<Item = (Id, Id, EdgeType, bool)>>(&mut self, data: EdgeDataIter) -> Vec<Result<Id>> {
        let new_edges: Vec<_> = data.collect();
        let mut results = Vec::with_capacity(new_edges.len());
        self.g.edges.reserve(new_edges.len());
        for (vid1, vid2, new_edge, directed) in new_edges {
            results.push(self.add(vid1, vid2, new_edge, directed));
        }
        results
    }



    /// # Remove edge
    /// 
    /// ## Description
    /// Removes an edge with the given ID.
    /// If the edge exists, returns `Some(deleted_edge)`.
    /// If the edge doesn't exist, returns `None`.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[()] ---[()]---} = Graph::null();
    /// g.v_mut().add(());
    /// g.v_mut().add(());
    /// g.e_mut().add(0, 1, (), false); // Adds edge with ID 0
    /// assert!(g.e_mut().remove(0).is_some());
    /// assert!(g.e_mut().remove(0).is_none());
    /// assert_eq!(g.e().count(), 0);
    /// ```
    /// 
    /// ## See also
    /// * [`g.e_mut().remove_from_iter`][removefromiter] -- Removes edges from an edge ID
    /// iterator.
    /// 
    /// [removefromiter]: MutableEdgeView::remove_from_iter
    pub fn remove(&mut self, id: Id) -> Option<EdgeType> {
        match self.g.edges.remove(&id) {
            Some((vid1, vid2, edge)) => {
                self.g.locales.get_mut(&vid1).unwrap().deregister_edge(id);
                self.g.locales.get_mut(&vid2).unwrap().deregister_edge(id);
                self.g.next_edge_id = if id < self.g.next_edge_id { id } else { self.g.next_edge_id };
                Some(edge)
            },
            None => None,
        }
    }



    /// # Remove edges from iterator
    /// 
    /// ## Description
    /// Removes edges with IDs from the given data iterator.
    /// The iterator must produce edge IDs.
    /// Returns a vector of `Option<EdgeType>`, where each value is either `Some(old_edge)` if an
    /// edge with the corresponding ID exists, or `None` otherwise.
    /// 
    /// ## Example
    /// ```
    /// let mut g: graph!{[()] ---[()]---} = Graph::null();
    /// g.v_mut().add_from_iter([(), (), ()].into_iter());
    /// g.e_mut().add_from_iter([
    ///     (0, 1, (), false),
    ///     (1, 2, (), false),
    ///     (2, 0, (), false)
    /// ].into_iter());
    /// g.e_mut().remove_from_iter([0, 1, 4].into_iter());
    /// assert_eq!(g.e().count(), 1);
    /// ```
    /// 
    /// ## See also
    /// * [`g.e_mut().remove`][remove] -- Removes a single edge.
    /// 
    /// [remove]: MutableEdgeView::remove
    pub fn remove_from_iter<EdgeDataIter: Iterator<Item = Id>>(&mut self, data: EdgeDataIter) {
        for id in data {
            self.remove(id);
        }
    }
}










/// # Graph
/// 
/// ## Description
/// The structure of graph itself, the central structure of the entire Connecto.rs crate.
/// A **graph** is understood as a collection of vertices and edges connecting them.
/// Since `Graph` is a generic structure, the specific properties of vertices and edges are largely
/// determined by the properties of the data types used to represent them.
/// 
/// ## Internal data representation
/// This structure stores graph data in 2 hash-maps.
/// The first hash-map maps vertex IDs to the corresponding [locales][lo], while the second
/// hash-map maps edge IDs to the corresponding edges.
/// Both of these hash-maps are private and users are not supposed to have direct access to them.
/// Instead, users should manipulate vertices and edges of their graph with the help of so-called
/// vertex and edge views respectively.
/// 
/// ## Views
/// **Views** are basic interfaces that allow you to interact with your graphs and do fundamental
/// operations with them.
/// There are 2 main kinds of views: vertex views and edge views.
/// A vertex or an edge view can be either mutable or immutable.
/// 
/// Naturally, a **vertex view** allows you to interact with the vertices of a graph and an **edge
/// view** allows you to interact with the edges of a graph.
/// Likewise, a **mutable view** allows you to change the data you're interacting with whereas
/// an **immutable view** guarantees that no changes to the data will be made.
/// 
/// Vertex views can be accessed with methods [`g.v()`][v] and [`g.v_mut()`][vmut].
/// They expose interfaces of an immutable and, respectively, a mutable vertex view.
/// Likewise, edge views can be accessed with [`g.e()`][e] and [`g.e_mut()`][emut].
/// 
/// Follow these links to see the documentation for all existing views:
/// * [`ImmutableVertexView`] -- immutable vertex methods, accessed with [`g.v()`][v].
/// * [`MutableVertexView`] -- mutable vertex methods, accessed with [`g.v_mut()`][vmut].
/// * [`ImmutableEdgeView`] -- immutable edge methods, accessed with [`g.e()`][e].
/// * [`MutableEdgeView`] -- mutable edge methods, accessed with [`g.e_mut()`][emut].
/// 
/// [lo]: Locale
/// [v]: Graph::v
/// [vmut]: Graph::v_mut
/// [e]: Graph::e
/// [emut]: Graph::e_mut
pub struct Graph<EdgeType, LocaleType, VertexType>
where
    LocaleType: Locale<VertexType>,
{
    edges: HashMap<Id, (Id, Id, EdgeType)>,
    locales: HashMap<Id, LocaleType>,
    next_vertex_id: Id,
    next_edge_id: Id,
    _phantom: PhantomData<VertexType>,
}



impl<'a, EdgeType, LocaleType, VertexType> Graph<EdgeType, LocaleType, VertexType>
where
    LocaleType: Locale<VertexType>,
{
    /// # Edge view (immutable)
    /// 
    /// ## Description
    /// Returns an [immutable edge view][iev] of a graph.
    /// 
    /// ## See also
    /// * [`g.e_mut()`][emut] -- A method to get a mutable edge view.
    /// 
    /// [iev]: ImmutableEdgeView
    /// [emut]: Graph::e_mut
    pub fn e(&'a self) -> ImmutableEdgeView<'a, EdgeType, LocaleType, VertexType> {
        ImmutableEdgeView { g: self }
    }



    /// # Edge view (mutable)
    /// 
    /// ## Description
    /// Returns an [mutable edge view][mev] of a graph.
    /// 
    /// ## See also
    /// * [`g.e()`][e] -- A method to get an immutable edge view.
    /// 
    /// [mev]: MutableEdgeView
    /// [e]: Graph::e
    pub fn e_mut(&'a mut self) -> MutableEdgeView<'a, EdgeType, LocaleType, VertexType> {
        MutableEdgeView { g: self }
    }



    /// # Create null graph
    /// 
    /// ## Description
    /// Creates a new graph with no vertices and no edges.
    pub fn null() -> Self {
        Graph {
            edges: HashMap::new(),
            locales: HashMap::new(),
            next_vertex_id: 0,
            next_edge_id: 0,
            _phantom: PhantomData
        }
    }



    /// # Vertex view (immutable)
    /// 
    /// ## Description
    /// Returns an [immutable vertex view][ivv] of a graph.
    /// 
    /// ## See also
    /// * [`g.v_mut()`][vmut] -- A method to get a mutable vertex view.
    /// 
    /// [ivv]: ImmutableVertexView
    /// [vmut]: Graph::v_mut
    pub fn v(&'a self) -> ImmutableVertexView<'a, EdgeType, LocaleType, VertexType> {
        ImmutableVertexView { g: self }
    }



    /// # Vertex view (mutable)
    /// 
    /// ## Description
    /// Returns an [mutable vertex view][mvv] of a graph.
    /// 
    /// ## See also
    /// * [`g.v()`][v] -- A method to get a immutable vertex view.
    /// 
    /// [mvv]: MutableVertexView
    /// [v]: Graph::v
    pub fn v_mut(&'a mut self) -> MutableVertexView<'a, EdgeType, LocaleType, VertexType> {
        MutableVertexView { g: self }
    }
}










#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::locales::UndirectedSimpleLocale;

    #[test]
    fn basic_operations() {
        let mut g: Graph<(), UndirectedSimpleLocale<()>, ()> = Graph::null();
        g.v_mut().add(());
        g.v_mut().add(());
        g.v_mut().add(());
        assert_eq!(g.v().count(), 3);
        g.v_mut().add_from_iter([(), ()].into_iter());
        assert_eq!(g.v().count(), 5);
    }

    #[test]
    fn iter_edges() {
        let mut g: Graph<bool, UndirectedSimpleLocale<u8>, u8> = Graph::null();
        g.v_mut().add_from_iter([1, 2, 4].into_iter());
        g.e_mut().add_from_iter([
            (0, 1, true, false),
            (1, 2, false, false),
            (2, 0, true, false),
        ].into_iter());
        // iter
        let mut edges = g.e().iter().collect::<Vec<_>>();
        edges.sort_by_key(|(eid, _, _, _)| *eid);
        assert_eq!(edges, vec![(0, 0, 1, &true), (1, 1, 2, &false), (2, 2, 0, &true)]);
        // iter_incident
        let mut edges = g.e().iter_incident(1).unwrap().collect::<Vec<_>>();
        edges.sort_by_key(|(eid, _, _)| *eid);
        assert_eq!(edges, vec![(0, 0, &true), (1, 2, &false)]);
    }
}
