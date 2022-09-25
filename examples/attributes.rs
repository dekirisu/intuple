#![allow(dead_code)]
use intuple::*;

#[derive(Intuple,Debug,Clone)]
struct Unnamed(u32, #[igno] u32, u32);

#[derive(Intuple,Debug,Clone)]
struct Named{a:u32, #[igno] b:u32, c:u32}

#[derive(Intuple,Debug,Clone)]
struct RecursiveA{a:u32, #[rcsv] b:Unnamed, c:Unnamed}

#[derive(Intuple,Debug)]
struct RecursiveB(u32, #[rcsv] RecursiveA, RecursiveA);

fn main(){

    println!("/* --------------------------------- Ignore --------------------------------- */");
        let intostruct = Named::from_tuple((10,20));        println!("{:?}",intostruct);
        let intostruct = Unnamed::from_tuple((10,20));      println!("{:?}",intostruct);
        let intotuple = intostruct.clone().into_tuple();    println!("{:?}",intotuple);

    println!("/* -------------------------------- Recursive ------------------------------- */");
        let rcsv_a = RecursiveA::from_tuple((10,(10,20),Unnamed(10,5,20)));  
        println!("{:?}",rcsv_a);
        let rcsv_b = RecursiveB::from_tuple((33,(11,(11,22),Unnamed(11,7,22)),rcsv_a.clone()));
        println!("{:?}",rcsv_b);

        let rcsv_a = rcsv_a.into_tuple(); 
        println!("{:?}",rcsv_a);
        let rcsv_b = rcsv_b.into_tuple();
        println!("{:?}",rcsv_b);

}