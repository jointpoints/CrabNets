use std::{collections::{HashMap, HashSet}, iter::empty};
use crate::{attribute::AttributeCollection, EdgeDirection, EdgeIteratorItem, Id};





/// # Vertex and edges incident on it
/// 
/// ## Description
/// This trait defines an interface for **locales**. Locales  are  typically  associated
/// with each vertex of a [`Graph`][graph]. They capture local topology of  the  network
/// by storing all vertices adjacent to the given one and, furthermore, they  store  all
/// [attributes][attrs] of the given vertex and edges incident on it.
/// 
/// [Structural features][kinds] may differ from network to  network.  Hence,  it  might
/// make sense to use a locale with data structures optimised for the specific needs  of
/// your case.
/// 
/// [graph]: crate::Graph
/// [attrs]: crate::Graph#attributes
/// [kinds]: crate::Graph#different-kinds-of-graphs
pub trait Locale<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>
where
    Self: Clone + Default,
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    /// # Add edge
    /// 
    /// ## Description
    /// Add a new edge to the locale.
    /// 
    /// ## Arguments
    /// * `&mut self` - a mutable reference to the caller.
    /// * `id2` : `VertexIdType` - the ID of a vertex an edge to which should be added.
    /// * `relation` : `EdgeToVertexRelation` - shows whether the  new  edge  should  be
    /// incoming, outgoing or undirected.
    /// * `edge_id` : `Option<EdgeIdType>` - if edge IDs are supported  (see [Details]),
    /// the new edge will be assigned the ID `value` in case `Some(value)` is  given  or
    /// the ID will be chosed automatically in case `None` is given.
    /// * `store_edge_attributes` :  `bool`  -  shows  whether  this  locale  should  be
    /// responsible for storing the edge attributes of this edge (see [Details]).
    /// 
    /// ## Returns
    /// * `EdgeIdType` - the ID of the new edge.
    /// 
    /// <div id="add-e-details" style="margin-top: -15px;">
    /// 
    /// ## Details
    /// 
    /// </div>
    /// 
    /// If the underlying graph is [simple][kinds],  the  value  of  `edge_id`  will  be
    /// ignored as simple graphs don't support edge IDs.
    /// 
    /// Note that each edge is stored twice: in the locale of the first  vertex  and  in
    /// the locale of the second vertex. This memory sacrifice is made to accelerate the
    /// enumeration of adjacent vertices, which is deemed to be an  important  operation
    /// for graph analysis. To avoid additional struggles with synchronising the  values
    /// of edge attributes, only one locale is chosen to be responsible for storing  the
    /// corresponding instance of the [attribute collection][attrs]. As a rule,  if  the
    /// edge is undirected, the instance is stored in the locale  of  a  vertex  with  a
    /// smaller ID, and if the edge is directed, the instance is stored in the locale of
    /// a source vertex. This behaviour is defined in [`BasicMutableGraph::add_e`][add_e].
    /// 
    /// [attrs]: crate::attribute::AttributeCollection
    /// [Details]: #add-e-details
    /// [add_e]: crate::BasicMutableGraph::add_e
    /// [kinds]: crate::Graph#different-kinds-of-graphs
    fn add_e(&mut self, id2: VertexIdType, relation: EdgeToVertexRelation, edge_id: Option<EdgeIdType>, store_edge_attributes: bool) -> EdgeIdType;
    /// # Number of incident edges
    /// 
    /// ## Description
    /// Get the number of edges incident on the vertex associated with this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `usize` - the number of incident edges.
    /// 
    /// ## Details
    /// Each parallel edge is counted  separately.  Mind  the  difference  between  this
    /// method and [`Locale::count_neighbours`].
    fn count_incident_e(&self) -> usize;
    /// # Number of incoming incident edges
    /// 
    /// ## Description
    /// Get the number of incoming edges incident on the  vertex  associated  with  this
    /// locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `usize` - the number of incoming incident edges.
    /// 
    /// ## Details
    /// Each parallel edge is counted  separately.  Mind  the  difference  between  this
    /// method and [`Locale::count_neighbours_in`].
    fn count_incident_e_in(&self) -> usize;
    /// # Number of outgoing incident edges
    /// 
    /// ## Description
    /// Get the number of outgoing edges incident on the  vertex  associated  with  this
    /// locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `usize` - the number of outgoing incident edges.
    /// 
    /// ## Details
    /// Each parallel edge is counted  separately.  Mind  the  difference  between  this
    /// method and [`Locale::count_neighbours_out`].
    fn count_incident_e_out(&self) -> usize;
    /// # Number of undirected incident edges
    /// 
    /// ## Description
    /// Get the number of undirected edges incident on the vertex associated  with  this
    /// locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `usize` - the number of undirected incident edges.
    /// 
    /// ## Details
    /// Each parallel edge is counted  separately.  Mind  the  difference  between  this
    /// method and [`Locale::count_neighbours_undir`].
    fn count_incident_e_undir(&self) -> usize;
    /// # Number of neighbours
    /// 
    /// ## Description
    /// Get the number of vertices adjacent to the vertex associated with this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    ///
    /// ## Returns
    /// * `usize` - the number of adjacent vertices.
    /// 
    /// ## Details
    /// Each adjacent vertex is only counted once, regardless of the number of  parallel
    /// edges connecting it and their direction. Mind the difference between this method
    /// and [`Locale::count_incident_e`].
    fn count_neighbours(&self) -> usize;
    /// # Number of 'incoming' neighbours
    /// 
    /// ## Description
    /// Get the number of vertices that serve as the source of  at  least  one  directed
    /// edge connecting them to the vertex associated with this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `usize` - the number of 'incoming' adjacent vertices.
    /// 
    /// ## Details
    /// Each adjacent vertex is only counted once, regardless of the number of  parallel
    /// edges connecting it.
    fn count_neighbours_in(&self) -> usize;
    /// # Number of 'outgoing' neighbours
    /// 
    /// ## Description
    /// Get the number of vertices that serve as the target of  at  least  one  directed
    /// edge connecting them to the vertex associated with this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `usize` - the number of 'outgoing' adjacent vertices.
    /// 
    /// ## Details
    /// Each adjacent vertex is only counted once, regardless of the number of  parallel
    /// edges connecting it.
    fn count_neighbours_out(&self) -> usize;
    /// # Number of 'undirected' neighbours
    /// 
    /// ## Description
    /// Get  the  number  of  adjacent  vertices  that  are  connected  to  the   vertex
    /// associated with this locale by at least one undirected edge.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `usize` - the number of 'undirected' adjacent vertices.
    /// 
    /// ## Details
    /// Each adjacent vertex is only counted once, regardless of the number of  parallel
    /// edges connecting it.
    fn count_neighbours_undir(&self) -> usize;
    /// # Immutable reference to edge attributes
    /// 
    /// ## Description
    /// Get an immutable reference to the [attribute collection][attrs] of the  specific
    /// edge incident on the vertex associated with this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// * `id2` : `&VertexIdType` - an immutable reference to the  ID  of  the  required
    /// adjacent vertex.
    /// * `edge_id` : `&EdgeIdType` - an immutable reference to the ID of  the  required
    /// edge.
    /// 
    /// ## Returns
    /// * `Option<&EdgeAttributeCollectionType>` -  `Some(value)`  is  returned  if  the
    /// required edge was found and this locale is responsible for storing its attribute
    /// collection (see [`Locale::add_e`] for more details).
    /// 
    /// [attrs]: crate::attribute::AttributeCollection
    /// [e_attrs]: crate::BasicImmutableGraph::e_attrs
    /// [`Locale::add_e`]: #add-e-details
    fn e_attrs(&self, id2: &VertexIdType, edge_id: &EdgeIdType) -> Option<&EdgeAttributeCollectionType>;
    /// # Mutable reference to edge attributes
    /// 
    /// ## Description
    /// Get a mutable reference to the  [attribute collection][attrs]  of  the  specific
    /// edge incident on the vertex associated with this locale.
    /// 
    /// ## Arguments
    /// * `&mut self` - a mutable reference to the caller.
    /// * `id2` : `&VertexIdType` - an immutable reference to the  ID  of  the  required
    /// adjacent vertex.
    /// * `edge_id` : `&EdgeIdType` - an immutable reference to the ID of  the  required
    /// edge.
    /// 
    /// ## Returns
    /// * `Option<&mut EdgeAttributeCollectionType>` - `Some(value)` is returned if  the
    /// required edge was found and this locale is responsible for storing its attribute
    /// collection (see [`Locale::add_e`] for more details).
    /// 
    /// [attrs]: crate::attribute::AttributeCollection
    /// [e_attrs_mut]: crate::BasicMutableGraph::e_attrs_mut
    fn e_attrs_mut(&mut self, id2: &VertexIdType, edge_id: &EdgeIdType) -> Option<&mut EdgeAttributeCollectionType>;
    /// # Edge direction
    /// 
    /// ## Description
    /// Get a direction of an edge from this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// * `id2` : `&VertexIdType` - an immutable reference to the  ID  of  the  required
    /// adjacent vertex.
    /// * `edge_id` : `&EdgeIdType` - an immutable reference to the ID of  the  required
    /// edge.
    /// 
    /// ## Returns
    /// * `Option<EdgeDirection>` - `Some(value)` is returned if the required  edge  was
    /// found; `None` is returned otherwise.
    fn e_direction(&self, id2: &VertexIdType, edge_id: &EdgeIdType) -> Option<EdgeDirection>;
    /// # Set of incident edges
    /// 
    /// ## Description
    /// Get a hash set of all edges in this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>` - a hash  set  with  the
    /// information about all edges incident on the vertex associated with this locale.
    fn incident_e<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>;
    /// # Set of incoming incident edges
    /// 
    /// ## Description
    /// Get a hash set of all incoming edges in this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>` - a hash  set  with  the
    /// information about all incoming edges incident on the vertex associated with this
    /// locale.
    fn incident_e_in<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>;
    /// # Set of outgoing incident edges
    /// 
    /// ## Description
    /// Get a hash set of all outgoing edges in this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>` - a hash  set  with  the
    /// information about all outgoing edges incident on the vertex associated with this
    /// locale.
    fn incident_e_out<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>;
    /// # Set of undirected incident edges
    /// 
    /// ## Description
    /// Get a hash set of all undirected edges in this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>` - a hash  set  with  the
    /// information about all undirected edges incident on the  vertex  associated  with
    /// this locale.
    fn incident_e_undir<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>;
    /// # Iterate over incident edges
    /// 
    /// ## Description
    /// Iterate over all edges in this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// *  `Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>>>`  -  an
    /// iterator over all edges incident on the vertex associated with this locale.
    fn iter_incident_e<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a>
    where
        EdgeIdType: 'a,
        VertexIdType: 'a;
    /// # Iterate over incoming incident edges
    /// 
    /// ## Description
    /// Iterate over all incoming edges in this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// *  `Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>>>`  -  an
    /// iterator over all incoming edges incident on the  vertex  associated  with  this
    /// locale.
    fn iter_incident_e_in<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a>
    where
        EdgeIdType: 'a,
        VertexIdType: 'a;
    /// # Iterate over outgoing incident edges
    /// 
    /// ## Description
    /// Iterate over all outgoing edges in this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// *  `Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>>>`  -  an
    /// iterator over all outgoing edges incident on the  vertex  associated  with  this
    /// locale.
    fn iter_incident_e_out<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a>
    where
        EdgeIdType: 'a,
        VertexIdType: 'a;
    /// # Iterate over undirected incident edges
    /// 
    /// ## Description
    /// Iterate over all undirected edges in this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// *  `Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>>>`  -  an
    /// iterator over all undirected edges incident on the vertex associated  with  this
    /// locale.
    fn iter_incident_e_undir<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a>
    where
        EdgeIdType: 'a,
        VertexIdType: 'a;
    /// # Iterate over neighbours
    /// 
    /// ## Description
    /// Iterate over all vertices adjacent to the vertex associated with this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `Box<dyn Iterator<Item = VertexIdType>>`  -  an  iterator  over  all  adjacent
    /// vertices.
    fn iter_neighbours<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    /// # Iterate over 'incoming' neighbours
    /// 
    /// ## Description
    /// Iterate over all vertices that serve as a source of at least one  directed  edge
    /// connecting them to the vertex associated with this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `Box<dyn Iterator<Item = VertexIdType>>` - an  iterator  over  all  'incoming'
    /// adjacent vertices.
    fn iter_neighbours_in<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    /// # Iterate over 'outgoing' neighbours
    /// 
    /// ## Description
    /// Iterate over all vertices that serve as a target of at least one  directed  edge
    /// connecting them to the vertex associated with this locale.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `Box<dyn Iterator<Item = VertexIdType>>` - an  iterator  over  all  'outgoing'
    /// adjacent vertices.
    fn iter_neighbours_out<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    /// # Iterate over 'undirected' neighbours
    /// 
    /// ## Description
    /// Iterate over all vertices that are connected to the vertex associated with  this
    /// locale by at least one undirected edge.
    /// 
    /// ## Arguments
    /// * `&self` - an immutable reference to the caller.
    /// 
    /// ## Returns
    /// * `Box<dyn Iterator<Item = VertexIdType>>` - an iterator over  all  'undirected'
    /// adjacent vertices.
    fn iter_neighbours_undir<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    fn new() -> Self;
    fn remove_e(&mut self, id2: &VertexIdType, edge_id: &EdgeIdType) -> bool;
    fn remove_neighbour(&mut self, id2: &VertexIdType) -> bool;
    fn v_attrs(&self) -> &VertexAttributeCollectionType;
    fn v_attrs_mut(&mut self) -> &mut VertexAttributeCollectionType;
}



pub enum EdgeToVertexRelation {
    Undirected,
    Incoming,
    Outcoming,
}



/// # Locale for undirected simple graphs
/// 
/// ## Description
/// This locale optimises memory consumption for undirected simple graphs. See
/// [`Graph`] for more details.
/// 
/// [`Graph`]: crate::Graph#different-kinds-of-graphs
#[derive(Clone, Default)]
pub struct UndirectedSimpleLocale<EdgeAttributeCollectionType, VertexAttributeCollectionType, VertexIdType>
where
    EdgeAttributeCollectionType: AttributeCollection,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    attributes: VertexAttributeCollectionType,
    edges: HashMap<VertexIdType, Option<EdgeAttributeCollectionType>>,
}

// UndirectedSimpleLocale::Locale
impl<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType> Locale<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType> for UndirectedSimpleLocale<EdgeAttributeCollectionType, VertexAttributeCollectionType, VertexIdType>
where
    EdgeAttributeCollectionType: AttributeCollection,
    EdgeIdType: Id,
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
    #[inline]
    fn add_e(&mut self, id2: VertexIdType, _relation: EdgeToVertexRelation, _edge_id: Option<EdgeIdType>, store_edge_attributes: bool) -> EdgeIdType {
        self.edges.insert(id2, if store_edge_attributes { Some(EdgeAttributeCollectionType::new()) } else { None });
        EdgeIdType::default()
    }

    #[inline]
    fn count_incident_e(&self) -> usize {
        self.edges.len()
    }

    #[inline]
    fn count_incident_e_in(&self) -> usize {
        0
    }

    #[inline]
    fn count_incident_e_out(&self) -> usize {
        0
    }

    #[inline]
    fn count_incident_e_undir(&self) -> usize {
        self.edges.len()
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

    #[inline]
    fn e_attrs(&self, id2: &VertexIdType, _edge_id: &EdgeIdType) -> Option<&EdgeAttributeCollectionType> {
        match self.edges.get(id2) {
            Some(value) => value.as_ref(),
            None => None,
        }
    }

    #[inline]
    fn e_attrs_mut(&mut self, id2: &VertexIdType, _edge_id: &EdgeIdType) -> Option<&mut EdgeAttributeCollectionType> {
        match self.edges.get_mut(id2) {
            Some(value) => value.as_mut(),
            None => None,
        }
    }

    #[inline]
    fn e_direction(&self, id2: &VertexIdType, _edge_id: &EdgeIdType) -> Option<EdgeDirection> {
        if self.edges.contains_key(id2) {
            Some(EdgeDirection::Undirected)
        } else {
            None
        }
    }

    fn incident_e<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>> {
        let mut answer: HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>> = HashSet::with_capacity(self.edges.len());
        for id2 in self.edges.keys() {
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
    fn incident_e_in<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>> {
        HashSet::new()
    }

    #[inline]
    fn incident_e_out<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>> {
        HashSet::new()
    }

    fn incident_e_undir<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>> {
        let mut answer: HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>> = HashSet::with_capacity(self.edges.len());
        for id2 in self.edges.keys() {
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
    fn iter_incident_e<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a>
    where
        EdgeIdType: 'a,
        VertexIdType: 'a,
    {
        Box::new(self.edges.keys().map(|id2| EdgeIteratorItem {
            direction: EdgeDirection::Undirected,
            edge_id: EdgeIdType::default(),
            id1: VertexIdType::default(),
            id2: id2.clone(),
        }))
    }

    #[inline]
    fn iter_incident_e_in<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a>
    where
        EdgeIdType: 'a,
        VertexIdType: 'a,
    {
        Box::new(empty())
    }

    #[inline]
    fn iter_incident_e_out<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a>
    where
        EdgeIdType: 'a,
        VertexIdType: 'a,
    {
        Box::new(empty())
    }

    #[inline]
    fn iter_incident_e_undir<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a>
    where
        EdgeIdType: 'a,
        VertexIdType: 'a,
    {
        Box::new(self.edges.keys().map(|id2| EdgeIteratorItem {
            direction: EdgeDirection::Undirected,
            edge_id: EdgeIdType::default(),
            id1: VertexIdType::default(),
            id2: id2.clone(),
        }))
    }

    #[inline]
    fn iter_neighbours<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a> {
        Box::new(self.edges.keys().cloned())
    }

    #[inline]
    fn iter_neighbours_in<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a> {
        Box::new(empty())
    }

    #[inline]
    fn iter_neighbours_out<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a> {
        Box::new(empty())
    }

    #[inline]
    fn iter_neighbours_undir<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a> {
        Box::new(self.edges.keys().cloned())
    }

    #[inline]
    fn new() -> Self {
        UndirectedSimpleLocale{
            attributes: VertexAttributeCollectionType::new(),
            edges: HashMap::new(),
        }
    }

    #[inline]
    fn remove_e(&mut self, id2: &VertexIdType, _edge_id: &EdgeIdType) -> bool {
        self.edges.remove(id2).is_some()
    }

    #[inline]
    fn remove_neighbour(&mut self, id2: &VertexIdType) -> bool {
        self.edges.remove(id2).is_some()
    }

    #[inline]
    fn v_attrs(&self) -> &VertexAttributeCollectionType {
        &self.attributes
    }

    #[inline]
    fn v_attrs_mut(&mut self) -> &mut VertexAttributeCollectionType {
        &mut self.attributes
    }
}
