pub trait LiteArgs: Clone + Sized {
    fn get_arg() -> Self;
}

impl LiteArgs for () {
    fn get_arg() -> Self {}
}

macro_rules! lite_args {
    ($($g:ident),+) => {
        impl<$($g),+> LiteArgs for ( $($g,)+)
        where
        $(
            $g : crate::lite_args::LiteArgs,
        )+
        {
            fn get_arg()->Self {
                (
                    $(
                        <$g as crate::lite_args::LiteArgs>::get_arg(),
                    )
                +)
            }
        }
    };
}

lite_args!(A);
lite_args!(A, B);
lite_args!(A, B, C);
lite_args!(A, B, C, D);
lite_args!(A, B, C, D, E);
lite_args!(A, B, C, D, E, F);
lite_args!(A, B, C, D, E, F, G);
lite_args!(A, B, C, D, E, F, G, I);
lite_args!(A, B, C, D, E, F, G, I, J);
lite_args!(A, B, C, D, E, F, G, I, J, K);
lite_args!(A, B, C, D, E, F, G, I, J, K, L);
lite_args!(A, B, C, D, E, F, G, I, J, K, L, M);
lite_args!(A, B, C, D, E, F, G, I, J, K, L, M, N);

#[cfg(test)]
mod test {
    use std::convert::Infallible;

    use futures::future::{ok, Ready};

    use crate::{CheckRequire, RefChecker};

    struct ComplexLiteArgNoChecker;

    impl RefChecker for ComplexLiteArgNoChecker {
        type Args = ((((((((),),),), (((),),)),), (((),),)),);
        type Err = Infallible;
        type Fut = Ready<Result<(), Infallible>>;
        type Target = i32;

        fn ref_checker(_: Self::Args, _: &Self::Target) -> Self::Fut {
            ok(())
        }
    }

    #[tokio::test]
    async fn test_complex_arg() {
        let init = CheckRequire::new(ComplexLiteArgNoChecker, 11212i32);

        let resp = init.lite_checking().await;

        assert_eq!(Ok(11212i32), resp);
    }
}
