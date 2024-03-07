//! # Module to handle input/output of graphs
//! 
//! ## Description
//! This module contains items that allow users to read/write graphs from/to files.  The
//! main component of the module that most users will most often interact  with  is  the
//! [`IO`] trait that defines 2 functions: [`IO::from_file`] and [`IO::into_file`]. This
//! trait   is   implemented   for   [`Graph`],   [`ImmutableGraphContainer`][igc]   and
//! [`MutableGraphContainer`][mgc].
//! 
//! ## Supported formats
//! Graph file formats currently supported are:
//! * GNBS
//! 
//! Graph file formats support of which may appear in the future releases:
//! * GEXF
//! * GR
//! 
//! [igc]: crate::ImmutableGraphContainer
//! [mgc]: crate::MutableGraphContainer
pub mod gnbs;

use std::{fs::File, hash::Hash, io::{BufReader, BufWriter, Read, Write}, iter::empty, str::FromStr};
use crate::{
    attributes::{AttributeCollection, DynamicDispatchAttributeValue, StaticDispatchAttributeValue}, errors::{CrabNetsError, CrabNetsResult}, io::gnbs::GNBSWriter, BasicImmutableGraph, BasicMutableGraph, DynamicDispatchAttributeMap, Graph, Id, Locale
};
use gnbs::GNBSReader;





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * ATTRIBUTE TOKEN                                                                   *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub struct AttributeToken<'a>
{
    pub name: &'a str,
    pub value: StaticDispatchAttributeValue,
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * IO ON ATTRIBUTE COLLECTION LEVEL                                                  *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub trait AttributeCollectionIO
where
    Self: AttributeCollection,
{
    fn io_iter_contents<'a>(&'a self) -> Box<dyn Iterator<Item = AttributeToken<'a>> + 'a>;

    fn io_reader_callback<'a, EdgeIdType, VertexIdType>(&mut self, token: AttributeToken<'a>)
    where
        EdgeIdType: Id,
        VertexIdType: Id;
    
    fn io_query_contents(&self, attribute_name: &str) -> Option<StaticDispatchAttributeValue>;
}



// ()::AttributeCollectionIO
impl AttributeCollectionIO for () {
    #[inline]
    fn io_iter_contents<'a>(&'a self) -> Box<dyn Iterator<Item = AttributeToken<'a>> + 'a> {
        Box::new(empty())
    }

    #[inline]
    fn io_reader_callback<'a, EdgeIdType, VertexIdType>(&mut self, _token: AttributeToken<'a>)
    where
        EdgeIdType: Id,
        VertexIdType: Id,
    {}

    #[inline]
    fn io_query_contents(&self, _attribute_name: &str) -> Option<StaticDispatchAttributeValue> {
        None
    }
}

// AttributeMap::AttributeCollectionIO
impl<KeyType> AttributeCollectionIO for DynamicDispatchAttributeMap<KeyType>
where
    KeyType: AsRef<str> + Clone + Default + Eq + for<'a> From<&'a str> + Hash,
{
    fn io_iter_contents<'a>(&'a self) -> Box<dyn Iterator<Item = AttributeToken<'a>> + 'a> {
        Box::new(self.iter().map(|(attribute_name, attribute_value)|
            (attribute_name.as_ref(), attribute_value.into())
        ).filter(|(_, attribute_value): &(&str, Option<_>)|
            attribute_value.is_some()
        ).map(|(attribute_name, attribute_value)| AttributeToken {
            name: attribute_name,
            value: attribute_value.unwrap()
        }))
    }
    
    #[inline]
    fn io_reader_callback<'a, EdgeIdType, VertexIdType>(&mut self, token: AttributeToken<'a>)
    where
        EdgeIdType: Id,
        VertexIdType: Id,
    {
        let value: Box<dyn DynamicDispatchAttributeValue> = token.value.into();
        self.insert(token.name.into(), value);
    }

    fn io_query_contents(&self, attribute_name: &str) -> Option<StaticDispatchAttributeValue> {
        self.get(&attribute_name.into())?.into()
    }
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * READER/WRITER TRAITS                                                              *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub trait Reader {
    fn read_graph<G, R, EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>(&self, buffer_reader: BufReader<R>) -> CrabNetsResult<G>
    where
        G: BasicMutableGraph<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
        R: Read,
        EdgeAttributeCollectionType: AttributeCollectionIO,
        EdgeIdType: Id,
        VertexAttributeCollectionType: AttributeCollectionIO,
        VertexIdType: FromStr + Id;
}



pub trait Writer {
    fn write_graph<G, W, EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>(&self, graph: &G, buffer_writer: &mut BufWriter<W>) -> CrabNetsResult<()>
    where
        G: BasicImmutableGraph<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
        W: Write,
        EdgeAttributeCollectionType: AttributeCollectionIO,
        EdgeIdType: Id,
        VertexAttributeCollectionType: AttributeCollectionIO,
        VertexIdType: Id;
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * MAIN IO TRAIT                                                                     *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



enum SupportedFormats {
    GNBS,
}



pub trait IO {
    fn from_file(file_name: &str) -> CrabNetsResult<Self>
    where
        Self: Sized;
    fn into_file(&self, file_name: &str) -> CrabNetsResult<()>;
}



// Graph::IO
impl<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType> IO for Graph<EdgeAttributeCollectionType, EdgeIdType, LocaleType, VertexAttributeCollectionType, VertexIdType>
where
    EdgeAttributeCollectionType: AttributeCollectionIO,
    EdgeIdType: Id,
    LocaleType: Locale<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
    VertexAttributeCollectionType: AttributeCollectionIO,
    VertexIdType: FromStr + Id + Into<usize>,
{
    fn from_file(file_name: &str) -> CrabNetsResult<Self> {
        const FUNCTION_PATH: &str = "Graph::IO::from_file";
        let file_format: SupportedFormats;
        if file_name.to_lowercase().ends_with(".gnbs") {
            file_format = SupportedFormats::GNBS;
        } else {
            return Err(CrabNetsError::new(FUNCTION_PATH, format!("Unsupported format of the file with name '{}'.", file_name)));
        }
        let file = match File::open(file_name) {
            Ok(value) => value,
            Err(_) => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Failed to open the file with name '{}'.", file_name))),
        };
        let buffer_reader = BufReader::new(file);
        match file_format {
            SupportedFormats::GNBS => {
                GNBSReader.read_graph(buffer_reader)
            },
        }
    }

    fn into_file(&self, file_name: &str) -> CrabNetsResult<()> {
        const FUNCTION_PATH: &str = "Graph::IO::into_file";
        let file_format: SupportedFormats;
        if file_name.to_lowercase().ends_with(".gnbs") {
            file_format = SupportedFormats::GNBS;
        } else {
            return Err(CrabNetsError::new(FUNCTION_PATH, format!("Unsupported format of the file with name '{}'.", file_name)));
        }
        let file = match File::options().create(true).write(true).truncate(true).open(file_name) {
            Ok(value) => value,
            Err(_) => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Failed to open the file with name '{}'.", file_name))),
        };
        let mut buffer_writer = BufWriter::new(file);
        match file_format {
            SupportedFormats::GNBS => {
                GNBSWriter.write_graph(self, &mut buffer_writer)
            },
        }
    }
}
