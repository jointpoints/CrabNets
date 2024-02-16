//! # Module to handle input/output of graphs
//! 
//! ## Description
//! This module contains items that enable users to read/write graphs from/to files. The
//! main component of the module that most users will most often interact  with  is  the
//! [`IO`] trait that defines 2 functions: [`IO::from_file`] and [`IO::into_file`]. This
//! trait is  implemented  for  [`Graph`][graph],  [`ImmutableGraphContainer`][igc]  and
//! [`MutableGraphContainer`][mgc].
//! 
//! ## Supported formats
//! Graph file formats currently supported are:
//! * GNBS
//! 
//! Graph file formats support of which is under development:
//! * GEXF
//! * GR
//! 
//! [graph]: crate::Graph
//! [igc]: crate::ImmutableGraphContainer
//! [mgc]: crate::MutableGraphContainer
mod gnbs;

use std::{fs::File, hash::Hash};
use crate::{
    errors::{NexusArtError, NexusArtResult}, AttributeCollection, AttributeMap, AttributeValue, BasicMutableGraph, EdgeDirection, Graph, Id, Locale, MutableGraphContainer
};
use gnbs::GNBSReader;





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * ATTRIBUTE TOKEN                                                                   *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub enum AttributeTokenOwner<EdgeIdType, VertexIdType>
where
    EdgeIdType: Id,
    VertexIdType: Id,
{
    Vertex { id: VertexIdType },
    Edge { id1: VertexIdType, id2: VertexIdType, direction: EdgeDirection, edge_i: EdgeIdType },
}



pub struct AttributeToken<'a>
{
    key: &'a str,
    value: Box<dyn AttributeValue>,
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * IO ON ATTRIBUTE COLLECTION LEVEL                                                  *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub trait AttributeCollectionIO
where
    Self: AttributeCollection,
{
    fn io_reader_callback<'a, EdgeIdType, VertexIdType>(&mut self, token: AttributeToken<'a>)
    where
        EdgeIdType: Id,
        VertexIdType: Id;
}



// ()::AttributeCollectionIO
impl AttributeCollectionIO for () {
    #[inline]
    fn io_reader_callback<'a, EdgeIdType, VertexIdType>(&mut self, _token: AttributeToken<'a>)
    where
        EdgeIdType: Id,
        VertexIdType: Id,
    {}
}

// AttributeMap::AttributeCollectionIO
impl<KeyType> AttributeCollectionIO for AttributeMap<KeyType>
where
    KeyType: Clone + Eq + for<'a> From<&'a str> + Hash,
{
    #[inline]
    fn io_reader_callback<'a, EdgeIdType, VertexIdType>(&mut self, token: AttributeToken<'a>)
    where
        EdgeIdType: Id,
        VertexIdType: Id,
    {
        self.attributes.insert(token.key.into(), token.value);
    }
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * READER/WRITER TRAITS                                                              *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub trait Reader {
    fn read_graph<G, EdgeIdType, VertexIdType>(&self, file: &File, graph: &mut G) -> NexusArtResult<()>
    where
        G: BasicMutableGraph<EdgeIdType, VertexIdType>,
        EdgeIdType: Id,
        VertexIdType: Id;
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * MAIN IO TRAIT                                                                     *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



enum SupportedFormats {
    GNBS,
}



pub trait IO {
    fn from_file(&mut self, file_name: &str) -> NexusArtResult<()>;
    fn into_file(&self, file_name: &str) -> NexusArtResult<()>;
}



// Graph::IO
impl<EdgeIdType, LocaleType, VertexIdType> IO for Graph<EdgeIdType, LocaleType, VertexIdType>
where
    EdgeIdType: Id,
    LocaleType: Locale<EdgeIdType, VertexIdType>,
    VertexIdType: Id,
{
    fn from_file(&mut self, file_name: &str) -> NexusArtResult<()> {
        const FUNCTION_PATH: &str = "Graph::IO::from_file";
        let file_format: SupportedFormats;
        if file_name.to_lowercase().ends_with(".gnbs") {
            file_format = SupportedFormats::GNBS;
        } else {
            return Err(NexusArtError::new(FUNCTION_PATH, format!("Unsupported format of the file with name {}", file_name)));
        }
        let file = match File::open(file_name) {
            Ok(value) => value,
            Err(_) => return Err(NexusArtError::new(FUNCTION_PATH, format!("Failed to open the file with name {}", file_name))),
        };
        match file_format {
            SupportedFormats::GNBS => {
                GNBSReader.read_graph(&file, self)?;
            },
        }
        Ok(())
    }

    fn into_file(&self, file_name: &str) -> NexusArtResult<()> {
        todo!();
    }
}

