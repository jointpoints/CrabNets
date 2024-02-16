use std::{collections::HashSet, iter::empty};
use crate::{AttributeCollection, EdgeDirection, EdgeIteratorItem, EdgeToVertexRelation, Id};





/// # Vertex together with its neighbours
/// 
/// ## Description
/// This trait defines functions for **locales**. Locales are typically associated  with
/// each vertex of a [`Graph`][graph]. They capture local topology  of  the  network  by
/// storing all vertices adjacent to the given one  and,  furthermore,  they  store  all
/// [attributes][attrs] of the given vertex and edges incident on it.
/// 
/// [Structural features][kinds] may differ from network to  network.  Hence,  it  might
/// make sense to use a locale with data structures optimised for the specific needs  of
/// your case.
/// 
/// [graph]: crate::Graph
/// [attrs]: crate::Graph#attributes
/// [kinds]: crate::Graph#different-kinds-of-graphs
pub trait Locale<EdgeIdType, VertexIdType>
where
    Self: Clone,
    EdgeIdType: Id,
    VertexIdType: Id,
{
    fn add_e(&mut self, id2: VertexIdType, relation: EdgeToVertexRelation, edge_id: Option<EdgeIdType>) -> EdgeIdType;
    fn count_neighbours(&self) -> usize;
    fn count_neighbours_in(&self) -> usize;
    fn count_neighbours_out(&self) -> usize;
    fn count_neighbours_undir(&self) -> usize;
    fn get_incident_e<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>;
    fn get_incident_e_in<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>;
    fn get_incident_e_out<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>;
    fn get_incident_e_undir<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>>;
    fn iter_incident_e<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a>;
    fn iter_neighbours<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    fn iter_neighbours_in<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    fn iter_neighbours_out<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    fn iter_neighbours_undir<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a>;
    fn new() -> Self;
    fn remove_e(&mut self, id2: &VertexIdType, edge_id: &EdgeIdType) -> bool;
    fn remove_neighbour(&mut self, id2: &VertexIdType) -> bool;
}



#[derive(Clone)]
pub struct UndirectedSimpleUnattributedLocale<VertexAttributeCollectionType, VertexIdType>
where
    VertexAttributeCollectionType: AttributeCollection,
    VertexIdType: Id,
{
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

    fn get_incident_e<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>> {
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
    fn get_incident_e_in<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>> {
        HashSet::new()
    }

    #[inline]
    fn get_incident_e_out<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>> {
        HashSet::new()
    }

    fn get_incident_e_undir<'a>(&'a self) -> HashSet<EdgeIteratorItem<EdgeIdType, VertexIdType>> {
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
    fn iter_incident_e<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIteratorItem<EdgeIdType, VertexIdType>> + 'a> {
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
        Box::new(empty::<VertexIdType>())
    }

    #[inline]
    fn iter_neighbours_out<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a> {
        Box::new(empty::<VertexIdType>())
    }

    #[inline]
    fn iter_neighbours_undir<'a>(&'a self) -> Box<dyn Iterator<Item = VertexIdType> + 'a> {
        Box::new(self.edges.iter().cloned())
    }

    #[inline]
    fn new() -> Self {
        UndirectedSimpleUnattributedLocale{
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
