use clap::{value_parser, Arg, ArgAction, Command};
use crate::structs::CompsoeService;

pub fn build_cli() -> Command {
    Command::new("devinit")
        .version("0.0.1")
        .author("Gorkiy")
        .about("Devinit")
        .arg_required_else_help(true)
        .subcommand(build_add_cli())
}

fn build_add_cli(service: &[ComposeService]) -> Command {
    let service_names: Vec<String> = services.iter().map(|s| s.name.clone()).collect();

    Command::new("add")
        .about("Adding dependency")
        .arg(
            Arg::new("service")
                .help(help_text)
                .required(1)
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