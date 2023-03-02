#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

macro_rules! with_lifetime {
    ($fn:ident, $trait:ident, $mac:ident, $b:block) => {
        pub async fn $fn() {
            use ::async_closure::$mac as cb;
            async fn check<'env, F, Args, Out>(_: F)
            where
                F: imp::$trait<'env, Args, Output = Out>,
            {
            }
            $b;
        }
    };
}

macro_rules! no_lifetime {
    ($fn:ident, $trait:ident, $mac:ident, $b:block) => {
        pub async fn $fn() {
            use ::async_closure::$mac as cb;
            async fn check<F, Args, Out>(f: F) -> F
            where
                F: imp::$trait<Args, Output = Out>,
            {
                f
            }
            $b;
        }
    };
}

mod referenced {
    use async_closure::capture_lifetimes as imp;

    macro_rules! check_with_lifetime {
        ($fn_prefix:ident, $b:block) => {
            check_with_lifetime! { @once $fn_prefix, $b }
            check_with_lifetime! { @mut $fn_prefix, $b }
            check_with_lifetime! { @fn $fn_prefix, $b }
        };
        (@once $fn_prefix:ident, $b:block) => {
            ::paste::paste! { with_lifetime! { [< $fn_prefix _once >], AsyncFnOnce, async_closure_once, $b } }
        };
        (@mut $fn_prefix:ident, $b:block) => {
            ::paste::paste! { with_lifetime! { [< $fn_prefix _mut >], AsyncFnMut, async_closure_mut, $b } }
        };
        (@fn $fn_prefix:ident, $b:block) => {
            ::paste::paste! { with_lifetime! { [< $fn_prefix _fn >], AsyncFn, async_closure, $b } }
        };
    }

    check_with_lifetime!(simple, {
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

    check_with_lifetime!(@once mutate_state, {
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

    check_with_lifetime!(@mut mutate_state, {
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

    check_with_lifetime!(@fn no_mutation, {
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

mod owned {
    use async_closure::capture_no_lifetimes as imp;

    macro_rules! check_no_lifetime {
        ($fn_prefix:ident, $b:block) => {
            check_no_lifetime! { @once $fn_prefix, $b }
            check_no_lifetime! { @mut $fn_prefix, $b }
            check_no_lifetime! { @fn $fn_prefix, $b }
        };
        (@once $fn_prefix:ident, $b:block) => {
            ::paste::paste! { no_lifetime! { [< $fn_prefix _once >], AsyncFnOnce, async_owned_closure_once, $b } }
        };
        (@mut $fn_prefix:ident, $b:block) => {
            ::paste::paste! { no_lifetime! { [< $fn_prefix _mut >], AsyncFnMut, async_owned_closure_mut, $b } }
        };
        (@fn $fn_prefix:ident, $b:block) => {
            ::paste::paste! { no_lifetime! { [< $fn_prefix _fn >], AsyncFn, async_owned_closure, $b } }
        };
    }

    check_no_lifetime!(simple, {
        // Empty fields and args are supported, though it's meaningless
        check::<_, (), ()>(cb!({}; async | | -> () {})).await;

        // non-capturing closure with an arg
        check::<_, (&str,), usize>(cb!({}; async |s: &str| -> usize { s.len() })).await;

        // capture one variable with zero args
        let f = check::<_, (), ()>(cb!({
            v: Vec<u8> = vec![],
        }; async | | -> () { let _ = &v; }))
        .await;
        // fields of the returned the closure type are public,
        // so we can use them as normal variables here
        let mut v: Vec<u8> = f.v;
        assert_eq!(v, &[]);
        v.push(0);
        assert_eq!(v, &[0]);
    });

    check_no_lifetime!(@once mutate_state, {
        let v = vec![1, 2];
        let f = check::<_, (&[u8],), ()>(cb!({
            v: Vec<u8> = v
        }; async |slice: &[u8]| -> () {
            let mut v = v; // rebinding to mutate
            v.pop();
            v.extend_from_slice(slice);
        }))
        .await;
        assert_eq!(f.v.len(), 2);
    });

    check_no_lifetime!(@mut mutate_state, {
        let v = vec![1, 2];
        let f = check::<_, (&[u8],), ()>(cb!({
            v: Vec<u8> = v
        }; async |slice: &[u8]| -> () {
            let v: &mut Vec<u8> = v; // don't have to do this
            v.pop();
            v.extend_from_slice(slice);
        }))
        .await;
        assert_eq!(f.v.len(), 2);
    });

    check_no_lifetime!(@fn no_mutation, {
        let v = vec![1, 2];
        let f = check::<_, (&[u8],), usize>(cb!({
            v: Vec<u8> = v
        }; async |slice: &[u8]| -> usize {
            let v: &Vec<u8> = v; // don't have to do this
            v.len() + slice.len()
        }))
        .await;
        assert_eq!(f.v.len(), 2);
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
    owned::tests().await;
}
