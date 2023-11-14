// you can serde, without implementing Serialize/Deserialize, don't really know if it's usefull, but nice to know!
use intuple::*;

#[derive(Intuple, PartialEq,Eq,Debug)]
struct Named{a:u32, b:u32, c:u32, d:u32, e:u32, #[rcsv]f:Enum}

#[derive(Intuple, PartialEq,Eq,Debug)]
enum Enum{
    None,
    Unnamed(u32,u32),
    Named{a:u32, b:u32},
    Unit
}

fn main(){
    let named = Named::from((1,2,3,4,5,(None,None,Some(false))));
    let json = serde_json::to_string(&named.as_tuple_ref()).unwrap();
    println!("{}",json);

    let tuple = serde_json::from_str::<<Named as Intuple>::Tuple>(&json).unwrap();
    let named_again = Named::from(tuple);
    assert_eq!(named,named_again);
}