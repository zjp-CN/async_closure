#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::{async_owned_closure, capture_no_lifetimes::AsyncFn};

async fn take_a_closure<T, F>(accessor: F) -> T
where
    F: for<'a> AsyncFn<(&'a str,), Output = T>,
{
    let s = String::from("-");
    accessor.call((&s,)).await
}

async fn take_and_return_a_closure<T, F>(accessor: F) -> F
where
    T: std::fmt::Debug,
    F: for<'a> AsyncFn<(&'a str,), Output = T>,
{
    let s = String::from("-");
    dbg!(accessor.call((&s,)).await);
    accessor
}

async fn test_access() {
    let outer = String::from("+");

    let cb = async_owned_closure!({
        outer: std::sync::Arc<str> = outer.into()
    }; async |s: &str| -> Result<usize, Box<dyn std::error::Error>> {
        let outer2 = outer.clone();
        let new_str = async move { Ok::<_, String>(outer2) }.await?;
        Ok(new_str.len() + outer.len() + s.len())
    });
    let cb_new = take_and_return_a_closure(cb).await;
    assert_eq!(&*cb_new.outer, "+"); // You own the values of fields
    let n = take_a_closure(cb_new).await; // or continue passing it around
    assert_eq!(n.unwrap(), 3);
}

#[pollster::test]
async fn test() {
    test_access().await;
}
