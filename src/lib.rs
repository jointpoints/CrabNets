//! <h2 id="crabnets" style="text-align: center; font-variant: small-caps"><a href="#crabnets">CrabNets</a></h2>
//! 
//! <div style="text-align: center"><b><i>Fast and flexible graph library for Rust</i></b></div>
//! 
//! ## Welcome!
//! CrabNets is one of the few Rust libraries that enable developers to  build,  analyse
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





pub mod attributes;
pub mod errors;
pub mod io;
pub mod locales;
pub mod topology_tests;

use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    marker::PhantomData,
    ops::AddAssign,
};
use attributes::{AttributeCollection, DynamicDispatchAttributeMap, StaticDispatchAttributeValue};
use errors::{CrabNetsError, CrabNetsResult};
use locales::*;





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * CONDITIONAL TYPE                                                                  *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



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
    Self: Clone + Display + Eq + Hash + Ord,
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
    fn default() -> Self;
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
            fn default() -> Self {
                <$t>::MIN
            }
        
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
impl<T> BasicImmutableGraph<T::EdgeAttributeCollectionType, T::EdgeIdType, T::VertexAttributeCollectionType, T::VertexIdType> for T
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
    fn e_attrs(&self, id1: &T::VertexIdType, id2: &T::VertexIdType, edge_id: &T::EdgeIdType) -> CrabNetsResult<&T::EdgeAttributeCollectionType> {
        self.unwrap().e_attrs(id1, id2, edge_id)
    }

    #[inline]
    fn iter_e<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<T::EdgeIdType, T::VertexIdType>> + 'a> {
        self.unwrap().iter_e()
    }

    #[inline]
    fn iter_v<'a>(&'a self) -> Box<dyn Iterator<Item = T::VertexIdType> + 'a> {
        self.unwrap().iter_v()
    }

    #[inline]
    fn v_attrs(&self, id: &T::VertexIdType) -> CrabNetsResult<&T::VertexAttributeCollectionType> {
        self.unwrap().v_attrs(id)
    }

    #[inline]
    fn v_degree(&self, id: &T::VertexIdType) -> CrabNetsResult<usize> {
        self.unwrap().v_degree(id)
    }

    #[inline]
    fn v_degree_in(&self, id: &T::VertexIdType) -> CrabNetsResult<usize> {
        self.unwrap().v_degree_in(id)
    }

    #[inline]
    fn v_degree_out(&self, id: &T::VertexIdType) -> CrabNetsResult<usize> {
        self.unwrap().v_degree_out(id)
    }

    #[inline]
    fn v_degree_undir(&self, id: &T::VertexIdType) -> CrabNetsResult<usize> {
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
    fn add_e(&mut self, id1: &T::VertexIdType, id2: &T::VertexIdType, directed: bool, edge_id: Option<T::EdgeIdType>) -> CrabNetsResult<T::EdgeIdType> {
        self.unwrap().add_e(id1, id2, directed, edge_id)
    }

    #[inline]
    fn add_v(&mut self, id: Option<T::VertexIdType>) -> T::VertexIdType {
        self.unwrap().add_v(id)
    }

    #[inline]
    fn e_attrs_mut(&mut self, id1: &T::VertexIdType, id2: &T::VertexIdType, edge_id: &T::EdgeIdType) -> CrabNetsResult<&mut T::EdgeAttributeCollectionType> {
        self.unwrap().e_attrs_mut(id1, id2, edge_id)
    }

    #[inline]
    fn remove_e(&mut self, id1: &T::VertexIdType, id2: &T::VertexIdType, edge_id: &T::EdgeIdType) -> CrabNetsResult<bool> {
        self.unwrap().remove_e(id1, id2, edge_id)
    }

    #[inline]
    fn remove_v(&mut self, id: &T::VertexIdType) -> bool {
        self.unwrap().remove_v(id)
    }

    #[inline]
    fn v_attrs_mut(&mut self, id: &T::VertexIdType) -> CrabNetsResult<&mut T::VertexAttributeCollectionType> {
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
pub trait BasicImmutableGraph<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>
where
    Self: Clone + Default,
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    VertexAttributeCollectionType: AttributeCollection,
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
    /// # Immutable reference to edge attributes
    /// ## Description
    /// Get an immutable reference to the [attribute collection][attrs] of the specified
    /// edge.
    ///
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// * `id1` : `&VertexIdType` - an immutable  reference  to  the  ID  of  the  first
    /// vertex.
    /// * `id2` : `&VertexIdType` - an immutable reference  to  the  ID  of  the  second
    /// vertex.
    /// * `edge_id` : `&EdgeIdType` - if edge IDs are supported (see [Details]), then an
    /// attribute collection of the edge between `id1` and `id2` with ID `edge_id`  will
    /// be retrieved.
    /// 
    /// ## Returns
    /// * `CrabNetsResult<&EdgeAttributeCollectionType>` - `Ok(value)`  is  returned  if
    /// given edge exists; `Err(CrabNetsError)` is returned otherwise.
    /// 
    /// <div id="e-attrs-details" style="margin-top: -15px;">
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
    /// [attrs]: attributes::AttributeCollection
    /// [Details]: #e-attrs-details
    /// [kinds]: Graph#different-kinds-of-graphs
    fn e_attrs(&self, id1: &VertexIdType, id2: &VertexIdType, edge_id: &EdgeIdType) -> CrabNetsResult<&EdgeAttributeCollectionType>;
    /// # Iterate over edges
    /// 
    /// ## Description
    /// Iterate over all edges of the graph.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// *  `Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>>>`  -  an
    /// interator over the edges of the graph.
    fn iter_e<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a>;
    /// # Iterate over vertices
    /// 
    /// ## Description
    /// Iterate over all vertices of the graph.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `Box<dyn Iterator<Item = VertexIdType>>` - an iterator over  the  vertices  of
    /// the vertices of the graph.
    fn iter_v<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    /// # Immutable reference to vertex attributes
    /// 
    /// ## Description
    /// Get an immutable reference to the [attribute collection][attrs] of the specified
    /// vertex.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// * `id` : `VertexIdType` - an immutable reference to the ID of interest.
    /// 
    /// ## Returns
    /// * `CrabNetsResult<&VertexAttributeCollectionType>` - `Ok(value)` is returned  if
    /// the  vertex  with  the  given  ID  exists;  `Err(CrabNetsError)`   is   returned
    /// otherwise.
    /// 
    /// [attrs]: attributes::AttributeCollection
    fn v_attrs(&self, id: &VertexIdType) -> CrabNetsResult<&VertexAttributeCollectionType>;
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
    /// * `CrabNetsResult<usize>` - `Ok(usize)` is returned when the vertex with ID `id`
    /// exists; `Err(CrabNetsError)` is returned otherwise.
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
    fn v_degree(&self, id: &VertexIdType) -> CrabNetsResult<usize>;
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
    /// * `CrabNetsResult<usize>` - `Ok(usize)` is returned when the vertex with ID `id`
    /// exists; `Err(CrabNetsError)` is returned otherwise.
    /// 
    /// ## Details
    /// If the underlying [`Graph`] `g` is [undirected](Graph#different-kinds-of-graphs),
    /// then `g.v_degree_in(id) == 0`.
    /// 
    /// If the underlying [`Graph`] is a [multi-graph](Graph#different-kinds-of-graphs),
    /// then each parallel incoming edge will be counted separately.
    fn v_degree_in(&self, id: &VertexIdType) -> CrabNetsResult<usize>;
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
    /// * `CrabNetsResult<usize>` - `Ok(usize)` is returned when the vertex with ID `id`
    /// exists; `Err(CrabNetsError)` is returned otherwise.
    /// 
    /// ## Details
    /// If the underlying [`Graph`] `g` is [undirected](Graph#different-kinds-of-graphs),
    /// then `g.v_degree_out(id) == 0`.
    /// 
    /// If the underlying [`Graph`] is a [multi-graph](Graph#different-kinds-of-graphs),
    /// then each parallel outcoming edge will be counted separately.
    fn v_degree_out(&self, id: &VertexIdType) -> CrabNetsResult<usize>;
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
    /// * `CrabNetsResult<usize>` - `Ok(usize)` is returned when the vertex with ID `id`
    /// exists; `Err(CrabNetsError)` is returned otherwise.
    /// 
    /// ## Details
    /// If the underlying [`Graph`] `g` is [undirected](Graph#different-kinds-of-graphs),
    /// then `g.v_degree_out(id) == g.v_degree(id)`.
    /// 
    /// If the underlying [`Graph`] is a [multi-graph](Graph#different-kinds-of-graphs),
    /// then each parallel undirected edge will be counted separately.
    fn v_degree_undir(&self, id: &VertexIdType) -> CrabNetsResult<usize>;
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
    Self: BasicImmutableGraph<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
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
    /// * `CrabNetsResult<EdgeIdType>` - `Ok(value)` is returned when the edge was added
    /// successfully with `value` being the ID of the new edge; `Err(CrabNetsError)`  is
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
    fn add_e(&mut self, id1: &VertexIdType, id2: &VertexIdType, directed: bool, edge_id: Option<EdgeIdType>) -> CrabNetsResult<EdgeIdType>;
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
    /// # Mutable reference to edge attributes
    /// ## Description
    /// Get a mutable reference to the [attribute collection][attrs]  of  the  specified
    /// edge.
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
    /// * `CrabNetsResult<&mut EdgeAttributeCollectionType>` - `Ok(value)`  is  returned
    /// if the given edge exists; `Err(CrabNetsError)` is returned otherwise.
    /// 
    /// <div id="e-attrs-mut-details" style="margin-top: -15px;">
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
    /// [attrs]: attributes::AttributeCollection
    /// [Details]: #e-attrs-mut-details
    /// [kinds]: Graph#different-kinds-of-graphs
    fn e_attrs_mut(&mut self, id1: &VertexIdType, id2: &VertexIdType, edge_id: &EdgeIdType) -> CrabNetsResult<&mut EdgeAttributeCollectionType>;
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
    /// * `CrabNetsResult<bool>` - `Ok(value)` is returned if the edge was  successfully
    /// deleted or if it didn't exist: Boolean `value` shows whether  the  edge  existed
    /// when this function was called; `Err(CrabNetsError)` is returned when at least  1
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
    fn remove_e(&mut self, id1: &VertexIdType, id2: &VertexIdType, edge_id: &EdgeIdType) -> CrabNetsResult<bool>;
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
    /// * `CrabNetsResult<&mut VertexAttributeCollectionType>` - `Ok(value)` is returned
    /// if the vertex  with  the  given  ID  exists;  `Err(CrabNetsError)`  is  returned
    /// otherwise.
    /// 
    /// [attrs]: attributes::AttributeCollection
    fn v_attrs_mut(&mut self, id: &VertexIdType) -> CrabNetsResult<&mut VertexAttributeCollectionType>;
}



/// # Graph
/// ## Description
/// This is the central struct of the entire library. It represents,  as  it's  easy  to
/// guess, a graph: a network made of _vertices_ and _edges_ connecting them.
/// 
/// ## Different kinds of graphs
/// There are many ways to categorise graphs into different classes, here we present the
/// most general and most commonly known classification.
/// 
/// Classes of graphs by edge orientation:
/// * **Undirected** - edges in such graphs have no direction, they  simply  indicate  a
/// link between a pair of vertices.
/// * **Directed** - edges in such graphs _can_ have direction; in  this  case,  we  say
/// that the vertex where the edge starts is the _source_ of the  edge  and  the  vertex
/// where the edge ends is the _target_ of the edge.
/// 
/// Classes of graphs by edge uniqueness:
/// * **Simple** - there's at most one edge between any 2 vertices.
/// * **Multi-graphs** - there can be arbitrarily many edges between any 2 vertices; any
/// pair of edges connecting the same pair of vertices will be called _parallel_.
/// 
/// Each graph can be assigned with one label from  the  first  classification  and  one
/// label from the second classification, thus making 4 possible combinations.  CrabNets
/// supports all of them.
/// 
/// > ⚠️ **Warning!** Currently, multi-graphs are under development and not available.
/// 
/// Furthermore,  CrabNets  implements  certain  optimisations   for   each   of   these
/// combinations, hence, it makes sense for you to carefully evaluate which exactly kind
/// of graphs you're going to be dealing in your program with to enjoy the best possible
/// performance you can get.
/// 
/// ## Representation of graphs
/// CrabNets store graphs as edge lists. The local properties of the neighbourhood of
/// each vertex is stored in a so-called _locale_ associated with it. See the
/// [`locales`] module for more details.
/// 
/// Generally speaking, each vertex is identified by its unique ID. IDs are typically
/// numbers, however, you can use any type that implements [`Id`] as the ID.
/// 
/// If you have a multi-graph, edges  will  also  have  their  own  IDs  to  distinguish
/// between multiple parallel edges. Unlike vertex IDs, edge IDs must  be  unique  among
/// all edges _incident on the same pair of vertices_!
/// 
/// ## Attributes
/// Vertices and  edges  of  graphs  can  store  _attributes_  in  so-called  _attribute
/// collections_. Attribute collection is a struct/enum that stores the information that
/// you want to associate with a vertex or with an edge. CrabNets provide some  standard
/// attribute collections for you  to  use,  see  the  [`attributes`]  module  for  more
/// details.
/// 
/// You  can  access  the  attribute   collection   of   each   vertex   using   methods
/// [`BasicImmutableGraph::v_attrs`],   [`BasicMutableGraph::v_attrs_mut`].    Likewise,
/// methods [`BasicImmutableGraph::e_attrs`] and [`BasicMutableGraph::e_attrs_mut`] give
/// you access to the attribute collection of a specific edge. These methods expose  the
/// interface of the attribute collection you use, so the exact things you can  do  with
/// these attribute collections depend on what functionality you implement for them.
/// 
/// ## Generic type parameters
/// Graph is a generic type with the following generic type parameters:
/// * `EdgeAttributeCollectionType` - the type of attribute collections for edges.
/// * `EdgeIdType` - the type to use for edge IDs.
/// * `LocaleType` - the locale to use to capture the local properties of the graph.
/// * `VertexAttributeCollectionType` - the type of attribute collections for vertices.
/// * `VertexIdType` - the type to use for vertex IDs.
#[derive(Clone)]
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
impl<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType> BasicImmutableGraph<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType> for Graph<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>
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
            answer += locale.count_incident_e();
        }
        answer / 2
    }

    fn e_attrs(&self, id1: &VertexIdType, id2: &VertexIdType, edge_id: &EdgeIdType) -> CrabNetsResult<&EdgeAttributeCollectionType> {
        const FUNCTION_PATH: &str = "Graph::BasicMutableGraph::e_attrs_mut";
        if self.edge_list.contains_key(id1) {
            if self.edge_list.contains_key(id2) {
                match self.contains_e(id1, id2, edge_id) {
                    Some(value) => match value {
                        EdgeDirection::Undirected => Ok(self.edge_list.get(if id1 <= id2 { id1 } else { id2 }).unwrap().e_attrs(if id1 <= id2 { id2 } else { id1 }, edge_id).unwrap()),
                        EdgeDirection::Directed1to2 => Ok(self.edge_list.get(id1).unwrap().e_attrs(id2, edge_id).unwrap()),
                        EdgeDirection::Directed2to1 => Ok(self.edge_list.get(id2).unwrap().e_attrs(id1, edge_id).unwrap()),
                    },
                    None => Err(CrabNetsError::new(FUNCTION_PATH, format!("Accessing attributes of a non-existing edge between vertices {} and {} with edge ID {}.", id1, id2, edge_id))),
                }
            } else {
                Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id2)))
            }
        } else {
            Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id1)))
        }
    }

    fn iter_e<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a> {
        Box::new(self.edge_list.iter().map(|(_, x)| x.iter_incident_e_with_attrs()).flatten())
    }

    #[inline]
    fn iter_v<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a> {
        Box::new(self.edge_list.keys().cloned())
    }

    #[inline]
    fn v_attrs(&self, id: &VertexIdType) -> CrabNetsResult<&VertexAttributeCollectionType> {
        const FUNCTION_PATH: &str = "Graph::BasicMutableGraph::v_attrs_mut";
        match self.edge_list.get(id) {
            Some(value) => Ok(value.v_attrs()),
            None => Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id)))
        }
    }

    #[inline]
    fn v_degree(&self, id: &VertexIdType) -> CrabNetsResult<usize> {
        const FUNCTION_PATH: &str = "Graph::BasicImmutableGraph::v_degree";
        match self.edge_list.get(id) {
            Some(value) => Ok(value.count_neighbours()),
            None => Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id))),
        }
    }

    #[inline]
    fn v_degree_in(&self, id: &VertexIdType) -> CrabNetsResult<usize> {
        const FUNCTION_PATH: &str = "Graph::BasicImmutableGraph::v_degree_in";
        match self.edge_list.get(id) {
            Some(value) => Ok(value.count_neighbours_in()),
            None => Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id))),
        }
    }

    #[inline]
    fn v_degree_out(&self, id: &VertexIdType) -> CrabNetsResult<usize> {
        const FUNCTION_PATH: &str = "Graph::BasicImmutableGraph::v_degree_out";
        match self.edge_list.get(id) {
            Some(value) => Ok(value.count_neighbours_out()),
            None => Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id))),
        }
    }

    #[inline]
    fn v_degree_undir(&self, id: &VertexIdType) -> CrabNetsResult<usize> {
        const FUNCTION_PATH: &str = "Graph::BasicImmutableGraph::v_degree_undir";
        match self.edge_list.get(id) {
            Some(value) => Ok(value.count_neighbours_undir()),
            None => Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id))),
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
    fn add_e(&mut self, id1: &VertexIdType, id2: &VertexIdType, directed: bool, edge_id: Option<EdgeIdType>) -> CrabNetsResult<EdgeIdType> {
        const FUNCTION_PATH: &str = "Graph::BasicMutableGraph::add_e";
        if self.contains_v(id1) {
            if self.contains_v(id2) {
                let actual_edge_id = self.edge_list
                    .get_mut(id1)
                    .unwrap()
                    .add_e(id2.clone(), if directed {
                        EdgeToVertexRelation::Outgoing
                    } else {
                        EdgeToVertexRelation::Undirected
                    }, edge_id, directed || id1 <= id2);
                self.edge_list
                    .get_mut(id2)
                    .unwrap()
                    .add_e(id1.clone(), if directed {
                        EdgeToVertexRelation::Incoming
                    } else {
                        EdgeToVertexRelation::Undirected
                    }, Some(actual_edge_id.clone()), !directed && id2 <= id1);
                Ok(actual_edge_id)
            } else {
                Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id2)))
            }
        } else {
            Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id1)))
        }
    }

    fn add_v(&mut self, id: Option<VertexIdType>) -> VertexIdType {
        let return_value: VertexIdType;
        match id {
            Some(value) => {
                self.edge_list.insert(value.clone(), LocaleType::new(value.clone()));
                return_value = value;
            },
            None => {
                self.edge_list.insert(self.min_free_vertex_id.clone(), LocaleType::new(self.min_free_vertex_id.clone()));
                return_value = self.min_free_vertex_id.clone();
            },
        }
        while self.edge_list.contains_key(&self.min_free_vertex_id) {
            self.min_free_vertex_id.increment();
        }
        return_value
    }

    fn e_attrs_mut(&mut self, id1: &VertexIdType, id2: &VertexIdType, edge_id: &EdgeIdType) -> CrabNetsResult<&mut EdgeAttributeCollectionType> {
        const FUNCTION_PATH: &str = "Graph::BasicMutableGraph::e_attrs_mut";
        if self.edge_list.contains_key(id1) {
            if self.edge_list.contains_key(id2) {
                match self.contains_e(id1, id2, edge_id) {
                    Some(value) => match value {
                        EdgeDirection::Undirected => Ok(self.edge_list.get_mut(if id1 <= id2 { id1 } else { id2 }).unwrap().e_attrs_mut(if id1 <= id2 { id2 } else { id1 }, edge_id).unwrap()),
                        EdgeDirection::Directed1to2 => Ok(self.edge_list.get_mut(id1).unwrap().e_attrs_mut(id2, edge_id).unwrap()),
                        EdgeDirection::Directed2to1 => Ok(self.edge_list.get_mut(id2).unwrap().e_attrs_mut(id1, edge_id).unwrap()),
                    },
                    None => Err(CrabNetsError::new(FUNCTION_PATH, format!("Accessing attributes of a non-existing edge between vertices {} and {} with edge ID {}.", id1, id2, edge_id))),
                }
            } else {
                Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id2)))
            }
        } else {
            Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id1)))
        }
    }

    fn remove_e(&mut self, id1: &VertexIdType, id2: &VertexIdType, edge_id: &EdgeIdType) -> CrabNetsResult<bool> {
        const FUNCTION_PATH: &str = "Graph::BasicMutableGraph::delete_e";
        if self.edge_list.contains_key(id1) {
            if self.edge_list.contains_key(id2) {
                self.edge_list.get_mut(id1).unwrap().remove_e(id2, edge_id);
                Ok(self.edge_list.get_mut(id2).unwrap().remove_e(id1, edge_id))
            } else {
                Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id2)))
            }
        } else {
            Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id1)))
        }
    }

    fn remove_v(&mut self, id: &VertexIdType) -> bool {
        if !self.edge_list.contains_key(id) {
            return false;
        }
        for edge in self.edge_list[id].iter_incident_e().collect::<Vec<_>>() {
            self.remove_e(&edge.id1, &edge.id2, &edge.edge_id).unwrap();
        }
        self.edge_list.remove(id);
        if self.min_free_vertex_id > *id {
            self.min_free_vertex_id = id.clone();
        }
        true
    }

    fn v_attrs_mut(&mut self, id: &VertexIdType) -> CrabNetsResult<&mut VertexAttributeCollectionType> {
        const FUNCTION_PATH: &str = "Graph::BasicMutableGraph::v_attrs_mut";
        match self.edge_list.get_mut(id) {
            Some(value) => Ok(value.v_attrs_mut()),
            None => Err(CrabNetsError::new(FUNCTION_PATH, format!("Vertex with ID {} doesn't exist.", id)))
        }
    }
}

// Graph::Default
impl<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType> Default for Graph<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>
where
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    LocaleType: Locale<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    fn default() -> Self {
        Graph { edge_list: HashMap::new(), min_free_vertex_id: VertexIdType::default(), phantom: PhantomData }
    }
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * CONVENIENT GRAPH CONSTRUCTION                                                     *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



#[derive(Clone, Copy)]
pub enum GraphProperty{
    EdgeAttributeCollectionType,
    EdgeIdType,
    VertexAttributeCollectionType,
    VertexIdType,
}



#[allow(unused_macros)]
#[macro_export]
macro_rules! graph_type_recognition_assistant {
    ([$first_property:ident = $first_value:ty, $($property:ident = $value:ty),+], $required_property:ident, $default_value:ty) => {
        ConditionalType<
            {(GraphProperty::$first_property as u8) == (GraphProperty::$required_property as u8)},
            $first_value,
            graph_type_recognition_assistant!([$($property = $value),+], $required_property, $default_value)
        >
    };

    ([$last_property:ident = $last_value:ty], $required_property:ident, $default_value:ty) => {
        ConditionalType<
            {(GraphProperty::$last_property as u8) == (GraphProperty::$required_property as u8)},
            $last_value,
            $default_value
        >
    }
}



/// # A macro to fast and easily define a graph type
/// 
/// ## Description
/// There's one feature of [`Graph`] that comes both as a blessing and a curse:  it's  a
/// generic type. [`Graph`] accepts multiple type  parameters  that  have  to  be  given
/// by you in a very specific order every time you create a  new  instance.  This  helps
/// CrabNets to maintain its flexibility and allows you to adjust the  functionality  of
/// your networks for your specific needs, however, it comes at its own cost of  tedious
/// enumeration of all the necessary type parameters, which names, of course, are  never
/// remembered and have to be looked up in the documentation.
/// 
/// This macro was created to avoid it as much as possible. With the help of `graph!()`,
/// you can easily pick out any [kind of graphs  CrabNets  has  to  offer][kinds]  while
/// keeping your code clean and aesthetically appealing.
/// 
/// ## Basic use
/// Let's look at the following simple example and try to  understand  what's  going  on
/// there.
/// 
/// ```ignore
/// let g: graph!(X ===A==> X) = Graph::new();
/// ```
/// 
/// Here, we have 3 tokens separated  by  whitespaces:  '`X`',  '`===A==>`'  and  '`X`'.
/// Collectively, they form the _structural pattern_ of a graph.
/// 
/// The first _and_ the last token symbolise vertices of our network  and  show  whether
/// they'll have any [attributes][attrs] or not. Value '`X`' serves as  a  marker  which
/// says: 'We'll _not_ store any attributes in the  vertices'.  Needless  to  say,  this
/// allows CrabNets to optimise memory usage as we know in advance that there'll  be  no
/// need to even declare [attribute collections][attrs2] for any of the vertices.
/// 
/// Note that the first and the last tokens must always be identical! Of course, one can
/// argue that this notation is redundant and can be shortened by  getting  rid  of  the
/// last token but we keep it this way for a better visual presentation.
/// 
/// As it's easy to guess now, the token in the middle symbolises edges of  the  network
/// and their properties.
/// 
/// Look closer at this second token and you'll notice that it looks like 2 arrows  with
/// the letter '`A`' slapped on top of them going from the left 'vertex'  to  the  right
/// 'vertex'. This pair of arrows means that we can have parallel edges,  i.e.  that  we
/// want our graph to be a [multi-graph][kinds], and,  furthermore,  our  edges  can  be
/// [directed][kinds]. Letter '`A`' in the middle, contrary to letter '`X`', means  that
/// our edges _can_ have attributes stored in them.
/// 
/// ## Possible configurations
/// The type of all kinds of graphs supported by  CrabNets  by  default  can  always  be
/// defined with this macro. For example, `graph!(A ---A--- A)` will stand for a  simple
/// undirected   graph   where   both   vertices    and    edges    store    attributes;
/// `graph!(A ---X--> A)` will stand for a simple directed  graph  where  only  vertices
/// have attributes.
/// 
/// > ⚠️ **Warning!**  Milti-graphs  are  currently  under  development,  so  structural
/// patterns with 'double arrows' won't work at this point.
/// 
/// ## Values of generic type parameters
/// When you create a graph will this macro,  the  [generic type parameters][typeparams]
/// of your graph will be defined as follows:
/// * `EdgeAttributeCollectionType` will be substituted with:
///     * [`DynamicDispatchAttributeMap<String>`] if the attribute marker for  edges  is
///       '`A`'.
///     * `()` if the attribute marker for edges is '`X`'.
/// * `EdgeIdType` will be substituted with:
///     * `u8` if the graph is simple (simple graphs don't support  [edge  IDs][edgeids]
///       anyway, this value is purely symbolic).
///     * `usize` if the graph is a multi-graph.
/// * `LocaleType` will be substituted with:
///     * [`SimpleUndirectedLocale`] if the graph is simple and undirected.
///     * [`SimpleDirectedLocale`] if the graph is simple and directed.
///     * `MultiUndirectedLocale` if the graph is a multi-graph  and  undirected  (under
///       development).
///     * `MultiDirectedLocale` if the  graph  is  a  multi-graph  and  directed  (under
///       development).
/// * `VertexAttributeCollectionType` will be substituted with:
///     * [`DynamicDispatchAttributeMap<String>`] if the attribute marker  for  vertices
///       is '`A`'.
///     * `()` if the attribute marker for vertices is '`X`'.
/// * `VertexIdType` will be substituted with `usize`.
/// 
/// Thus, `graph!(A ---X--- A)` expands to
/// 
/// ```ignore
/// Graph<(), u8, SimpleUndirectedLocale<(), DynamicDispatchAttributeMap<String>, usize>, DynamicDispatchAttributeMap<String>, usize>
/// ```
/// 
/// Obviously, the `graph!` macro is much more pleasant to work with...
/// 
/// ## Advanced use
/// You can overwrite the default assignments for many generic type parameters  of  your
/// graph. To do this, enumerate the desired values for the generic type  parameters  in
/// any order separating them from the structural pattern with the word `with`:
/// 
/// ```ignore
/// let g: graph!(X ===X=== X with VertexIdType = i32) = Graph::from_file("a.gnbs").unwrap();
/// ```
/// 
/// If you don't explicitly set the value for some of the generic type parameters, their
/// default values from the previous section will be taken.
/// 
/// If you set the value for a generic type parameter that contradicts the  restrictions
/// set by the structural pattern, this value will be ignored. For example, if  you  try
/// setting the type of the [attribute collection][attrs2] for the vertices of the graph
/// that is declared not to support any vertex attributes:
/// 
/// ```ignore
/// let g: graph!(X ===X=== X
///     with
///         VertexIdType = i32,
///         VertexAttributeCollectionType = MyCollection
/// ) = Graph::from_file("a.gnbs").unwrap();
/// ```
/// 
/// This macro call will expand to the exactly same  graph  type  as  the  previous  one
/// (`VertexAttributeCollectionType = MyCollection` will be ignored).
/// 
/// The only generic type parameter that you'll never be able to change with this  macro
/// is `LocaleType`. If you implement your own [locale], you won't be able to choose  it
/// in `graph!`. If you try setting the value for `LocaleType` in this macro, you'll get
/// an error.
/// 
/// [attrs]: Graph#attributes
/// [attrs2]: attributes::AttributeCollection
/// [edgeids]: Graph#representation-of-graphs
/// [typeparams]: Graph#generic-type-parameters
/// [kinds]: Graph#different-kinds-of-graphs
/// [locale]: locales::Locale
#[macro_export]
macro_rules! graph {
    (X ---X--- X with $($property:ident = $value:ty),+) => {
        Graph<
            /* EdgeAttributeType */ (),
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleUndirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ (),
            /* VertexIdType */ graph_type_recognition_assistant!([$($property = $value),+], VertexIdType, usize)
        >
    };

    (X ---X--- X) => {
        Graph<
            /* EdgeAttributeType */ (),
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleUndirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ (),
            /* VertexIdType */ usize
        >
    };

    (A ---X--- A with $($property:ident = $value:ty),+) => {
        Graph<
            /* EdgeAttributeType */ (),
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleUndirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ graph_type_recognition_assistant!([$($property = $value),+], VertexAttributeCollectionType, DynamicDispatchAttributeMap<String>),
            /* VertexIdType */ graph_type_recognition_assistant!([$($property = $value),+], VertexIdType, usize)
        >
    };

    (A ---X--- A) => {
        Graph<
            /* EdgeAttributeType */ (),
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleUndirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ DynamicDispatchAttributeMap<String>,
            /* VertexIdType */ usize
        >
    };

    (X ---A--- X with $($property:ident = $value:ty),+) => {
        Graph<
            /* EdgeAttributeType */ graph_type_recognition_assistant!([$($property = $value),+], EdgeAttributeType, DynamicDispatchAttributeMap<String>),
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleUndirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ (),
            /* VertexIdType */ graph_type_recognition_assistant!([$($property = $value),+], VertexIdType, usize)
        >
    };

    (X ---A--- X) => {
        Graph<
            /* EdgeAttributeType */ DynamicDispatchAttributeMap<String>,
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleUndirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ (),
            /* VertexIdType */ usize
        >
    };

    (A ---A--- A with $($property:ident = $value:ty),+) => {
        Graph<
            /* EdgeAttributeType */ graph_type_recognition_assistant!([$($property = $value),+], EdgeAttributeType, DynamicDispatchAttributeMap<String>),
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleUndirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ graph_type_recognition_assistant!([$($property = $value),+], VertexAttributeCollectionType, DynamicDispatchAttributeMap<String>),
            /* VertexIdType */ graph_type_recognition_assistant!([$($property = $value),+], VertexIdType, usize)
        >
    };

    (A ---A--- A) => {
        Graph<
            /* EdgeAttributeType */ DynamicDispatchAttributeMap<String>,
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleUndirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ DynamicDispatchAttributeMap<String>,
            /* VertexIdType */ usize
        >
    };

    (X ---X--> X with $($property:ident = $value:ty),+) => {
        Graph<
            /* EdgeAttributeType */ (),
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleDirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ (),
            /* VertexIdType */ graph_type_recognition_assistant!([$($property = $value),+], VertexIdType, usize)
        >
    };

    (X ---X--> X) => {
        Graph<
            /* EdgeAttributeType */ (),
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleDirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ (),
            /* VertexIdType */ usize
        >
    };

    (A ---X--> A with $($property:ident = $value:ty),+) => {
        Graph<
            /* EdgeAttributeType */ (),
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleDirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ graph_type_recognition_assistant!([$($property = $value),+], VertexAttributeCollectionType, DynamicDispatchAttributeMap<String>),
            /* VertexIdType */ graph_type_recognition_assistant!([$($property = $value),+], VertexIdType, usize)
        >
    };

    (A ---X--> A) => {
        Graph<
            /* EdgeAttributeType */ (),
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleDirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ DynamicDispatchAttributeMap<String>,
            /* VertexIdType */ usize
        >
    };

    (X ---A--> X with $($property:ident = $value:ty),+) => {
        Graph<
            /* EdgeAttributeType */ graph_type_recognition_assistant!([$($property = $value),+], EdgeAttributeType, DynamicDispatchAttributeMap<String>),
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleDirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ (),
            /* VertexIdType */ graph_type_recognition_assistant!([$($property = $value),+], VertexIdType, usize)
        >
    };

    (X ---A--> X) => {
        Graph<
            /* EdgeAttributeType */ DynamicDispatchAttributeMap<String>,
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleDirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ (),
            /* VertexIdType */ usize
        >
    };

    (A ---A--> A with $($property:ident = $value:ty),+) => {
        Graph<
            /* EdgeAttributeType */ graph_type_recognition_assistant!([$($property = $value),+], EdgeAttributeType, DynamicDispatchAttributeMap<String>),
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleDirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ graph_type_recognition_assistant!([$($property = $value),+], VertexAttributeCollectionType, DynamicDispatchAttributeMap<String>),
            /* VertexIdType */ graph_type_recognition_assistant!([$($property = $value),+], VertexIdType, usize)
        >
    };

    (A ---A--> A) => {
        Graph<
            /* EdgeAttributeType */ DynamicDispatchAttributeMap<String>,
            /* EdgeIdType */ u8,
            /* LocaleType */ SimpleDirectedLocale<_, _, _>,
            /* VertexAttributeCollectionType */ DynamicDispatchAttributeMap<String>,
            /* VertexIdType */ usize
        >
    };
}





#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn graph_new_xxx_with() {
        let mut g: graph!(X ---X--- X
        with
            VertexIdType = i8
        ) = Graph::new();
        assert_eq!(g.add_v(None), -128);
    }

    #[test]
    fn graph_new_axa() {
        let mut g: graph!(A ---X--- A) = Graph::new();
        assert_eq!(g.add_v(None), 0);
    }

    #[test]
    fn graph_new_axa_with() {
        let mut g: graph!(A ---X--- A
        with
            VertexIdType = i8,
            VertexAttributeCollectionType = ()
        ) = Graph::new();
        assert_eq!(g.add_v(None), -128);
    }

    #[test]
    fn add_degree_delete() {
        // Undirected simple unattributed graph
        let mut g: graph!(X ---X--- X) = Graph::new();
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
}
