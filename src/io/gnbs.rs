use std::{fs::File, io::{BufRead, BufReader}};
use regex::Regex;
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



#[derive(Clone)]
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
    type_name: AttributeTypeName,
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
    Boolean(&'a str),
    String(&'a str),
    List(Vec<Token<'a>>),
    Collection(Vec<Token<'a>>),
}

#[derive(PartialEq, Eq)]
enum TokeniserState {
    ExpectingDeclarationSpecifier,
    ExpectingTypeName,
    ExpectingAttributeName,
    ExpectingValue,
    Terminated,
}

enum DocumentState {
    ExpectingVertexAttributeOrEdgeAttributeOrVertex,
    ExpectingVertexOrEdgeAttributeOrEdge,
    ExpectingEdge,
}



fn identify_atomic_value_type(value: &str, line_number: usize) -> NexusArtResult<Token> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    let integer_regex = Regex::new(r"^[+-]?(0|[1-9][0-9]*)$").unwrap();
    let float_regex = Regex::new(r"^[+-]?[0-9]*\.?[0-9]+([eE][+-]?[0-9]+)?$").unwrap();
    let token = if value == "X" {
        Token::Empty
    } else if integer_regex.is_match(value) {
        Token::Integer(value)
    } else if float_regex.is_match(value) {
        Token::Float(value)
    } else if value == "T" || value == "F" {
        Token::Boolean(value)
    } else {
        return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected value, found '{}'.", line_number, value)));
    };
    Ok(token)
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
        "#" | "" => DeclarationSpecifierName::Comment,
        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected declaration specifier, found '{}'.", line_number, target))),
    };
    let next_state = match declaration_specifier {
        DeclarationSpecifierName::AV | DeclarationSpecifierName::AE => TokeniserState::ExpectingTypeName,
        DeclarationSpecifierName::V | DeclarationSpecifierName::E | DeclarationSpecifierName::A => TokeniserState::ExpectingValue,
        DeclarationSpecifierName::Comment => TokeniserState::Terminated,
    };
    Ok((Token::DeclarationSpecifier(declaration_specifier), split.next().unwrap_or("").trim_start(), next_state))
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

fn extract_attribute_name(line: &str, line_number: usize) -> NexusArtResult<(Token, &str, TokeniserState)> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    let target = line.trim_start();
    if target == "" {
        return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected attribute name, found ''.", line_number)));
    }
    Ok((Token::String(target), "", TokeniserState::Terminated))
}

fn extract_value(mut line: &str, line_number: usize) -> NexusArtResult<(Token, &str, TokeniserState)> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    line = line.trim_start();
    let token: Token;
    let remainder = match line.chars().next() {
        Some('"') => {
            let split = match line.get(1..).unwrap().split_once('"') {
                Some(value) => value,
                None => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Closing quotation mark (\") wasn't found for a string value.", line_number))),
            };
            token = Token::String(split.0);
            split.1.trim_start()
        },
        Some('[') => {
            let split = match line.split_once(']') {
                Some(value) => value,
                None => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Closing bracket (]) wasn't found for a list value.", line_number))),
            };
            let elements = split.0
                .split(',')
                .map(|x| Ok(extract_value(x.trim(), line_number)?.0))
                .collect::<NexusArtResult<Vec<_>>>()?;
            token = Token::List(elements);
            split.1.trim_start()
        },
        Some('{') => {
            let split = match line.split_once('}') {
                Some(value) => value,
                None => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Closing bracket (}}) wasn't found for a collection value.", line_number))),
            };
            let elements = split.0
                .split(',')
                .map(|x| Ok(extract_value(x.trim(), line_number)?.0))
                .collect::<NexusArtResult<Vec<_>>>()?;
            token = Token::Collection(elements);
            split.1.trim_start()
        },
        Some(_) => {
            let mut split = line.splitn(2, char::is_whitespace);
            let target = split.next().unwrap();
            token = identify_atomic_value_type(target, line_number)?;
            split.next().unwrap_or("").trim_start()
        },
        None => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected value, found '{}'.", line_number, line))),
    };
    let next_state = if remainder == "" {
        TokeniserState::Terminated
    } else {
        TokeniserState::ExpectingValue
    };
    Ok((token, remainder, next_state))
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
            TokeniserState::ExpectingAttributeName => extract_attribute_name(remainder, line_number)?,
            TokeniserState::ExpectingValue => extract_value(remainder, line_number)?,
            TokeniserState::Terminated => (Token::Empty, remainder, state),
        };
        answer.push(new_token);
    }
    Ok(answer)
}

fn parse_attribute_metadata(tokens: Vec<Token>, line_number: usize) -> NexusArtResult<AttributeMetadata> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    if tokens.len() != 3 {
        return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected attribute declaration of the form 'AV <type> <name>' or 'AE <type> <name>, found statement with {} token(s).", line_number, tokens.len())));
    }
    let type_name = match &tokens[1] {
        Token::TypeName(value) => value.clone(),
        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected type in the attribute declaration.", line_number))),
    };
    let name = match tokens[2] {
        Token::String(value) => value.to_string(),
        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected name in the attribute declaration.", line_number))),
    };
    Ok(AttributeMetadata { name, type_name })
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
        let mut state = DocumentState::ExpectingVertexAttributeOrEdgeAttributeOrVertex;
        let mut vertex_attributes: Vec<AttributeMetadata> = Vec::new();
        let mut edge_attributes: Vec<AttributeMetadata> = Vec::new();
        let buffer = BufReader::new(file);
        let mut line_number = 0usize;
        for line_result in buffer.lines() {
            line_number += 1;
            let line = match line_result {
                Ok(value) => value,
                Err(_) => {
                    return Err(NexusArtError::new(FUNCTION_PATH, format!("Couldn't read line {} of the input file.", line_number)));
                },
            };
            let tokens = tokenise_line(line.as_ref(), line_number)?;
            if let Token::DeclarationSpecifier(declaration_specifier) = &tokens[0] {
                match declaration_specifier {
                    DeclarationSpecifierName::AV => match state {
                        DocumentState::ExpectingVertexAttributeOrEdgeAttributeOrVertex => vertex_attributes.push(parse_attribute_metadata(tokens, line_number)?),
                        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Vertex attribute declaration after a vertex declaration.", line_number))),
                    },
                    DeclarationSpecifierName::AE => match state {
                        DocumentState::ExpectingVertexAttributeOrEdgeAttributeOrVertex | DocumentState::ExpectingVertexOrEdgeAttributeOrEdge => edge_attributes.push(parse_attribute_metadata(tokens, line_number)?),
                        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Edge attribute declaration after an edge declaration.", line_number))),
                    },
                    DeclarationSpecifierName::Comment => (),
                    _ => todo!(),
                };
            }
        }
        *graph = new_graph;
        Ok(())
    }
}
