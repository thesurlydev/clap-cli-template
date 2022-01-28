extern crate clap;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use clap::{AppSettings, Parser, Subcommand};

fn main() {
    let cli: Cli = Cli::parse();
    match &cli.command {
        Command::Add { name, r#type } => {
            println!("type: {}, name: {}", r#type, name);
        }
        Command::Info { name } => todo!(),
        Command::List { name } => todo!(),
    }

    // TODO do stuff
}

#[derive(Subcommand)]
enum Command {
    #[clap(alias = "a", about = "Add a component", setting(AppSettings::ArgRequiredElseHelp))]
    Add {
        #[clap()]
        /// The type of component. Valid values are: alias, func, secret, or tool
        r#type: String,

        /// The name of the component
        name: String
    },
    #[clap(alias = "i", about = "Get information about a component", setting(AppSettings::ArgRequiredElseHelp))]
    Info {

        name: String
    },
    #[clap(alias = "l", about = "List all components", setting(AppSettings::ArgRequiredElseHelp))]
    List { name: String },
}

#[derive(Parser)]
#[clap(version, author, about, setting = AppSettings::ArgRequiredElseHelp)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}
