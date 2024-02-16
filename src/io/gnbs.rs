use std::{fs::File, io::{BufRead, BufReader}};
use crate::{
    BasicMutableGraph,
    Id,
    NexusArtError,
    NexusArtResult
};
use super::Reader;





pub struct GNBSReader;



// GNBSReader::Reader
impl Reader for GNBSReader {
    fn read_graph<G, EdgeIdType, VertexIdType>(&self, file: &File, graph: &mut G) -> NexusArtResult<()>
    where
        G: BasicMutableGraph<EdgeIdType, VertexIdType>,
        EdgeIdType: Id,
        VertexIdType: Id,
    {
        const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
        let mut new_graph = graph.clone();
        let buffer = BufReader::new(file);
        let mut line_i = 0usize;
        for line_result in buffer.lines() {
            line_i += 1;
            let line = match line_result {
                Ok(value) => value,
                Err(_) => {
                    return Err(NexusArtError::new(FUNCTION_PATH, format!("Couldn't read line {} of the input file.", line_i)));
                },
            };
        }
        Ok(())
    }
}
