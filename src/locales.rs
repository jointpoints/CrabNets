//! # Collection of standard locales
//! 
//! ## Description
//! Connecto.rs provides you with a number of fully implemented [locales][loc] that you can use out
//! of the box.
//! These are:
//! * [`UndirectedSimpleLocale`] -- only undirected edges, no parallel edges, loops are allowed.
//! * `DirectedSimpleLocale` -- directed and undirected edges, no parallel edges, loops are allowed.
//! * `UndirectedMultiLocale` -- only undirected edges, possibly parallel edges, loops are allowed.
//! * `DirectedMultiLocale` -- directed and undirected edges, possibly parallel edges, loops are allowed.
//! 
//! `DirectedMultiLocale` is a 'universal' locale type in the sense that all other locale types
//! above are the special cases of it.
//! However, we recommend that you use the most restrictive locale type possible to optimise memory
//! usage (and in some cases, runtime).
//! 
//! [loc]: crate::Locale
use std::{collections::HashMap, iter::empty, mem::replace};
use anyhow::*;
use crate::{connecto_rs_error, ConnectorsError, Id, Locale};










/// # Locale type for undirected simple graphs
/// 
/// ## Description
/// Use this locale type if the following constraints are suitable for your graph topology:
/// * Only undirected edges are allowed (if $i$ and $j$ are vertices, there will be no distinction
/// between edges $ij$ and $ji$).
/// * No parallel edges are allowed (there is at most 1 edge between any 2 vertices).
pub struct UndirectedSimpleLocale<VertexType>
{
    vertex: VertexType,
    neighbourhood: HashMap<Id, Id>,
}



impl<VertexType> Locale<VertexType> for UndirectedSimpleLocale<VertexType>
{
    #[inline]
    fn deregister_edge(&mut self, id: Id) {
        self.neighbourhood.retain(|_, eid| *eid != id);
    }



    #[inline]
    fn expose(&self) -> &VertexType {
        &self.vertex
    }



    #[inline]
    fn expose_mut(&mut self) -> &mut VertexType {
        &mut self.vertex
    }



    #[inline]
    fn is_registered(&self, eid: Id) -> bool {
        self.neighbourhood.values().find(|inc_eid| **inc_eid == eid).is_some()
    }



    #[inline]
    fn is_registered_in(&self, _eid: Id) -> bool {
        false
    }



    #[inline]
    fn is_registered_out(&self, _eid: Id) -> bool {
        false
    }



    #[inline]
    fn iter_incident<'a>(&'a self) -> Box<dyn Iterator<Item = (Id, Id)> + 'a> {
        Box::new(self.neighbourhood.iter().map(|(&vid, &eid)| (vid, eid)))
    }



    #[inline]
    fn iter_incident_in<'a>(&'a self) -> Box<dyn Iterator<Item = (Id, Id)> + 'a> {
        Box::new(empty::<(Id, Id)>())
    }



    #[inline]
    fn iter_incident_out<'a>(&'a self) -> Box<dyn Iterator<Item = (Id, Id)> + 'a> {
        Box::new(empty::<(Id, Id)>())
    }



    #[inline]
    fn iter_incident_undir<'a>(&'a self) -> Box<dyn Iterator<Item = (Id, Id)> + 'a> {
        Box::new(self.neighbourhood.iter().map(|(&vid, &eid)| (vid, eid)))
    }



    #[inline]
    fn leak(self) -> VertexType {
        self.vertex
    }



    fn register_edge(&mut self, eid: Id, vid: Id, _metadata: bool) -> Result<()> {
        if self.neighbourhood.contains_key(&vid) {
            return connecto_rs_error!(format!("Cannot add an edge with ID {eid} because an edge incident on the same pair of vertices already exists."));
        }
        self.neighbourhood.insert(vid, eid);
        Ok(())
    }



    #[inline]
    fn replace_vertex(&mut self, new_vertex: VertexType) -> VertexType {
        let old_vertex = replace(&mut self.vertex, new_vertex);
        old_vertex
    }



    #[inline]
    fn with_vertex(vertex: VertexType) -> Self {
        UndirectedSimpleLocale { vertex, neighbourhood: HashMap::new() }
    }
}
