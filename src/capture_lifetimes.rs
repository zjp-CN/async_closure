//! Traits that can be written in trait bounds on the callers of
//! async closures **carrying temporary lifetimes**.

/// The type implemented with this trait can be only used once
/// with its states obtained **by value**.
#[doc = include_str!("./doc/trait/lifetimes_fnonce.md")]
pub trait AsyncFnOnce<'env, Args> {
    type Output;
    async fn call_once(self, args: Args) -> Self::Output;
}

/// The type implemented with this trait can be used multiple
/// times with its states obtained by **exclusive** references.
#[doc = include_str!("./doc/trait/lifetimes_fnmut.md")]
pub trait AsyncFnMut<'env, Args>: AsyncFnOnce<'env, Args> {
    async fn call_mut(&mut self, args: Args) -> Self::Output;
}

/// The type implemented with this trait can be used multiple
/// times with its states obtained by **shared** references.
#[doc = include_str!("./doc/trait/lifetimes_fn.md")]
pub trait AsyncFn<'env, Args>: AsyncFnMut<'env, Args> {
    async fn call(&self, args: Args) -> Self::Output;
}

/// Generate a value that is of unnamed closure type implemented with
/// [`AsyncFnOnce`].
#[doc = include_str!("./doc/macro_syntax.md")]
#[doc = include_str!("./doc/macro/lifetimes_details.md")]
#[doc = include_str!("./doc/macro/lifetimes_examples_once.md")]
#[macro_export]
macro_rules! async_closure_once {
    (
        { $( $field:ident : $t:ty  = $init:expr ),* $(,)? };
        async | $( $args:ident  : $a:ty ),* $(,)? | -> $out:ty
        $e:block
    ) => {{
        struct __AsyncClosure<'a> {
            $( pub $field: $t , )*
            _ph: ::std::marker::PhantomData<&'a ()>,
        }
        impl<'a> $crate::capture_lifetimes::AsyncFnOnce<'a, ( $($a,)* )> for __AsyncClosure<'a> {
            type Output = $out;
            async fn call_once(self, __args: ( $($a,)* )) -> $out {
                let Self { $( $field , )* .. } = self;
                #[allow(unused_parens)]
                let ( $( $args ,)*  ) = __args;
                $e
            }
        }
        #[allow(clippy::redundant_field_names)]
        __AsyncClosure { $($field: $init , )* _ph: ::std::marker::PhantomData }
    }};
}

/// Generate a value that is of unnamed closure type implemented with
/// [`AsyncFnMut`].
#[doc = include_str!("./doc/macro_syntax.md")]
#[doc = include_str!("./doc/macro/lifetimes_details.md")]
#[doc = include_str!("./doc/macro/lifetimes_examples_mut.md")]
#[macro_export]
macro_rules! async_closure_mut {
    (
        { $( $field:ident : $t:ty  = $init:expr ),* $(,)? };
        async | $( $args:ident  : $a:ty ),* $(,)? | -> $out:ty
        $e:block
    ) => {{
        struct __AsyncClosure<'a> {
            $( pub $field: $t , )*
            _ph: ::std::marker::PhantomData<&'a ()>,
        }
        impl<'a> $crate::capture_lifetimes::AsyncFnOnce<'a, ( $($a,)* )> for __AsyncClosure<'a> {
            type Output = $out;
            async fn call_once(mut self, __args: ( $($a,)* )) -> $out {
                <Self as $crate::capture_lifetimes::AsyncFnMut<'a, ( $($a,)* )>>::call_mut(&mut self, __args).await
            }
        }
        impl<'a> $crate::capture_lifetimes::AsyncFnMut<'a, ( $($a,)* )> for __AsyncClosure<'a> {
            async fn call_mut(&mut self, __args: ( $($a,)* )) -> $out {
                let Self { $( $field , )* .. } = self;
                #[allow(unused_parens)]
                let ( $( $args ,)*  ) = __args;
                $e
            }
        }
        #[allow(clippy::redundant_field_names)]
        __AsyncClosure { $($field: $init , )* _ph: ::std::marker::PhantomData }
    }};
}

/// Generate a value that is of unnamed closure type implemented with
/// [`AsyncFn`].
#[doc = include_str!("./doc/macro_syntax.md")]
#[doc = include_str!("./doc/macro/lifetimes_details.md")]
#[doc = include_str!("./doc/macro/lifetimes_examples_fn.md")]
#[macro_export]
macro_rules! async_closure {
    (
        { $( $field:ident : $t:ty  = $init:expr ),* $(,)? };
        async | $( $args:ident  : $a:ty ),* $(,)? | -> $out:ty
        $e:block
    ) => {{
        struct __AsyncClosure<'a> {
            $( pub $field: $t , )*
            _ph: ::std::marker::PhantomData<&'a ()>,
        }
        impl<'a> $crate::capture_lifetimes::AsyncFnOnce<'a, ( $($a,)* )> for __AsyncClosure<'a> {
            type Output = $out;
            async fn call_once(self, __args: ( $($a,)* )) -> $out {
                <Self as $crate::capture_lifetimes::AsyncFn<'a, ( $($a,)* )>>::call(&self, __args).await
            }
        }
        impl<'a> $crate::capture_lifetimes::AsyncFnMut<'a, ( $($a,)* )> for __AsyncClosure<'a> {
            async fn call_mut(&mut self, __args: ( $($a,)* )) -> $out {
                <Self as $crate::capture_lifetimes::AsyncFn<'a, ( $($a,)* )>>::call(&*self, __args).await
            }
        }
        impl<'a> $crate::capture_lifetimes::AsyncFn<'a, ( $($a,)* )> for __AsyncClosure<'a> {
            async fn call(&self, __args: ( $($a,)* )) -> $out {
                let Self { $( $field , )* .. } = self;
                #[allow(unused_parens)]
                let ( $( $args ,)* ) = __args;
                $e
            }
        }
        #[allow(clippy::redundant_field_names)]
        __AsyncClosure { $($field: $init , )* _ph: ::std::marker::PhantomData }
    }};
}
