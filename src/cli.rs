//main cli logics
use crate::{api, commands};

pub async fn cli_logic() {
    let matches = commands::build_cli().get_matches();
    if let Some(sub_matches) = matches.subcommand_matches("add"){

        let add_service = *sub_matches.get_one::<str>("service")
            .expect("add service");

        let port = sub_matches.get_flag("port");

        let version = sub_matches.get_flag("version");

        println!("Finding service");

        match api::add_service(&add_service).await {
            Ok(recipe) => {
                let ports = if port {

                } else {
                    todo!()
                };

                let version = if version {

                } else{
                    todo!()
                };
            }
            Err(e) => eprintln!("Error {e}")
        }
    }

    if let Some(sub_matches) = matches.subcommand_matches("list"){
        let list_service = *sub_matches.get_one::<str>("list")
            .expect("list_service");

        let type_of = sub_matches.get_flag("type");

        let page = sub_matches.get_flag("page");

        println!("list of services");
    }

    if let Some(sub_matches) = matches.subcommand_matches("get"){
        let get_service = *sub_matches.get_one::<str>("get")
            .expect("get_service");

        let service_url = sub_matches.get_one::<str>("service_url");
        match api::add_service(&add_service).await {
            Ok(recipe) => {
                let service_url = if service_url {

                } else {
                    todo!()
                };
            }
            Err(e) => eprintln!("Error {e}")
        }
    }
}