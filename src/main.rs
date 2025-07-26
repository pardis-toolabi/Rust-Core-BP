use rust_core_bp::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
