use checker::{
    check_gen, prefabs::no_check::NoCheck, CheckRequire, LiteChecker,
};
use typed_builder::TypedBuilder;

#[tokio::main]
async fn main() {
    let uncheck = TestUncheck {
        a: CheckRequire::new(NoCheck::default(), 11),
        b: CheckRequire::new(NoCheck::default(), "Str".into()),
    };
    let resp = TestChecker::lite_check(uncheck).await.unwrap();

    println!("checked {:?}", resp);
}

#[derive(Debug, TypedBuilder, PartialEq, Eq)]
pub struct TestChecked {
    a: i32,
    b: String,
    #[builder(default = 11)]
    default_bar :i32,
    #[builder(default)]
    default_foo :Option<String>
}

#[check_gen(
    uncheck = TestUncheck,
    checked = self::TestChecked,
    error = std::convert::Infallible
)]
pub struct TestChecker {
    a: NoCheck<i32>,
    b: NoCheck<String>,
}
