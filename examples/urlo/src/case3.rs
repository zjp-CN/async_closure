use async_closure::{async_closure_once as cb, capture_lifetimes::AsyncFnOnce};

pub async fn main() -> Result<()> {
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
