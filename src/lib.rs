//! <h2 id="nexusart" style="text-align: center; font-variant: small-caps"><a href="#nexusart">NexusArt</a></h2>
//! 
//! <div style="text-align: center"><b><i>Fast and flexible graph library for Rust</i></b></div>
//! 
//! ## Welcome!
//! NexusArt is one of the few Rust libraries that enable developers to  build,  analyse
//! and manipulate graphs/networks.
//! 
//! ## Features
//! * **Different families of graphs with unified interface**  Build  simple  graphs  or
//! multi-graphs with directed or undirected edges and maniputate all of them  with  the
//! same set of functions! [More about this...][kinds]
//! * **Attributes** Add your custom attributes to vertices or edges  or  even  both  of
//! them! [More about this...][attrs]
//! 
//! [kinds]: Graph#different-kinds-of-graphs
//! [attrs]: Graph#attributes





pub(self) mod private{
    use std::marker::PhantomData;

    pub trait Conditional {
        type Type: ?Sized;
    }

    pub struct ConditionalTypeCore<const C: bool, T, F>
    where
        T: ?Sized,
        F: ?Sized,
    {
        t: PhantomData<T>,
        f: PhantomData<F>,
    }

    impl<T, F> Conditional for ConditionalTypeCore<true, T, F>
    where
        T: ?Sized,
        F: ?Sized,
    {
        type Type = T;
    }

    impl<T, F> Conditional for ConditionalTypeCore<false, T, F>
    where
        T: ?Sized,
        F: ?Sized,
    {
        type Type = F;
    }

    #[allow(dead_code)]
    pub type ConditionalType<const C: bool, T, F> = <ConditionalTypeCore<C, T, F> as Conditional>::Type;
}

pub mod attribute;
pub mod errors;
pub mod io;
pub mod locales;

use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    marker::PhantomData,
    ops::AddAssign,
};
use attribute::{AttributeCollection, DynamicDispatchAttributeMap, StaticDispatchAttributeValue};
use errors::{NexusArtError, NexusArtResult};
use locales::*;
#[allow(unused_imports)]
use private::ConditionalType;





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * ID                                                                                *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



/// # ID trait
/// ## Description
/// Types that implement `Id` can be used as unique identifiers of vertices and parallel
/// edges within a [`Graph`]. This trait is already implemented for all standard integer
/// types (both signed and unsigned).
/// 
/// Types  that  implement  `Id`  must  be  linearly   ordered,   hashable,   cloneable,
/// displayable.
pub trait Id
where
    Self: Clone + Default + Display + Eq + Hash + Ord,
{
    /// # Default value of ID
    /// 
    /// ## Description
    /// This function returns the default value for the first possible ID. For integers,
    /// this function is implemented to return  the  minimum  value  of  the  respective
    /// integer type.
    /// 
    /// ## Arguments
    /// None.
    /// 
    /// ## Returns
    /// `Self` - the default value for the first possible ID.
    // fn default() -> Self;
    /// # Advance the element
    /// 
    /// ## Description
    /// This function assigns to the callee the following element in  the  linear  order
    /// defined for the type. For integers, this function increases the value by 1.
    /// 
    /// ## Arguments
    /// * `&mut self` - a mutable reference to the caller.
    /// 
    /// ## Returns
    /// None.
    fn increment(&mut self);
}

macro_rules! implement_vertex_id_trait_for {
    ($t: ty) => {
        impl Id for $t {
            /*fn default() -> Self {
                <$t>::MIN
            }*/
        
            fn increment(&mut self) {
                self.add_assign(1)
            }
        }
    };
}

implement_vertex_id_trait_for!(u8);
implement_vertex_id_trait_for!(u16);
implement_vertex_id_trait_for!(u32);
implement_vertex_id_trait_for!(u64);
implement_vertex_id_trait_for!(u128);
implement_vertex_id_trait_for!(usize);
implement_vertex_id_trait_for!(i8);
implement_vertex_id_trait_for!(i16);
implement_vertex_id_trait_for!(i32);
implement_vertex_id_trait_for!(i64);
implement_vertex_id_trait_for!(i128);
implement_vertex_id_trait_for!(isize);





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * AUXILIARY ITEMS TO WORK WITH EDGES                                                *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



#[derive(Hash, PartialEq, Eq)]
pub enum EdgeDirection {
    Undirected,
    Directed1to2,
    Directed2to1,
}



pub enum EdgeToVertexRelation {
    Undirected,
    Incoming,
    Outcoming,
}



#[derive(Hash, PartialEq, Eq)]
pub struct EdgeIteratorItem<EdgeIdType, VertexIdType>
where
    EdgeIdType: Id,
    VertexIdType: Id,
{
    direction: EdgeDirection,
    edge_id: EdgeIdType,
    id1: VertexIdType,
    id2: VertexIdType,
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * GRAPH CONTAINERS                                                                  *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub trait ImmutableGraphContainer
where
    Self: Clone + Default,
{
    type EdgeAttributeCollectionType: AttributeCollection;
    type EdgeIdType: Id;
    type LocaleType: Locale<Self::EdgeAttributeCollectionType, Self::EdgeIdType, Self::VertexAttributeCollectionType, Self::VertexIdType>;
    type VertexAttributeCollectionType: AttributeCollection;
    type VertexIdType: Id;
    fn unwrap(&self) -> &Graph<Self::EdgeAttributeCollectionType, Self::EdgeIdType, Self::LocaleType, Self::VertexAttributeCollectionType, Self::VertexIdType>;
}

// <T:ImmutableGraphContainer>::BasicImmutableGraph
impl<T> BasicImmutableGraph<T::EdgeIdType, T::VertexIdType> for T
where
    T: Clone + ImmutableGraphContainer,
{
    #[inline]
    fn contains_e(&self, id1: &T::VertexIdType, id2: &T::VertexIdType, edge_id: &T::EdgeIdType) -> Option<EdgeDirection> {
        self.unwrap().contains_e(id1, id2, edge_id)
    }

    #[inline]
    fn contains_v(&self, id: &T::VertexIdType) -> bool {
        self.unwrap().contains_v(id)
    }

    #[inline]
    fn count_e(&self) -> usize {
        self.unwrap().count_e()
    }

    #[inline]
    fn count_v(&self) -> usize {
        self.unwrap().count_v()
    }

    #[inline]
    fn v_degree(&self, id: &T::VertexIdType) -> NexusArtResult<usize> {
        self.unwrap().v_degree(id)
    }

    #[inline]
    fn v_degree_in(&self, id: &T::VertexIdType) -> NexusArtResult<usize> {
        self.unwrap().v_degree_in(id)
    }

    #[inline]
    fn v_degree_out(&self, id: &T::VertexIdType) -> NexusArtResult<usize> {
        self.unwrap().v_degree_out(id)
    }

    #[inline]
    fn v_degree_undir(&self, id: &T::VertexIdType) -> NexusArtResult<usize> {
        self.unwrap().v_degree_undir(id)
    }
}



pub trait MutableGraphContainer
where
    Self: ImmutableGraphContainer,
{
    fn unwrap(&mut self) -> &mut Graph<Self::EdgeAttributeCollectionType, Self::EdgeIdType, Self::LocaleType, Self::VertexAttributeCollectionType, Self::VertexIdType>;
}

// <T:MutableGraphContainer>::BasicMutableGraph
impl<T> BasicMutableGraph<T::EdgeAttributeCollectionType, T::EdgeIdType, T::VertexAttributeCollectionType, T::VertexIdType> for T
where
    T: Clone + MutableGraphContainer,
{
    #[inline]
    fn add_e(&mut self, id1: &T::VertexIdType, id2: &T::VertexIdType, directed: bool, edge_id: Option<T::EdgeIdType>) -> NexusArtResult<T::EdgeIdType> {
        self.unwrap().add_e(id1, id2, directed, edge_id)
    }

    #[inline]
    fn add_v(&mut self, id: Option<T::VertexIdType>) -> T::VertexIdType {
        self.unwrap().add_v(id)
    }

    #[inline]
    fn e_attrs_mut(&mut self, id1: &T::VertexIdType, id2: &T::VertexIdType, edge_id: &T::EdgeIdType) -> NexusArtResult<&mut T::EdgeAttributeCollectionType> {
        self.unwrap().e_attrs_mut(id1, id2, edge_id)
    }

    #[inline]
    fn remove_e(&mut self, id1: &T::VertexIdType, id2: &T::VertexIdType, edge_id: &T::EdgeIdType) -> NexusArtResult<bool> {
        self.unwrap().remove_e(id1, id2, edge_id)
    }

    #[inline]
    fn remove_v(&mut self, id: &T::VertexIdType) -> bool {
        self.unwrap().remove_v(id)
    }

    #[inline]
    fn v_attrs_mut(&mut self, id: &T::VertexIdType) -> NexusArtResult<&mut T::VertexAttributeCollectionType> {
        self.unwrap().v_attrs_mut(id)
    }
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * GRAPH                                                                             *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



/// # Basic immutable functions for graphs
/// 
/// ## Description
/// This trait defines functions and methods that give a user low-level access to  graph
/// topology and _never_ change its structure (hence, immutable).
/// 
/// This  trait  is  implemented  for   [`Graph`]   and   any   type   that   implements
/// [`ImmutableGraphContainer`].
pub trait BasicImmutableGraph<EdgeIdType, VertexIdType>
where
    Self: Clone + Default,
    EdgeIdType: Id,
    VertexIdType: Id,
{
    /// # Check existence of edge
    /// 
    /// ## Description
    /// Check whether the given edge exists and, if it exists, get its orientation.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// * `id1` : `&VertexIdType` - an immutable  reference  to  the  ID  of  the  first
    /// vertex.
    /// * `id2` : `&VertexIdType` - an immutable reference  to  the  ID  of  the  second
    /// vertex.
    /// * `edge_id` : `&EdgeIdType` - if edge IDs are  supported  (see [Details]),  then
    /// the existence of the edge between `id1` and `id2`  with  ID  `edge_id`  will  be
    /// checked.
    /// 
    /// ## Returns
    /// * `Option<EdgeDirection>` - `Some(value)` is returned if both given vertices and
    /// the given edge exist, `value` indicates the direction of  the  edge;  `None`  is
    /// returned otherwise.
    /// 
    /// <div id="contains-e-details" style="margin-top: -15px;">
    /// 
    /// ## Details
    /// 
    /// </div>
    /// 
    /// If the underlying [`Graph`] is [simple][kinds], the value of `edge_id`  will  be
    /// ignored as simple graphs don't support edge IDs.  In  this  case,  if  the  edge
    /// between  `id1`  and  `id2`   exists,   the   return   value   will   always   be
    /// `Some(EdgeDirection::Undirected)`.
    /// 
    /// [Details]: #contains-e-details
    /// [kinds]: Graph#different-kinds-of-graphs
    fn contains_e(&self, id1: &VertexIdType, id2: &VertexIdType, edge_id: &EdgeIdType) -> Option<EdgeDirection>;
    /// # Check existence of vertex
    /// 
    /// ## Description
    /// Assert existence of a vertex with the given ID in the graph.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// * `id` : `&VertexIdType` - an immutable reference to the ID of interest.
    /// 
    /// ## Returns
    /// * `bool` - `true` if such vertex exists, `false` otherwise.
    fn contains_v(&self, id: &VertexIdType) -> bool;
    /// # Count edges
    /// 
    /// ## Description
    /// Get the number of edges in the graph.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `usize` - the number of edges in the graph.
    fn count_e(&self) -> usize;
    /// # Count vertices
    /// 
    /// ## Description
    /// Get the number of vertices in the graph.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `usize` - the number of vertices in the graph.
    fn count_v(&self) -> usize;
    /// # Vertex degree
    /// 
    /// ## Description
    /// Get the number of vertices adjacent to the vertex with the given ID.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// * `id` : `&VertexIdType` - an immutable reference to the ID of interest.
    /// 
    /// ## Returns
    /// * `NexusArtResult<usize>` - `Ok(usize)` is returned when the vertex with ID `id`
    /// exists; `Err(NexusArtError)` is returned otherwise.
    /// 
    /// ## Details
    /// This function counts  _all_  vertices  of  the  underlying  [`Graph`]  that  are
    /// directly connected to the vertex with ID `id` with _any_  possible  edge.  Thus,
    /// `g.v_degree(id) == g.v_degree_in(id) + g.v_degree_out(id) + g.v_degree_undir(id)`.
    /// 
    /// If the underlying [`Graph`] `g` is [undirected](Graph#different-kinds-of-graphs),
    /// then `g.v_degree(id) == g.v_degree_undir(id)`.
    /// 
    /// If the underlying [`Graph`] is a [multi-graph](Graph#different-kinds-of-graphs),
    /// then each parallel edge will be counted separately.
    fn v_degree(&self, id: &VertexIdType) -> NexusArtResult<usize>;
    /// # Vertex in-degree
    /// 
    /// ## Description
    /// Get the number of vertices adjacent to the vertex with the  given  ID  that  are
    /// connected with it by an incoming edge.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// * `id` : `&VertexIdType` - an immutable reference to the ID of interest.
    /// 
    /// ## Return
    /// * `NexusArtResult<usize>` - `Ok(usize)` is returned when the vertex with ID `id`
    /// exists; `Err(NexusArtError)` is returned otherwise.
    /// 
    /// ## Details
    /// If the underlying [`Graph`] `g` is [undirected](Graph#different-kinds-of-graphs),
    /// then `g.v_degree_in(id) == 0`.
    /// 
    /// If the underlying [`Graph`] is a [multi-graph](Graph#different-kinds-of-graphs),
    /// then each parallel incoming edge will be counted separately.
    fn v_degree_in(&self, id: &VertexIdType) -> NexusArtResult<usize>;
    /// # Vertex out-degree
    /// 
    /// ## Description
    /// Get the number of vertices adjacent to the vertex with the  given  ID  that  are
    /// connected with it by an outcoming edge.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// * `id` : `&VertexIdType` - an immutable reference to the ID of interest.
    /// 
    /// ## Return
    /// * `NexusArtResult<usize>` - `Ok(usize)` is returned when the vertex with ID `id`
    /// exists; `Err(NexusArtError)` is returned otherwise.
    /// 
    /// ## Details
    /// If the underlying [`Graph`] `g` is [undirected](Graph#different-kinds-of-graphs),
    /// then `g.v_degree_out(id) == 0`.
    /// 
    /// If the underlying [`Graph`] is a [multi-graph](Graph#different-kinds-of-graphs),
    /// then each parallel outcoming edge will be counted separately.
    fn v_degree_out(&self, id: &VertexIdType) -> NexusArtResult<usize>;
    /// # Vertex undirected degree
    /// 
    /// ## Description
    /// Get the number of vertices adjacent to the vertex with the  given  ID  that  are
    /// connected with it by an undirected edge.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// * `id` : `&VertexIdType` - an immutable reference to the ID of interest.
    /// 
    /// ## Return
    /// * `NexusArtResult<usize>` - `Ok(usize)` is returned when the vertex with ID `id`
    /// exists; `Err(NexusArtError)` is returned otherwise.
    /// 
    /// ## Details
    /// If the underlying [`Graph`] `g` is [undirected](Graph#different-kinds-of-graphs),
    /// then `g.v_degree_out(id) == g.v_degree(id)`.
    /// 
    /// If the underlying [`Graph`] is a [multi-graph](Graph#different-kinds-of-graphs),
    /// then each parallel undirected edge will be counted separately.
    fn v_degree_undir(&self, id: &VertexIdType) -> NexusArtResult<usize>;
}



/// # Basic mutable functions for graphs
/// ## Description
/// This trait defines functions and methods that give a user low-level access to  graph
/// topology and _may_ change its structure (hence, mutable).
/// 
/// This  trait  is  implemented  for   [`Graph`]   and   any   type   that   implements
/// [`MutableGraphContainer`].
pub trait BasicMutableGraph<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>
where
    Self: BasicImmutableGraph<EdgeIdType, VertexIdType>,
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    /// # Add edge
    /// 
    /// ## Description
    /// Add a new edge between two vertices.
    /// 
    /// ## Arguments
    /// * `&mut self` - a mutable reference to the caller.
    /// * `id1` : `&VertexIdType` - an immutable  reference  to  the  ID  of  the  first
    /// vertex.
    /// * `id2` : `&VertexIdType` - an immutable reference  to  the  ID  of  the  second
    /// vertex.
    /// * `directed` : `bool` - if edge directions are supported (see [Details][details]),
    /// this flag shows whether the edge should be directed or not.
    /// * `edge_id`  :  `Option<EdgeIdType>`  -  if  edge   IDs   are   supported   (see
    /// [Details][details]) and `Some(value)` is passed, a new edge with ID `value` will
    /// be created; if `None` is passed,  the  ID  for  the  new  edge  will  be  chosen
    /// automatically.
    /// 
    /// ## Returns
    /// * `NexusArtResult<EdgeIdType>` - `Ok(value)` is returned when the edge was added
    /// successfully with `value` being the ID of the new edge; `Err(NexusArtError)`  is
    /// returned when at least 1 of the vertices `id1` and `id2` doesn't exist.
    /// 
    /// ## Details
    /// If the underlying [`Graph`] is [undirected][kinds], then the value of `directed`
    /// is ignored: undirected graphs don't support edge direction and the new edge will
    /// be undirected in any case.
    /// 
    /// If the underlying [`Graph`] is  [simple][kinds]  and  there's  already  an  edge
    /// between vertices `id1` and `id2`, then the existing edge  will  be  removed  and
    /// replaced with the new one. This means that all properties of the  existing  edge
    /// (e.g. [attributes][attrs]) will be lost. Furthermore,  the  value  of  `edge_id`
    /// will be ignored as there're no parallel edges in simple graphs. If both vertices
    /// `id1` and `id2` exist, the return value will always be `Ok(0)`.
    /// 
    /// If the underlying [`Graph`] is a [multi-graph][kinds]  and  there's  already  an
    /// edge between vertices `id1` and `id2` with ID `edge_id`, then the existing  edge
    /// will be removed and replaced with the new one. This means that all properties of
    /// the existing edge (e.g. [attributes][attrs]) will be lost. If  there's  no  edge
    /// between `id1` and `id2` with ID `edge_id` or if `edge_id == None`, then the  new
    /// parallel edge will be created without affecting the existing ones in any way.
    /// 
    /// [attrs]: Graph#attributes
    /// [details]: #details
    /// [kinds]: Graph#different-kinds-of-graphs
    fn add_e(&mut self, id1: &VertexIdType, id2: &VertexIdType, directed: bool, edge_id: Option<EdgeIdType>) -> NexusArtResult<EdgeIdType>;
    /// # Add vertex
    /// 
    /// ## Description
    /// Add a new isolated vertex to the graph.
    /// 
    /// ## Arguments
    /// * `&mut self` - a mutable reference to the caller.
    /// * `id` : `Option<VertexIdType>` - if `Some(value)` is passed, a new vertex  with
    /// ID `value` will be created; if `None` is passed, the ID for the new vertex  will
    /// be chosen automatically.
    /// 
    /// ## Returns
    /// * `VertexIdType` - the ID of the new vertex.
    /// 
    /// ## Details
    /// If there's already a vertex with ID `id`,  then  the  existing  vertex  will  be
    /// removed and replaced with the new one. This means that  all  properties  of  the
    /// existing vertex (e.g. [attributes][attrs], incident edges) will be lost.
    /// 
    /// [attrs]: Graph#attributes
    fn add_v(&mut self, id: Option<VertexIdType>) -> VertexIdType;
    /// # Get a mutable reference to edge attributes
    /// ## Description
    /// Get a mutable reference to the [attribute collection][attrs]  of  the  specified
    /// vertex.
    ///
    /// ## Arguments
    /// * `&mut self` - a mutable reference to the caller.
    /// * `id1` : `&VertexIdType` - an immutable  reference  to  the  ID  of  the  first
    /// vertex.
    /// * `id2` : `&VertexIdType` - an immutable reference  to  the  ID  of  the  second
    /// vertex.
    /// * `edge_id` : `&EdgeIdType` - if edge IDs are supported (see [Details]), then an
    /// attribute collection of the edge between `id1` and `id2` with ID `edge_id`  will
    /// be retrieved.
    /// 
    /// ## Returns
    /// * `NexusArtResult<&mut EdgeAttributeCollectionType>` - `Ok(value)`  is  returned
    /// if the given edge exists; `Err(NexusArtError)` is returned otherwise.
    /// 
    /// <div id="get-e-attrs-mut-details" style="margin-top: -15px;">
    /// 
    /// ## Details
    /// 
    /// </div>
    /// 
    /// Even if the edge with ID `edge_id` is directed, the order of  values  `id1`  and
    /// `id2` doesn't matter.
    /// 
    /// If the underlying [`Graph`] is [simple][kinds], the value of `edge_id`  will  be
    /// ignored as simple graphs don't support edge IDs.
    /// 
    /// [Details]: #get-e-attrs-mut-details
    /// [kinds]: Graph#different-kinds-of-graphs
    fn e_attrs_mut(&mut self, id1: &VertexIdType, id2: &VertexIdType, edge_id: &EdgeIdType) -> NexusArtResult<&mut EdgeAttributeCollectionType>;
    /// # Remove edge
    /// 
    /// ## Description
    /// Delete an edge between 2 given vertices.
    /// 
    /// ## Arguments
    /// * `&mut self` - a mutable reference to the caller.
    /// * `id1` : `&VertexIdType` - an immutable  reference  to  the  ID  of  the  first
    /// vertex.
    /// * `id2` : `&VertexIdType` - an immutable reference  to  the  ID  of  the  second
    /// vertex.
    /// * `edge_id` : `EdgeIdType` - if edge IDs are supported (see [Details]), then  an
    /// edge between `id1` and `id2` with ID `edge_id` will be removed.
    /// 
    /// ## Returns
    /// * `NexusArtResult<bool>` - `Ok(value)` is returned if the edge was  successfully
    /// deleted or if it didn't exist: Boolean `value` shows whether  the  edge  existed
    /// when this function was called; `Err(NexusArtError)` is returned when at least  1
    /// of the vertices `id1` and `id2` doesn't exist.
    /// 
    /// <div id="remove-e-details" style="margin-top: -15px;">
    /// 
    /// ## Details
    /// 
    /// </div>
    /// 
    /// Even if the edge with ID `edge_id` is directed, the order of  values  `id1`  and
    /// `id2` doesn't matter.
    /// 
    /// If the underlying [`Graph`] is [simple][kinds], the value of `edge_id`  will  be
    /// ignored as simple graphs don't support edge IDs.
    /// 
    /// [Details]: #remove-e-details
    /// [kinds]: Graph#different-kinds-of-graphs
    fn remove_e(&mut self, id1: &VertexIdType, id2: &VertexIdType, edge_id: &EdgeIdType) -> NexusArtResult<bool>;
    /// # Remove vertex
    /// 
    /// ## Description
    /// Delete a vertex with the given ID.
    /// 
    /// ## Arguments
    /// * `&mut self` - a mutable reference to the caller.
    /// * `id` : `&VertexIdType` - the ID of a vertex to be removed.
    /// 
    /// ## Returns
    /// * `bool` - shows whether the vertex with ID `id` existed when this function  was
    /// called.
    fn remove_v(&mut self, id: &VertexIdType) -> bool;
    /// # Get a mutable reference to vertex attributes
    /// 
    /// ## Description
    /// Get a mutable reference to the [attribute collection][attrs]  of  the  specified
    /// vertex.
    /// 
    /// ## Arguments
    /// * `&mut self` - a mutable reference to the caller.
    /// * `id` : `VertexIdType` - an immutable reference to the ID of interest.
    /// 
    /// ## Returns
    /// * `NexusArtResult<&mut VertexAttributeCollectionType>` - `Ok(value)` is returned
    /// if the vertex  with  the  given  ID  exists;  `Err(NexusArtError)`  is  returned
    /// otherwise.
    /// 
    /// [attrs]: attribute::AttributeCollection
    fn v_attrs_mut(&mut self, id: &VertexIdType) -> NexusArtResult<&mut VertexAttributeCollectionType>;
}



/// # Graph
/// ## Description
/// ...
/// 
/// ## Different kinds of graphs
/// There are many ways to categorise graphs into different classes, here we present the
/// most general and most commonly known classification.
/// 
/// Classes of graphs by edge orientation:
/// * **Undirected** - edges in such graphs have no direction, they  simply  indicate  a
/// link between a pair of vertices.
/// * **Directed** - edges in such graphs _can_ have direction; in  this  case,  we  say
/// that the vertex where the edge starts is the **source** of the edge and  the  vertex
/// where the edge ends is the **target** of the edge.
/// 
/// Classes of graphs by edge uniqueness:
/// * **Simple** - there's at most one edge between any 2 vertices.
/// * **Multi-graphs** - there can be arbitrarily many edges between any 2 vertices; any
/// pair of edges connecting the same pair of vertices will be called **parallel**.
/// 
/// Each graph can be assigned with one label from  the  first  classification  and  one
/// label from the second classification, thus making 4 possible combinations.  NexusArt
/// supports all of them.
/// 
/// Furthermore,  NexusArt  implements  certain  optimisations   for   each   of   these
/// combinations, hence, it makes sense for you to carefully evaluate which exactly kind
/// of graphs you're going to be dealing in your program with to enjoy the best possible
/// performance you can get.
/// 
/// ## Attributes
/// Vertices and edges of graphs can store attributes.
#[derive(Clone, Default)]
pub struct Graph<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>
where
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    LocaleType: Locale<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    edge_list: HashMap<VertexIdType, LocaleType>,
    min_free_vertex_id: VertexIdType,
    phantom: PhantomData<(EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType)>,
}

// Graph::Graph
impl<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType> Graph<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>
where
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    LocaleType: Locale<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    pub fn new() -> Self {
        Graph { edge_list: HashMap::new(), min_free_vertex_id: VertexIdType::default(), phantom: PhantomData }
    }
}

// Graph::BasicImmutableGraph
impl<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType> BasicImmutableGraph<EdgeIdType, VertexIdType> for Graph<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>
where
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    LocaleType: Locale<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    #[inline]
    fn contains_e(&self, id1: &VertexIdType, id2: &VertexIdType, edge_id: &EdgeIdType) -> Option<EdgeDirection> {
        match self.edge_list.get(id1) {
            Some(value) => value.e_direction(id2, edge_id),
            None => None,
        }
    }

    #[inline]
    fn contains_v(&self, id: &VertexIdType) -> bool {
        self.edge_list.contains_key(id)
    }

    #[inline]
    fn count_v(&self) -> usize {
        self.edge_list.len()
    }

    fn count_e(&self) -> usize {
        let mut answer: usize = 0;
        for (_, locale) in self.edge_list.iter() {
            answer += locale.count_neighbours();
        }
        answer / 2
    }

    #[inline]
    fn v_degree(&self, id: &VertexIdType) -> NexusArtResult<usize> {
        const FUNCTION_PATH: &str = "Graph::BasicImmutableGraph::v_degree";
        match self.edge_list.get(id) {
            Some(value) => Ok(value.count_neighbours()),
            None => Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id))),
        }
    }

    #[inline]
    fn v_degree_in(&self, id: &VertexIdType) -> NexusArtResult<usize> {
        const FUNCTION_PATH: &str = "Graph::BasicImmutableGraph::v_degree_in";
        match self.edge_list.get(id) {
            Some(value) => Ok(value.count_neighbours_in()),
            None => Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id))),
        }
    }

    #[inline]
    fn v_degree_out(&self, id: &VertexIdType) -> NexusArtResult<usize> {
        const FUNCTION_PATH: &str = "Graph::BasicImmutableGraph::v_degree_out";
        match self.edge_list.get(id) {
            Some(value) => Ok(value.count_neighbours_out()),
            None => Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id))),
        }
    }

    #[inline]
    fn v_degree_undir(&self, id: &VertexIdType) -> NexusArtResult<usize> {
        const FUNCTION_PATH: &str = "Graph::BasicImmutableGraph::v_degree_undir";
        match self.edge_list.get(id) {
            Some(value) => Ok(value.count_neighbours_undir()),
            None => Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id))),
        }
    }
}

// Graph::BasicMutableGraph
impl<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType> BasicMutableGraph<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType> for Graph<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>
where
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    LocaleType: Locale<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    fn add_e(&mut self, id1: &VertexIdType, id2: &VertexIdType, directed: bool, edge_id: Option<EdgeIdType>) -> NexusArtResult<EdgeIdType> {
        const FUNCTION_PATH: &str = "Graph::BasicMutableGraph::add_e";
        if self.contains_v(id1) {
            if self.contains_v(id2) {
                let actual_edge_id = self.edge_list
                    .get_mut(id1)
                    .unwrap()
                    .add_e(id2.clone(), if directed {
                        EdgeToVertexRelation::Outcoming
                    } else {
                        EdgeToVertexRelation::Undirected
                    }, edge_id);
                self.edge_list
                    .get_mut(id2)
                    .unwrap()
                    .add_e(id1.clone(), if directed {
                        EdgeToVertexRelation::Incoming
                    } else {
                        EdgeToVertexRelation::Undirected
                    }, Some(actual_edge_id.clone()));
                Ok(actual_edge_id)
            } else {
                Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id2)))
            }
        } else {
            Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id1)))
        }
    }

    fn add_v(&mut self, id: Option<VertexIdType>) -> VertexIdType {
        let return_value: VertexIdType;
        match id {
            Some(value) => {
                self.edge_list.insert(value.clone(), LocaleType::new());
                return_value = value;
            },
            None => {
                self.edge_list.insert(self.min_free_vertex_id.clone(), LocaleType::new());
                return_value = self.min_free_vertex_id.clone();
            },
        }
        while self.edge_list.contains_key(&self.min_free_vertex_id) {
            self.min_free_vertex_id.increment();
        }
        return_value
    }

    fn e_attrs_mut(&mut self, id1: &VertexIdType, id2: &VertexIdType, edge_id: &EdgeIdType) -> NexusArtResult<&mut EdgeAttributeCollectionType> {
        const FUNCTION_PATH: &str = "Graph::BasicMutableGraph::e_attrs_mut";
        if self.edge_list.contains_key(id1) {
            if self.edge_list.contains_key(id2) {
                match self.contains_e(id1, id2, edge_id) {
                    Some(value) => match value {
                        EdgeDirection::Undirected => Ok(self.edge_list.get_mut(if id1 <= id2 { id1 } else { id2 }).unwrap().e_attrs_mut(if id1 <= id2 { id2 } else { id1 }, edge_id)),
                        EdgeDirection::Directed1to2 => Ok(self.edge_list.get_mut(id1).unwrap().e_attrs_mut(id2, edge_id)),
                        EdgeDirection::Directed2to1 => Ok(self.edge_list.get_mut(id2).unwrap().e_attrs_mut(id1, edge_id)),
                    },
                    None => Err(NexusArtError::new(FUNCTION_PATH, format!("Accessing attributes of a non-existing edge between vertices {} and {} with edge ID {}.", id1, id2, edge_id))),
                }
            } else {
                Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id2)))
            }
        } else {
            Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id1)))
        }
    }

    fn remove_e(&mut self, id1: &VertexIdType, id2: &VertexIdType, edge_id: &EdgeIdType) -> NexusArtResult<bool> {
        const FUNCTION_PATH: &str = "Graph::BasicMutableGraph::delete_e";
        if self.edge_list.contains_key(id1) {
            if self.edge_list.contains_key(id2) {
                self.edge_list.get_mut(id1).unwrap().remove_e(id2, edge_id);
                Ok(self.edge_list.get_mut(id2).unwrap().remove_e(id1, edge_id))
            } else {
                Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id2)))
            }
        } else {
            Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id1)))
        }
    }

    fn remove_v(&mut self, id: &VertexIdType) -> bool {
        if !self.edge_list.contains_key(id) {
            return false;
        }
        for edge in self.edge_list[id].incident_e() {
            self.remove_e(&edge.id1, &edge.id2, &edge.edge_id).unwrap();
        }
        self.edge_list.remove(id);
        if self.min_free_vertex_id > *id {
            self.min_free_vertex_id = id.clone();
        }
        true
    }

    fn v_attrs_mut(&mut self, id: &VertexIdType) -> NexusArtResult<&mut VertexAttributeCollectionType> {
        const FUNCTION_PATH: &str = "Graph::BasicMutableGraph::v_attrs_mut";
        match self.edge_list.get_mut(id) {
            Some(value) => Ok(value.v_attrs_mut()),
            None => Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id)))
        }
    }
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * CONVENIENT GRAPH CONSTRUCTION                                                     *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



#[derive(Clone, Copy)]
pub enum GraphProperty{
    EdgeIdType,
    VertexAttributeCollectionType,
    VertexIdType,
}



#[allow(unused_macros)]
macro_rules! graph_type_recognition_assistant {
    ([$first_property:path = $first_value:ty, $($property:path = $value:ty),+], $required_property:path, $default_value:ty) => {
        ConditionalType<
            {($first_property as u8) == ($required_property as u8)},
            $first_value,
            graph_type_recognition_assistant!([$($property = $value),+], $required_property, $default_value)
        >
    };

    ([$last_property:path = $last_value:ty], $required_property:path, $default_value:ty) => {
        ConditionalType<
            {($last_property as u8) == ($required_property as u8)},
            $last_value,
            $default_value
        >
    }
}



/// # A macro to fast and easily create a new graph
/// ## Description
/// There's one feature of [`Graph`] that comes both as a blessing and a curse:  it's  a
/// generic type. [`Graph`] accepts multiple type parameters that have to  be  specified
/// by you in a very specific order every time you create a  new  instance.  This  helps
/// NexusArt to maintain its flexibility and allows you to adjust the  functionality  of
/// your networks for your specific needs, however, it comes at its own cost of  tedious
/// enumeration of all the necessary type parameters, which names, of course, are  never
/// remembered and have to be looked up in the documentation.
/// 
/// This macro was created to avoid it as much as possible. With the help of `graph!()`,
/// you can easily create all possible kinds of  graphs  NexusArt  has  to  offer  while
/// keeping your code clean and aesthetically appealing.
/// 
/// ## Basic use
/// Let's look at the following simple example and try to  understand  what's  going  on
/// there.
/// 
/// ```
/// let g = graph!(X ===A==> X);
/// ```
/// 
/// Here, we have 3 tokens separated by whitespaces: '`X`', '`===A==>`' and '`X`'.
/// 
/// The first _and_ the last token symbolise vertices of our network  and  show  whether
/// they'll have any [attributes](Graph#attributes) or not.  Value  '`X`'  serves  as  a
/// marker which says: 'We'll _never_ try to store  or  access  any  attributes  of  any
/// vertex of the graph'. Needless to say, this allows NexusArt to optimise memory usage
/// as we know in advance that there'll be no need to even declare attribute collections
/// for any of the vertices.
/// 
/// Note that the first and the last tokens must always be identical! Of course, one can
/// argue that this notation is redundant and can be shortened by  getting  rid  of  the
/// last token but we keep it for a better visual presentation.
/// 
/// As it's easy to guess now, the token in the middle symbolises edges of  the  network
/// and their properties.
/// 
/// Look closer at this second token and you'll notice that it looks like 2 arrows  with
/// the letter 'A' slapped on top of them going from the  left  'vertex'  to  the  right
/// 'vertex'. This pair of arrows means that we can have parallel edges,  i.e.  that  we
/// want  our  graph  to  be  a   [multi-graph](Graph#different-kinds-of-graphs),   and,
/// furthermore,   our   edges   can   be   [directed](Graph#different-kinds-of-graphs).
/// Letter '`A`' in the middle, contrary to letter '`X`', means  that  our  edges  _can_
/// have attributes stored in them.
#[macro_export]
macro_rules! graph {
    (X ---X--- X with $($property:path = $value:ty),+) => {
        {
            type VertexIdType = graph_type_recognition_assistant!([$($property = $value),+], GraphProperty::VertexIdType, usize);
            Graph::<(), u8, UndirectedSimpleUnattributedLocale<(), VertexIdType>, (), VertexIdType>::new()
        }
    };

    (X ---X--- X) => {
        Graph::<(), u8, UndirectedSimpleUnattributedLocale<(), usize>, (), usize>::new()
    };

    (A ---X--- A with $($property:path = $value:ty),+) => {
        {
            type VertexAttributeCollectionType = graph_type_recognition_assistant!([$($property = $value),+], GraphProperty::VertexAttributeCollectionType, DynamicDispatchAttributeMap<String>);
            type VertexIdType = graph_type_recognition_assistant!([$($property = $value),+], GraphProperty::VertexIdType, usize);
            Graph::<(), u8, UndirectedSimpleUnattributedLocale<VertexAttributeCollectionType, VertexIdType>, VertexAttributeCollectionType, VertexIdType>::new()
        }
    };

    (A ---X--- A) => {
        Graph::<(), u8, UndirectedSimpleUnattributedLocale<DynamicDispatchAttributeMap<String>, usize>, DynamicDispatchAttributeMap<String>, usize>::new()
    };
}





#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::{io::{gnbs::GNBSReader, Reader}, *};

    #[test]
    fn graph_new_xxx_with() {
        let mut g = graph!(X ---X--- X
        with
            GraphProperty::VertexIdType = i8
        );
        assert_eq!(g.add_v(None), 0);
    }

    #[test]
    fn graph_new_axa() {
        let mut g = graph!(A ---X--- A);
        assert_eq!(g.add_v(None), 0);
    }

    #[test]
    fn graph_new_axa_with() {
        let mut g = graph!(A ---X--- A
        with
            GraphProperty::VertexIdType = i8,
            GraphProperty::VertexAttributeCollectionType = ()
        );
        assert_eq!(g.add_v(None), 0);
    }

    #[test]
    fn undirected_graph1() {
        // Undirected simple unattributed graph
        let mut g = graph!(X ---X--- X);
        // Add vertices
        assert_eq!(g.add_v(None), 0);
        assert_eq!(g.add_v(Some(1)), 1);
        assert_eq!(g.add_v(None), 2);
        assert_eq!(g.add_v(Some(218)), 218);
        // Add edges
        assert!(g.add_e(&0, &1, false, None).is_ok());
        assert!(g.add_e(&218, &2, true, Some(30)).is_ok_and(|x| x == 0));
        assert!(g.add_e(&5, &6, false, None).is_err());
        assert!(g.add_e(&0, &218, true, None).is_ok());
        // Degrees
        assert!(g.v_degree(&0).is_ok_and(|x| x == 2));
        assert!(g.v_degree_out(&218).is_ok_and(|x| x == 0));
        assert!(g.v_degree_undir(&218).is_ok_and(|x| x == 2));
        assert!(g.v_degree_in(&5).is_err());
        // Remove edges
        assert!(g.remove_e(&2, &218, &0).is_ok_and(|x| x));
        assert!(g.v_degree(&218).is_ok_and(|x| x == 1));
        assert!(g.v_degree_undir(&2).is_ok_and(|x| x == 0));
        // Add new edges
        assert!(g.add_e(&0, &2, false, None).is_ok());
        assert!(g.add_e(&218, &2, true, None).is_ok_and(|x| x == 0));
        // Remove vertex
        assert!(g.remove_v(&0));
        assert!(g.v_degree(&0).is_err());
        assert!(g.v_degree(&1).is_ok_and(|x| x == 0));
        assert!(g.v_degree(&2).is_ok_and(|x| x == 1));
        assert!(g.v_degree(&218).is_ok_and(|x| x == 1));
        // Add new vertices
        assert_eq!(g.add_v(None), 0);
        assert_eq!(g.add_v(None), 3);
        assert_eq!(g.count_v(), 5);
        assert_eq!(g.count_e(), 1);
    }

    #[test]
    fn read_gnbs() {
        const INPUT: &str = "
        # Test number 1
        AV LS Names
        V 1 X
        V 3 [ \"Romy , \" , ]
        V 2 [     ]
        E 1 2
        E 2 3
        ";
        let buffer_reader = BufReader::new(INPUT.as_bytes());
        let gnbs_reader = GNBSReader;
        let mut g = graph!(A ---X--- A);
        let g_result = gnbs_reader.read_graph(buffer_reader);
        assert!(g_result.is_ok());
        g = g_result.unwrap();
        assert_eq!(g.count_v(), 3);
        assert_eq!(g.count_e(), 2);
        assert_eq!(g.v_attrs_mut(&3).unwrap().get(&"Names".to_string()).unwrap().downcast::<Vec<String>>().unwrap(), &vec!["Romy , ".to_string()]);
    }
}
