use clap::Args;
use base64::{Engine as _, engine::general_purpose};

#[derive(Args)]
pub struct Decode{
    #[arg(short, long)]
    cookie: String
}

fn validate_split_cookie(arg: &str) -> Result<String, String> {
    // A valid cookie must contain exactly two dots
    
    let dot_count = arg.matches('.').count();
    if dot_count != 2 {
        return Err(format!("Invalid cookie: {}", arg));
    }
    let split: Vec<&str> = arg.split('.').collect();
    Ok(split[0].to_string())
}

pub fn decode_cookie(arg: Decode) {
    // Decodes a flask cookie
    let first_part = validate_split_cookie(&arg.cookie).unwrap();
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(first_part.as_bytes()).unwrap();
    println!("{}", String::from_utf8(decoded).unwrap());
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_invalid_cookie(){
        let invalid_cookie = "invalid.cookie";
        let result = validate_split_cookie(invalid_cookie);
        assert!(result.is_err());
    }
    #[test]
    fn test_valid_cookie_splitted(){
        let valid_cookie = "eyJhZG1pbiI6ZmFsc2UsInVpZCI6ImNlbmVrc2FuemFrIn0.Y8LELg.Be7MYQQSD-rm0xm4XGDk6IJ4aWQ";
        let expected_result = "eyJhZG1pbiI6ZmFsc2UsInVpZCI6ImNlbmVrc2FuemFrIn0";
        let result = validate_split_cookie(valid_cookie);
        assert!(result.is_ok());
        let decoded = result.unwrap();
        assert!(decoded == expected_result)
    }
}