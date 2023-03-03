

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::async_owned_closure_once;

// correspond to `capture_no_lifetimes::AsyncFnOnce<(), usize>`
// note that the concept of arguments is represented by a tuple
async_owned_closure_once!({}; async | | -> usize { 0 }); 

// `for<'any> capture_no_lifetimes::AsyncFnOnce< (&'any str,), usize>`
// note that single argument is represented by `(type,)` where the comma is important
async_owned_closure_once!({}; async |_s: &str| -> usize { 0 }); 

// `for<'any> capture_no_lifetimes::AsyncFnOnce<(&'any str, &'any mut Vec<u8>), ()>`
async_owned_closure_once!({}; async |_s: &str, _buf: &mut Vec<u8>| -> () {}); 

// etc.
``` 

# Examples

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::async_owned_closure_once as cb;

let v = vec![];

// zero arguments
cb!({
    v: Vec<u8> = v,
}; async | | -> usize {
    // to show the type; you don't have to do this
    let v: Vec<u8> = v;
    // but rebind when you mutate it
    let mut v: Vec<u8> = v;
    v.push(0);
    v.len()
});

// one argument and returns unit type
cb!({
    v: Vec<u8> = vec![],
}; async |_s: &mut [u8]| -> () { }); // `_` is not an ident, thus not allowed

// two arguments and returns the owned data
use std::borrow::Cow;
cb!(
    { v: Vec<u8> = Vec::new() };
    async |arg1: &mut [u8], arg2: Cow<'_, str>| -> Vec<u8> { v }
);

// three arguments and rebinds the variables
cb!({ v: Vec<u8> = vec![] };
    async |arg1: &mut [u8], arg2: Cow<'_, str>, vec: Vec<u8>| -> () {
    let mut vec = vec; // rebinding
    let mut v = v;     // rebinding
    std::mem::swap(&mut vec, &mut v);
});
```

