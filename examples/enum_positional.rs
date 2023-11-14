use intuple::*;

#[derive(Intuple,Debug)]
struct Inside (u32,u32);

#[derive(Intuple,Debug)]
enum Test {
    None,
    Unit,
    Unnamed(u32,#[recursive]Inside),
    Named{a:u32, b:u32}
}

#[derive(Intuple)]
struct Struct (u32, #[recursive]Test);

fn main(){
    // Unit
    let unit = Test::Unit;
    println!("{:?}",unit);
    println!("{:?}",unit.as_tuple_ref());
    let tuple = unit.into_tuple();
    println!("{:?}",tuple);
    println!("{:?}",Test::from(tuple));

    // Unit
    let unit = Test::Unnamed(5,Inside(1,2));
    println!("{:?}",unit);
    println!("{:?}",unit.as_tuple_ref());
    let tuple = unit.into_tuple();
    println!("{:?}",tuple);
    println!("{:?}",Test::from(tuple));

    // Named
    let unit = Test::Named {a:1,b:7};
    println!("{:?}",unit);
    println!("{:?}",unit.as_tuple_ref());
    let tuple = unit.into_tuple();
    println!("{:?}",tuple);
    println!("{:?}",Test::from(tuple));
}