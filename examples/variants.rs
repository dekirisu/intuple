use intuple::*;

#[derive(Intuple,Debug,Clone)]
struct Unnamed(u32, u32, u32, u32, u32, u32);

#[derive(Intuple,Debug,Clone)]
struct Named{a:u32, b:u32, c:u32, d:u32, e:u32, f:u32}

fn main(){

    /* -------------------------------------------------------------------------- */
    /*                                    Named                                   */
    /* -------------------------------------------------------------------------- */
    /* ----------------------------- struct -> tuple ---------------------------- */
    // std - struct from tuple
    let intostruct = Named::from((10,20,30,40,50,60));              println!("{:?}",intostruct);
    // std - tuple into struct
    let intostruct: Named = (10,20,30,40,50,60).into();             println!("{:?}",intostruct);
    // intuple - struct from tuple
    let intostruct = Named::from_tuple((10,20,30,40,50,60));        println!("{:?}",intostruct);
    /* ----------------------------- tuple -> struct ---------------------------- */
    // std - tuple from struct
    let intotuple: <Named as Intuple>::Tuple = intostruct.clone().into();        println!("{:?}",intotuple);
    // std - struct into tuple
    let intotuple = <Named as Intuple>::Tuple::from(intostruct.clone());         println!("{:?}",intotuple);
    // intuple - tuple from struct
    let intotuple = intostruct.clone().into_tuple();                println!("{:?}",intotuple);
    let intotuple = intostruct.clone().intuple();                   println!("{:?}",intotuple);
    /* --------------------------- struct -> tuple refs-------------------------- */
    let mut named = Named::from((10,20,30,40,50,60));
    let reference= named.as_tuple_ref();            println!("{:?}",reference);
    let reference_mut = named.as_tuple_ref_mut();   println!("{:?}",reference_mut);

    /* -------------------------------------------------------------------------- */
    /*                                   Unnamed                                  */
    /* -------------------------------------------------------------------------- */
    /* ----------------------------- struct -> tuple ---------------------------- */
    // std - struct from tuple
    let intostruct = Unnamed::from((10,20,30,40,50,60));            println!("{:?}",intostruct);
    // std - tuple into struct
    let intostruct: Unnamed = (10,20,30,40,50,60).into();           println!("{:?}",intostruct);
    // intuple - struct from tuple
    let intostruct = Unnamed::from_tuple((10,20,30,40,50,60));      println!("{:?}",intostruct);

    /* ----------------------------- tuple -> struct ---------------------------- */
    // std - tuple from struct
    let intotuple: <Unnamed as Intuple>::Tuple = intostruct.clone().into();      println!("{:?}",intotuple);
    // std - struct into tuple
    let intotuple = <Unnamed as Intuple>::Tuple::from(intostruct.clone());       println!("{:?}",intotuple);
    // intuple - tuple from struct
    let intotuple = intostruct.clone().into_tuple();                println!("{:?}",intotuple);
    let intotuple = intostruct.clone().intuple();                   println!("{:?}",intotuple);
    /* --------------------------- struct -> tuple refs-------------------------- */
    let mut named = Unnamed::from((10,20,30,40,50,60));
    let reference= named.as_tuple_ref();            println!("{:?}",reference);
    let reference_mut = named.as_tuple_ref_mut();   println!("{:?}",reference_mut);

}