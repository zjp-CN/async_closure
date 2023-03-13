use async_lock::RwLock;

async fn access<T, F>(data: &RwLock<String>, accessor: F) -> T
where
    F: for<'any> async_closure::capture_no_lifetimes::AsyncFnOnce<(&'any str,), Output = T>,
{
    let guard = data.read().await;
    let args = (&guard[..],);
    accessor.call_once(args).await
}

pub async fn main() {
    use async_closure::async_owned_closure_once as cb;
    let data = RwLock::new("sehr problematisch".to_owned());
    let _data_len = access(
        &data,
        cb!({}; async |message: &str| -> usize { message.len() }),
    )
    .await;
}
