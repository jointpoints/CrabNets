use std::iter::once;
use crabnets::{*, attributes::*, io::{AttributeCollectionIO, AttributeToken}, locales::*};





#[derive(Clone, Default)]
struct VertexCoordinates {
    pub x: u8,
    pub y: u8,
}

// VertexCoordinates::AttributeCollection
impl AttributeCollection for VertexCoordinates {
    fn new() -> Self {
        VertexCoordinates { x: 0, y: 0 }
    }
}

// VertexCoordinates::AttributeCollectionIO
impl AttributeCollectionIO for VertexCoordinates {
    fn io_iter_contents<'a>(&'a self) -> Box<dyn Iterator<Item = AttributeToken<'a>> + 'a> {
        Box::new(
            once(AttributeToken { name: "x", value: StaticDispatchAttributeValue::UInt8(self.x) })
            .chain(once(AttributeToken { name: "y", value: StaticDispatchAttributeValue::UInt8(self.y) }))
        )
    }

    fn io_query_contents(&self, attribute_name: &str) -> Option<StaticDispatchAttributeValue> {
        match attribute_name {
            "x" => Some(StaticDispatchAttributeValue::UInt8(self.x)),
            "y" => Some(StaticDispatchAttributeValue::UInt8(self.y)),
            _ => None,
        }
    }

    fn io_reader_callback<'a, EdgeIdType, VertexIdType>(&mut self, token: AttributeToken<'a>)
        where
            EdgeIdType: Id,
            VertexIdType: Id,
    {
        match token.name {
            "x" => if let StaticDispatchAttributeValue::UInt8(value) = token.value {
                self.x = value;
            },
            "y" => if let StaticDispatchAttributeValue::UInt8(value) = token.value {
                self.y = value;
            },
            _ => (),
        }
    }
}



type MyNetwork = graph!(A ---X--- A with VertexAttributeCollectionType = VertexCoordinates);





fn main() {
    let mut g  = MyNetwork::new();
    g.add_v(None);
    g.v_attrs_mut(&0).unwrap().x = 2;
    println!("{} {}", g.v_attrs(&0).unwrap().x, g.v_attrs(&0).unwrap().y);
}
