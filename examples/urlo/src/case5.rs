use async_closure::{async_owned_closure_mut as cb, capture_no_lifetimes::AsyncFnMut};
use serde::{Deserialize, Serialize};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

// Deserialize target
#[derive(Serialize, Deserialize, Debug)]
pub struct Sample<'a> {
    a: &'a str,
    b: u64,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<'a> {
    #[serde(borrow)]
    A(Sample<'a>),
    // ...
}

pub struct AbstructStruct;
impl Arg for AbstructStruct {
    type V<'a> = Response<'a>;
}

// Fn's
async fn f<A, C>(mut callback: C) -> Result<()>
where
    A: Arg,
    C: for<'any> AsyncFnMut<(A::V<'any>,), Output = ()>,
{
    let sample = Sample { a: "hello", b: 10 };

    // Deserialize
    let serialized = serde_json::to_string(&sample).unwrap();
    match serde_json::from_str(&serialized) {
        Ok(res) => callback.call_mut((res,)).await,
        Err(_e) => (),
    }
    Ok(())
}

pub async fn run<C>(callback: C) -> Result<()>
where
    C: for<'any> AsyncFnMut<(<AbstructStruct as Arg>::V<'any>,), Output = ()>,
{
    f::<AbstructStruct, _>(callback).await
}

// Argument trait
pub trait Arg {
    type V<'a>: Deserialize<'a>;
}

pub async fn main() -> Result<()> {
    let _ = run(cb!({}; async |res: Response<'_>| -> () {
        println!("{:?}", res);
    }))
    .await;

    Ok(())
}
