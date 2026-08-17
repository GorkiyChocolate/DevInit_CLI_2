use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("devinit")
        .version("0.0.1")
        .author("Gorkiy")
        .about("Devinit CLI tool")
        .arg_required_else_help(true)
        .subcommand(build_add_cli())
        .subcommand(build_list_cli())
        .subcommand(build_get_cli())
}

fn build_add_cli() -> Command {
    Command::new("add")
        .about("Adding dependency")
        .arg(
            Arg::new("service")
                .help("Importing service command")
                .required(true)
                .value_parser(clap::value_parser!(String))
                .index(1),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Default Port"),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("Specific version of service"),
        )
}

fn build_list_cli() -> Command {
    Command::new("list")
        .about("List of services")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .help("Type of services"),
        )
        .arg(
            Arg::new("page")
                .short('p')
                .long("page")
                .help("Page of lists"),
        )
}

fn build_get_cli() -> Command {
    Command::new("get")
        .about("Importing services repo like space")
        .arg(
            Arg::new("service_url")
                .help("Get service from repo")
                .required(true)
                .index(1),
        )
}