use intuple::*;

#[derive(Intuple,Debug)]
struct Inside (u32,u32);

#[derive(IntupleEnum,Debug)]
#[intuple(derive(Debug))]
enum Test {
    None,
    Unit,
    Unnamed(u32,#[recursive]Inside),
    Named{a:u32, b:u32}
}

#[derive(Intuple)]
struct Struct (u32, #[recursive_enum]Test);

fn main(){
    // Unit
    let unit = Test::Unit;
    println!("{:?}",unit);
    println!("{:?}",unit.as_tuple_enum_ref());
    let tuple = unit.into_tuple_enum();
    println!("{:?}",tuple);
    println!("{:?}",Test::from(tuple));

    // Unit
    let unit = Test::Unnamed(5,Inside(5,1));
    println!("{:?}",unit);
    println!("{:?}",unit.as_tuple_enum_ref());
    let tuple = unit.into_tuple_enum();
    println!("{:?}",tuple);
    println!("{:?}",Test::from(tuple));

    // Named
    let unit = Test::Named {a:1,b:7};
    println!("{:?}",unit);
    println!("{:?}",unit.as_tuple_enum_ref());
    let tuple = unit.into_tuple_enum();
    println!("{:?}",tuple);
    println!("{:?}",Test::from(tuple));
}