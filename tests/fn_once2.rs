#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

async fn take_a_closure<T, F>(accessor: F) -> T
where
    F: for<'a> AsyncFnOnce<(&'a str,), Output = T>,
{
    let s = String::from("-");
    accessor.call_once((&s,)).await
}

trait AsyncFnOnce<In> {
    type Output;
    async fn call_once(self, args: In) -> Self::Output;
}

trait AsyncFnMut<In>: AsyncFnOnce<In> {
    async fn call_mut(&mut self, args: In) -> Self::Output;
}

trait AsyncFn<In>: AsyncFnMut<In> {
    async fn call(&self, args: In) -> Self::Output;
}

struct Cb {
    s: std::sync::Arc<str>,
}
impl AsyncFnOnce<(&str,)> for Cb {
    type Output = usize;
    async fn call_once(self, args: (&str,)) -> usize {
        futures::future::ready(self.s.clone()).await;
        self.s.len() + args.0.len()
    }
}
impl AsyncFnMut<(&str,)> for Cb {
    async fn call_mut(&mut self, args: (&str,)) -> usize {
        futures::future::ready(self.s.clone()).await;
        self.s.len() + args.0.len()
    }
}
impl AsyncFn<(&str,)> for Cb {
    async fn call(&self, args: (&str,)) -> usize {
        futures::future::ready(self.s.clone()).await;
        self.s.len() + args.0.len()
    }
}

async fn test_access() {
    let outer = String::from("+");

    let n = take_a_closure(Cb { s: outer.into() }).await;
    assert_eq!(n, 2);
}

#[tokio::test]
async fn test() {
    test_access().await;
}
