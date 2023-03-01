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
        $( pub $field: $t , )+
        _ph: ::std::marker::PhantomData<&'a ()>,
    }
    impl<'a> $crate::capture_lifetime::AsyncFnOnce<'a, ( $($a,)+ ), $out> for __AsyncClosure<'a> {
        async fn call_once(self, __args: ( $($a,)+ )) -> $out {
            let Self { $( $field , )+ .. } = self;
            #[allow(unused_parens)]
            let ( $( $args ),+ , ) = __args;
            $e
        }
    }
    #[allow(clippy::redundant_field_names)]
    __AsyncClosure { $($field: $init , )+ _ph: ::std::marker::PhantomData }
}};
}
