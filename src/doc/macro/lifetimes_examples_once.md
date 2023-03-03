

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::async_closure_once;

// correspond to `capture_lifetimes::AsyncFnOnce<'env, (), usize>`
// note that the concept of arguments is represented by a tuple
async_closure_once!({}; async | | -> usize { 0 }); 

// `for<'any> acapture_lifetimes::AsyncFnOnce<'env, (&'any str,), usize>`
// note that single argument is represented by `(type,)` where the comma is important
async_closure_once!({}; async |_s: &str| -> usize { 0 }); 

// `for<'any> acapture_lifetimes::AsyncFnOnce<'env, (&'any str, &'any mut Vec<u8>), ()>`
async_closure_once!({}; async |_s: &str, _buf: &mut Vec<u8>| -> () {}); 

// etc.
``` 

# Examples

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::async_closure_once as cb;

let mut v = vec![];

// zero arguments
cb!({
    v: &'a mut Vec<u8> = &mut v,
}; async | | -> usize {
    // to show the type; you don't have to do this
    let v: &mut Vec<u8> = v;
    v.push(0);
    v.len()
});

// one argument and returns unit type
cb!({
    v: &'a mut Vec<u8> = &mut v,
}; async |_s: &mut [u8]| -> () { }); // `_` is not an ident, thus not allowed

// two arguments and returns the captured data (though meaningless)
use std::borrow::Cow;
cb!(
    { v: &'a mut Vec<u8> = &mut v };
    async |arg1: &mut [u8], arg2: Cow<'_, str>| -> &'a mut Vec<u8> { v }
);

// three arguments and rebinds the variable
cb!({ v: &'a mut Vec<u8> = &mut v };
    async |arg1: &mut [u8], arg2: Cow<'_, str>, vec: Vec<u8>| -> () {
    let mut vec = vec; // rebinding
    std::mem::swap(&mut vec, v);
});
cb!({ v: Vec<u8> = v };
    async |arg1: &mut [u8], arg2: Cow<'_, str>, vec: Vec<u8>| -> () {
    let mut vec = vec; // rebinding
    let mut v = v; // rebinding
    std::mem::swap(&mut vec, &mut v);
});
```

