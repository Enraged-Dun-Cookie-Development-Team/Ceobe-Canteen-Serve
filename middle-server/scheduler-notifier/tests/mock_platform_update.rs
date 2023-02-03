use general_request_client::axum_starter::request_client_prepare;
use http::StatusCode;
use scheduler_notifier::{
    notifies::NotifyPlatformUpdate, NotifyPath, PathOverwriteRequester,
    SchedulerNotifier,
};

pub struct MockPath;

impl NotifyPath for MockPath {
    // the url base on the nock service
    const PATH: &'static str = "/m1/2130514-0-default/update-config";
}

#[tokio::test]
async fn test() {
    let client = request_client_prepare().expect("init client failure").0;

    let notifier = SchedulerNotifier::new(
        "http://127.0.0.1:4523/".parse().unwrap(),
        client,
    );

    let a = notifier.send_notify::<PathOverwriteRequester<NotifyPlatformUpdate,MockPath>>("bilil").await.expect("request error");
    assert_eq!(a.status(), StatusCode::OK)
}
