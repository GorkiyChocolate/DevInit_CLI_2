use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
mod cli;
pub mod structs;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: u32,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u32,
}


async fn fetch_post(id:u32) -> Result<Post, reqwest::Error> {
    let url = format!("https://jsonplaceholder.typicode.com/posts/{id}");
    let response = reqwest::get(&url).await?.error_for_status()?;
    let post = response.json::<Post>().await?;
    Ok(post)
}

fn save_to_file(path: &PathBuf, content: &str) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main(){
    let matches = cli::build_cli().get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("get") {
        let post_id = *sub_matches
            .get_one::<u32>("post_id")
            .expect("validation");
        let output_path = sub_matches.get_one::<PathBuf>("output");
        let is_pretty = sub_matches.get_flag("pretty");

        println!("Fetching post {post_id}");

        match fetch_post(post_id).await {
            Ok(post) => {
                let json_data = if is_pretty {
                    serde_json::to_string_pretty(&post).unwrap()
                } else {
                    serde_json::to_string(&post).unwrap()
                };

                if let Some(path) = output_path {
                    match save_to_file(path, &json_data) {
                        Ok(_) => println!("Success save in: {}", path.display()),
                        Err(e) => eprintln!("Fail: {e}"),
                    }
                } else {
                    println!("\n -------");
                    println!("{json_data}");
                }
            }
            Err(e) => eprintln!("Fail connection {e}"),
        }
    }
}