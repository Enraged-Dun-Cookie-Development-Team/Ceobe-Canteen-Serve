fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(&["D:/rust-project/ceobo-cateen/Ceobe-Canteen-Serve/assets/proto/protos/log.proto"], &["pb"])
        .expect("building proto file failure")
        ;
}
