use clap::{Arg, Command};

//cli builder funciton
pub fn build_cli() -> Command {
    Command::new("devinit")
        .version("0.0.1")
        .author("Gorkiy")
        .about("Devinit")
        .arg_required_else_help(true)
        .subcommand(build_add_cli())
        .subcommand(build_list_cli())
        .subcommand(build_get_cli())
}

//add comand logic
fn build_add_cli() -> Command {
    let help_text = String::from("Importing service command");
    Command::new("add")
        .about("Adding dependency")
        .arg(
            Arg::new("service")
                .help(help_text)
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("default Port")
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("specific version of service")
        )
}

//list command logic
fn build_list_cli() -> Command {
    Command::new("list")
        .about("list of services")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .help("type of services")
        )
        .arg(
            Arg::new("page")
                .short('p')
                .long("page")
                .help("page of lists")
        )
}

//get command logic
fn build_get_cli() -> Command {
    Command::new("get")
        .about("importing services repo like space")
        .arg(
            Arg::new("service_url")
                .help("get service from repo")
                .required(true)
                .index(1)
        )
}