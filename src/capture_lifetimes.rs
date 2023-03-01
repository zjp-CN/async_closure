pub trait AsyncFnOnce<'a, In> {
    type Output;
    async fn call_once(self, args: In) -> Self::Output;
}

pub trait AsyncFnMut<'a, In>: AsyncFnOnce<'a, In> {
    async fn call_mut(&mut self, args: In) -> Self::Output;
}

pub trait AsyncFn<'a, In>: AsyncFnMut<'a, In> {
    async fn call(&self, args: In) -> Self::Output;
}

// #[doc = concat!("```\n", include_str!("../tests/new_fn_once.rs"), "```\n")]
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
