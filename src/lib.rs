#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(type_alias_impl_trait)]

#[doc = concat!("```\n", include_str!("../tests/async_callback.rs"), "```\n")]
#[macro_export]
macro_rules! async_closure {
    (
        { $( $field:ident : $t:ty  = $init:expr ),+ };
        ( $( $args:ident ),+ , );
        $e:expr
    ) => {{
        struct __AsyncClosure {
            $( $field: $t ),+
        }
        impl<'a> ::core::ops::FnOnce<(&'a str,)> for __AsyncClosure {
            type Output = impl 'a + core::future::Future<Output = usize>;
            extern "rust-call" fn call_once(self, args: (&'a str,)) -> Self::Output {
                let Self { $( $field ),+ } = self;
                #[allow(unused_parens)]
                let ( $( $args ),+ , ) = args;
                async move { $e }
            }
        }
        #[allow(clippy::redundant_field_names)]
        __AsyncClosure { $($field: $init),+ }
    }};
}

pub mod capture_lifetime {
    pub trait AsyncFnOnce<'a, In, Out> {
        async fn call_once(self, message: In) -> Out;
    }

    #[doc = concat!("```\n", include_str!("../tests/new_fn_once.rs"), "```\n")]
    #[macro_export]
    macro_rules! async_closure_once {
    (
        { $( $field:ident : $t:ty  = $init:expr ),+ $(,)? };
        async | $( $args:ident  : $a:ty ),+ $(,)? | -> $out:ty
        $e:block
    ) => {{
        struct __AsyncClosure<'a> {
            $( $field: $t , )+
            _ph: std::marker::PhantomData<&'a ()>,
        }
        impl<'a> $crate::capture_lifetime::AsyncFnOnce<'a, ( $($a,)+ ), $out> for __AsyncClosure<'a> {
            async fn call_once(self, args: ( $($a,)+ )) -> $out {
                let Self { $( $field , )+ .. } = self;
                #[allow(unused_parens)]
                let ( $( $args ),+ , ) = args;
                $e
            }
        }
        #[allow(clippy::redundant_field_names)]
        __AsyncClosure { $($field: $init , )+ _ph: std::marker::PhantomData }
    }};
    }
}

pub mod capture_no_lifetime {
    pub trait AsyncFnOnce<'a, In, Out> {
        async fn call_once(self, message: In) -> Out;
    }

    #[doc = concat!("```\n", include_str!("../tests/new_fn_once.rs"), "```\n")]
    #[macro_export]
    macro_rules! async_owned_closure_once {
    (
        { $( $field:ident : $t:ty  = $init:expr ),+ $(,)? };
        async | $( $args:ident  : $a:ty ),+ $(,)? | -> $out:ty
        $e:block
    ) => {{
        struct __AsyncClosure<'a> {
            $( $field: $t , )+
            _ph: std::marker::PhantomData<&'a ()>,
        }
        impl<'a> $crate::capture_lifetime::AsyncFnOnce<'a, ( $($a,)+ ), $out> for __AsyncClosure<'a> {
            async fn call_once(self, args: ( $($a,)+ )) -> $out {
                let Self { $( $field , )+ .. } = self;
                #[allow(unused_parens)]
                let ( $( $args ),+ , ) = args;
                $e
            }
        }
        #[allow(clippy::redundant_field_names)]
        __AsyncClosure { $($field: $init , )+ _ph: std::marker::PhantomData }
    }};
    }
}
