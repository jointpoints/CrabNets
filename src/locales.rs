use crate::{Id, RelativeEdgeDirection};
use std::collections::HashSet;

pub trait Locale<EdgeIdType>
where
    Self: Clone,
    EdgeIdType: Id,
{
    const IS_DIRECTED: bool;
    const IS_SIMPLE: bool;
    fn insert(&mut self, eid: EdgeIdType, reldir: RelativeEdgeDirection);
    fn iter_all(&self) -> Box<dyn Iterator<Item = EdgeIdType> + '_>;
    fn count_all(&self) -> usize;
    fn count_dir_from(&self) -> usize;
    fn count_dir_to(&self) -> usize;
    fn count_undir(&self) -> usize;
    fn new() -> Self;
    fn remove(&mut self, eid: &EdgeIdType);
}

#[derive(Clone)]
pub struct UndirectedSimpleLocale<EdgeIdType>
where
    EdgeIdType: Id,
{
    pub undirected_edges: HashSet<EdgeIdType>,
}

impl<EdgeIdType> Locale<EdgeIdType> for UndirectedSimpleLocale<EdgeIdType>
where
    EdgeIdType: Id,
{
    const IS_DIRECTED: bool = false;

    const IS_SIMPLE: bool = true;

    #[inline(always)]
    fn insert(&mut self, eid: EdgeIdType, _reldir: RelativeEdgeDirection) {
        self.undirected_edges.insert(eid);
    }

    fn iter_all(&self) -> Box<dyn Iterator<Item = EdgeIdType> + '_> {
        Box::new(self.undirected_edges.iter().copied())
    }

    #[inline(always)]
    fn count_all(&self) -> usize {
        self.undirected_edges.len()
    }

    #[inline(always)]
    fn count_dir_from(&self) -> usize {
        0
    }

    #[inline(always)]
    fn count_dir_to(&self) -> usize {
        0
    }

    #[inline(always)]
    fn count_undir(&self) -> usize {
        self.undirected_edges.len()
    }

    #[inline(always)]
    fn new() -> Self {
        Self {
            undirected_edges: HashSet::new(),
        }
    }

    #[inline(always)]
    fn remove(&mut self, eid: &EdgeIdType) {
        self.undirected_edges.remove(eid);
    }
}
