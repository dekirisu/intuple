#![allow(dead_code)]
use intuple::*;

#[derive(Intuple,Debug,Clone)]
struct Unnamed <T> (u32, #[igno] u32, u32, T);

#[derive(Intuple,Debug,Clone)]
struct RecursiveA<T>{a:u32, #[rcsv] b:Unnamed<T>, c:Unnamed<T>}

#[derive(Intuple,Debug)]
struct RecursiveB<T>(u32, #[intuple(rcsv)] RecursiveA<T>, RecursiveA<T>);

#[derive(Intuple)]
struct Nice {a:u32, b:u32, c:u32}
fn main(){

    println!("/* -------------------------------- Recursive ------------------------------- */");
        let mut rcsv_a = RecursiveA::from_tuple((10,(10,20,1u32),Unnamed(10,5,20,1u32)));  
        println!("{:?}",rcsv_a);
        let mut rcsv_b = RecursiveB::from_tuple((33,(11,(11,22,1u32),Unnamed(11,7,22,1u32)),rcsv_a.clone()));
        println!("{:?}",rcsv_b);

        println!("---------------------");

        let ref_a = rcsv_a.as_tuple_ref();
        println!("{:?}",ref_a);
        let ref_b = rcsv_b.as_tuple_ref();
        println!("{:?}",ref_b);
        let ref_a = rcsv_a.as_tuple_ref_mut();
        println!("{:?}",ref_a);
        let ref_b = rcsv_b.as_tuple_ref_mut();
        println!("{:?}",ref_b);        

        println!("---------------------");

        let tup_a = rcsv_a.into_tuple(); 
        println!("{:?}",tup_a);
        let tup_b = rcsv_b.into_tuple();
        println!("{:?}",tup_b);
        
}