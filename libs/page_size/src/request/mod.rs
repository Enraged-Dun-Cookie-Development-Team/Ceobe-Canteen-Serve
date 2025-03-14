use checker::{
    check_obj,
    prefabs::num_check::{
        NonZeroUnsigned, NonZeroUnsignedChecker, NonZeroUnsignedError,
    },
    serde_as, DisplayFromStr,
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Copy, Clone, Debug, TypedBuilder, Serialize)]
pub struct Paginator {
    pub page: NonZeroUnsigned<usize>,
    pub size: NonZeroUnsigned<usize>,
}

#[check_obj(
    uncheck = PageSizeUncheck,
    checked = Paginator,
    error = NonZeroUnsignedError,
    sync
)]
#[serde_as(crate = "::checker::serde_with")]
#[derive(Debug, Deserialize)]
pub struct PageSizeChecker {
    #[serde_as(as = "DisplayFromStr")]
    page: NonZeroUnsignedChecker<usize>,
    #[serde_as(as = "DisplayFromStr")]
    size: NonZeroUnsignedChecker<usize>,
}

#[cfg(test)]
mod test {
    use std::convert::Infallible;

    use checker::{
        check_gen,
        prefabs::{no_check::NoCheck, num_check::NonZeroUnsignedError},
        LiteChecker,
    };
    use serde::Deserialize;
    use typed_builder::TypedBuilder;

    use super::{PageSizeChecker, Paginator};

    #[derive(Debug, TypedBuilder)]
    #[allow(dead_code)]
    struct TestChecked {
        a: i32,
        b: String,
        test_page_size: Paginator,
    }

    #[derive(Debug)]
    pub struct TestError(NonZeroUnsignedError);
    impl From<Infallible> for TestError {
        fn from(_: Infallible) -> Self { unreachable!() }
    }
    impl From<NonZeroUnsignedError> for TestError {
        fn from(err: NonZeroUnsignedError) -> Self { Self(err) }
    }

    #[check_gen(
        uncheck = TestUncheck,
        checked = self::TestChecked,
        error = TestError
    )]
    #[derive(Deserialize)]
    struct TestChecker {
        a: NoCheck<i32>,
        b: NoCheck<String>,
        #[serde(flatten)]
        test_page_size: PageSizeChecker,
    }

    #[tokio::test]
    async fn test_page_size() {
        let a = serde_json::json!({
            "a": 10,
            "b": "嘿嘿嘿",
            "page": 1,
            "size": 10
        });
        let v = serde_json::from_value::<TestUncheck>(a).unwrap();
        let result = TestChecker::lite_check(v).await.unwrap();
        println!("{:?}", result);
    }
}
