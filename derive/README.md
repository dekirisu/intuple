<h1 align="center">intuple derive</h1>
<p align="center">
    <a href="https://github.com/dekirisu/intuple" style="position:relative">
        <img src="https://img.shields.io/badge/github-dekirisu/intuple-ee6677">
    </a>
    <a href="https://crates.io/crates/intuple_derive" style="position:relative">
        <img src="https://img.shields.io/crates/v/intuple_derive">
    </a>
    <a href="https://discord.gg/kevWvBuPFg" style="position:relative">
        <img src="https://img.shields.io/discord/515100001903312898">
    </a>
</p>

## Easy conversion between structs and tuples
Mainly developed as derive macro for [intuple](https://crates.io/crates/intuple), but can also be used as standalone library with less features.
### `IntupleLite` derive features:<br>
🐍 convert structs into/from tuples - via std `From<..>` and `Into<..>`<br>
🦥 ignore specific fields - via `#[igno]`<br>
🦆 recursion - via `#[rcsv]`/`#[recursive]` on a field <br>
🦊 easy access to the resulting tuple type of the struct<br>

### See [intuple](https://crates.io/crates/intuple) ([crates.io](https://crates.io/crates/intuple)/[github](https://github.com/dekirisu/intuple)) if you also need:<br>
🦝 `Intuple` trait, making the tuple type accessible via `<AStruct as Intuple>::Intuple`<br>
🐺 and implementing `.from_tuple(..)` and `.into_tuple()`/`.intuple()` methods

## Standalone Usage
🐠 add **intuple_derive** to the dependencies in the `Cargo.toml`:
```toml
[dependencies]
intuple_derive = "0.2.0"
```
🦀 use/import everything into rust:
```rust 
use intuple_derive::*;
```
🦚 goes both ways:
```rust 
#[derive(IntupleLite)]
struct Struct {a:u32, b:u32, c:u32}

fn main(){
    let strct: Struct = (3,2,1).into();
    let tuple = <(u32, u32, u32)>::from(strct);
    // OR
    let strct = Struct::from((3,2,1));
    let tuple: (u32, u32, u32) = strct.into();
}
```
## Tuple Type
🦊 access the resulting tuple type of a struct easily:
```rust 
#[derive(IntupleLite)]
struct Nice {a:u32, b:u32, c:u32}
fn main(){
    // easiest: through {StructName}Intuple
    let tup: NiceIntuple = (3,2,1);
    // is equal to
    let tup: (u32, u32, u32) = (3,2,1);
}
```
## Ignoring
🦥 ignore specific fields with `#[igno]`<br>
🐼 ignored fields need/use [Default](https://doc.rust-lang.org/std/default/trait.Default.html) while converting to a struct
```rust 
#[derive(IntupleLite)]
struct Struct {a:u32, #[igno] b:u32, c:u32}
fn main(){
    let strct = Struct::from((2,1));     
    // => {a:2, b:0, c:1}  
    let tuple: (u32, u32) = strct.into();
    // => (2, 1)
}
```
## Recursion
🦊 convert recursively with `#[recursive]` or `#[rcsv]`
```rust 
#[derive(IntupleLite)]
struct Struct {a:u32, b:u32, c:u32}
#[derive(IntupleLite)]
struct Recursive {a:u32, #[recursive] b:Struct, c:u32}
fn main(){
    let rcsv: Recursive = (9,(3,2,1),8).into(); 
    // => Recursive{a:9, b:Struct{a:3,b:2,c:1}, c:8}
    let tuple: RecursiveIntuple = rcsv.into(); 
    // => (9,(3,2,1),8)
}
```
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