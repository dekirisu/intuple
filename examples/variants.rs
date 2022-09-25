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
    // let intostruct = Named::from((10,20,30,40,50,60));              println!("{:?}",intostruct);
    // std - tuple into struct
    // let intostruct: Named = (10,20,30,40,50,60).into();             println!("{:?}",intostruct);
    // intuple - struct from tuple
    let intostruct = Named::from_tuple((10,20,30,40,50,60));        println!("{:?}",intostruct);
    let intostruct = Named::fruple((10,20,30,40,50,60));            println!("{:?}",intostruct);
    // intuple - tuple into struct
    let intostruct: Named = (10,20,30,40,50,60).into_struct();      println!("{:?}",intostruct);
    let intostruct: Named = (10,20,30,40,50,60).intruct();          println!("{:?}",intostruct);
    /* ----------------------------- tuple -> struct ---------------------------- */
    // std - tuple from struct
    // let intotuple: NamedIntuple = intostruct.clone().into();        println!("{:?}",intotuple);
    // std - struct into tuple
    // let intotuple = NamedIntuple::from(intostruct.clone());         println!("{:?}",intotuple);
    // intuple - tuple from struct
    let intotuple = intostruct.clone().into_tuple();                println!("{:?}",intotuple);
    let intotuple = intostruct.clone().intuple();                   println!("{:?}",intotuple);
    // intuple - struct into tuple
    let intotuple = NamedIntuple::from_struct(intostruct.clone());  println!("{:?}",intotuple);
    let intotuple = NamedIntuple::fruct(intostruct.clone());        println!("{:?}",intotuple);

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
    let intostruct = Unnamed::fruple((10,20,30,40,50,60));          println!("{:?}",intostruct);
    // intuple - tuple into struct
    let intostruct: Unnamed = (10,20,30,40,50,60).into_struct();    println!("{:?}",intostruct);
    let intostruct: Unnamed = (10,20,30,40,50,60).intruct();        println!("{:?}",intostruct);

    /* ----------------------------- tuple -> struct ---------------------------- */
    // std - tuple from struct
    let intotuple: UnnamedIntuple = intostruct.clone().into();      println!("{:?}",intotuple);
    // std - struct into tuple
    let intotuple = UnnamedIntuple::from(intostruct.clone());       println!("{:?}",intotuple);
    // intuple - tuple from struct
    let intotuple = intostruct.clone().into_tuple();                println!("{:?}",intotuple);
    let intotuple = intostruct.clone().intuple();                   println!("{:?}",intotuple);
    // intuple - struct into tuple
    let intotuple = UnnamedIntuple::from_struct(intostruct.clone());println!("{:?}",intotuple);
    let intotuple = UnnamedIntuple::fruct(intostruct.clone());      println!("{:?}",intotuple);

}