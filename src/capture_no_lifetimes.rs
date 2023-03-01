pub trait AsyncFnOnce<In> {
    type Output;
    async fn call_once(self, args: In) -> Self::Output;
}

pub trait AsyncFnMut<In>: AsyncFnOnce<In> {
    async fn call_mut(&mut self, args: In) -> Self::Output;
}

pub trait AsyncFn<In>: AsyncFnMut<In> {
    async fn call(&self, args: In) -> Self::Output;
}

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
