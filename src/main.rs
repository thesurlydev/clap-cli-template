extern crate clap;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use clap::{Parser, Subcommand};
use reqwest::Response;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cli: Cli = Cli::parse();
    match &cli.command {
        Command::Add { name, r#type } => {
            println!("type: {}, name: {}", r#type, name);
        }
        Command::Info { .. } => todo!(),
        Command::List { .. } => todo!(),
        Command::Get { url } => {
            let response = do_get(url).await?;
            let status = response.status();
            println!("Status: {}", status);

            eprintln!("Headers: {:#?}\n", response.headers());

            let body = response.text().await?;
            println!("body={:?}", body);
        }
    }
    Ok(())
}

async fn do_get(url: &String) -> reqwest::Result<Response> {
    reqwest::get(url).await
}

#[derive(Subcommand)]
enum Command {
    #[clap(alias = "a", about = "Add a component")]
    Add {
        #[clap()]
        /// The type of component. Valid values are: alias, func, secret, or tool
        r#type: String,

        /// The name of the component
        name: String,
    },

    #[clap(alias = "g", about = "Get a URL")]
    Get {
        url: String
    },

    #[clap(alias = "i", about = "Get information about a component")]
    Info { name: String },
    #[clap(alias = "l", about = "List all components")]
    List { name: String },
}

#[derive(Parser)]
#[clap(version, author, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}
