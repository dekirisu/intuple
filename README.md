<h1 align="center">intuple</h1>
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
🐍 convert structs into/from tuples - via std `From<..>` and `Into<..>`<br>
🦥 ignore specific fields - via `#[igno]`<br>
🦆 recursion - via `#[rcsv]`/`#[recursive]` per field <br>
🦊 easy access to the resulting tuple type of the struct<br>
🦝 `Intuple` trait for easier conversion and dynamic handling<br>

## Usage
🐠 add **intuple** to the dependencies in the `Cargo.toml`:
```toml
[dependencies]
intuple = "0.2.0"
```
🦀 use/import everything into rust:
```rust 
use intuple::*;
```
🦚 multiple ways to convert, combine as you wish - whatever fits your use case:
```rust 
#[derive(Intuple)]
struct Struct {a:u32, b:u32, c:u32}

fn main(){
    // use std traits
        let strct: Struct = (3,2,1).into();
        let tuple = <(u32, u32, u32)>::from(strct);
        // OR
        let strct = Struct::from((3,2,1));
        let tuple: (u32, u32, u32) = strct.into();
    // OR intuple traits
        let strct = Struct::from_tuple((3,2,1));
        let tuple = strct.into_tuple();
        let tuple = strct.intuple();
}
```
## Tuple Type
🦊 access the resulting tuple type of a struct easily:
```rust 
#[derive(Intuple)]
struct Nice {a:u32, b:u32, c:u32}
fn main(){
    // easiest: through {StructName}Intuple
    let tup: NiceIntuple = (3,2,1);
    // is ALWAYS equal to
    let tup: <Nice as Intuple>::Intuple = (3,2,1);
    // is IN THIS CASE equal to
    let tup: (u32, u32, u32) = (3,2,1);
}
```
## Ignoring
🦥 ignore specific fields with `#[igno]`<br>
🐼 ignored fields need/use [Default](https://doc.rust-lang.org/std/default/trait.Default.html) while converting to a struct
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
🦊 convert recursively with `#[recursive]` or `#[rcsv]` <br>
🐼 recursive fields need to derive `Intuple`
```rust 
#[derive(Intuple)]
struct Struct {a:u32, b:u32, c:u32}
#[derive(Intuple)]
struct Recursive {a:u32, #[recursive] b:Struct, c:u32}
fn main(){
    let rcsv: Recursive = (9,(3,2,1),8).into(); 
    // => Recursive{a:9, b:Struct{a:3,b:2,c:1}, c:8}
    let tuple: RecursiveIntuple = rcsv.into(); 
    // => (9,(3,2,1),8)
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