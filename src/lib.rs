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
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
    ops::AddAssign,
};
use dyn_clone::{clone_trait_object, DynClone};
use errors::{NexusArtError, NexusArtResult};
use private::ConditionalType;





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * VERTEX ID                                                                         *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



/// # Vertex ID trait
/// ## Description
/// Types that implement `VertexId` can be used as unique identifiers of vertices within
/// a graph. This trait is already implemented for  all  standard  integer  types  (both
/// signed and unsigned).
/// 
/// Types that implement  `VertexId`  must  be  linearly  ordered,  hashable,  copyable,
/// displayable.
pub trait VertexId
where
    Self: Copy + Display + Eq + Hash + Ord,
{
    /// # Default value of the vertex ID
    /// ## Description
    /// This function returns the default value for the first vertex ID. For integers,
    /// this function is implemented to return the minimum value of the respective
    /// integer type.
    /// ## Arguments
    /// None.
    /// ## Returns
    /// `Self` - The default value for the first vertex ID.
    fn default() -> Self;
    /// # Advance the element
    /// ## Description
    /// This function assigns to the callee the following element in the linear order
    /// defined for the type. For integers, this function increases the value by 1.
    /// ## Arguments
    /// * `&mut self` - A mutable reference to the caller.
    /// ## Returns
    /// None.
    fn increment(&mut self);
}

macro_rules! implement_vertex_id_trait_for {
    ($t: ty) => {
        impl VertexId for $t {
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



struct AttributeMap {
    attributes: HashMap<String, Box<dyn AttributeValue>>,
}

impl AttributeCollection for AttributeMap {
    fn new() -> Self {
        AttributeMap { attributes: HashMap::new() }
    }
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * NEIGHBOURHOODS                                                                    *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub enum EdgeToVertexRelation {
    Undirected,
    Ingoing,
    Outgoing,
}



pub trait Neighbourhood<VertexIdType>
where
    VertexIdType: VertexId,
{
    fn add_neighbour(&mut self, id2: VertexIdType, relation: EdgeToVertexRelation);
    fn new() -> Self;
    fn count_neighbours(&self) -> usize;
}



struct UndirectedUnattributedSimpleNeighbourhood<VertexIdType>
where
    VertexIdType: VertexId,
{
    edges: HashSet<VertexIdType>,
}

// UndirectedUnattributedSimpleNeighbourhood::Neighbourhood
impl<VertexIdType> Neighbourhood<VertexIdType> for UndirectedUnattributedSimpleNeighbourhood<VertexIdType>
where
    VertexIdType: VertexId,
{
    #[inline]
    fn add_neighbour(&mut self, id2: VertexIdType, _relation: EdgeToVertexRelation) {
        self.edges.insert(id2);
    }

    #[inline]
    fn new() -> Self {
        UndirectedUnattributedSimpleNeighbourhood { edges: HashSet::new() }
    }

    #[inline]
    fn count_neighbours(&self) -> usize {
        self.edges.len()
    }
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * GRAPH CONTAINERS                                                                  *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub trait ImmutableGraphContainer
{
    type NeighbourhoodType: Neighbourhood<Self::VertexIdType>;
    type VertexIdType: VertexId;
    fn unwrap(&self) -> &Graph<Self::NeighbourhoodType, Self::VertexIdType>;
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
}



pub trait MutableGraphContainer
where
    Self: ImmutableGraphContainer,
{
    fn unwrap(&mut self) -> &mut Graph<Self::NeighbourhoodType, Self::VertexIdType>;
}

// <T:MutableGraphContainer>::BasicMutableGraph
impl<T> BasicMutableGraph<T::VertexIdType> for T
where
    T: MutableGraphContainer,
{
    #[inline]
    fn add_e(&mut self, id1: T::VertexIdType, id2: T::VertexIdType, directed: bool) -> NexusArtResult<()> {
        self.unwrap().add_e(id1, id2, directed)
    }

    #[inline]
    fn add_v(&mut self, id: Option<T::VertexIdType>) -> T::VertexIdType {
        self.unwrap().add_v(id)
    }
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * GRAPH                                                                             *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



/// # Basic immutable functions for graphs
/// ## Description
/// This trait defines functions and methods that give a user low-level access to  graph
/// topology and _never_ change its structure (hence, immutable).
/// 
/// This  trait  is  implemented  for   [`Graph`]   and   any   type   that   implements
/// [`ImmutableGraphContainer`].
pub trait BasicImmutableGraph<VertexIdType>
where
    VertexIdType: VertexId,
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
    /// directly connected to the vertex with ID `id` with _any_ possible edge.
    /// 
    /// If the underlying [`Graph`] `g` is [undirected](Graph#different-kinds-of-graphs),
    /// then                  `g.v_degree(id) == g.v_degree_in(id)`                  and
    /// `g.v_degree(id) == g.v_degree_out(id)`.
    /// 
    /// If the underlying [`Graph`] `g` is  [directed](Graph#different-kinds-of-graphs),
    /// then `g.v_degree(id) == g.v_degree_in(id) + g.v_degree_out(id)`.
    /// 
    /// If the underlying [`Graph`] is a [multi-graph](Graph#different-kinds-of-graphs),
    /// then each parallel edge will be counted separately.
    fn v_degree(&self, id: &VertexIdType) -> NexusArtResult<usize>;
}



/// # Basic mutable functions for graphs
/// ## Description
/// This trait defines functions and methods that give a user low-level access to  graph
/// topology and _may_ change its structure (hence, mutable).
/// 
/// This  trait  is  implemented  for   [`Graph`]   and   any   type   that   implements
/// [`MutableGraphContainer`].
pub trait BasicMutableGraph<VertexIdType>
where
    Self: BasicImmutableGraph<VertexIdType>,
    VertexIdType: VertexId,
{
    /// # Add edge
    /// 
    /// ## Description
    /// Add a new edge between two vertices.
    /// 
    /// ## Arguments
    /// * `&mut self` - a mutable reference to the caller.
    /// * `id1` : `VertexIdType` - the ID of the first vertex.
    /// * `id2` : `VertexIdType` - the ID of the second vertex.
    /// * `directed` : `bool` - flag showing whether the edge should be directed or not.
    /// 
    /// ## Returns
    /// * `NexusArtResult<()>`  -  `Ok(())`  is  returned  when  the  edge   was   added
    /// successfully; `Err(NexusArtError)` is returned when at least one of the vertices
    /// `id1` and `id2` doesn't exist.
    /// 
    /// ## Details
    /// If the underlying  [`Graph`]  is  [undirected](Graph#different-kinds-of-graphs),
    /// then the value of `directed` is ignored: the new edge will be undirected in  any
    /// case.
    /// 
    /// If the underlying  [`Graph`]  is  [simple](Graph#different-kinds-of-graphs)  and
    /// there's already an edge between vertices `id1`  and  `id2`,  then  the  existing
    /// edge will be removed and replaced by the new one.
    /// 
    /// If the underlying [`Graph`] is a [multi-graph](Graph#different-kinds-of-graphs),
    /// existing edges between vertices `id1` and `id2` won't be affected.
    fn add_e(&mut self, id1: VertexIdType, id2: VertexIdType, directed: bool) -> NexusArtResult<()>;
    /// # Add vertex
    /// 
    /// ## Description
    /// Add a new isolated vertex to the graph.
    /// 
    /// ## Arguments
    /// * `&mut self` - a mutable reference to the caller.
    /// * `id` : `Option<VertexIdType>` - if `Some(value)` is passed, a new vertex  with
    /// ID `value` will be created, if a vertex with this  ID  already  exists,  nothing
    /// will happen; if `None` is passed, the ID for  the  new  vertex  will  be  chosen
    /// automatically.
    /// 
    /// ## Returns
    /// * `VertexIdType` - the ID of the new vertex.
    fn add_v(&mut self, id: Option<VertexIdType>) -> VertexIdType;
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
pub struct Graph<NeighbourhoodType, VertexIdType>
where
    NeighbourhoodType: Neighbourhood<VertexIdType>,
    VertexIdType: VertexId,
{
    edge_list: HashMap<VertexIdType, NeighbourhoodType>,
    min_free_vertex_id: VertexIdType,
}

// Graph::Graph
impl<NeighbourhoodType, VertexIdType> Graph<NeighbourhoodType, VertexIdType>
where
    NeighbourhoodType: Neighbourhood<VertexIdType>,
    VertexIdType: VertexId,
{
    pub fn new() -> Self {
        Graph { edge_list: HashMap::new(), min_free_vertex_id: VertexIdType::default() }
    }
}

// Graph::BasicImmutableGraph
impl<NeighbourhoodType, VertexIdType> BasicImmutableGraph<VertexIdType> for Graph<NeighbourhoodType, VertexIdType>
where
    NeighbourhoodType: Neighbourhood<VertexIdType>,
    VertexIdType: VertexId,
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
}

// Graph::BasicMutableGraph
impl<NeighbourhoodType, VertexIdType> BasicMutableGraph<VertexIdType> for Graph<NeighbourhoodType, VertexIdType>
where
    NeighbourhoodType: Neighbourhood<VertexIdType>,
    VertexIdType: VertexId,
{
    fn add_e(&mut self, id1: VertexIdType, id2: VertexIdType, _directed: bool) -> NexusArtResult<()> {
        const FUNCTION_PATH: &str = "Graph::BasicMutableGraph::add_e";
        if self.contains_v(&id1) {
            if self.contains_v(&id2) {
                self.edge_list.get_mut(&id1).unwrap().add_neighbour(id2, EdgeToVertexRelation::Undirected);
                self.edge_list.get_mut(&id2).unwrap().add_neighbour(id1, EdgeToVertexRelation::Undirected);
                Ok(())
            } else {
                Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex {} doesn't exist.", id2)))
            }
        } else {
            Err(NexusArtError::new(FUNCTION_PATH, format!("Vertex {} doesn't exist.", id1)))
        }
    }

    fn add_v(&mut self, id: Option<VertexIdType>) -> VertexIdType {
        let return_value: VertexIdType;
        match id {
            Some(value) => {
                if self.edge_list.contains_key(&value) {
                    return value
                }
                self.edge_list.insert(value.clone(), NeighbourhoodType::new());
                return_value = value;
            },
            None => {
                self.edge_list.insert(self.min_free_vertex_id.clone(), NeighbourhoodType::new());
                return_value = self.min_free_vertex_id.clone();
            },
        }
        while self.edge_list.contains_key(&self.min_free_vertex_id) {
            self.min_free_vertex_id.increment();
        }
        return_value
    }
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * CONVENIENT GRAPH CONSTRUCTION                                                     *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



#[derive(Clone, Copy)]
pub enum GraphProperty{
    NeighbourhoodType,
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
/// As it's easy to guess now, the token in the middle symbolises edges of the network
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
            type NeighbourhoodType = graph_type_recognition_assistant!([$($property = $value),+], GraphProperty::NeighbourhoodType, UndirectedUnattributedSimpleNeighbourhood<VertexIdType>);
            Graph::<NeighbourhoodType, VertexIdType>::new()
        }
    };

    (X ---X--- X) => {
        Graph::<UndirectedUnattributedSimpleNeighbourhood<usize>, usize>::new()
    };
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_standard_graph_new() {
        let mut g = graph!(X ---X--- X
            with
            GraphProperty::VertexIdType = i8,
            GraphProperty::NeighbourhoodType = UndirectedUnattributedSimpleNeighbourhood<i8>
        );
        assert_eq!(g.add_v(None), -128);
    }

    #[test]
    fn add_e() {
        let mut g = graph!(X ---X--- X);
        g.add_v(None);
        g.add_v(None);
        assert!(g.add_e(0, 1, true).is_ok());
        assert!(g.add_e(1, 2, false).is_err());
    }

    #[test]
    fn add_v() {
        let mut g = graph!(X ---X--- X);
        assert_eq!(g.add_v(Some(1)), 1);
        assert_eq!(g.add_v(None), 0);
        assert_eq!(g.add_v(Some(218)), 218);
        assert_eq!(g.add_v(None), 2);
        return;
    }

    #[test]
    fn v_degree() {
        let mut g = graph!(X ---X--- X);
        g.add_v(None);
        g.add_v(None);
        g.add_v(None);
        g.add_v(None);
        g.add_e(0, 1, false).unwrap();
        g.add_e(0, 2, false).unwrap();
        g.add_e(0, 3, false).unwrap();
        assert_eq!(g.v_degree(&0).unwrap(), 3);
        assert_eq!(g.v_degree(&2).unwrap(), 1);
    }
}
