#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

// src: https://users.rust-lang.org/t/lifetime-bounds-to-use-for-future-that-isnt-supposed-to-outlive-calling-scope/89277
mod case1;
// src: https://users.rust-lang.org/t/how-to-express-that-the-future-returned-by-a-closure-lives-only-as-long-as-its-argument/90039
mod case2;
// src: https://users.rust-lang.org/t/async-constrain-lifetime-of-fut-type-param-to-lifetime-of-for-func-f-fn-fut/90676
mod case3;
// https://users.rust-lang.org/t/how-to-pass-async-closure-with-references-in-the-argument/90745
mod case4;
// src: https://users.rust-lang.org/t/async-callback-with-serde-deserialize-zero-copy/90801
mod case5;

#[pollster::main]
async fn main() {
    case1::main().await;
    case2::main().await;
    case3::main().await.unwrap();
    case4::main().await;
    case5::main().await.unwrap();
}
