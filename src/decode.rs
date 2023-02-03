use clap::Args;
use base64::{Engine as _, engine::general_purpose};

#[derive(Args)]
pub struct Decode{
    #[arg(short, long)]
    pub cookie: String
}

fn validate_split_cookie(arg: &str) -> Result<(String, String), String> {
    // A valid cookie must contain exactly two dots
    
    let dot_count = arg.matches('.').count();
    if dot_count != 2 {
        return Err(format!("Invalid cookie: {}", arg));
    }
    let split: Vec<&str> = arg.split('.').collect();
    Ok((split[0].to_string(), split[1].to_string()))
}

fn array_to_u32(array: &[u8]) -> u32 {
    return array
        .iter()
        .rev()
        .map(|&x| x as u32)
        .enumerate()
        .map(|(i, x)| x<<(8*i))
        .fold(0, |acc, x| acc + x);
}

fn decode_payload(payload: &str) -> Result<String, String> {
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(payload.as_bytes()).unwrap();
    let result = String::from_utf8(decoded).unwrap();
    return Ok(result);
}

fn decode_timestamp(timestamp: &str) -> Result<u32, String> {
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(timestamp.as_bytes()).unwrap();
    let result = array_to_u32(&decoded);
    return Ok(result);
}

pub fn decode_cookie(arg: Decode) -> Result<(String, u32), String> {
    let (first_part, second_part) = validate_split_cookie(&arg.cookie).unwrap();
    let payload = decode_payload(&first_part).unwrap();
    let timestamp = decode_timestamp(&second_part).unwrap();

    Ok((payload, timestamp))
}

pub fn decode_and_print_cookie(arg: Decode) {
    let result = decode_cookie(arg);
    if result.is_err() {
        println!("{}", result.err().unwrap());
        return;
    }
    let (decoded, timestamp) = result.unwrap();
    println!("Decoded: {}", decoded);
    println!("Timestamp: {}", timestamp);
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
        let valid_cookie = "eyJsb2dnZWRfaW4iOmZhbHNlfQ.XDuWxQ.E2Pyb6x3w-NODuflHoGnZOEpbH8";
        let expected_result = ("eyJsb2dnZWRfaW4iOmZhbHNlfQ".to_owned(), "XDuWxQ".to_owned());
        let result = validate_split_cookie(valid_cookie);
        assert!(result.is_ok());
        let decoded = result.unwrap();
        assert!(decoded == expected_result)
    }

    #[test]
    fn test_decode_timestamp(){
        let timestamp = "XDuWxQ";
        let expected_result = 1547409093;
        let result = decode_timestamp(timestamp);
        assert!(result.is_ok());
        let decoded = result.unwrap();
        assert!(decoded == expected_result)
        
    }
}