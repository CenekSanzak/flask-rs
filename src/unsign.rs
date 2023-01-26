use clap::Args;

#[derive(Args)]
pub struct Unsign{
    #[arg(short, long)]
    cookie: String,
    #[arg(short, long)]
    wordlist: String
}

pub fn unsign_cookie(arg: Unsign) {
    println!("Unsigning cookie: {}", arg.cookie);
    println!("Using wordlist: {}", arg.wordlist);
}