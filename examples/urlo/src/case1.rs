type Error = ();
struct Session {}

impl Session {
    pub async fn dispatch_request<T, F>(&self, interpreter: F) -> Result<T, Error>
    where
        F: for<'any> async_closure::capture_no_lifetimes::AsyncFnOnce<
            (&'any mut String,),
            Output = Result<T, Error>,
        >,
    {
        let mut stream = "hello".to_string();
        interpreter.call_once((&mut stream,)).await
    }
}

pub async fn main() {
    use async_closure::async_owned_closure_once as cb;
    let session = Session {};

    session
        .dispatch_request(
            cb!({}; async |input_stream: &mut String| -> Result<String, Error> {
                 Ok(input_stream.clone())
            }),
        )
        .await
        .unwrap();
}
