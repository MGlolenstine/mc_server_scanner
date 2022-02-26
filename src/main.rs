#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dbg!(server_scanner::scan_ports().await);
}
