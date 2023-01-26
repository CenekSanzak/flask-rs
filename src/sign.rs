use clap::Args;
#[derive(Args)]
pub struct Sign{
    #[arg(short, long)]
    payload: String
}

pub fn sign_cookie(arg: Sign) {
    println!("Signing: {}", arg.payload);
}