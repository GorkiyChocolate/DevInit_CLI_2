fn main() {
    if devinit_cli_2::cli::cli_logic::cli_logic().is_err() {
        std::process::exit(1);
    }
}
