#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::{async_owned_closure_mut, capture_no_lifetimes::AsyncFnMut};

async fn take_a_closure<T, F>(mut accessor: F) -> T
where
    F: for<'a> AsyncFnMut<(&'a str,), Output = T>,
{
    let s = String::from("-");
    accessor.call_mut((&s,)).await
}

async fn take_and_return_a_closure<T, F>(mut accessor: F) -> F
where
    T: std::fmt::Debug,
    F: for<'a> AsyncFnMut<(&'a str,), Output = T>,
{
    let s = String::from("-");
    dbg!(accessor.call_mut((&s,)).await);
    accessor
}

async fn test_access() {
    let outer = String::from("+");

    let cb = async_owned_closure_mut!({
        outer : String = outer
    }; async |s: &str| -> usize {
        outer.push_str(s);
        outer.len()
    });
    let cb_new = take_and_return_a_closure(cb).await;
    assert_eq!(&cb_new.outer, "+-"); // You own the values of fields
    let n = take_a_closure(cb_new).await; // or continue passing it around
    assert_eq!(n, 3);
}

#[tokio::test]
async fn test() {
    test_access().await;
}
