fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .emit_rerun_if_changed(true)
        .message_attribute("LogRequest", "#[derive(typed_builder::TypedBuilder)]")
        .field_attribute("LogRequest.server", "#[builder(default = ServeType::RUST)]")
        .compile(&["assets/proto/protos/log.proto"], &["assets/proto/protos/"])
        .expect("building proto file failure")
        ;
}
