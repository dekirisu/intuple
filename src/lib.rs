pub use intuple_derive::*;

pub trait Intuple {
    type Tuple:Sized;
    fn from_tuple(item: Self::Tuple)->Self;
    fn into_tuple(self)->Self::Tuple;
    fn intuple(self)->Self::Tuple;
}
pub trait IntupleRef <'intuple> {
    type Tuple;
    type TupleMut;
    fn as_tuple_ref(&'intuple self) -> Self::Tuple;
    fn as_tuple_ref_mut(&'intuple mut self) -> Self::TupleMut;
}

pub trait IntupleEnum {
    type Tuple:Sized;
    fn from_tuple_enum(item: Self::Tuple)->Self;
    fn into_tuple_enum(self)->Self::Tuple;
    fn intuple_enum(self)->Self::Tuple;
}
pub trait IntupleEnumRef <'intuple> {
    type Tuple;
    type TupleMut;
    fn as_tuple_enum_ref(&'intuple self) -> Self::Tuple;
    fn as_tuple_enum_ref_mut(&'intuple mut self) -> Self::TupleMut;
}