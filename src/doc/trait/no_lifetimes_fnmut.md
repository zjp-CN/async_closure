
# Examples

Normally, a [HRTB] to express any referenced value passed in as an argument.

[HRTB]: https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds

<details>
  <summary>
  A function caller returning generics... 
  </summary>

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::{capture_no_lifetimes::AsyncFnMut, async_owned_closure_mut};

// Here a caller requires a generic output.
async fn caller<T, F>(mut f: F) -> (T, F)
where F: for<'any> AsyncFnMut<(&'any str,), Output = T>
{
    let s = String::from("Hi ");
    f.call_mut((&s[..],)).await;
    (f.call_mut(("there!",)).await, f)
}

#[tokio::main]
async fn main() {
    let context = String::new();

    let cb = async_owned_closure_mut!({
        buf: String = context
    }; async |s: &str| -> usize {
        buf.push_str(s);
        s.len()
    });
    let (last_len, cb) = caller(cb).await;
    assert_eq!(last_len, 6);
    let buf = cb.buf;
    assert_eq!(buf, "Hi there!"); // have access to the closure's states

    let cb = async_owned_closure_mut!({
        buf: String = buf
    }; async |s: &str| -> std::fmt::Result {
        use std::fmt::Write;
        write!(buf, " {s}")?;
        Ok(())
    });
    let (res, cb) = caller(cb).await;
    assert!(res.is_ok());
    assert_eq!(cb.buf, "Hi there! Hi  there!");
    {
        // Subtrait relation
        use async_closure::capture_no_lifetimes::AsyncFnOnce;
        assert!(cb.call_once((":)",)).await.is_ok());
    }
}
```

</details>

<details>
  <summary>A struct caller implemented with specific types...</summary>

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::{capture_no_lifetimes::AsyncFnMut, async_owned_closure_mut};
use std::marker::PhantomData;

struct Caller<T, F> {
    async_closure: F,
    _ph: PhantomData<*mut T>,
}

// Generic impls like the caller function above are similar.
// But here we present a specific scenario where its arguments and output are defined clearly.
impl<F> Caller<(), F>
where F: for<'any> AsyncFnMut<(&'any str,), Output = ()>
{
    async fn run(mut self, s: &str) -> F {
        self.async_closure.call_mut((s,)).await;
        self.async_closure.call_mut((" world!",)).await;
        self.async_closure
    }
}

#[tokio::main]
async fn main() {
    let context = String::new();

    let cb = async_owned_closure_mut!({
        buf: String = context
    }; async |s: &str| -> () {
        buf.push_str(s);
    });
    let caller = Caller { async_closure: cb, _ph: PhantomData };
    let cb = caller.run("Hello").await;
    assert_eq!(cb.buf, "Hello world!");
}
```

</details>

