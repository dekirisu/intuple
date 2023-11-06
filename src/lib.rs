pub use intuple_derive::*;

pub trait Intuple where Self: Sized {
    type Intuple: Sized;
    fn from_tuple(item: Self::Intuple)->Self;
    fn into_tuple(self)->Self::Intuple;
    fn intuple(self)->Self::Intuple;
}