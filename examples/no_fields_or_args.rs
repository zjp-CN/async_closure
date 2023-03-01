#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

mod referenced {
    mod caller {
        use async_closure::capture_lifetimes as imp;

        pub async fn take_once<'env, T, F>(f: F) -> T
        where
            F: imp::AsyncFnOnce<'env, (), Output = T>,
        {
            f.call_once(()).await
        }
        pub async fn take_mut<'env, T, F>(f: F) -> T
        where
            F: imp::AsyncFnMut<'env, (), Output = T>,
        {
            f.call_once(()).await
        }
        pub async fn take_ref<'env, T, F>(f: F) -> T
        where
            F: imp::AsyncFn<'env, (), Output = T>,
        {
            f.call_once(()).await
        }
        pub async fn check_once<'env, F, Args, Out>(_: F)
        where
            F: imp::AsyncFnOnce<'env, Args, Output = Out>,
        {
        }
    }

    mod test {
        use super::caller;
        pub async fn take_once() {
            use async_closure::async_closure_once as cb;

            caller::take_once(cb!({}; async | | -> () {})).await;

            let v = vec![];
            caller::take_once(cb!({
                v: &'a [u8] = &v,
            }; async | | -> usize { v.len() }))
            .await;
        }
        pub async fn check_once() {
            use async_closure::async_closure_once as cb;
            use caller::check_once as check;

            check::<_, (), ()>(cb!({}; async | | -> () {})).await;

            let v = vec![];
            check::<_, (), usize>(cb!({
                v: &'a [u8] = &v,
            }; async | | -> usize { v.len() }))
            .await;

            check::<_, (usize,), usize>(cb!({
                v: &'a [u8] = &v,
            }; async |u: usize| -> usize { v.len() + u }))
            .await;

            check::<_, (usize,), usize>(cb!({
                v: Vec<u8> = v,
            }; async |u: usize| -> usize { v.len() + u }))
            .await;
        }
    }

    pub async fn tests() {
        test::take_once().await;
        test::check_once().await;
    }
}

#[tokio::main]
async fn main() {
    referenced::tests().await;
}
