use std::path::PathBuf;
use clap::{arg, value_parser, Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("file-loader")
        .version("0.0.1")
        .about("Ratatata")
        .arg_required_else_help(true)
        .arg(
            Arg::new("INPUT")
                .help("Path way")
                .required(true)
                .index(1)
                .value_parser(value_parser!(PathBuf)),
        )

        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output")
                .value_name("Dir")
                .value_parser(value_parser!(PathBuf)),
        )

        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .long("verbose")
                .help("LO")
                .action(ArgAction::SetTrue),
        )

        .subcommand(
            Command::new("stats")
                .about("Collect")
                .arg(
                    Arg::new("detailed")
                        .short('d')
                        .long("detaield")
                        .action(ArgAction::SetTrue)
                        .help("detailed"),
                ),
        )
        .get_matches();

    if let Some(input_path) = matches.get_one::<PathBuf>("INPUT") {
        println!("file: {}", input_path.display());
    }

    if let Some(output_dir) = matches.get_one::<PathBuf>("output") {
        println!("Output: {}", output_dir.display());
    }

    if matches.get_flag("verbose") {
        println!("[Debug] debug mod");
    }

    if let Some(sub_matches) = matches.subcommand_matches("stats") {
        let is_detailed = sub_matches.get_flag("detailed");
        println!("launching stats")
    }
}