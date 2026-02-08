use std::{collections::{HashSet, VecDeque}};

use anyhow::{Result, anyhow};

use crate::{AbsoluteEdgeDirection, Graph, Id, RelativeEdgeDirectionSelector, locales::Locale};










pub struct AdjacentVerticesIter<'a, EId, VId, L, EWgt, VWgt>
where
EId: Id,
VId: Id,
L: Locale<EId>,
{
    g: &'a Graph<EId, VId, L, EWgt, VWgt>,
    vid: VId,
    inc_es_iter: Box<dyn Iterator<Item = (EId, AbsoluteEdgeDirection)> + 'a>,
}



impl<'a, EId, VId, L, EWgt, VWgt> AdjacentVerticesIter<'a, EId, VId, L, EWgt, VWgt>
where
EId: Id,
VId: Id,
L: Locale<EId>,
{
    pub fn new(
        g: &'a Graph<EId, VId, L, EWgt, VWgt>,
        vid: VId,
        reldirsel: RelativeEdgeDirectionSelector
    ) -> Result<Self> {
        if !g.vertices.contains_key(&vid) {
            return Err(anyhow!("Can't iterate over vertices adjacent to a non-existing vertex with ID {:?}.", vid));
        }
        let vlocale = &g.vertices.get(&vid).unwrap().locale;
        let inc_es_iter = match reldirsel {
            RelativeEdgeDirectionSelector::Any => vlocale.iter_all(),
            RelativeEdgeDirectionSelector::AnyDirected => Box::new(
                vlocale.iter_dir_from().chain(vlocale.iter_dir_to())
            ),
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



impl<'a, EId, VId, L, EWgt, VWgt> Iterator for AdjacentVerticesIter<'a, EId, VId, L, EWgt, VWgt>
where
EId: Id,
VId: Id,
L: Locale<EId>,
{
    type Item = VId;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inc_es_iter.next() {
            Some((inc_eid, dir)) => {
                let e = match dir {
                    AbsoluteEdgeDirection::Directed => self.g.edges_dir.get(&inc_eid).unwrap(),
                    AbsoluteEdgeDirection::Undirected => self.g.edges_undir.get(&inc_eid).unwrap(),
                };
                if self.vid == e.vid1 {
                    Some(e.vid2)
                } else {
                    Some(e.vid1)
                }
            },
            None => None,
        }
    }
}










pub struct DFSPostorderIter<'a, EId, VId, L, EWgt, VWgt>
where
EId: Id,
VId: Id,
L: Locale<EId>,
{
    g: &'a Graph<EId, VId, L, EWgt, VWgt>,
    in_progress_vids: HashSet<VId>,
    visited_vids: HashSet<VId>,
    scheduled_vids: VecDeque<VId>,
}



impl<'a, EId, VId, L, EWgt, VWgt> DFSPostorderIter<'a, EId, VId, L, EWgt, VWgt>
where
EId: Id,
VId: Id,
L: Locale<EId>,
{
    pub fn new(g: &'a Graph<EId, VId, L, EWgt, VWgt>, vid: VId) -> Result<Self> {
        if !g.v().contains(vid) {
            return Err(anyhow!(
                "Can't initialise a DFSPostorderIter for a graph because the initial vertex with ID {:?} doesn't \
                exist.",
                vid
            ));
        }
        Ok(DFSPostorderIter {
            g,
            in_progress_vids: HashSet::new(),
            visited_vids: HashSet::new(),
            scheduled_vids: VecDeque::from([vid]),
        })
    }
}



impl<'a, EId, VId, L, EWgt, VWgt> Iterator for DFSPostorderIter<'a, EId, VId, L, EWgt, VWgt>
where
EId: Id,
VId: Id,
L: Locale<EId>,
{
    type Item = VId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.scheduled_vids.is_empty() {
            return None;
        }
        while let Some(&curr_vid) = self.scheduled_vids.back() {
            if self.visited_vids.contains(&curr_vid) {
                self.scheduled_vids.pop_back().unwrap();
                continue;
            }
            self.in_progress_vids.insert(curr_vid);
            let unseen_adjvids = self.g.v()
                .iter_adjacent(curr_vid, RelativeEdgeDirectionSelector::DirectedFrom).unwrap()
                .chain(self.g.v().iter_adjacent(curr_vid, RelativeEdgeDirectionSelector::Undirected).unwrap())
                .filter(|adjvid| !self.visited_vids.contains(&adjvid) && !self.in_progress_vids.contains(&adjvid))
                .collect::<Vec<_>>();
            if unseen_adjvids.is_empty() {
                self.in_progress_vids.remove(&curr_vid);
                self.visited_vids.insert(curr_vid);
                return self.scheduled_vids.pop_back();
            }
            self.scheduled_vids.extend(unseen_adjvids.into_iter());
        }
        None
    }
}










pub struct DFSPreorderIter<'a, EId, VId, L, EWgt, VWgt>
where
EId: Id,
VId: Id,
L: Locale<EId>,
{
    g: &'a Graph<EId, VId, L, EWgt, VWgt>,
    visited_vids: HashSet<VId>,
    scheduled_vids: VecDeque<VId>,
}



impl<'a, EId, VId, L, EWgt, VWgt> DFSPreorderIter<'a, EId, VId, L, EWgt, VWgt>
where
EId: Id,
VId: Id,
L: Locale<EId>,
{
    pub fn new(g: &'a Graph<EId, VId, L, EWgt, VWgt>, vid: VId) -> Result<Self> {
        if !g.v().contains(vid) {
            return Err(anyhow!(
                "Can't initialise a DFSPreorderIter for a graph because the initial vertex with ID {:?} doesn't \
                exist.",
                vid
            ));
        }
        Ok(DFSPreorderIter {
            g,
            visited_vids: HashSet::new(),
            scheduled_vids: VecDeque::from([vid]),
        })
    }
}



impl<'a, EId, VId, L, EWgt, VWgt> Iterator for DFSPreorderIter<'a, EId, VId, L, EWgt, VWgt>
where
EId: Id,
VId: Id,
L: Locale<EId>,
{
    type Item = VId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.scheduled_vids.is_empty() {
            return None;
        }
        while let Some(curr_v) = self.scheduled_vids.pop_back() {
            if self.visited_vids.insert(curr_v) {
                self.scheduled_vids.extend(
                    self.g.v().iter_adjacent(curr_v, RelativeEdgeDirectionSelector::DirectedFrom).unwrap()
                        .chain(self.g.v().iter_adjacent(curr_v, RelativeEdgeDirectionSelector::Undirected).unwrap())
                );
                return Some(curr_v);
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
    fn iters_for_null() {
        let g: graph!{ [u8|()] ---[u8|()]--- } = Graph::new();
        assert!(AdjacentVerticesIter::new(&g, 0, RelativeEdgeDirectionSelector::Any).is_err());
        assert!(AdjacentVerticesIter::new(&g, 0, RelativeEdgeDirectionSelector::AnyDirected).is_err());
        assert!(AdjacentVerticesIter::new(&g, 0, RelativeEdgeDirectionSelector::DirectedFrom).is_err());
        assert!(AdjacentVerticesIter::new(&g, 0, RelativeEdgeDirectionSelector::DirectedTo).is_err());
        assert!(AdjacentVerticesIter::new(&g, 0, RelativeEdgeDirectionSelector::Undirected).is_err());
        assert!(DFSPostorderIter::new(&g, 0).is_err());
        assert!(DFSPreorderIter::new(&g, 0).is_err());
    }

    #[test]
    fn iters_for_one_vertex() {
        let mut g: graph!{ [u8|()] ---[u8|()]--- } = Graph::new();
        g.v_mut().insert(Some(0), ()).unwrap();
        assert_eq!(
            AdjacentVerticesIter::new(&g, 0, RelativeEdgeDirectionSelector::Any).unwrap().collect::<Vec<_>>(),
            vec![]
        );
        assert_eq!(
            AdjacentVerticesIter::new(&g, 0, RelativeEdgeDirectionSelector::AnyDirected).unwrap().collect::<Vec<_>>(),
            vec![]
        );
        assert_eq!(
            AdjacentVerticesIter::new(&g, 0, RelativeEdgeDirectionSelector::DirectedFrom).unwrap().collect::<Vec<_>>(),
            vec![]
        );
        assert_eq!(
            AdjacentVerticesIter::new(&g, 0, RelativeEdgeDirectionSelector::DirectedTo).unwrap().collect::<Vec<_>>(),
            vec![]
        );
        assert_eq!(
            AdjacentVerticesIter::new(&g, 0, RelativeEdgeDirectionSelector::Undirected).unwrap().collect::<Vec<_>>(),
            vec![]
        );
        assert_eq!(DFSPostorderIter::new(&g, 0).unwrap().collect::<Vec<_>>(), vec![0]);
        assert_eq!(DFSPreorderIter::new(&g, 0).unwrap().collect::<Vec<_>>(), vec![0]);
    }

    #[test]
    fn iters_for_undirected_path() {
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
        // AdjacetVerticesIter
        let correct_answers = HashSet::from([
            vec![1, 3],
            vec![3, 1],
        ]);
        assert!(correct_answers.contains(
            &AdjacentVerticesIter::new(&g, 2, RelativeEdgeDirectionSelector::Any).unwrap().collect::<Vec<_>>()
        ));
        // DFSPostorderIter
        assert_eq!(
            DFSPostorderIter::new(&g, 0).unwrap().collect::<Vec<_>>(),
            vec![4, 3, 2, 1, 0]
        );
        // DFSPreorderIter
        assert_eq!(
            DFSPreorderIter::new(&g, 0).unwrap().collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn dfs_preorder_undirected_pentagon_with_path() {
        let mut g: graph!{ [u8|()] ---[u8|()]--- } = Graph::new();
        g.v_mut().insert(Some(0), ()).unwrap();
        g.v_mut().insert(Some(1), ()).unwrap();
        g.v_mut().insert(Some(2), ()).unwrap();
        g.v_mut().insert(Some(3), ()).unwrap();
        g.v_mut().insert(Some(4), ()).unwrap();
        g.v_mut().insert(Some(5), ()).unwrap();
        g.e_mut().insert(None, 0, 1, AbsoluteEdgeDirection::Undirected, ()).unwrap();
        g.e_mut().insert(None, 0, 2, AbsoluteEdgeDirection::Undirected, ()).unwrap();
        g.e_mut().insert(None, 0, 3, AbsoluteEdgeDirection::Undirected, ()).unwrap();
        g.e_mut().insert(None, 1, 4, AbsoluteEdgeDirection::Undirected, ()).unwrap();
        g.e_mut().insert(None, 2, 5, AbsoluteEdgeDirection::Undirected, ()).unwrap();
        g.e_mut().insert(None, 3, 5, AbsoluteEdgeDirection::Undirected, ()).unwrap();
        g.e_mut().insert(None, 4, 5, AbsoluteEdgeDirection::Undirected, ()).unwrap();
        const CORRECT_ANSWERS: [[u8; 6]; 6] = [
            [0, 1, 4, 5, 2, 3],
            [0, 1, 4, 5, 3, 2],
            [0, 2, 5, 4, 1, 3],
            [0, 2, 5, 3, 4, 1],
            [0, 3, 5, 4, 1, 2],
            [0, 3, 5, 2, 4, 1],
        ];
        let answer = DFSPreorderIter::new(&g, 0).unwrap().collect::<Vec<_>>();
        let mut is_answer_correct = false;
        for correct_answer in CORRECT_ANSWERS {
            if answer == correct_answer {
                is_answer_correct = true;
                break;
            }
        }
        assert!(is_answer_correct);
    }
}
