use intuple_derive::*;

#[derive(IntupleLite,Debug)]
struct Test(u32, u32, u32, u32, u32, u32);

#[derive(IntupleLite,Debug)]
struct Testss{a:u32, b:u32, c:u32, d:u32, e:u32, f:u32}

#[derive(IntupleLite,Debug)]
struct TestIgno(u32, u32, u32, #[igno] u32, u32, u32, #[rcsv] Test);
  
#[derive(IntupleLite,Debug)]
struct TestIgno2(u32, u32, u32, #[intuple(igno)] u32, u32, u32, #[intuple(rcsv)] Test);

fn main(){

    let into: Test = (10,20,30,40,50,60).into();
    println!("into struct: {:?}",into);
    let into2: (u32, u32, u32, u32, u32, u32) = into.into();
    println!("into tuple: {:?}",into2);

    let into: Testss = (10,20,30,40,50,60).into();
    println!("into struct: {:?}",into);
    let into2: (u32, u32, u32, u32, u32, u32) = into.into();
    println!("into tuple: {:?}",into2);

    let into: TestIgno = (10,20,30,40,50,(10,20,30,40,50,60)).into();
    println!("into struct: {:?}",into);
    let into2: (u32, u32, u32, u32, u32,(u32, u32, u32, u32, u32, u32)) = into.into();
    println!("into tuple: {:?}",into2);

    let into: TestIgno2 = (10,20,30,40,50,(10,20,30,40,50,60)).into();
    println!("into struct: {:?}",into);
    let into2: (u32, u32, u32, u32, u32,(u32, u32, u32, u32, u32, u32)) = into.into();
    println!("into tuple: {:?}",into2);

}