#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

async fn access<'a, T, F>(mut accessor: F) -> T
where
    F: AsyncFnMut<'a, T>,
{
    let s = String::from("-");
    accessor.call(&s).await;
    accessor.call(&s).await
}

trait AsyncFnMut<'a, T> {
    async fn call(&mut self, message: &str) -> T;
}

struct Cb<'s> {
    s: &'s mut String,
}

impl<'s> AsyncFnMut<'s, usize> for Cb<'s> {
    async fn call(&mut self, message: &str) -> usize {
        futures::future::ready(&*self.s).await;
        self.s.push_str(message);
        self.s.len()
    }
}

async fn test_access() {
    let mut outer = String::from("+");
    let n = access(Cb { s: &mut outer }).await;
    assert_eq!(n, 3);
}

#[tokio::test]
async fn test() {
    test_access().await;
}
