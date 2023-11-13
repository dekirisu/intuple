# v0.2.0 (2023-11-13)
- 🦎 removed `IntupleLite` (didn't have any benefit)
- 🦎 removed `Intuple` trait for tuples (didn't have any benefit)
- 🐍 renamed `IntupleStruct` trait to `Intuple`
    - removed `fruple(..)`
- 🐤 added the possibility to get a tuple of references to struct fields
    - added `IntupleRef` trait with functions `as_tuple_ref()` and `as_tuple_ref_mut()`
    - reference tuples are affected by `#[ignore]`
    - reference tuples support recursion using `#[recursive]`
# v0.1.0 (2022-09-26)
* 🐣 first version: convert structs into/from tuples:
    - with field ignoring: `#[igno]`
    - with optional recursion: `#[recursive]`/`#[rcsv]`
    - destinct traits - for special use cases, e.g. dyn ... or impl parameters
    - basic examples
    - basic tests