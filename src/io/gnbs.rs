use std::{fmt::Display, fs::File, io::{BufRead, BufReader}, str::FromStr};
use regex::Regex;
use crate::{
    BasicMutableGraph, Id, NexusArtError, NexusArtResult, StaticDispatchAttributeValue
};
use super::{AttributeToken, Reader};





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * AUXILIARY ITEMS                                                                   *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



macro_rules! define_gnbs_attribute_type {
    ($($type_name: ident),+) => {
        #[derive(Clone, Eq, PartialEq)]
        enum GNBSAttributeType {
            $($type_name),+
        }

        impl Display for GNBSAttributeType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    $(GNBSAttributeType::$type_name => stringify!($type_name)),+
                })
            }
        }
    };
}

define_gnbs_attribute_type!(
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
    CB, CS
);



struct AttributeMetadata {
    name: String,
    gnbs_type: GNBSAttributeType,
}



struct VertexMetadata<'a, VertexIdType>
where
    VertexIdType: Id,
{
    id: VertexIdType,
    attribute_tokens: Vec<AttributeToken<'a>>,
}



enum DeclarationSpecifierName {
    AV, AE, V, A, E, Comment
}



enum Token<'a> {
    DeclarationSpecifier(DeclarationSpecifierName),
    AttributeType(GNBSAttributeType),
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
        "I1" => GNBSAttributeType::I1,
        "I2" => GNBSAttributeType::I2,
        "I4" => GNBSAttributeType::I4,
        "I8" => GNBSAttributeType::I8,
        "U1" => GNBSAttributeType::U1,
        "U2" => GNBSAttributeType::U2,
        "U4" => GNBSAttributeType::U4,
        "U8" => GNBSAttributeType::U8,
        "F4" => GNBSAttributeType::F4,
        "F8" => GNBSAttributeType::F8,
        "B" => GNBSAttributeType::B,
        "S" => GNBSAttributeType::S,
        "LI1" => GNBSAttributeType::LI1,
        "LI2" => GNBSAttributeType::LI2,
        "LI4" => GNBSAttributeType::LI4,
        "LI8" => GNBSAttributeType::LI8,
        "LU1" => GNBSAttributeType::LU1,
        "LU2" => GNBSAttributeType::LU2,
        "LU4" => GNBSAttributeType::LU4,
        "LU8" => GNBSAttributeType::LU8,
        "LF4" => GNBSAttributeType::LF4,
        "LF8" => GNBSAttributeType::LF8,
        "LB" => GNBSAttributeType::LB,
        "LS" => GNBSAttributeType::LS,
        "CI1" => GNBSAttributeType::CI1,
        "CI2" => GNBSAttributeType::CI2,
        "CI4" => GNBSAttributeType::CI4,
        "CI8" => GNBSAttributeType::CI8,
        "CU1" => GNBSAttributeType::CU1,
        "CU2" => GNBSAttributeType::CU2,
        "CU4" => GNBSAttributeType::CU4,
        "CU8" => GNBSAttributeType::CU8,
        "CB" => GNBSAttributeType::CB,
        "CS" => GNBSAttributeType::CS,
        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected type name, found '{}'.", line_number, target))),
    };
    Ok((Token::AttributeType(type_name), split.next().unwrap_or(""), TokeniserState::ExpectingAttributeName))
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

fn parse_numeric_value<IntoType>(original_value: &str, value_type: &GNBSAttributeType, line_number: usize) -> NexusArtResult<IntoType>
where
    IntoType: FromStr,
{
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    match original_value.parse::<IntoType>()
    {
        Ok(value) => Ok(value),
        Err(_) => Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected value of type {}, found '{}'.", line_number, value_type, original_value))) 
    }
}

macro_rules! convert_token_to_static_dispatch_attribute_value {
    ($function_path: ident, $line_number: ident, $value: ident, $given_gnbs_value_type: ident, $conversion_expression: expr, $($origin_gnbs_value_type: ident --> $target_static_dispatch_attribute_value_variant: ident),+) => {
        match $given_gnbs_value_type {
            $(GNBSAttributeType::$origin_gnbs_value_type => Ok(StaticDispatchAttributeValue::$target_static_dispatch_attribute_value_variant($conversion_expression)),)+
            _ => return Err(NexusArtError::new($function_path, format!("Line {}. Expected value of type {}, found '{}'.", $line_number, $given_gnbs_value_type, $value))),
        }
    };
}

fn parse_value(token: Token, gnbs_value_type: GNBSAttributeType, line_number: usize) -> NexusArtResult<StaticDispatchAttributeValue> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    match token {
        Token::Integer(value) => convert_token_to_static_dispatch_attribute_value!(
            FUNCTION_PATH, line_number, value, gnbs_value_type,
            parse_numeric_value(value, &gnbs_value_type, line_number)?,
            I1 --> Int8, I2 --> Int16, I4 --> Int32, I8 --> Int64, U1 --> UInt8, U2 --> UInt16, U4 --> UInt32, U8 --> UInt64
        ),
        Token::Float(value) => convert_token_to_static_dispatch_attribute_value!(
            FUNCTION_PATH, line_number, value, gnbs_value_type,
            parse_numeric_value(value, &gnbs_value_type, line_number)?,
            F4 --> Float32, F8 --> Float64
        ),
        Token::Boolean(value) => match gnbs_value_type {
            GNBSAttributeType::B => Ok(StaticDispatchAttributeValue::Bool(match value {
                "T" => true,
                "F" => false,
                _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected value of type B, found '{}'.", line_number, value))),
            })),
            _ => Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected value of type {}, found '{}'.", line_number, gnbs_value_type, value))),
        },
        Token::String(value) => match gnbs_value_type {
            GNBSAttributeType::S => Ok(StaticDispatchAttributeValue::Str(value.to_string())),
            _ => Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected value of type {}, found '{}'.", line_number, gnbs_value_type, value))),
        },
        _ => Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected value.", line_number))),
    }
}

fn parse_attribute_declaration(tokens: Vec<Token>, line_number: usize) -> NexusArtResult<AttributeMetadata> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    if tokens.len() != 3 {
        return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected attribute declaration in the form 'AV <type> <name>' or 'AE <type> <name>', found statement with {} token(s).", line_number, tokens.len())));
    }
    let type_name = match &tokens[1] {
        Token::AttributeType(value) => value.clone(),
        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected type in the attribute declaration.", line_number))),
    };
    let name = match tokens[2] {
        Token::String(value) => value.to_string(),
        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected name in the attribute declaration.", line_number))),
    };
    Ok(AttributeMetadata { name, gnbs_type: type_name })
}

fn parse_vertex_declaration<'a, VertexIdType>(tokens: Vec<Token<'a>>, attributes: &Vec<AttributeMetadata>, line_number: usize) -> NexusArtResult<VertexMetadata<'a, VertexIdType>>
where
    VertexIdType: Id,
{
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    if tokens.len() != attributes.len() + 2 {
        return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Expected vertex declaration in the form 'V <id> <attribute values>' with {} token(s) in <attribute values>, found statement with {} token(s).", line_number, attributes.len(), tokens.len())));
    }
    todo!();
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * READER/WRITER                                                                     *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub struct GNBSReader;



// GNBSReader::Reader
impl<'a> Reader for GNBSReader {
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
                        DocumentState::ExpectingVertexAttributeOrEdgeAttributeOrVertex => vertex_attributes.push(parse_attribute_declaration(tokens, line_number)?),
                        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Vertex attribute declaration after a vertex declaration.", line_number))),
                    },
                    DeclarationSpecifierName::AE => match state {
                        DocumentState::ExpectingVertexAttributeOrEdgeAttributeOrVertex | DocumentState::ExpectingVertexOrEdgeAttributeOrEdge => edge_attributes.push(parse_attribute_declaration(tokens, line_number)?),
                        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Edge attribute declaration after an edge declaration.", line_number))),
                    },
                    DeclarationSpecifierName::V => match state {
                        DocumentState::ExpectingVertexAttributeOrEdgeAttributeOrVertex | DocumentState::ExpectingVertexOrEdgeAttributeOrEdge => {
                            state = DocumentState::ExpectingVertexOrEdgeAttributeOrEdge;
                            let vertex_metadata: VertexMetadata<'_, VertexIdType> = parse_vertex_declaration(tokens, &vertex_attributes, line_number)?;
                            new_graph.add_v(Some(vertex_metadata.id));
                            // Add attributes...
                        },
                        _ => return Err(NexusArtError::new(FUNCTION_PATH, format!("Line {}. Vertex declaration after an edge declaration.", line_number))),
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
