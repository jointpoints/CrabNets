use std::{collections::{HashSet, VecDeque}};

use anyhow::{Result, anyhow};

use crate::{AbsoluteEdgeDirection, Graph, Id, RelativeEdgeDirectionSelector, locales::Locale};










pub struct AdjacentVerticesIter<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    g: &'a Graph<EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>,
    vid: VertexIdType,
    inc_es_iter: Box<dyn Iterator<Item = (EdgeIdType, AbsoluteEdgeDirection)> + 'a>,
}



impl<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> AdjacentVerticesIter<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    pub fn new(g: &'a Graph<EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>, vid: VertexIdType, reldirsel: RelativeEdgeDirectionSelector) -> Result<Self> {
        if !g.vertices.contains_key(&vid) {
            return Err(anyhow!("Can't iterate over vertices adjacent to a non-existing vertex with ID {:?}.", vid));
        }
        let vlocale = &g.vertices.get(&vid).unwrap().locale;
        let inc_es_iter = match reldirsel {
            RelativeEdgeDirectionSelector::Any => vlocale.iter_all(),
            RelativeEdgeDirectionSelector::AnyDirected => Box::new(vlocale.iter_dir_from().chain(vlocale.iter_dir_to())),
            RelativeEdgeDirectionSelector::DirectedFrom => vlocale.iter_dir_from(),
            RelativeEdgeDirectionSelector::DirectedTo => vlocale.iter_dir_to(),
            RelativeEdgeDirectionSelector::Undirected => vlocale.iter_undir(),
        };
        Ok(AdjacentVerticesIter {
            g,
            vid,
            inc_es_iter,
        })
    }
}



impl<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> Iterator for AdjacentVerticesIter<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    type Item = (VertexIdType, &'a VertexWeightType);

    fn next(&mut self) -> Option<Self::Item> {
        match self.inc_es_iter.next() {
            Some((inc_eid, dir)) => {
                let e = match dir {
                    AbsoluteEdgeDirection::Directed => self.g.edges_dir.get(&inc_eid).unwrap(),
                    AbsoluteEdgeDirection::Undirected => self.g.edges_undir.get(&inc_eid).unwrap(),
                };
                if self.vid == e.vid1 {
                    Some((e.vid2, &self.g.vertices.get(&e.vid2).unwrap().weight))
                } else {
                    Some((e.vid1, &self.g.vertices.get(&e.vid1).unwrap().weight))
                }
            },
            None => None,
        }
    }
}










pub struct DFSPreorderIter<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    g: &'a Graph<EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>,
    visited_vs: HashSet<VertexIdType>,
    scheduled_vs: VecDeque<VertexIdType>,
}



impl<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> DFSPreorderIter<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    pub fn new(g: &'a Graph<EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>, vid: VertexIdType) -> Result<Self> {
        if !g.v().contains(vid) {
            return Err(anyhow!("Can't initialise a DFSPreorderIter for a graph because the initial vertex with ID {:?} doesn't exist.", vid));
        }
        Ok(DFSPreorderIter {
            g,
            visited_vs: HashSet::new(),
            scheduled_vs: VecDeque::from([vid]),
        })
    }
}



impl<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType> Iterator for DFSPreorderIter<'a, EdgeIdType, VertexIdType, LocaleType, EdgeWeightType, VertexWeightType>
where
EdgeIdType: Id,
VertexIdType: Id,
LocaleType: Locale<EdgeIdType>,
{
    type Item = (VertexIdType, &'a VertexWeightType);

    fn next(&mut self) -> Option<Self::Item> {
        if self.scheduled_vs.is_empty() {
            return None;
        }
        while let Some(curr_v) = self.scheduled_vs.pop_back() {
            if self.visited_vs.insert(curr_v) {
                self.scheduled_vs.extend(
                    self.g.v().iter_adjacent(curr_v, RelativeEdgeDirectionSelector::DirectedFrom).unwrap()
                        .chain(self.g.v().iter_adjacent(curr_v, RelativeEdgeDirectionSelector::Undirected).unwrap())
                        .map(|(vid, _)| vid)
                );
                return Some((curr_v, self.g.v().get(curr_v).unwrap()));
            }
        }
        None
    }
}










#[cfg(test)]
mod tests {
    use crate::essentials::*;
    use super::*;

    #[test]
    fn undirected_path() {
        let mut g: graph!{ [u8|()] ---[u8|()]--- } = Graph::new();
        g.v_mut().insert(Some(0), ()).unwrap();
        g.v_mut().insert(Some(1), ()).unwrap();
        g.v_mut().insert(Some(2), ()).unwrap();
        g.v_mut().insert(Some(3), ()).unwrap();
        g.v_mut().insert(Some(4), ()).unwrap();
        g.e_mut().insert(None, 0, 1, AbsoluteEdgeDirection::Undirected, ()).unwrap();
        g.e_mut().insert(None, 1, 2, AbsoluteEdgeDirection::Undirected, ()).unwrap();
        g.e_mut().insert(None, 2, 3, AbsoluteEdgeDirection::Undirected, ()).unwrap();
        g.e_mut().insert(None, 3, 4, AbsoluteEdgeDirection::Undirected, ()).unwrap();
        assert_eq!(
            DFSPreorderIter::new(&g, 0).unwrap().map(|(vid, _)| vid).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4]
        );
    }
}
