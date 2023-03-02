#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

macro_rules! with_lifetime {
    ($fn:ident, $trait:ident, $mac:ident, $b:block) => {
        pub async fn $fn() {
            use async_closure::$mac as cb;
            async fn check<'env, F, Args, Out>(_: F)
            where
                F: imp::$trait<'env, Args, Output = Out>,
            {
            }
            $b;
        }
    };
}

macro_rules! check {
    ($check:ident, $fn_prefix:ident, $b:block) => {
        check! { @once $check, $fn_prefix, $b }
        check! { @mut $check, $fn_prefix, $b }
        check! { @fn $check, $fn_prefix, $b }
    };
    (@once $check:ident, $fn_prefix:ident, $b:block) => {
        ::paste::paste! { $check! { [< $fn_prefix _once >], AsyncFnOnce, async_closure_once, $b } }
    };
    (@mut $check:ident, $fn_prefix:ident, $b:block) => {
        ::paste::paste! { $check! { [< $fn_prefix _mut >], AsyncFnMut, async_closure_mut, $b } }
    };
    (@fn $check:ident, $fn_prefix:ident, $b:block) => {
        ::paste::paste! { $check! { [< $fn_prefix _fn >], AsyncFn, async_closure, $b } }
    };
}

mod referenced {
    use async_closure::capture_lifetimes as imp;

    check!(with_lifetime, simple, {
        // Empty fields and args are supported, though it's meaningless
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
    });

    check!(@once with_lifetime, mutate_state, {
        let mut v = vec![];

        check::<_, (), usize>(cb!({
            v: &'a mut Vec<u8> = &mut v,
        }; async | | -> usize {
            // to show the type; you don't have to do this
            let v: &mut Vec<u8> = v;
            v.push(0);
            v.len()
        }))
        .await;

        check::<_, (), usize>(cb!({
            v: Vec<u8> = v,
        }; async | | -> usize {
            // rebinding is a must to mutate it
            let mut v: Vec<u8> = v;
            v.push(0);
            v.len()
        }))
        .await;
    });

    check!(@mut with_lifetime, mutate_state, {
        let mut v = vec![];

        check::<_, (), usize>(cb!({
            v: &'a mut Vec<u8> = &mut v,
        }; async | | -> usize {
            let v: &mut &mut Vec<u8> = v;
            v.push(0);
            v.len()
        }))
        .await;

        check::<_, (), usize>(cb!({
            v: Vec<u8> = v,
        }; async | | -> usize {
            let v: &mut Vec<u8> = v;
            v.push(0);
            v.len()
        }))
        .await;
    });

    check!(@fn with_lifetime, no_mutation, {
        check::<_, (), usize>(cb!({
            v: &'a [u8] = &[],
        }; async | | -> usize {
            let v: &&[u8] = v;
            v.len()
        }))
        .await;

        // both types with 'static lifetime
        check::<_, (), usize>(cb!({
            v: &'static [u8] = &[],
        }; async | | -> usize {
            let v: &&[u8] = v;
            v.len()
        }))
        .await;

        // and types without lifetime parameter
        // can be successfully captured
        check::<_, (), usize>(cb!({
            v: Vec<u8> = vec![],
        }; async | | -> usize {
            let v: &Vec<u8> = v;
            v.len()
        }))
        .await;
    });

    pub async fn tests() {
        simple_once().await;
        simple_mut().await;
        simple_fn().await;

        mutate_state_once().await;
        mutate_state_mut().await;
        no_mutation_fn().await;
    }
}

#[tokio::main]
async fn main() {
    referenced::tests().await;
}
