use checker::{
    check_obj,
    prefabs::num_check::{NonZeroUnsignedChecker, NonZeroUnsignedError},
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Copy, Clone, Debug, TypedBuilder, Serialize)]
pub struct PageSize {
    pub page: usize,
    pub size: usize,
}

#[check_obj(
    uncheck = PageSizeUncheck,
    checked = PageSize,
    error = NonZeroUnsignedError
)]
#[derive(Debug, Deserialize)]
pub struct PageSizeChecker {
    page: NonZeroUnsignedChecker<usize>,
    size: NonZeroUnsignedChecker<usize>,
}

#[cfg(test)]
mod test {
    use std::convert::Infallible;

    use checker::{
        check_gen,
        prefabs::{no_check::NoCheck, num_check::NonZeroUnsignedError},
        CheckRequire, LiteChecker,
    };
    use serde::Deserialize;
    use typed_builder::TypedBuilder;

    use super::{PageSize, PageSizeChecker};

    #[derive(Debug, TypedBuilder)]
    pub struct TestChecked {
        a: i32,
        b: String,
        test_page_size: PageSize,
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
    pub struct TestChecker {
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
