```rust
fn main() {
    use tokio;
    let arr = &[1_u8; 5];
    let arr2 = &arr;
    let hi = &arr2[1..2];
    let lo = &arr[1..3];
    let ivec = MYIVec::new(arr);
    let comp1 = hi.partial_cmp(lo);
    let comp2 = hi.partial_cmp(&ivec);
    println!("{:?} {:?}", comp1, comp2);
}
```
This code does not compile when `use tokio`: 
```bash
   Compiling report_bug v0.1.0 (/data01/home/zhuhuiming.sy/git/report_bug)
warning: the item `tokio` is imported redundantly
  --> src/main.rs:24:9
   |
24 |     use tokio;
   |         ^^^^^ the item `tokio` is already defined here
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `tokio`
  --> src/main.rs:24:9
   |
24 |     use tokio;
   |         ^^^^^

error[E0277]: can't compare `[u8]` with `MYIVec`
  --> src/main.rs:31:32
   |
31 |     let comp2 = hi.partial_cmp(&ivec);
   |                    ----------- ^^^^^ no implementation for `[u8] < MYIVec` and `[u8] > MYIVec`
   |                    |
   |                    required by a bound introduced by this call
   |
   = help: the trait `PartialOrd<MYIVec>` is not implemented for `[u8]`
   = help: the following other types implement trait `PartialOrd<Rhs>`:
             <[u8] as PartialOrd<bytes::bytes::Bytes>>
             <[u8] as PartialOrd<bytes::bytes_mut::BytesMut>>
             <&[u8] as PartialOrd<bytes::bytes::Bytes>>
             <&[u8] as PartialOrd<bytes::bytes_mut::BytesMut>>

For more information about this error, try `rustc --explain E0277`.
warning: `report_bug` (bin "report_bug") generated 2 warnings
error: could not compile `report_bug` (bin "report_bug") due to previous error; 2 warnings emitted
```

Compiles when comment `use tokio`: 
```rust
fn main() {
    // use tokio;
    let arr = &[1_u8; 5];
    let arr2 = &arr;
    let hi = &arr2[1..2];
    let lo = &arr[1..3];
    let ivec = MYIVec::new(arr);
    let comp1 = hi.partial_cmp(lo);
    let comp2 = hi.partial_cmp(&ivec); // Cannot automatic-dereferencing!!!
    println!("{:?} {:?}", comp1, comp2);
}
```
Also compiles when deref manually: 
```rust
fn main() {
    use tokio;
    let arr = &[1_u8; 5];
    let arr2 = &arr;
    let hi = &arr2[1..2];
    let lo = &arr[1..3];
    let ivec = MYIVec::new(arr);
    let comp1 = hi.partial_cmp(lo);
    let comp2 = hi.partial_cmp(ivec.deref()); // manually deref working
    println!("{:?} {:?}", comp1, comp2);
}
```

