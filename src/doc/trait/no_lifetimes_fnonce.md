
# Examples

Normally, a [HRTB] is used to express any referenced value passed in as an argument.

[HRTB]: https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds

<details>
  <summary>
  A function caller returning generics... 
  </summary>

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::{capture_no_lifetimes::AsyncFnOnce, async_owned_closure_once};

// Here a caller requires a generic output.
async fn caller<T, F>(f: F) -> T
where F: for<'any> AsyncFnOnce<(&'any str,), Output = T>
{
    let s = String::from("Hi!");
    let args = (&s[..],);
    f.call_once(args).await
}

#[tokio::main]
async fn main() {
    let mut context = String::new();

    let cb = async_owned_closure_once!({
        buf: String = context
    }; async |s: &str| -> (usize, String) {
        let mut buf = buf; // rebinding
        buf.push_str(s);
        (s.len(), buf)
    });
    assert_eq!(caller(cb).await, (3, "Hi!".into()));

    let cb = async_owned_closure_once!({
        buf: String = String::new()
    }; async |s: &str| -> Result<String, Box<dyn std::error::Error>> {
        use std::fmt::Write;
        let mut buf = buf; // rebinding
        write!(buf, " {s}")?;
        Ok(buf)
    });
    assert_eq!(caller(cb).await.unwrap(), " Hi!");
}
```

</details>

<details>
  <summary>A struct caller implemented with specific types...</summary>

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::{capture_no_lifetimes::AsyncFnOnce, async_owned_closure_once};
use std::{marker::PhantomData, sync::Arc};

struct Caller<T, F> {
    async_closure: F,
    _ph: PhantomData<*mut T>,
}

// Generic impls like the caller function above are similar.
// But here we present a specific scenario where its arguments and output are defined clearly.
impl<F> Caller<String, F>
where F: for<'any> AsyncFnOnce<(&'any str,), Output = String>
{
    async fn run(self, s: &str) -> Arc<str> {
        let mut buf = self.async_closure.call_once((s,)).await;
        buf.push_str(" world!");
        buf.into()
    }
}

#[tokio::main]
async fn main() {
    let mut context = String::new();

    let cb = async_owned_closure_once!({
        buf: String = context
    }; async |s: &str| -> String {
        let mut buf = buf;
        buf.push_str(s);
        buf
    });
    let caller = Caller { async_closure: cb, _ph: PhantomData };
    assert_eq!(&*caller.run("Hello").await, "Hello world!");
}
```

</details>

