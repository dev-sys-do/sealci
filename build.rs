fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["api/proto/scheduler/agent.proto","api/proto/scheduler/controller.proto"], &["api/proto/scheduler"])
        .unwrap();
}
