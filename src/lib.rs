//! <h2 id="nexusart" style="text-align: center; font-variant: small-caps"><a href="#nexusart">NexusArt</a></h2>
//! 
//! <div style="text-align: center"><b><i>Fast and flexible graph library for Rust</i></b></div>
//! 
//! ## Welcome!
//! NexusArt is one of the few Rust libraries that enable developers to build,  analyse,
//! manipulate and visualise graphs/networks.
//! 
//! ## Features
//! * 123456789





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

    pub type ConditionalType<const C: bool, T: ?Sized, F: ?Sized> = <ConditionalTypeCore<C, T, F> as Conditional>::Type;
}

pub mod errors;

use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet, hash_set},
    fmt::{Debug, Display},
    hash::Hash,
    iter::{Cloned, Map},
    marker::PhantomData,
    ops::AddAssign,
};
use dyn_clone::{clone_trait_object, DynClone};
use errors::{NexusArtError, NexusArtResult};
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
// * ATTRIBUTES                                                                        *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



/// # Dynamic attribute value trait
pub trait AttributeValue
where
    Self: Any + Debug + DynClone + Send + Sync,
{}

impl<AttributeValueType> AttributeValue for AttributeValueType
where
    AttributeValueType: Any + Debug + Clone + Send + Sync,
{}

impl dyn AttributeValue {
    #[inline]
    pub fn is<T: Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }

    pub fn downcast<T: Any>(&self) -> Option<&T> {
        if !self.is::<T>() {
            return None;
        }
        unsafe { Some(&*(self as *const dyn AttributeValue as *const T)) }
    }

    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if !self.is::<T>() {
            return None;
        }
        unsafe { Some(&mut *(self as *mut dyn AttributeValue as *mut T)) }
    }
}

clone_trait_object!(AttributeValue);





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * ATTRIBUTE COLLECTIONS                                                             *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub trait AttributeCollection {
    fn new() -> Self;
}



struct AttributeMap<KeyType>
where
    KeyType: Hash,
{
    attributes: HashMap<KeyType, Box<dyn AttributeValue>>,
}

impl<KeyType> AttributeCollection for AttributeMap<KeyType>
where
    KeyType: Hash,
{
    fn new() -> Self {
        AttributeMap { attributes: HashMap::new() }
    }
}



// ()::AttributeCollection
impl AttributeCollection for () {
    fn new() -> Self {
        ()
    }
}





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
// * LOCALES                                                                           *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



/// # Vertex together with its neighbours
/// 
/// ## Description
/// This trait defines functions for **locales**. Locales are typically associated  with
/// each vertex of a [`Graph`]. They capture local topology of the  network  by  storing
/// all  vertices  adjacent  to  the  given  one  and,  furthermore,  they   store   all
/// [attributes](Graph#attributes) of the given vertex and edges incident on it.
/// 
/// [Structural features](Graph#different-kinds-of-graphs) may differ  from  network  to
/// network. Hence, it might make sense to use a locale with data  structures  optimised
/// for the specific needs of your case.
pub trait Locale<EdgeIdType, VertexIdType>
where
    EdgeIdType: Id,
    VertexIdType: Id,
{
    type VertexIterator<'a>: Iterator<Item = VertexIdType>
    where
        Self: 'a;
    fn add_e(&mut self, id2: VertexIdType, relation: EdgeToVertexRelation, edge_id: Option<EdgeIdType>) -> EdgeIdType;
    fn count_neighbours(&self) -> usize;
    fn count_neighbours_in(&self) -> usize;
    fn count_neighbours_out(&self) -> usize;
    fn count_neighbours_undir(&self) -> usize;
    fn get_incident_es<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>;
    fn iter_incident_es<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a>;
    fn iter_neighbours<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    fn iter_neighbours_in<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    fn iter_neighbours_out<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    fn iter_neighbours_undir<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    fn new() -> Self;
    fn remove_e(&mut self, id2: &VertexIdType, edge_id: &EdgeIdType) -> bool;
    fn remove_neighbour(&mut self, id2: &VertexIdType) -> bool;
}



struct UndirectedSimpleUnattributedLocale<VertexAttributeCollectionType, VertexIdType>
where
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    always_empty_set: HashSet<VertexIdType>,
    attributes: VertexAttributeCollectionType,
    edges: HashSet<VertexIdType>,
}

// UndirectedSimpleUnattributedLocale::Locale
impl<EdgeIdType, VertexAttributeCollectionType, VertexIdType> Locale<EdgeIdType, VertexIdType> for UndirectedSimpleUnattributedLocale<VertexAttributeCollectionType, VertexIdType>
where
    EdgeIdType: Id,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    type VertexIterator<'a> = Cloned<hash_set::Iter<'a, VertexIdType>>
    where
        Self: 'a;

    #[inline]
    fn add_e(&mut self, id2: VertexIdType, _relation: EdgeToVertexRelation, _edge_id: Option<EdgeIdType>) -> EdgeIdType {
        self.edges.insert(id2);
        EdgeIdType::default()
    }

    #[inline]
    fn count_neighbours(&self) -> usize {
        self.edges.len()
    }

    #[inline]
    fn count_neighbours_in(&self) -> usize {
        0
    }

    #[inline]
    fn count_neighbours_out(&self) -> usize {
        0
    }

    #[inline]
    fn count_neighbours_undir(&self) -> usize {
        self.edges.len()
    }

    fn get_incident_es<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>> {
        let mut answer: HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>> = HashSet::with_capacity(self.edges.len());
        for id2 in self.edges.iter() {
            answer.insert(EdgeIteratorItem {
                direction: EdgeDirection::Undirected,
                edge_id: EdgeIdType::default(),
                id1: VertexIdType::default(),
                id2: id2.clone(),
            });
        }
        answer
    }

    #[inline]
    fn iter_incident_es<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a> {
        Box::new(self.edges.iter().map(|id2| EdgeIteratorItem {
            direction: EdgeDirection::Undirected,
            edge_id: EdgeIdType::default(),
            id1: VertexIdType::default(),
            id2: id2.clone(),
        }))
    }

    #[inline]
    fn iter_neighbours<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a> {
        Box::new(self.edges.iter().cloned())
    }

    #[inline]
    fn iter_neighbours_in<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a> {
        Box::new(self.always_empty_set.iter().cloned())
    }

    #[inline]
    fn iter_neighbours_out<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a> {
        Box::new(self.always_empty_set.iter().cloned())
    }

    #[inline]
    fn iter_neighbours_undir<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a> {
        Box::new(self.edges.iter().cloned())
    }

    #[inline]
    fn new() -> Self {
        UndirectedSimpleUnattributedLocale{
            always_empty_set: HashSet::new(),
            attributes: VertexAttributeCollectionType::new(),
            edges: HashSet::new(),
        }
    }

    #[inline]
    fn remove_e(&mut self, id2: &VertexIdType, _edge_id: &EdgeIdType) -> bool {
        self.edges.remove(id2)
    }

    #[inline]
    fn remove_neighbour(&mut self, id2: &VertexIdType) -> bool {
        self.edges.remove(id2)
    }
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * GRAPH CONTAINERS                                                                  *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub trait ImmutableGraphContainer
{
    type EdgeIdType: Id;
    type LocaleType: Locale<Self::EdgeIdType, Self::VertexIdType>;
    type VertexIdType: Id;
    fn unwrap(&self) -> &Graph<Self::EdgeIdType, Self::LocaleType, Self::VertexIdType>;
}

// <T:ImmutableGraphContainer>::BasicImmutableGraph
impl<T> BasicImmutableGraph<T::VertexIdType> for T
where
    T: ImmutableGraphContainer,
{
    #[inline]
    fn contains_v(&self, id: &T::VertexIdType) -> bool {
        self.unwrap().contains_v(id)
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
    fn unwrap(&mut self) -> &mut Graph<Self::EdgeIdType, Self::LocaleType, Self::VertexIdType>;
}

// <T:MutableGraphContainer>::BasicMutableGraph
impl<T> BasicMutableGraph<T::EdgeIdType, T::VertexIdType> for T
where
    T: MutableGraphContainer,
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
    fn remove_e(&mut self, id1: &T::VertexIdType, id2: &T::VertexIdType, edge_id: &T::EdgeIdType) -> NexusArtResult<bool> {
        self.unwrap().remove_e(id1, id2, edge_id)
    }

    #[inline]
    fn remove_v(&mut self, id: &T::VertexIdType) -> bool {
        self.unwrap().remove_v(id)
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
pub trait BasicImmutableGraph<VertexIdType>
where
    VertexIdType: Id,
{
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
pub trait BasicMutableGraph<EdgeIdType, VertexIdType>
where
    Self: BasicImmutableGraph<VertexIdType>,
    EdgeIdType: Id,
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
pub struct Graph<EdgeIdType, LocaleType, VertexIdType>
where
    EdgeIdType: Id,
    LocaleType: Locale<EdgeIdType, VertexIdType>,
    VertexIdType: Id,
{
    edge_list: HashMap<VertexIdType, LocaleType>,
    min_free_vertex_id: VertexIdType,
    phantom: PhantomData<EdgeIdType>,
}

// Graph::Graph
impl<EdgeIdType, LocaleType, VertexIdType> Graph<EdgeIdType, LocaleType, VertexIdType>
where
    EdgeIdType: Id,
    LocaleType: Locale<EdgeIdType, VertexIdType>,
    VertexIdType: Id,
{
    pub fn new() -> Self {
        Graph { edge_list: HashMap::new(), min_free_vertex_id: VertexIdType::default(), phantom: PhantomData }
    }
}

// Graph::BasicImmutableGraph
impl<EdgeIdType, LocaleType, VertexIdType> BasicImmutableGraph<VertexIdType> for Graph<EdgeIdType, LocaleType, VertexIdType>
where
    EdgeIdType: Id,
    LocaleType: Locale<EdgeIdType, VertexIdType>,
    VertexIdType: Id,
{
    #[inline]
    fn contains_v(&self, id: &VertexIdType) -> bool {
        self.edge_list.contains_key(id)
    }

    #[inline]
    fn count_v(&self) -> usize {
        self.edge_list.len()
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
impl<EdgeIdType, LocaleType, VertexIdType> BasicMutableGraph<EdgeIdType, VertexIdType> for Graph<EdgeIdType, LocaleType, VertexIdType>
where
    EdgeIdType: Id,
    LocaleType: Locale<EdgeIdType, VertexIdType>,
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
        for edge in self.edge_list[id].get_incident_es() {
            self.remove_e(&edge.id1, &edge.id2, &edge.edge_id).unwrap();
        }
        self.edge_list.remove(id);
        if self.min_free_vertex_id > *id {
            self.min_free_vertex_id = id.clone();
        }
        true
    }
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * CONVENIENT GRAPH CONSTRUCTION                                                     *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



#[derive(Clone, Copy)]
pub enum GraphProperty{
    VertexIdType,
}



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
            Graph::<u8, UndirectedSimpleUnattributedLocale<(), VertexIdType>, VertexIdType>::new()
        }
    };

    (X ---X--- X) => {
        Graph::<u8, UndirectedSimpleUnattributedLocale<(), usize>, usize>::new()
    };
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_standard_graph_new() {
        let mut g = graph!(X ---X--- X
            with
            GraphProperty::VertexIdType = i8
        );
        assert_eq!(g.add_v(None), -128);
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
    }
}
