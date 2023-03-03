#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

async fn access<'a, T, F>(accessor: F) -> T
where
    F: AsyncFn<'a, T>,
{
    let s = String::from("-");
    accessor.call(&s).await;
    accessor.call(&String::from("123")).await;
    accessor.call("?").await
}

trait AsyncFn<'a, T> {
    async fn call(&self, message: &str) -> T;
}

struct Cb<'s> {
    s: &'s str,
}

impl<'s> AsyncFn<'s, usize> for Cb<'s> {
    async fn call(&self, message: &str) -> usize {
        async move { self.s }.await;
        self.s.len() + message.len()
    }
}

async fn test_access() {
    let outer = String::from("+");
    let n = access(Cb { s: &outer }).await;
    assert_eq!(n, 2);
}

#[pollster::test]
async fn test() {
    test_access().await;
}
