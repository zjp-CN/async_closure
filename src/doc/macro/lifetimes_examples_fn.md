

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::async_closure;

// correspond to `capture_lifetimes::AsyncFn<'env, (), usize>`
// note that the concept of arguments is represented by a tuple
async_closure!({}; async | | -> usize { 0 }); 

// `for<'any> capture_lifetimes::AsyncFn<'env, (&'any str,), usize>`
// note that single argument is represented by `(type,)` where the comma is important
async_closure!({}; async |_s: &str| -> usize { 0 }); 

// `for<'any> capture_lifetimes::AsyncFn<'env, (&'any str, &'any mut Vec<u8>), ()>`
async_closure!({}; async |_s: &str, _buf: &mut Vec<u8>| -> () {}); 

// etc.
``` 

# Examples

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::async_closure as cb;

let mut v = vec![];

// zero arguments
cb!({
    v: &'a mut Vec<u8> = &mut v,
}; async | | -> usize {
    // to show the type; you don't have to do this
    let v: &&mut Vec<u8> = v; // you can never obtain &mut via `AsyncFn`
    v.len()
});

// one argument and returns unit type
cb!({
    v: &'a [u8] = &v,
}; async |_s: &mut [u8]| -> () { }); // `_` is not an ident, thus not allowed

// two arguments and returning referenced captured data with lifetime 'a
use std::borrow::Cow;
cb!(
    { v: &'a [u8] = &v };
    async |arg1: &mut [u8], arg2: Cow<'_, str>| -> &'a [u8] { v }
);

// three arguments and rebinds the variables
cb!({ v: &'a [u8] = &v };
    async |arg1: &mut [u8], arg2: Cow<'_, str>, vec: Vec<u8>| -> () {
    let mut vec = vec; // rebinding
    vec.extend_from_slice(v); // v: &&[u8]
    vec.extend_from_slice(*v);
});
cb!({ v: Vec<u8> = v };
    async |arg1: &mut [u8], arg2: Cow<'_, str>, vec: Vec<u8>| -> () {
    let _: &Vec<u8> = v;
});
```

