<p align="center">
<img src="https://user-images.githubusercontent.com/78398528/282761791-a98ce5c2-2e51-4edb-8652-f89325192726.gif">
</p>
<p align="center">
    <a href="https://github.com/dekirisu/intuple" style="position:relative">
        <img src="https://img.shields.io/badge/github-dekirisu/intuple-ee6677">
    </a>
    <a href="https://crates.io/crates/intuple" style="position:relative">
        <img src="https://img.shields.io/crates/v/intuple">
    </a>
    <a href="https://discord.gg/kevWvBuPFg" style="position:relative">
        <img src="https://img.shields.io/discord/515100001903312898">
    </a>
</p>

## Features
🐍 convert a **struct** into a tuple and back<br>
🦎 convert an **enum** into a tuple and back<br>
🦢 get a tuple of (mut) references of fields of a **struct**<br>
🐓 get a tuple of (mut) references of fields of an **enum**<br>
🦥 ignore specific fields<br>
🦆 do it all recursively
## Usage
🐠 add **intuple** to the dependencies in the `Cargo.toml`:
```toml
[dependencies]
intuple = "0.2"
```
🦀 use/import everything into rust:
```rust 
use intuple::*;
```
🦚 multiple ways to convert:
```rust 
#[derive(Intuple)]
struct Struct {a:u32, b:u32, c:u32}

fn main(){
    // use std traits
    let strct: Struct = (3,2,1).into();
    let tuple = <(u32, u32, u32)>::from(strct);
    let strct = Struct::from((3,2,1));
    let tuple: (u32, u32, u32) = strct.into();
    // OR intuple trait
    let strct = Struct::from_tuple((3,2,1));
    let tuple = strct.into_tuple(); // or strct.intuple()
    // references
    let strct = Struct::from_tuple((3,2,1));    
    let tupref = strct.as_tuple_ref(); // (&u32,&u32,&u32)
    let tupref = strct.as_tuple_ref_mut(); // (&mut u32,&mut u32,&mut u32)
    *tupref.1 = 3;
}
```
## Tuple Type
🦊 access the resulting tuple types through a qualified path:
```rust 
#[derive(Intuple)]
struct Nice {a:u32, b:u32, c:u32}
fn main(){
    let tup: <Nice as Intuple>::Tuple = (3,2,1);
    let tup: (u32, u32, u32) = (3,2,1); // <- same as above
    // reference tuple types
    let tup: <Nice as IntupleRef>::Tuple = (&3,&2,&1);
    let tup: (&u32, &u32, &u32) = (&3,&2,&1); // <- same as above
    // mut reference tuple types
    let tup: <Nice as IntupleRef>::TupleMut = (&mut 3,&mut 2,&mut 1);
    let tup: (&mut u32, &mut u32, &mut u32) = (&mut 3,&mut 2,&mut 1); // <- same as above
}
```
## Ignoring
🦥 ignore specific fields with `#[igno]`/`#[ignore]`<br>
🐻 or `#[intuple(igno)]`/`#[intuple(ignore)]`<br>
🐼 ignored fields need to implement [Default](https://doc.rust-lang.org/std/default/trait.Default.html) while converting to a struct
```rust 
#[derive(Intuple)]
struct Struct {a:u32, #[igno] b:u32, c:u32}
fn main(){
    let strct = Struct::from((2,1));     
    // => {a:2, b:0, c:1}  
    let tuple: (u32, u32) = strct.into();
    // => (2, 1)
}
```
## Recursion
🦊 convert recursively with `#[recursive]`/`#[rcsv]`<br>
🦐 or `#[intuple(rcsv)]`/`#[intuple(recursive)]`<br>
🐼 recursive fields need to derive `Intuple`
```rust 
#[derive(Intuple)]
struct Struct {a:u32, b:u32, c:u32}
#[derive(Intuple)]
struct Recursive {a:u32, #[recursive] b:Struct, c:u32}
fn main(){
    let rcsv: Recursive = (9,(3,2,1),8).into(); 
    // => Recursive{a:9, b:Struct{a:3,b:2,c:1}, c:8}
    let tuple = rcsv.into_tuple(); 
    // => (9,(3,2,1),8)
}
```
🦆 recursion also works with `.as_tuple_ref()` and `as_tuple_ref_mut()`
```rust 
#[derive(Intuple)]
struct Struct {a:u32, b:u32, c:u32}
#[derive(Intuple)]
struct Recursive {a:u32, #[recursive] b:Struct, c:u32}
fn main(){
    let rcsv = Recursive::from((9,(3,2,1),8)); 
    let tuple = rcsv.as_tuple_ref(); 
    // => (&9,(&3,&2,&1),&8)
}
```
## Enums
🙉 converting enums to tuples isn't as straight forward as structs, therefore two methods are implemented!
### 🐍 1. Positional
🐆 using `Intuple` - no additional enums or structs are generated<br>
🐢 field tuples are wrapped in an `Option<>`, which are inside another tuple<br>
🦎 the outer tuple has as many fields as there are enum variants<br>
🐊 the required `None` variant will convert to `(None,None,None,...)`<br>
🐉 any other variant will occupy a slot, depending on its position `(None,Some(tuple),None,...)`
```rust 
// Positional
#[derive( Intuple, Debug )]
// enums require a 'None' variant
enum Enum { None, Unit, Unnamed(u32,u32), Another(u8,u8) }
fn main(){
    let enum = Enum::Unnamed(1,2); 
    let tuple = enum.as_tuple_ref(); 
    // => (None, Some((&1,&2)), None)
    let tuple = enum.into_tuple(); 
    // => (None, Some((1,2)), None)
    let enum = Enum::None; 
    let tuple = rcsv.into_tuple(); 
    // => (None,None,None)
}
```
### 🦊 2. Generated Tuple Enums
🐈 using `IntupleEnum` -  **three** additional enums will be generated: <br>
🐕 `{EnumName}Intuple`, `{EnumName}IntupleRef` and `{EnumName}IntupleRefMut`<br>
🦄 each of those will use the original variant names and contain a tuple<br>
🐔 to set derives for them, use `#[intuple(derive(...))]`<br>
⚠ to use them recursivly **ANYWHERE**, use `#[recursive_enum]` or `#[rcsve]`<br>
🦢 `.into()`/`.from(..)` are implemented, but the custom methods change to:<br>
🐓 `.from_tuple_enum(..)`, `.into_tuple_enum()`, `.as_tuple_enum_ref()` and `.as_tuple_enum_ref_mut()`
```rust 
// Generated
#[derive( IntupleEnum, Debug )]
#[intuple(derive( Debug ))]
enum Enum { Unit, Unnamed(u32,u32), Another(u8,u8) }
fn main(){
    let enum = Enum::Unnamed(1,2); 
    let tuple = enum.as_tuple_enum_ref(); 
    // => EnumIntupleRef::Unnamed((&1,&2))
    let tuple = enum.into_tuple_enum(); 
    // => EnumIntupleRef::Unnamed((1,2))
}
```
## Example: Serde - Thinking out of the box
🦄 You could use `serde` without implementing Serialize/Deserialize<br>
🐔 This only works with the **positional** enum tuples!
```rust
use intuple::*;

#[derive(Intuple)]
struct Named{a:u32, b:u32, c:u32, d:u32, e:u32, f:u32}

fn main(){
    let named = Named::from((1,2,3,4,5,6));
    let json = serde_json::to_string(&named.as_tuple_ref()).unwrap();
    println!("{}",json); //=> "[1,2,3,4,5,6]"

    let tuple = serde_json::from_str::<<Named as Intuple>::Tuple>(&json).unwrap();
    let named_again = Named::from(tuple);
    // named == named_again
}
```
## More Information
<a href="CHANGELOG.md">🦎 Changelog</a><br>
[🐱 GitHub](https://github.com/dekirisu/intuple)<br>
[👾 Discord Server](https://discord.gg/kevWvBuPFg)<br>

---
### License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>