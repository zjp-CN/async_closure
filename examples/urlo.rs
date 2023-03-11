#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

// src: https://users.rust-lang.org/t/lifetime-bounds-to-use-for-future-that-isnt-supposed-to-outlive-calling-scope/89277
mod case1 {
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

    pub async fn test() {
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
}

// src: https://users.rust-lang.org/t/how-to-express-that-the-future-returned-by-a-closure-lives-only-as-long-as-its-argument/90039
mod case2 {
    use async_lock::RwLock;

    async fn access<T, F>(data: &RwLock<String>, accessor: F) -> T
    where
        F: for<'any> async_closure::capture_no_lifetimes::AsyncFnOnce<(&'any str,), Output = T>,
    {
        let guard = data.read().await;
        let args = (&guard[..],);
        accessor.call_once(args).await
    }

    pub async fn test() {
        use async_closure::async_owned_closure_once as cb;
        let data = RwLock::new("sehr problematisch".to_owned());
        let _data_len = access(
            &data,
            cb!({}; async |message: &str| -> usize { message.len() }),
        )
        .await;
    }
}

// src: https://users.rust-lang.org/t/async-constrain-lifetime-of-fut-type-param-to-lifetime-of-for-func-f-fn-fut/90676
mod case3 {
    use async_closure::{async_closure_once as cb, capture_lifetimes::AsyncFnOnce};

    pub async fn test() -> Result<()> {
        generic_fn(cb!({
            ctx: Ctx<'a> = Ctx(&1)
        }; async | | -> Result<u32> {
            specific_fun_1(&ctx).await
        }))
        .await?;

        generic_fn(cb!({
            ctx: Ctx<'a> = Ctx(&2)
        }; async | | -> Result<u64> {
            specific_fun_2(&ctx).await
        }))
        .await?;

        Ok(())
    }

    async fn generic_fn<'env, T, F>(f: F) -> Result<()>
    where
        F: AsyncFnOnce<'env, (), Output = Result<T>>,
        T: std::fmt::Display,
    {
        let t = f.call_once(()).await?;
        println!("{t}");

        Ok(())
    }

    async fn specific_fun_1(ctx: &Ctx<'_>) -> Result<u32> {
        Ok(u32::from(*ctx.0))
    }

    async fn specific_fun_2(ctx: &Ctx<'_>) -> Result<u64> {
        Ok(u64::from(*ctx.0))
    }

    struct Ctx<'ctx>(&'ctx u16);
    type AppErr = Box<dyn std::error::Error>;
    type Result<T> = std::result::Result<T, AppErr>;
}

#[pollster::main]
async fn main() {
    case1::test().await;
    case2::test().await;
    case3::test().await.unwrap();
}
