use std::{cmp::Ordering, collections::HashSet, fmt::Display, io::{BufRead, BufReader, BufWriter, Read, Write}, str::FromStr};
use itertools::Itertools;
use regex::Regex;
use crate::{
    BasicImmutableGraph, BasicMutableGraph, CrabNetsError, CrabNetsResult, EdgeDirection, Id, StaticDispatchAttributeValue
};
use super::{AttributeCollectionIO, AttributeToken, Reader, Writer};





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * AUXILIARY ITEMS (COMMON)                                                          *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



macro_rules! define_gnbs_attribute_type {
    ($($type_name: ident -- $static_dispatch_attribute_value_variant:ident),+) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

        impl From<StaticDispatchAttributeValue> for GNBSAttributeType {
            fn from(value: StaticDispatchAttributeValue) -> Self {
                match value {
                    $(StaticDispatchAttributeValue::$static_dispatch_attribute_value_variant(_) => GNBSAttributeType::$type_name,)+
                }
            }
        }
    };
}

define_gnbs_attribute_type!(
    I1 -- Int8, I2 -- Int16, I4 -- Int32, I8 -- Int64,
    U1 -- UInt8, U2 -- UInt16, U4 -- UInt32, U8 -- UInt64,
    F4 -- Float32, F8 -- Float64,
    B -- Bool, S -- Str,
    LI1 -- VecInt8, LI2 -- VecInt16, LI4 -- VecInt32, LI8 -- VecInt64,
    LU1 -- VecUInt8, LU2 -- VecUInt16, LU4 -- VecUInt32, LU8 -- VecUInt64,
    LF4 -- VecFloat32, LF8 -- VecFloat64,
    LB -- VecBool, LS -- VecStr,
    CI1 -- SetInt8, CI2 -- SetInt16, CI4 -- SetInt32, CI8 -- SetInt64,
    CU1 -- SetUInt8, CU2 -- SetUInt16, CU4 -- SetUInt32, CU8 -- SetUInt64,
    CB -- SetBool, CS -- SetStr
);



struct AttributeMetadata {
    name: String,
    gnbs_type: GNBSAttributeType,
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * AUXILIARY ITEMS (GNBSREADER)                                                      *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



struct VertexMetadata<'a, VertexIdType>
where
    VertexIdType: Id,
{
    id: VertexIdType,
    attribute_tokens: Vec<AttributeToken<'a>>,
}



struct EdgeMetadata<'a, VertexIdType>
where
    VertexIdType: Id,
{
    id1: VertexIdType,
    id2: VertexIdType,
    attribute_tokens: Vec<AttributeToken<'a>>,
}



#[derive(Clone, Debug)]
enum DeclarationSpecifierName {
    AV, AE, V, A, E, Comment
}



#[derive(Clone, Debug)]
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



fn identify_atomic_value_type(value: &str, line_number: usize) -> CrabNetsResult<Token> {
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
        return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected value, found '{}'.", line_number, value)));
    };
    Ok(token)
}

fn extract_declaration_specifier(line: &str, line_number: usize) -> CrabNetsResult<(Token, &str, TokeniserState)> {
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
        _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected declaration specifier, found '{}'.", line_number, target))),
    };
    let next_state = match declaration_specifier {
        DeclarationSpecifierName::AV | DeclarationSpecifierName::AE => TokeniserState::ExpectingTypeName,
        DeclarationSpecifierName::V | DeclarationSpecifierName::E | DeclarationSpecifierName::A => TokeniserState::ExpectingValue,
        DeclarationSpecifierName::Comment => TokeniserState::Terminated,
    };
    Ok((Token::DeclarationSpecifier(declaration_specifier), split.next().unwrap_or("").trim_start(), next_state))
}

fn extract_type_name(line: &str, line_number: usize) -> CrabNetsResult<(Token, &str, TokeniserState)> {
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
        _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected type name, found '{}'.", line_number, target))),
    };
    Ok((Token::AttributeType(type_name), split.next().unwrap_or(""), TokeniserState::ExpectingAttributeName))
}

fn extract_attribute_name(line: &str, line_number: usize) -> CrabNetsResult<(Token, &str, TokeniserState)> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    let target = line.trim_start();
    if target == "" {
        return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected attribute name, found ''.", line_number)));
    }
    Ok((Token::String(target), "", TokeniserState::Terminated))
}

fn split_complex_literal_into_atomics(values: &str) -> Vec<&str> {
    let mut answer = Vec::new();
    let mut curr_value_start_i = 0;
    let mut curr_value_end_i = 0;
    let mut inside_string = false;
    for symbol in values.chars() {
        match symbol {
            '"' => {
                inside_string = !inside_string;
                curr_value_end_i += 1;
            },
            ',' => if inside_string {
                curr_value_end_i += 1;
            } else {
                answer.push(&values[curr_value_start_i..curr_value_end_i]);
                curr_value_end_i += 1;
                curr_value_start_i = curr_value_end_i;
            },
            c if c.is_whitespace() => {
                curr_value_end_i += 1;
                if !inside_string && curr_value_start_i + 1 == curr_value_end_i {
                    curr_value_start_i += 1;
                }
            },
            _ => curr_value_end_i += 1,
        }
    }
    if curr_value_start_i != curr_value_end_i {
        answer.push(&values[curr_value_start_i..curr_value_end_i]);
    }
    answer
}

fn extract_value(mut line: &str, line_number: usize) -> CrabNetsResult<(Token, &str, TokeniserState)> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    line = line.trim_start();
    let token: Token;
    let remainder = match line.chars().next() {
        Some('"') => {
            let split = match line.get(1..).unwrap().split_once('"') {
                Some(value) => value,
                None => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Closing quotation mark (\") wasn't found for a string value.", line_number))),
            };
            token = Token::String(split.0);
            split.1.trim_start()
        },
        Some('[') => {
            let split = match line.get(1..).unwrap().split_once(']') {
                Some(value) => value,
                None => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Closing bracket (]) wasn't found for a list value.", line_number))),
            };
            let elements = split_complex_literal_into_atomics(split.0)
                .into_iter()
                .map(|x| Ok(extract_value(x.trim(), line_number)?.0))
                .collect::<CrabNetsResult<Vec<_>>>()?;
            token = Token::List(elements);
            split.1.trim_start()
        },
        Some('{') => {
            let split = match line.get(1..).unwrap().split_once('}') {
                Some(value) => value,
                None => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Closing bracket (}}) wasn't found for a collection value.", line_number))),
            };
            let elements = split_complex_literal_into_atomics(split.0)
                .into_iter()
                .map(|x| Ok(extract_value(x.trim(), line_number)?.0))
                .collect::<CrabNetsResult<Vec<_>>>()?;
            token = Token::Collection(elements);
            split.1.trim_start()
        },
        Some(_) => {
            let mut split = line.splitn(2, char::is_whitespace);
            let target = split.next().unwrap();
            token = identify_atomic_value_type(target, line_number)?;
            split.next().unwrap_or("").trim_start()
        },
        None => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected value, found '{}'.", line_number, line))),
    };
    let next_state = if remainder == "" {
        TokeniserState::Terminated
    } else {
        TokeniserState::ExpectingValue
    };
    Ok((token, remainder, next_state))
}

fn tokenise_line(line: &str, line_number: usize) -> CrabNetsResult<Vec<Token>> {
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

fn parse_numeric_value<IntoType>(original_value: &str, value_type: &GNBSAttributeType, line_number: usize) -> CrabNetsResult<IntoType>
where
    IntoType: FromStr,
{
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    match original_value.parse::<IntoType>()
    {
        Ok(value) => Ok(value),
        Err(_) => Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected value of type {}, found '{}'.", line_number, value_type, original_value))) 
    }
}

macro_rules! convert_token_to_static_dispatch_attribute_value {
    // Atomic values
    (atomic value: $function_path: ident, $line_number: ident, $value: ident, $given_gnbs_value_type: ident, $($origin_gnbs_value_type: ident --> $target_static_dispatch_attribute_value_variant: ident),+) => {
        match $given_gnbs_value_type {
            $(GNBSAttributeType::$origin_gnbs_value_type => Ok(Some(StaticDispatchAttributeValue::$target_static_dispatch_attribute_value_variant(parse_numeric_value($value, &$given_gnbs_value_type, $line_number)?))),)+
            _ => Err(CrabNetsError::new($function_path, format!("Line {}. Expected value of type {}, found '{}'.", $line_number, $given_gnbs_value_type, $value))),
        }
    };

    // Complex values
    (complex value ($target_core_container: ident, $target_core_container_insert_function: ident): $function_path: ident, $line_number: ident, $value: ident, $given_gnbs_value_type: ident, $($origin_gnbs_value_type: ident --[$atomic_gnbs_value_type: ident --[$target_core_container_value_type: ty]--> $atomic_static_dispatch_attribute_value_variant:ident]--> $target_static_dispatch_attribute_value_variant: ident),+) => {
        match $given_gnbs_value_type {
            $(GNBSAttributeType::$origin_gnbs_value_type => {
                let mut core_container: $target_core_container<$target_core_container_value_type> = $target_core_container::with_capacity($value.len());
                for atomic_value_token in $value.iter() {
                    core_container.$target_core_container_insert_function(match parse_value(atomic_value_token.clone(), GNBSAttributeType::$atomic_gnbs_value_type, $line_number) {
                        Ok(Some(StaticDispatchAttributeValue::$atomic_static_dispatch_attribute_value_variant(atomic_value))) => atomic_value,
                        _ => return Err(CrabNetsError::new($function_path, format!("Line {}. Expected value of type {}.", $line_number, $given_gnbs_value_type))),
                    });
                }
                Ok(Some(StaticDispatchAttributeValue::$target_static_dispatch_attribute_value_variant(core_container)))
            },)+
            _ => Err(CrabNetsError::new($function_path, format!("Line {}. Expected value of type {}.", $line_number, $given_gnbs_value_type))),
        }
    };
}

fn parse_value(token: Token, gnbs_value_type: GNBSAttributeType, line_number: usize) -> CrabNetsResult<Option<StaticDispatchAttributeValue>> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    match token {
        Token::Integer(value) => convert_token_to_static_dispatch_attribute_value!(
            atomic value:
            FUNCTION_PATH, line_number, value, gnbs_value_type,
            I1 --> Int8, I2 --> Int16, I4 --> Int32, I8 --> Int64, U1 --> UInt8, U2 --> UInt16, U4 --> UInt32, U8 --> UInt64
        ),
        Token::Float(value) => convert_token_to_static_dispatch_attribute_value!(
            atomic value:
            FUNCTION_PATH, line_number, value, gnbs_value_type,
            F4 --> Float32, F8 --> Float64
        ),
        Token::Boolean(value) => match gnbs_value_type {
            GNBSAttributeType::B => Ok(Some(StaticDispatchAttributeValue::Bool(match value {
                "T" => true,
                "F" => false,
                _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected value of type B, found '{}'.", line_number, value))),
            }))),
            _ => Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected value of type {}, found '{}'.", line_number, gnbs_value_type, value))),
        },
        Token::String(value) => match gnbs_value_type {
            GNBSAttributeType::S => Ok(Some(StaticDispatchAttributeValue::Str(value.to_string()))),
            _ => Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected value of type {}, found '{}'.", line_number, gnbs_value_type, value))),
        },
        Token::List(value) => convert_token_to_static_dispatch_attribute_value!(
            complex value (Vec, push):
            FUNCTION_PATH, line_number, value, gnbs_value_type,
            LI1 --[I1 --[i8]--> Int8]--> VecInt8, LI2 --[I2 --[i16]--> Int16]--> VecInt16, LI4 --[I4 --[i32]--> Int32]--> VecInt32, LI8 --[I8 --[i64]--> Int64]--> VecInt64,
            LU1 --[U1 --[u8]--> UInt8]--> VecUInt8, LU2 --[U2 --[u16]--> UInt16]--> VecUInt16, LU4 --[U4 --[u32]--> UInt32]--> VecUInt32, LU8 --[U8 --[u64]--> UInt64]--> VecUInt64,
            LF4 --[F4 --[f32]--> Float32]--> VecFloat32, LF8 --[F8 --[f64]--> Float64]--> VecFloat64,
            LB --[B --[bool]--> Bool]--> VecBool, LS --[S --[String]--> Str]--> VecStr
        ),
        Token::Collection(value) => convert_token_to_static_dispatch_attribute_value!(
            complex value (HashSet, insert):
            FUNCTION_PATH, line_number, value, gnbs_value_type,
            CI1 --[I1 --[i8]--> Int8]--> SetInt8, CI2 --[I2 --[i16]--> Int16]--> SetInt16, CI4 --[I4 --[i32]--> Int32]--> SetInt32, CI8 --[I8 --[i64]--> Int64]--> SetInt64,
            CU1 --[U1 --[u8]--> UInt8]--> SetUInt8, CU2 --[U2 --[u16]--> UInt16]--> SetUInt16, CU4 --[U4 --[u32]--> UInt32]--> SetUInt32, CU8 --[U8 --[u64]--> UInt64]--> SetUInt64,
            CB --[B --[bool]--> Bool]--> SetBool, CS --[S --[String]--> Str]--> SetStr
        ),
        Token::Empty => Ok(None),
        _ => Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected value.", line_number))),
    }
}

fn parse_attribute_declaration(tokens: Vec<Token>, line_number: usize) -> CrabNetsResult<AttributeMetadata> {
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    if tokens.len() != 3 {
        return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected attribute declaration in the form 'AV <type> <name>' or 'AE <type> <name>', found statement with {} token(s).", line_number, tokens.len())));
    }
    let type_name = match &tokens[1] {
        Token::AttributeType(value) => value.clone(),
        _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected type in the attribute declaration.", line_number))),
    };
    let name = match tokens[2] {
        Token::String(value) => value.to_string(),
        _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected name in the attribute declaration.", line_number))),
    };
    Ok(AttributeMetadata { name, gnbs_type: type_name })
}

fn parse_vertex_declaration<'a, VertexIdType>(tokens: Vec<Token<'a>>, attributes: &'a Vec<AttributeMetadata>, line_number: usize) -> CrabNetsResult<VertexMetadata<'a, VertexIdType>>
where
    VertexIdType: FromStr + Id,
{
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    if tokens.len() != attributes.len() + 2 {
        return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected vertex declaration in the form 'V <id> <attribute values>' with {} token(s) in <attribute values>, found statement with {} token(s) in total.", line_number, attributes.len(), tokens.len())));
    }
    let id: VertexIdType = match tokens[1] {
        Token::Integer(value) => match value.parse() {
            Ok(parsed_value) => parsed_value,
            Err(_) => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected vertex ID, found {}.", line_number, value))),
        },
        _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected vertex ID, found token of non-integral type.", line_number))),
    };
    let mut attribute_tokens: Vec<AttributeToken> = Vec::with_capacity(attributes.len());
    for attribute_i in 0..attributes.len() {
        let curr_attribute = &attributes[attribute_i];
        let curr_token = tokens[attribute_i + 2].clone();
        match parse_value(curr_token, curr_attribute.gnbs_type, line_number)? {
            Some(value) => attribute_tokens.push(AttributeToken { name: &curr_attribute.name, value }),
            None => (),
        };
    }
    Ok(VertexMetadata { id, attribute_tokens })
}

fn parse_edge_declaration<'a, VertexIdType>(tokens: Vec<Token<'a>>, attributes: &'a Vec<AttributeMetadata>, line_number: usize) -> CrabNetsResult<EdgeMetadata<'a, VertexIdType>>
where
    VertexIdType: FromStr + Id,
{
    const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
    if tokens.len() != attributes.len() + 3 {
        return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected vertex declaration in the form 'V <id> <attribute values>' with {} token(s) in <attribute values>, found statement with {} token(s) in total.", line_number, attributes.len(), tokens.len())));
    }
    let id1: VertexIdType = match tokens[1] {
        Token::Integer(value) => match value.parse() {
            Ok(parsed_value) => parsed_value,
            Err(_) => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected vertex ID, found {}.", line_number, value))),
        },
        _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected vertex ID, found token of non-integral type.", line_number))),
    };
    let id2: VertexIdType = match tokens[2] {
        Token::Integer(value) => match value.parse() {
            Ok(parsed_value) => parsed_value,
            Err(_) => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected vertex ID, found {}.", line_number, value))),
        },
        _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Expected vertex ID, found token of non-integral type.", line_number))),
    };
    let mut attribute_tokens: Vec<AttributeToken> = Vec::with_capacity(attributes.len());
    for attribute_i in 0..attributes.len() {
        let curr_attribute = &attributes[attribute_i];
        let curr_token = tokens[attribute_i + 3].clone();
        match parse_value(curr_token, curr_attribute.gnbs_type, line_number)? {
            Some(value) => attribute_tokens.push(AttributeToken { name: &curr_attribute.name, value }),
            None => (),
        }
    }
    Ok(EdgeMetadata { id1, id2, attribute_tokens })
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * AUXILIARY ITEMS (GNBSWRITER)                                                      *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



fn collect_vertex_attributes<G, EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>(graph: &G) -> CrabNetsResult<Vec<AttributeMetadata>>
where
    G: BasicImmutableGraph<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
    EdgeAttributeCollectionType: AttributeCollectionIO,
    EdgeIdType: Id,
    VertexAttributeCollectionType: AttributeCollectionIO,
    VertexIdType: Id,
{
    const FUNCTION_PATH: &str = "GNBSWriter::Writer::write_graph";
    let mut answer = Vec::new();
    for attribute_collection in graph.iter_v().map(|x| graph.v_attrs(&x).unwrap()) {
        for attribute_token in attribute_collection.io_iter_contents() {
            match answer.binary_search_by_key(&attribute_token.name, |x: &AttributeMetadata| &x.name) {
                Ok(value) => if answer[value].gnbs_type != attribute_token.value.into() {
                    return Err(CrabNetsError::new(FUNCTION_PATH, format!("The type of vertex attribute '{}' differs across different attribute collections.", attribute_token.name)));
                },
                Err(value) => answer.insert(value, AttributeMetadata { name: attribute_token.name.to_string(), gnbs_type: attribute_token.value.into() }),
            }
        }
    }
    Ok(answer)
}

fn collect_edge_attributes<G, EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>(graph: &G) -> CrabNetsResult<Vec<AttributeMetadata>>
where
    G: BasicImmutableGraph<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
    EdgeAttributeCollectionType: AttributeCollectionIO,
    EdgeIdType: Id,
    VertexAttributeCollectionType: AttributeCollectionIO,
    VertexIdType: Id,
{
    const FUNCTION_PATH: &str = "GNBSWriter::Writer::write_graph";
    let mut answer = Vec::new();
    for attribute_collection in graph.iter_e().map(|x| graph.e_attrs(&x.id1, &x.id2, &x.edge_id).unwrap()) {
        for attribute_token in attribute_collection.io_iter_contents() {
            match answer.binary_search_by_key(&attribute_token.name, |x: &AttributeMetadata| &x.name) {
                Ok(value) => if answer[value].gnbs_type != attribute_token.value.into() {
                    return Err(CrabNetsError::new(FUNCTION_PATH, format!("The type of edge attribute '{}' differs across different attribute collections.", attribute_token.name)));
                },
                Err(value) => answer.insert(value, AttributeMetadata { name: attribute_token.name.to_string(), gnbs_type: attribute_token.value.into() }),
            }
        }
    }
    Ok(answer)
}





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * READER/WRITER                                                                     *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub struct GNBSReader;

// GNBSReader::Reader
impl<'a> Reader for GNBSReader {
    fn read_graph<G, R, EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>(&self, buffer_reader: BufReader<R>) -> CrabNetsResult<G>
    where
        G: BasicMutableGraph<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
        R: Read,
        EdgeAttributeCollectionType: AttributeCollectionIO,
        EdgeIdType: Id,
        VertexAttributeCollectionType: AttributeCollectionIO,
        VertexIdType: FromStr + Id,
    {
        const FUNCTION_PATH: &str = "GNBSReader::Reader::read_graph";
        let mut new_graph = G::default();
        let mut state = DocumentState::ExpectingVertexAttributeOrEdgeAttributeOrVertex;
        let mut vertex_attributes = Vec::new();
        let mut edge_attributes = Vec::new();
        let mut line_number = 0usize;
        for line_result in buffer_reader.lines() {
            line_number += 1;
            let line = match line_result {
                Ok(value) => value,
                Err(_) => {
                    return Err(CrabNetsError::new(FUNCTION_PATH, format!("Couldn't read line {} of the input file.", line_number)));
                },
            };
            let tokens = tokenise_line(line.as_ref(), line_number)?;
            if let Token::DeclarationSpecifier(declaration_specifier) = &tokens[0] {
                match declaration_specifier {
                    DeclarationSpecifierName::AV => match state {
                        DocumentState::ExpectingVertexAttributeOrEdgeAttributeOrVertex => vertex_attributes.push(parse_attribute_declaration(tokens, line_number)?),
                        _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Vertex attribute declaration after a vertex declaration.", line_number))),
                    },
                    DeclarationSpecifierName::AE => match state {
                        DocumentState::ExpectingVertexAttributeOrEdgeAttributeOrVertex | DocumentState::ExpectingVertexOrEdgeAttributeOrEdge => edge_attributes.push(parse_attribute_declaration(tokens, line_number)?),
                        _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Edge attribute declaration after an edge declaration.", line_number))),
                    },
                    DeclarationSpecifierName::V => match state {
                        DocumentState::ExpectingVertexAttributeOrEdgeAttributeOrVertex | DocumentState::ExpectingVertexOrEdgeAttributeOrEdge => {
                            state = DocumentState::ExpectingVertexOrEdgeAttributeOrEdge;
                            let vertex_metadata: VertexMetadata<'_, VertexIdType> = parse_vertex_declaration(tokens, &vertex_attributes, line_number)?;
                            new_graph.add_v(Some(vertex_metadata.id.clone()));
                            for attribute_token in vertex_metadata.attribute_tokens {
                                new_graph.v_attrs_mut(&vertex_metadata.id).unwrap().io_reader_callback::<EdgeIdType, VertexIdType>(attribute_token);
                            }
                        },
                        _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Vertex declaration after an edge declaration.", line_number))),
                    },
                    DeclarationSpecifierName::A => match state {
                        DocumentState::ExpectingVertexOrEdgeAttributeOrEdge | DocumentState::ExpectingEdge => {
                            state = DocumentState::ExpectingEdge;
                            let edge_metadata: EdgeMetadata<'_, VertexIdType> = parse_edge_declaration(tokens, &edge_attributes, line_number)?;
                            let edge_id = new_graph.add_e(&edge_metadata.id1, &edge_metadata.id2, true, None)?;
                            for attribute_token in edge_metadata.attribute_tokens {
                                new_graph.e_attrs_mut(&edge_metadata.id1, &edge_metadata.id2, &edge_id).unwrap().io_reader_callback::<EdgeIdType, VertexIdType>(attribute_token);
                            }
                        },
                        _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Edge declaration before a vertex declaration.", line_number))),
                    },
                    DeclarationSpecifierName::E => match state {
                        DocumentState::ExpectingVertexOrEdgeAttributeOrEdge | DocumentState::ExpectingEdge => {
                            state = DocumentState::ExpectingEdge;
                            let edge_metadata: EdgeMetadata<'_, VertexIdType> = parse_edge_declaration(tokens, &edge_attributes, line_number)?;
                            let edge_id = new_graph.add_e(&edge_metadata.id1, &edge_metadata.id2, false, None)?;
                            for attribute_token in edge_metadata.attribute_tokens {
                                new_graph.e_attrs_mut(&edge_metadata.id1, &edge_metadata.id2, &edge_id).unwrap().io_reader_callback::<EdgeIdType, VertexIdType>(attribute_token);
                            }
                        },
                        _ => return Err(CrabNetsError::new(FUNCTION_PATH, format!("Line {}. Edge declaration before a vertex declaration.", line_number))),
                    },
                    DeclarationSpecifierName::Comment => (),
                };
            }
        }
        Ok(new_graph)
    }
}



pub struct GNBSWriter;

// GNBSWriter::Writer
impl Writer for GNBSWriter {
    fn write_graph<G, W, EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>(&self, graph: &G, buffer_writer: &mut BufWriter<W>) -> CrabNetsResult<()>
    where
        G: BasicImmutableGraph<EdgeAttributeCollectionType, EdgeIdType, VertexAttributeCollectionType, VertexIdType>,
        W: std::io::prelude::Write,
        EdgeAttributeCollectionType: AttributeCollectionIO,
        EdgeIdType: Id,
        VertexAttributeCollectionType: AttributeCollectionIO,
        VertexIdType: Id,
    {
        const HEADER: &[u8] =
b"# GNBS serialisation of a graph
# 
# GNBS format specification:
#     ...

# VERTEX ATTRIBUTES
";
        let vertex_attributes = collect_vertex_attributes(graph)?;
        let edge_attributes = collect_edge_attributes(graph)?;
        buffer_writer.write(HEADER).unwrap();
        if vertex_attributes.len() == 0 {
            buffer_writer.write(b"# No vertex attributes have been defined for this graph\n\n# VERTICES\n").unwrap();
        } else {
            for attribute in vertex_attributes.iter() {
                buffer_writer.write(format!("AV {:?} {}\n", attribute.gnbs_type, attribute.name).as_bytes()).unwrap();
            }
            buffer_writer.write(b"\n# VERTICES\n").unwrap();
        }
        if graph.count_v() == 0 {
            buffer_writer.write(b"# This graph doesn't contain any vertices\n").unwrap();
        }
        for id in graph.iter_v().sorted() {
            buffer_writer.write(format!("V {}", id).as_bytes()).unwrap();
            for attribute in vertex_attributes.iter() {
                buffer_writer.write(match graph.v_attrs(&id).unwrap().io_query_contents(&attribute.name) {
                    Some(value) => format!(" {:?}", value),
                    None => " X".to_string()
                }.as_bytes()).unwrap();
            }
            buffer_writer.write(b"\n").unwrap();
        }
        buffer_writer.write(b"\n# EDGE ATTRIBUTES\n").unwrap();
        if edge_attributes.len() == 0 {
            buffer_writer.write(b"# No edge attributes have been defined for this graph\n\n# EDGES\n").unwrap();
        } else {
            for attribute in edge_attributes.iter() {
                buffer_writer.write(format!("AE {:?} {}\n", attribute.gnbs_type, attribute.name).as_bytes()).unwrap();
            }
            buffer_writer.write(b"\n# EDGES\n").unwrap();
        }
        if graph.count_e() == 0 {
            buffer_writer.write(b"# This graph doesn't contain any edges\n").unwrap();
        }
        for edge in graph.iter_e().sorted_by(|x, y| match x.id1.cmp(&y.id1) { Ordering::Equal => x.id2.cmp(&y.id2), value => value, }) {
            buffer_writer.write(if edge.direction == EdgeDirection::Undirected { format!("E {} {}", edge.id1, edge.id2) } else { format!("A {} {}", edge.id1, edge.id2) }.as_bytes()).unwrap();
            for attribute in edge_attributes.iter() {
                buffer_writer.write(match graph.e_attrs(&edge.id1, &edge.id2, &edge.edge_id).unwrap().io_query_contents(&attribute.name) {
                    Some(value) => format!(" {:?}", value),
                    None => " X".to_string()
                }.as_bytes()).unwrap();
            }
            buffer_writer.write(b"\n").unwrap();
        }
        Ok(())
    }
}





#[cfg(test)]
mod tests {
    use std::io::{BufWriter, Cursor};
    use crate::*;
    use super::*;

    #[test]
    fn read_from_gnbs1() {
        const INPUT: &str = "
        # Test number 1
        AV LS Names
        V 1 X
        V 3 [ \"Romy , \" , ]
        V 2 [     ]
        E 1 2
        E 2 3
        ";
        let buffer_reader = BufReader::new(INPUT.as_bytes());
        let gnbs_reader = GNBSReader;
        let g: graph!(A ---X--- A);
        let g_result = gnbs_reader.read_graph(buffer_reader);
        assert!(g_result.is_ok());
        g = g_result.unwrap();
        assert_eq!(g.count_v(), 3);
        assert_eq!(g.count_e(), 2);
        assert_eq!(g.v_attrs(&3).unwrap().get(&"Names".to_string()).unwrap().downcast::<Vec<String>>().unwrap(), &vec!["Romy , ".to_string()]);
    }

    #[test]
    fn read_from_gnbs2() {
        const INPUT: &str = "
        # Test number 2
        AV LS Names
        V 1 X
        V 3 [ Romy , \" , ]
        V 2 [     ]
        E 1 2
        E 2 3
        ";
        let buffer_reader = BufReader::new(INPUT.as_bytes());
        let gnbs_reader = GNBSReader;
        let g_result: Result<graph!(X ---X--- X), CrabNetsError> = gnbs_reader.read_graph(buffer_reader);
        assert!(g_result.is_err());
    }

    #[test]
    fn write_to_gnbs1() {
        const OUTPUT: &[u8] =
b"# GNBS serialisation of a graph
# 
# GNBS format specification:
#     ...

# VERTEX ATTRIBUTES
AV LS Names

# VERTICES
V 0 [\"Romy\"]
V 1 []
V 2 X

# EDGE ATTRIBUTES
# No edge attributes have been defined for this graph

# EDGES
A 0 1
E 1 2
";
        let mut g: graph!(A ---X--> A) = Graph::new();
        g.add_v(None);
        g.v_attrs_mut(&0).unwrap().insert("Names".to_string(), Box::new(vec!["Romy".to_string()]));
        g.add_v(None);
        g.v_attrs_mut(&1).unwrap().insert("Names".to_string(), Box::new(Vec::<String>::new()));
        g.add_v(None);
        g.add_e(&1, &2, true, None).unwrap();
        g.add_e(&0, &1, false, None).unwrap();
        let mut output = vec![0u8; 500];
        {
            let mut cursor = Cursor::new(&mut output);
            let mut buffer_writer = BufWriter::new(&mut cursor);
            let gnbs_writer = GNBSWriter;
            gnbs_writer.write_graph(&g, &mut buffer_writer).unwrap();
            // buffer_writer.flush().unwrap();
        }
        assert_eq!(&output[0..OUTPUT.len()], OUTPUT);
    }
}
