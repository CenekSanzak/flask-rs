use clap::{Parser, Subcommand};
use flask_rs::unsign::{Unsign, unsign_cookie};
use flask_rs::sign::{Sign, sign_cookie};
use flask_rs::decode::{Decode, decode_and_print_cookie};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Unsign(Unsign),
    Sign(Sign),
    Decode(Decode)
}

fn main() {
    let args = Cli::parse();
 
    match args.command {
        Commands::Unsign(unsign) => {
            unsign_cookie(unsign);
        },
        Commands::Sign(sign) => {
            sign_cookie(sign);
        },
        Commands::Decode(decode) => {
            decode_and_print_cookie(decode);
        }
    }
}
