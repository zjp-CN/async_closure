
# Examples

Normally, `'env` is used to express all referenced values outside the function and
a [HRTB] to express any referenced value passed in as an argument.

[HRTB]: https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds

<details>
  <summary>
  A function caller returning generics... 
  </summary>

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::{capture_lifetimes::AsyncFnOnce, async_closure_once};

// Here a caller requires a generic output.
async fn caller<'env, T, F>(f: F) -> T
where F: for<'any> AsyncFnOnce<'env, (&'any str,), Output = T>
{
    let s = String::from("Hi!");
    let args = (&s[..],);
    f.call_once(args).await
}

#[pollster::main]
async fn main() {
    let mut context = String::new();

    let cb = async_closure_once!({
        buf: &'a mut String = &mut context
    }; async |s: &str| -> usize {
        buf.push_str(s);
        s.len()
    });
    assert_eq!(caller(cb).await, 3);
    assert_eq!(context, "Hi!");

    let cb = async_closure_once!({
        buf: &'a mut String = &mut context
    }; async |s: &str| -> std::fmt::Result {
        use std::fmt::Write;
        write!(buf, " {s}")?;
        Ok(())
    });
    assert!(caller(cb).await.is_ok());
    assert_eq!(context, "Hi! Hi!");
}
```

</details>

<details>
  <summary>A struct caller implemented with specific types...</summary>

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::{capture_lifetimes::AsyncFnOnce, async_closure_once};
use std::marker::PhantomData;

struct Caller<'env, T, F> {
    async_closure: F,
    _ph: PhantomData<&'env mut T>,
}

// Generic impls like the caller function above are similar.
// But here we present a specific scenario where its arguments and output are defined clearly.
impl<'env, F> Caller<'env, &'env mut String, F>
where F: for<'any> AsyncFnOnce<'env, (&'any str,), Output = &'env mut String>
{
    async fn run(self, s: &str) {
        let buf = self.async_closure.call_once((s,)).await;
        buf.push_str(" world!");
    }
}

#[pollster::main]
async fn main() {
    let mut context = String::new();

    let cb = async_closure_once!({
        buf: &'a mut String = &mut context
    }; async |s: &str| -> &'a mut String {
        buf.push_str(s);
        buf
    });
    let caller = Caller { async_closure: cb, _ph: PhantomData };
    caller.run("Hello").await;
    assert_eq!(context, "Hello world!");
}
```

</details>

