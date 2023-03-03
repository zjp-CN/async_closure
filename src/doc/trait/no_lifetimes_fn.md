
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
use async_closure::{capture_no_lifetimes::AsyncFn, async_owned_closure};
use std::sync::{Mutex, Arc};

// Here a caller requires a generic output.
async fn caller<T, F>(f: F) -> (T, F)
where F: for<'any> AsyncFn<(&'any str,), Output = T>
{
    let s = String::from("Hi ");
    f.call((&s[..],)).await;
    (f.call(("there!",)).await, f)
}

#[pollster::main]
async fn main() {
    let context = Arc::new(Mutex::new(String::new()));

    let cb = async_owned_closure!({
        buf: Arc<Mutex<String>> = context.clone()
    }; async |s: &str| -> usize {
        buf.lock().unwrap().push_str(s);
        s.len()
    });
    let (last_len, cb) = caller(cb).await;
    assert_eq!(last_len, 6);
    // have access to the closure's states
    assert_eq!(&**cb.buf.lock().unwrap(), "Hi there!");

    let cb = async_owned_closure!({
        buf: Arc<Mutex<String>> = context.clone()
    }; async |s: &str| -> std::fmt::Result {
        use std::fmt::Write;
        write!(&mut *buf.lock().unwrap(), " {s}")?;
        Ok(())
    });
    let (res, cb) = caller(cb).await;
    assert!(res.is_ok());
    assert_eq!(&**cb.buf.lock().unwrap(), "Hi there! Hi  there!");
    {
        // Subtrait relation
        use async_closure::capture_no_lifetimes::AsyncFnOnce;
        assert!(cb.call_once((":)",)).await.is_ok());
    }

    assert_eq!(&**context.lock().unwrap(), "Hi there! Hi  there! :)");
}
```

</details>

<details>
  <summary>A struct caller implemented with specific types...</summary>

```rust
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::{capture_no_lifetimes::AsyncFn, async_owned_closure};
use std::{marker::PhantomData, sync::{Mutex, Arc}};

struct Caller<T, F> {
    async_closure: F,
    _ph: PhantomData<*mut T>,
}

// Generic impls like the caller function above are similar.
// But here we present a specific scenario where its arguments and output are defined clearly.
impl<F> Caller<Arc<Mutex<String>>, F>
where F: for<'any> AsyncFn<(&'any str,), Output = Arc<Mutex<String>>>
{
    async fn run(self, s: &str) {
        self.async_closure.call((s,)).await;
        let mutex = self.async_closure.call((" world!",)).await;
        mutex.lock().unwrap().push_str(" :)");
    }
}

#[pollster::main]
async fn main() {
    let context = Arc::new(Mutex::new(String::new()));

    let cb = async_owned_closure!({
        buf: Arc<Mutex<String>> = context.clone()
    }; async |s: &str| -> Arc<Mutex<String>> {
        buf.lock().unwrap().push_str(s);
        buf.clone()
    });
    let caller = Caller { async_closure: cb, _ph: PhantomData };
    caller.run("Hello").await;
    assert_eq!(&**context.lock().unwrap(), "Hello world! :)");
}
```

</details>

