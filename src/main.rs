mod cli;
mod api;
mod structs;

#[tokio::main]
async fn main(){
    let matches = cli::build_cli().get_matches();
    if let Some(sub_matches) = matches.subcommand_matches("add") {
        let add_service = *sub_matches
            .get_one::<String>("service")
            .expect("add service");
        let port = sub_matches
            .get_flag("port");

        let version = sub_matches
            .get_flag("version");

        println!("finding service");

        match api::get_service("/localhost:3000", &add_service).await {
            Ok(recipe) => {
                let ports = if port {

                } else {
                    todo!();
                };

                let versions = if version {

                } else{
                    todo!();
                };
            }
            Err(e) => eprintln!("Error {e}")
        }
    }
}