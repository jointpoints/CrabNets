use std::{any::{Any, TypeId}, collections::{HashMap, HashSet, hash_map::Iter}, fmt::Debug, hash::Hash};
use dyn_clone::{DynClone, clone_trait_object};





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * ATTRIBUTE VALUES                                                                  *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



/// # Trait for dynamic disptach attribute values
pub trait DynamicDispatchAttributeValue
where
    Self: Any + Debug + DynClone + Send + Sync,
{}

impl<AttributeValueType> DynamicDispatchAttributeValue for AttributeValueType
where
    AttributeValueType: Any + Debug + Clone + Send + Sync,
{}

impl dyn DynamicDispatchAttributeValue {
    #[inline]
    pub fn is<T: Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }

    pub fn downcast<T: Any>(&self) -> Option<&T> {
        if !self.is::<T>() {
            return None;
        }
        unsafe { Some(&*(self as *const dyn DynamicDispatchAttributeValue as *const T)) }
    }

    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if !self.is::<T>() {
            return None;
        }
        unsafe { Some(&mut *(self as *mut dyn DynamicDispatchAttributeValue as *mut T)) }
    }
}

clone_trait_object!(DynamicDispatchAttributeValue);



macro_rules! define_static_dispatch_attribute_value_variant {
    ($variant_name: ident, $variant_type: ty) => {
        struct $variant_name($variant_type);

        impl Into<$variant_type> for $variant_name {
            #[inline]
            fn into(self) -> $variant_type {
                self.0
            }
        }
    };
}

macro_rules! define_static_dispatch_attribute_value_enum {
    ($($variant_name: ident($variant_type: ty)),+) => {
        $(define_static_dispatch_attribute_value_variant!($variant_name, $variant_type);)+

        /// Enum for static dispatch attribute values
        pub enum StaticDispatchAttributeValue {
            $($variant_name($variant_type)),+
        }

        impl Into<Box<dyn DynamicDispatchAttributeValue>> for StaticDispatchAttributeValue {
            fn into(self) -> Box<dyn DynamicDispatchAttributeValue> {
                match self {
                    $(StaticDispatchAttributeValue::$variant_name(value) => Box::new(value)),+
                }
            }
        }

        impl From<&Box<dyn DynamicDispatchAttributeValue>> for Option<StaticDispatchAttributeValue> {
            fn from(dynamic_dispatch_value: &Box<dyn DynamicDispatchAttributeValue>) -> Self {
                $(
                    if let Some(value) = dynamic_dispatch_value.downcast::<$variant_type>() {
                        return Some(StaticDispatchAttributeValue::$variant_name(value.clone()));
                    }
                )+
                None
            }
        }

        impl Debug for StaticDispatchAttributeValue {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    $(StaticDispatchAttributeValue::$variant_name(value) => format!("{:?}", value),)+
                }.as_ref())
            }
        }
    };
}

define_static_dispatch_attribute_value_enum!(
    Int8(i8), Int16(i16), Int32(i32), Int64(i64),
    UInt8(u8), UInt16(u16), UInt32(u32), UInt64(u64),
    Float32(f32), Float64(f64),
    Bool(bool), Str(String),
    VecInt8(Vec<i8>), VecInt16(Vec<i16>), VecInt32(Vec<i32>), VecInt64(Vec<i64>),
    VecUInt8(Vec<u8>), VecUInt16(Vec<u16>), VecUInt32(Vec<u32>), VecUInt64(Vec<u64>),
    VecFloat32(Vec<f32>), VecFloat64(Vec<f64>),
    VecBool(Vec<bool>), VecStr(Vec<String>),
    SetInt8(HashSet<i8>), SetInt16(HashSet<i16>), SetInt32(HashSet<i32>), SetInt64(HashSet<i64>),
    SetUInt8(HashSet<u8>), SetUInt16(HashSet<u16>), SetUInt32(HashSet<u32>), SetUInt64(HashSet<u64>),
    SetBool(HashSet<bool>), SetStr(HashSet<String>)
);





// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * ATTRIBUTE COLLECTIONS                                                             *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *



pub trait AttributeCollection
where
    Self: Clone + Default,
{
    fn new() -> Self;
}



#[derive(Clone, Default)]
pub struct DynamicDispatchAttributeMap<KeyType>
where
    KeyType: Clone + Default + Eq + Hash,
{
    attributes: HashMap<KeyType, Box<dyn DynamicDispatchAttributeValue>>,
}

// DynamicDispatchAttributeMap::DynamicDispatchAttributeMap
impl<KeyType> DynamicDispatchAttributeMap<KeyType>
where
    KeyType: Clone + Default + Eq + Hash,
{
    pub fn get(&self, attribute_name: &KeyType) -> Option<&Box<dyn DynamicDispatchAttributeValue>> {
        self.attributes.get(attribute_name)
    }
    
    pub fn insert(&mut self, attribute_name: KeyType, attribute_value: Box<dyn DynamicDispatchAttributeValue>) -> Option<Box<dyn DynamicDispatchAttributeValue>> {
        self.attributes.insert(attribute_name, attribute_value)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, KeyType, Box<dyn DynamicDispatchAttributeValue>> {
        self.attributes.iter()
    }
}

// DynamicDispatchAttributeMap::AttributeCollection
impl<KeyType> AttributeCollection for DynamicDispatchAttributeMap<KeyType>
where
    KeyType: Clone + Default + Eq + Hash,
{
    fn new() -> Self {
        DynamicDispatchAttributeMap { attributes: HashMap::new() }
    }
}



// ()::AttributeCollection
impl AttributeCollection for () {
    fn new() -> Self {
        ()
    }
}
