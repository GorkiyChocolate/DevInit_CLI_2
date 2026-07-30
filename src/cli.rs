use clap::{Arg, Command};

//cli builder funciton
pub fn build_cli() -> Command {
    Command::new("devinit")
        .version("0.0.1")
        .author("Gorkiy")
        .about("Devinit")
        .arg_required_else_help(true)
        .subcommand(build_add_cli())
}

//add comand logic
fn build_add_cli() -> Command {
    let help_text = String::from("helps");
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

