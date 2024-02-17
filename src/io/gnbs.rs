use std::{fs::File, io::{BufRead, BufReader}};
use crate::{
    BasicMutableGraph,
    Id,
    NexusArtError,
    NexusArtResult
};
use super::Reader;





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * AUXILIARY ITEMS                                                                   *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



enum AttributeTypeName {
    I1, I2, I4, I8,
    U1, U2, U4, U8,
    F4, F8,
    B, S,
    LI1, LI2, LI4, LI8,
    LU1, LU2, LU4, LU8,
    LF4, LF8,
    LB, LS,
    CI1, CI2, CI4, CI8,
    CU1, CU2, CU4, CU8,
    CB, CS,
}

struct AttributeMetadata {
    name: String,
    typename: AttributeTypeName,
}

enum DeclarationSpecifierName {
    AV, AE, V, A, E, Comment
}

enum Token<'a> {
    DeclarationSpecifier(DeclarationSpecifierName),
    TypeName(AttributeTypeName),
    Empty,
    Integer(&'a str),
    Float(&'a str),
    String(&'a str),
}

#[derive(PartialEq, Eq)]
enum TokeniserState {
    ExpectingDeclarationSpecifier,
    ExpectingTypeName,
    ExpectingAttributeName,
    ExpectingValue,
    Terminated,
}



fn extract_declaration_specifier(line: &str, line_number: usize) -> NexusArtResult<(Token, &str, TokeniserState)> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    let mut split = line.trim_start().splitn(2, char::is_whitespace);
    let target = split.next().unwrap();
    let declaration_specifier = match target {
        "AV" => DeclarationSpecifierName::AV,
        "AE" => DeclarationSpecifierName::AE,
        "V" => DeclarationSpecifierName::V,
        "E" => DeclarationSpecifierName::E,
        "A" => DeclarationSpecifierName::A,
        "#" => DeclarationSpecifierName::Comment,
        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected declaration specifier, found '{}'.", line_number, target))),
    };
    let next_state = match declaration_specifier {
        DeclarationSpecifierName::AV | DeclarationSpecifierName::AE => TokeniserState::ExpectingTypeName,
        DeclarationSpecifierName::V | DeclarationSpecifierName::E | DeclarationSpecifierName::A => TokeniserState::ExpectingValue,
        DeclarationSpecifierName::Comment => TokeniserState::Terminated,
    };
    Ok((Token::DeclarationSpecifier(declaration_specifier), split.next().unwrap_or(""), next_state))
}

fn extract_type_name(line: &str, line_number: usize) -> NexusArtResult<(Token, &str, TokeniserState)> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    let mut split = line.trim_start().splitn(2, char::is_whitespace);
    let target = split.next().unwrap();
    let type_name = match target {
        "I1" => AttributeTypeName::I1,
        "I2" => AttributeTypeName::I2,
        "I4" => AttributeTypeName::I4,
        "I8" => AttributeTypeName::I8,
        "U1" => AttributeTypeName::U1,
        "U2" => AttributeTypeName::U2,
        "U4" => AttributeTypeName::U4,
        "U8" => AttributeTypeName::U8,
        "F4" => AttributeTypeName::F4,
        "F8" => AttributeTypeName::F8,
        "B" => AttributeTypeName::B,
        "S" => AttributeTypeName::S,
        "LI1" => AttributeTypeName::LI1,
        "LI2" => AttributeTypeName::LI2,
        "LI4" => AttributeTypeName::LI4,
        "LI8" => AttributeTypeName::LI8,
        "LU1" => AttributeTypeName::LU1,
        "LU2" => AttributeTypeName::LU2,
        "LU4" => AttributeTypeName::LU4,
        "LU8" => AttributeTypeName::LU8,
        "LF4" => AttributeTypeName::LF4,
        "LF8" => AttributeTypeName::LF8,
        "LB" => AttributeTypeName::LB,
        "LS" => AttributeTypeName::LS,
        "CI1" => AttributeTypeName::CI1,
        "CI2" => AttributeTypeName::CI2,
        "CI4" => AttributeTypeName::CI4,
        "CI8" => AttributeTypeName::CI8,
        "CU1" => AttributeTypeName::CU1,
        "CU2" => AttributeTypeName::CU2,
        "CU4" => AttributeTypeName::CU4,
        "CU8" => AttributeTypeName::CU8,
        "CB" => AttributeTypeName::CB,
        "CS" => AttributeTypeName::CS,
        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected type name, found '{}'.", line_number, target))),
    };
    Ok((Token::TypeName(type_name), split.next().unwrap_or(""), TokeniserState::ExpectingAttributeName))
}

fn tokenise_line(line: &str, line_number: usize) -> NexusArtResult<Vec<Token>> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    let mut answer: Vec<Token> = Vec::new();
    let mut state = TokeniserState::ExpectingDeclarationSpecifier;
    let mut new_token: Token;
    let mut remainder = line;
    while state != TokeniserState::Terminated {
        (new_token, remainder, state) = match state {
            TokeniserState::ExpectingDeclarationSpecifier => extract_declaration_specifier(remainder, line_number)?,
            TokeniserState::ExpectingTypeName => extract_type_name(remainder, line_number)?,
            _ => todo!(),
        };
        answer.push(new_token);
    }
    Ok(answer)
}

fn parse_attribute_metadata(line: &str) -> AttributeMetadata {
    todo!();
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * READER/WRITER                                                                     *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



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
        // add .clear() above
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
            let tokens = tokenise_line(line.as_ref(), line_i)?;
        }
        Ok(())
    }
}
