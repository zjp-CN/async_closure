//! Traits that can be written in trait bounds on the callers of
//! async closures **without temporary lifetimes**.

/// The type implemented with this trait can be only used once
/// with its states obtained **by value**.
#[doc = include_str!("./doc/trait/no_lifetimes_fnonce.md")]
pub trait AsyncFnOnce<Args> {
    type Output;
    async fn call_once(self, args: Args) -> Self::Output;
}

/// The type implemented with this trait can be used multiple
/// times with its states obtained by **exclusive** references.
#[doc = include_str!("./doc/trait/no_lifetimes_fnmut.md")]
pub trait AsyncFnMut<Args>: AsyncFnOnce<Args> {
    async fn call_mut(&mut self, args: Args) -> Self::Output;
}

/// The type implemented with this trait can be used multiple
/// times with its states obtained by **shared** references.
#[doc = include_str!("./doc/trait/no_lifetimes_fn.md")]
pub trait AsyncFn<Args>: AsyncFnMut<Args> {
    async fn call(&self, args: Args) -> Self::Output;
}

/// Generate a value that is of unnamed closure type implemented with
/// [`AsyncFnOnce`].
#[doc = include_str!("./doc/macro_syntax.md")]
#[doc = include_str!("./doc/macro/lifetimes_details.md")]
#[doc = include_str!("./doc/macro/no_lifetimes_examples_once.md")]
#[macro_export]
macro_rules! async_owned_closure_once {
    (
        { $( $fields:ident : $t:ty  = $init:expr ),* $(,)? };
        async | $( $args:ident  : $a:ty ),* $(,)? | -> $out:ty
        $e:block
    ) => {{
        struct __AsyncClosure {
            $( pub $fields: $t , )*
        }
        impl $crate::capture_no_lifetimes::AsyncFnOnce<( $($a,)* )> for __AsyncClosure {
            type Output = $out;
            async fn call_once(self, __args: ( $($a,)* )) -> $out {
                let Self { $( $fields , )* .. } = self;
                #[allow(unused_parens)]
                let ( $( $args ,)* ) = __args;
                $e
            }
        }
        #[allow(clippy::redundant_field_names)]
        __AsyncClosure { $($fields: $init , )* }
    }};
}

/// Generate a value that is of unnamed closure type implemented with
/// [`AsyncFnMut`].
#[doc = include_str!("./doc/macro_syntax.md")]
#[doc = include_str!("./doc/macro/lifetimes_details.md")]
#[doc = include_str!("./doc/macro/no_lifetimes_examples_mut.md")]
#[macro_export]
macro_rules! async_owned_closure_mut {
    (
        { $( $fields:ident : $t:ty  = $init:expr ),* $(,)? };
        async | $( $args:ident  : $a:ty ),* $(,)? | -> $out:ty
        $e:block
    ) => {{
        struct __AsyncClosure {
            $( pub $fields: $t , )*
        }
        impl $crate::capture_no_lifetimes::AsyncFnOnce<( $($a,)* )> for __AsyncClosure {
            type Output = $out;
            async fn call_once(mut self, __args: ( $($a,)* )) -> $out {
                <Self as $crate::capture_no_lifetimes::AsyncFnMut<( $($a,)* )>>::call_mut(&mut self, __args).await
            }
        }
        impl $crate::capture_no_lifetimes::AsyncFnMut<( $($a,)* )> for __AsyncClosure {
            async fn call_mut(&mut self, __args: ( $($a,)* )) -> $out {
                let Self { $( $fields , )* .. } = self;
                #[allow(unused_parens)]
                let ( $( $args ,)* ) = __args;
                $e
            }
        }
        #[allow(clippy::redundant_field_names)]
        __AsyncClosure { $($fields: $init , )* }
    }};
}

/// Generate a value that is of unnamed closure type implemented with
/// [`AsyncFn`].
#[doc = include_str!("./doc/macro_syntax.md")]
#[doc = include_str!("./doc/macro/lifetimes_details.md")]
#[doc = include_str!("./doc/macro/no_lifetimes_examples_fn.md")]
#[macro_export]
macro_rules! async_owned_closure {
    (
        { $( $fields:ident : $t:ty  = $init:expr ),* $(,)? };
        async | $( $args:ident  : $a:ty ),* $(,)? | -> $out:ty
        $e:block
    ) => {{
        struct __AsyncClosure {
            $( pub $fields: $t , )*
        }
        impl $crate::capture_no_lifetimes::AsyncFnOnce<( $($a,)* )> for __AsyncClosure {
            type Output = $out;
            async fn call_once(self, __args: ( $($a,)* )) -> $out {
                <Self as $crate::capture_no_lifetimes::AsyncFn<( $($a,)* )>>::call(&self, __args).await
            }
        }
        impl $crate::capture_no_lifetimes::AsyncFnMut<( $($a,)* )> for __AsyncClosure {
            async fn call_mut(&mut self, __args: ( $($a,)* )) -> $out {
                <Self as $crate::capture_no_lifetimes::AsyncFn<( $($a,)* )>>::call(&*self, __args).await
            }
        }
        impl $crate::capture_no_lifetimes::AsyncFn<( $($a,)* )> for __AsyncClosure {
            async fn call(&self, __args: ( $($a,)* )) -> $out {
                let Self { $( $fields , )* .. } = self;
                #[allow(unused_parens)]
                let ( $( $args ,)* ) = __args;
                $e
            }
        }
        #[allow(clippy::redundant_field_names)]
        __AsyncClosure { $($fields: $init , )* }
    }};
}
