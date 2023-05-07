fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .emit_rerun_if_changed(true)
        .message_attribute(
            "LogRequest",
            "#[derive(typed_builder::TypedBuilder)]",
        )
        .field_attribute(
            "LogRequest.server",
            "#[builder(default = (log_request::ServeType::Rust as i32))]",
        )
        .field_attribute(
            "LogRequest.server",
            "#[builder(setter(transform = |ty:log_request::ServeType|ty as \
             i32))]",
        )
        .field_attribute(
            "LogRequest.level",
            "#[builder(setter(transform = |ty:log_request::LogType|ty as \
             i32))]",
        )
        .field_attribute(
            "LogRequest.manual",
            "#[builder(setter(transform = ||true))]",
        )
        .field_attribute("LogRequest.manual", "#[builder(default = false)]")
        .field_attribute("LogRequest.extra", "#[builder(default)]")
        .compile(
            &["../../assets/proto/protos/log.proto"],
            &["../../assets/proto/protos/"],
        )
        .expect("building proto file failure");
}
