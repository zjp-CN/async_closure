#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
use async_closure::{async_closure_once, capture_lifetimes::AsyncFnOnce};
use futures::future::ready;

async fn take_a_closure<'a, T, F>(cb: F) -> T
where
    F: for<'s> AsyncFnOnce<'a, (&'s str,), Output = T>,
{
    let s = String::from("-");
    let args = (&s[..],);
    cb.call_once(args).await
}

async fn test() {
    let outer = String::from("+");

    let cb1 = async_closure_once!({
        s: &'a str = &outer,
    }; async |arg: &str| -> usize {
        ready(s).await;
        s.len() + arg.len()
    });
    let n = take_a_closure(cb1).await;
    assert_eq!(n, 2);

    let cb2 = async_closure_once!({
        s: &'a str = &outer,
        u: usize = 123
    }; async |arg: &str| -> &'a str {
        ready(u + s.len() + arg.len()).await;
        s
    });
    let s = take_a_closure(cb2).await;
    assert_eq!(s, "+");

    let mut buf = String::new();
    let cb3 = async_closure_once!({
        s: &'a str = &outer,
        buf: &'a mut String = &mut buf,
        u: usize = 123
    }; async |arg: &str| -> std::fmt::Result {
        use std::fmt::Write;
        write!(buf, "{s}{u}{arg}")
    });
    take_a_closure(cb3).await.unwrap();
    assert_eq!(buf, "+123-");
}

#[tokio::test]
async fn fn_once() {
    test().await;
    test2().await;
}

async fn take_another_closure<'a, T, F>(cb: F) -> T
where
    F: for<'s> AsyncFnOnce<'a, (&'s str, &'s mut String), Output = T>,
{
    let s1 = String::from("-");
    let mut s2 = String::from("-");
    let args = (&s1[..], &mut s2);
    cb.call_once(args).await
}

async fn test2() {
    let outer = String::from("+");
    let cb = async_closure_once!({
        outer: String = outer
    }; async |s: &str, s_mut: &mut String| -> String {
        let mut outer = outer; // rebinding
        s_mut.push_str(&outer);
        s_mut.push_str(s);
        std::mem::swap(&mut outer, s_mut);
        outer
    });
    assert_eq!(take_another_closure(cb).await, "-+-");
}
