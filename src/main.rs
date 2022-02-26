#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dbg!(mc_server_scanner::scan_ports().await);
}
