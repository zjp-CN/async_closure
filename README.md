# `async_closure`

[![GitHub issues](https://img.shields.io/github/issues/zjp-CN/async_closure)](https://github.com/zjp-CN/async_closure)
[<img alt="github" src="https://img.shields.io/github/issues/zjp-CN/async_closure?color=db2043" height="20">](https://github.com/zjp-CN/async_closure/issues)
[![test suite](https://github.com/zjp-CN/async_closure/actions/workflows/ci.yml/badge.svg)](https://github.com/zjp-CN/async_closure/actions/workflows/ci.yml)
[<img alt="crates.io" src="https://img.shields.io/crates/v/async_closure?style=flat&color=fc8d62&logo=rust&label=async_closure" height="20">](https://crates.io/crates/async_closure)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/async_closure" height="20">](https://docs.rs/async_closure)

This crate utilizes the nightly-only feature [`async_fn_in_trait`] to imitate [async_closures].

You don't have to Box the return Future from a local closure in async code this time!

The steps to use this crate:
1. Choose an async trait which is used in trait bounds
    * use traits in [`capture_no_lifetimes`] mod when you're sure there won't be any temporarily
      referenced type to be captured (i.e. all captured types statisfy `'static` bound).
    * otherwise, use traits in [`capture_lifetimes`] mod.

    ```rust,ignore
    // e.g. take a closure that potentially captures references and will change its states.

    // 0. import the `AsyncFnMut` trait and companion macro `async_closure_mut`
    #![feature(async_fn_in_trait)]
    #![allow(incomplete_features)]
    use async_closure::{capture_lifetimes::AsyncFnMut, async_closure_mut};

    // 1. Adjust your caller where the trait bound looks like the following
    async fn take_a_mut_closure<'env, T, F>(mut cb: F) -> T
    where
        // 'env means the lifetime of captured variables outside the function
        // 'any means the lifetime of arguments coming from inside the function
        // also note how we express and represent one argument via `(arg1, )`,
        // likewise we can declare more arguments via `(arg1, arg2, arg3, )` etc.
        F: for<'any> AsyncFnMut<'env, (&'any str,), Output = T>,
    {
        let mut s = String::from("-");
        let args = (&s[..],); // type inference doesn't work well here
        cb.call_mut(args).await;

        s.push('+');
        let args = (&s[..],);
        cb.call_mut(args).await
    }
    ```

2. Use its companion macro to auto generate a value, the type of which is distinct and unnamed
   but implemented with the trait. The syntax in macros contains two parts:
    * For capture_lifetimes style macros:
        1. a block `{}` where multiple assignments `field_name: field_type = field_value` seperated by `,` are declared
        2. an async closure `async |arg1: arg1_type, ...| -> return_type { /* any async code here */ }`

      Note: `AsyncFn*` family only contains single lifetime parameter `'a`, and you must use it in `field_type` and `return_type`
            to express the non-static referenced type. But if the type there doesn't contain a reference, `'a` is needless.

    ```rust,ignore
    let (outer, mut buf, buffer) = (String::new(), String::new(), String::new());

    // 2. Define a capturing closure
    let cb1 = async_closure_mut!({
        s: &'a str = &outer,            // shared reference
        buf: &'a mut String = &mut buf, // mutable reference
        arc: Arc<str> = buffer.into(),  // owned type without explicit mutation
        len: usize = 0,                 // owned type with mutation, see the code below
    }; async |arg: &str| -> Result<usize, Box<dyn Error>> 
       // Annotate both inputs and output: no lifetime parameter on arguments' type!
    {
        // Write async code here, using the field names and argument names as variables.
        // Note: the type of fields here depends on `AsyncFn*`.
        // i.e. for this macro, all field comes from `let Self { pattern_match } = &mut self;`
        // thus `s: &mut &'a str`, `buf: &mut &'a mut String` etc.
        // If you use `async_closure!`, then `s: &&'a str`, `buf: &&'a mut String` etc.
        // If you use `async_closure_once!`, then `s: &'a str`, `buf: &'a mut String` etc.

        tokio::spawn({
            let arc = arc.clone();
            async move { arc }
        }).await?;
        buf.push_str(arg);
        dbg!(&arc, &buf, &s, &arg);
        *len += arc.len() + buf.len() + s.len() + arg.len();
        Ok(*len)
    });
    ```

    * For non capture_lifetimes style macros, same components except that
        * you can't use `'a`
            * i.e. `field_type` and `return_type` can't be temporary referenced types

3. Pass the value into the caller function

    ```rust,ignore
    take_a_mut_closure(cb1).await.unwrap(); // That's it :)
    ```

| macro                         | trait                                 | capture references | mutate fields | times to be used |
|-------------------------------|---------------------------------------|:------------------:|:-------------:|:----------------:|
| [`async_closure!`]            | [`capture_lifetimes::AsyncFn`]        |          √         |       ×       |     no limit     |
| [`async_closure_mut!`]        | [`capture_lifetimes::AsyncFnMut`]     |          √         |       √       |     no limit     |
| [`async_closure_once!`]       | [`capture_lifetimes::AsyncFnOnce`]    |          √         |       √       |         1        |
| [`async_owned_closure!`]      | [`capture_no_lifetimes::AsyncFn`]     |          ×         |       ×       |     no limit     |
| [`async_owned_closure_mut!`]  | [`capture_no_lifetimes::AsyncFnMut`]  |          ×         |       √       |     no limit     |
| [`async_owned_closure_once!`] | [`capture_no_lifetimes::AsyncFnOnce`] |          ×         |       √       |         1        |


<details>
  <summary>FAQ</summary>
  
  1. Requirement for Rust?

  MSRV: v1.69.0, and nightly-only due to the [`async_fn_in_trait`] feature.

  2. Why do I need this?

  To avoid boxing the return Future from a local closure as I said.

  Try this crate if you're not statisfied with the traditional approaches [as discussed here][discussed].
  But they do work on stable Rust. If you're not familiar, it's worth reading.

  If you can use [`async_fn_in_trait`] feature, of course you probably define a custom trait with
  meaningful method calls. But it also means to define context-based structs that are hardly used twice.

  So this crate can generate these structs behind the scenes to reduce boilerplate code.

  And an advantage over closures is you're able to keep the (non-once) structs alive as long as you want.

  ```rust,ignore
  async fn take_and_return_a_mut_closure<'env, T, F>(mut cb: F) -> (T, F)
  where
      F: for<'any> AsyncFnMut<'env, (&'any str,), Output = T>,
  {
      let s = String::from("-");
      (cb.call_mut((&s[..],)).await, cb) // Note: return the closure type
  }

  async fn test4() {
      let mut buf = String::new();
      let cb = async_closure_mut!({
          buf: &'a mut String = &mut buf
      }; async |arg: &str| -> () {
          buf.push_str(arg);
      });
      let (_output, cb_again) = take_and_return_a_mut_closure(cb).await;

      cb_again.buf.push('+'); // Still use it
      assert_eq!(cb_again.buf, "-+");
      
      take_a_mut_closure(cb_again).await; // And pass it into another function
      // Note: since AsyncFnMut is the subtrait to AsyncFnOnce,
      //       you can pass it into a fucntion that requires AsyncFnOnce
      //       as long as they have identical generic parameters.
  }
  ```

  3. How to work on stable Rust?

  Impossible for now. See the second question above that gives a link to show traditional *well-known* stable ways,
  especially for non-capturing async callbacks/functions.

  4. More examples?

  Yes. It took me hours to write examples for each trait and macro. Besides have a look at [examples] folder.

  [examples]: https://github.com/zjp-CN/async_closure/tree/main/examples

</details>

[`async_fn_in_trait`]: https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html
[async_closures]: https://rust-lang.github.io/async-fundamentals-initiative/roadmap/async_closures.html
[discussed]: https://users.rust-lang.org/t/lifetime-bounds-to-use-for-future-that-isnt-supposed-to-outlive-calling-scope/89277
