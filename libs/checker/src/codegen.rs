#[cfg(test)]
mod test {
    use std::convert::Infallible;

    use typed_builder::TypedBuilder;

    use crate as checker;
    use crate::{prefabs::no_check::NoCheck, CheckRequire};

    #[derive(Debug, TypedBuilder, PartialEq, Eq)]
    pub struct TestChecked {
        a: i32,
        b: String,
    }
    #[check_obj_macro::check_obj(
        uncheck = TestUncheck,
        checked = TestChecked,
        error = Infallible,
        sync
    )]
    pub struct TestChecker {
        a: NoCheck<i32>,
        b: NoCheck<String>,
    }

    #[tokio::test]
    async fn test_pre_lite_check() {
        let uncheck = TestUncheck {
            a: CheckRequire::new(NoCheck::new(), 112),
            b: CheckRequire::new(NoCheck::new(), "121212".into()),
        };

        let init = CheckRequire::new(TestChecker, uncheck);

        let resp = init.lite_checking().await.unwrap();

        assert_eq!(
            resp,
            TestChecked {
                a: 112,
                b: "121212".into()
            }
        )
    }

    #[test]
    fn test_pre_lite_check_sync() {
        let uncheck = TestUncheck {
            a: CheckRequire::new(NoCheck::new(), 112),
            b: CheckRequire::new(NoCheck::new(), "121212".into()),
        };

        let init = CheckRequire::new(TestChecker, uncheck);

        let resp = init.sync_lite_check().unwrap();

        assert_eq!(
            resp,
            TestChecked {
                a: 112,
                b: "121212".into()
            }
        )
    }
}
