#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

async fn access<'a, T, F>(accessor: F) -> T
where
    F: AsyncFnonce<'a, T>,
{
    let s = String::from("-");
    accessor.call(&s).await
}

trait AsyncFnOnce<'a, In, Out> {
    async fn call(self, message: In) -> Out;
}
impl<'s> AsyncFnOnce<'s, &str, usize> for Cb<'s> {
    async fn call(self, message: &str) -> usize {
        futures::future::ready(self.s).await;
        self.s.len() + message.len()
    }
}
async fn access2<'a, T, F>(accessor: F) -> T
where
    F: for<'s> AsyncFnOnce<'a, &'s str, T>,
{
    let s = String::from("-");
    accessor.call(&s).await
}

trait AsyncFnonce<'a, T> {
    async fn call(self, message: &str) -> T;
}

struct Cb<'s> {
    s: &'s str,
}
impl<'s> AsyncFnonce<'s, usize> for Cb<'s> {
    async fn call(self, message: &str) -> usize {
        futures::future::ready(self.s).await;
        self.s.len() + message.len()
    }
}

struct Cb2<'s> {
    s: &'s str,
    u: usize,
}
impl<'s> AsyncFnonce<'s, &'s str> for Cb2<'s> {
    async fn call(self, message: &str) -> &'s str {
        futures::future::ready(message.len() + self.u).await;
        self.s
    }
}

async fn test_access() {
    let outer = String::from("+");

    let n = access(Cb { s: &outer }).await;
    assert_eq!(n, 2);
    let n = access2(Cb { s: &outer }).await;
    assert_eq!(n, 2);

    let s = access(Cb2 { s: &outer, u: 123 }).await;
    assert_eq!(s, "+");
}

#[tokio::test]
async fn test() {
    test_access().await;
}
