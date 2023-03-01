#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(type_alias_impl_trait)]

use async_closure::async_closure;
use core::future::Future;

#[tokio::test]
async fn async_callback() {
    let string = String::from("Context");
    let sleep = 1usize;
    let cb = async_closure!({
        s: String = string,
        secs: u64 = sleep as _
    }; (arg, ); {
        println!("The first captured variable is {s:?}");
        println!("Sleep for {secs} secs");
        tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await;
        s.len() + arg.len()
    });
    let len = call(cb).await;
    assert_eq!(len, 18);
}

#[tokio::test]
async fn fn_mut() {
    let string = String::from("Context");
    let sleep = 1usize;
    let cb = {
        #[derive(Clone)]
        struct __AsyncClosure {
            s: String,
            secs: u64,
        }
        type Fut<'a> = impl 'a + core::future::Future<Output = usize>;
        impl<'a> ::core::ops::FnOnce<(&'a str,)> for __AsyncClosure {
            type Output = Fut<'a>;
            extern "rust-call" fn call_once(mut self, args: (&'a str,)) -> Self::Output {
                self.call_mut(args)
            }
        }
        impl<'a> ::core::ops::FnMut<(&'a str,)> for __AsyncClosure {
            extern "rust-call" fn call_mut(&mut self, args: (&'a str,)) -> Fut<'a> {
                let Self { s, secs } = self.clone();
                #[allow(unused_parens)]
                let (arg,) = args;
                async move {
                    {
                        println!("The first captured variable is {s:?}");
                        println!("Sleep for {secs} secs");
                        tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await;
                        s.len() + arg.len()
                    }
                }
            }
        }
        #[allow(clippy::redundant_field_names)]
        __AsyncClosure {
            s: string,
            secs: sleep as _,
        }
    };
    let len = call(cb).await;
    assert_eq!(len, 18);
}

trait AsyncFn<'a, T>: FnOnce(&'a str) -> Self::Fut {
    type Fut: 'a + Future<Output = T>;
}
impl<'a, T, F, Fut> AsyncFn<'a, T> for F
where
    F: FnOnce(&'a str) -> Fut,
    Fut: 'a + Future<Output = T>,
{
    type Fut = Fut;
}

async fn call<T, F>(f: F) -> T
where
    F: for<'a> AsyncFn<'a, T>,
{
    let string = String::from("Hello World");
    f(&string).await
}

async fn call_mut<T, F>(f: F) -> T
where
    F: for<'a> AsyncFn<'a, T>,
{
    let mut string = String::from("Hello World");
    f(&mut string).await
}
