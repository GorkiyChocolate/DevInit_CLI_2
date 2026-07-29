use crate::structs;
use clap::{value_parser, Arg, Command};

//cli builder funciton
pub fn build_cli() -> Command {
    Command::new("devinit")
        .version("0.0.1")
        .author("Gorkiy")
        .about("Devinit")
        .arg_required_else_help(true)
        .subcommand(build_add_cli(&[]))
}

//add comand logic
fn build_add_cli(service: &[structs::ComposeService]) -> Command {
    let service_names: Vec<String> = service.iter().map(|s| s.name.clone()).collect();
    let help_text = String::from("helps");
    Command::new("add")
        .about("Adding dependency")
        .arg(
            Arg::new("service")
                .help(help_text)
                .required(true)
                .index(1)
                .value_parser(builder_possible_values(service_names)),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("default Port")
                .value_parser(value_parser![u16]),
        )
}

fn builder_possible_values(values: Vec<String>) -> clap::builder::PossibleValuesParser {
    clap::builder::PossibleValuesParser::new(values)
}