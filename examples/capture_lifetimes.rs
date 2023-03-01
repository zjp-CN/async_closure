#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
// use async_closure::capture_lifetimes::AsyncFnMut;
use std::{error::Error, sync::Arc};

use async_closure::{
    async_closure_mut, async_closure_once,
    capture_lifetimes::{AsyncFnMut, AsyncFnOnce},
};
use futures::future::ready;

async fn take_a_closure<'a, T, F>(cb: F) -> T
where
    F: for<'s> AsyncFnOnce<'a, (&'s str,), Output = T>,
{
    let s = String::from("-");
    let args = (&s[..],);
    cb.call_once(args).await
}

// 2. Adjust your caller where the trait bound looks like the following
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

async fn test3() {
    let (outer, mut buf, buffer) = (String::new(), String::new(), String::new());

    // 2. Define a capturing closure
    let cb1 = async_closure_mut!({
        s: &'a str = &outer,            // shared reference
        buf: &'a mut String = &mut buf, // mutable reference
        arc: Arc<str> = buffer.into(),  // owned type without explicit mutation
        len: usize = 0,                 // owned type with mutation, see the code below
    }; async |arg: &str| -> Result<usize, Box<dyn Error>> {
        // Write async code here, using the field names and argument names as variables

        tokio::spawn({
            let arc = arc.clone();
            async move { arc }
        }).await?;
        buf.push_str(arg);
        dbg!(&arc, &buf, &s, &arg);
        *len += arc.len() + buf.len() + s.len() + arg.len();
        Ok(*len)
    });

    // assert_eq!(take_a_closure(cb1).await.unwrap(), 2);
    assert_eq!(take_a_mut_closure(cb1).await.unwrap(), 7);
}

async fn take_and_return_a_mut_closure<'env, T, F>(mut cb: F) -> (T, F)
where
    F: for<'any> AsyncFnMut<'env, (&'any str,), Output = T>,
{
    let s = String::from("-");
    (cb.call_mut((&s[..],)).await, cb)
}

async fn test4() {
    let mut buf = String::new();
    let cb = async_closure_mut!({
        buf: &'a mut String = &mut buf
    }; async |arg: &str| -> () {
        buf.push_str(arg);
    });
    let (_output, cb_again) = take_and_return_a_mut_closure(cb).await;
    cb_again.buf.push('+');
    assert_eq!(cb_again.buf, "-+");
    take_a_mut_closure(cb_again).await;
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

#[tokio::main]
async fn main() {
    test().await;
    test2().await;
    test3().await;
    test4().await;
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
