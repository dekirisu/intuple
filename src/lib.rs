pub use intuple_derive::*;

pub trait IntupleStruct where Self: Sized {
    type Intuple: Intuple<Self>;
    fn from_tuple(item: Self::Intuple)->Self;
    fn into_tuple(self)->Self::Intuple;
    fn fruple(item: Self::Intuple)->Self { Self::from_tuple(item) }
    fn intuple(self)->Self::Intuple { self.into_tuple() }
}

pub trait Intuple<IS: IntupleStruct> where Self: Sized {
    fn from_struct(item: IS)-><IS as IntupleStruct>::Intuple {item.into_tuple()}
    fn into_struct(self)->IS;
    fn fruct(item: IS)-><IS as IntupleStruct>::Intuple { Self::from_struct(item) }
    fn intruct(self)->IS { self.into_struct() }
}