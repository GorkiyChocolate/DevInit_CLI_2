#[tokio::main]
async fn main() {
    if devinit_cli_2::cli_logic().await.is_err() {
        std::process::exit(1);
    }
}
