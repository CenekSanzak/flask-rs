use clap::Args;
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{Engine as _, engine::general_purpose};
// use flate2::read::ZlibDecoder;
use ring::hmac;

#[derive(Args)]
pub struct Sign{
    #[arg(short, long)]
    payload: String,
    #[arg(short, long)]
    secret: String
}

fn u32_to_bytes(timestamp: u32) -> [u8; 4] {
    let mut bytes = [0; 4];
    bytes[0] = (timestamp >> 24) as u8;
    bytes[1] = (timestamp >> 16) as u8;
    bytes[2] = (timestamp >> 8) as u8;
    bytes[3] = timestamp as u8;
    bytes
}

fn encode_timestamp(timestamp: u32) -> Result<String, String> {
    let bytes = u32_to_bytes(timestamp);
    let encoded = general_purpose::URL_SAFE_NO_PAD.encode(&bytes);
    Ok(encoded)
}

fn encode_payload(payload: String) -> Result<String, String> {
    let encoded = general_purpose::URL_SAFE_NO_PAD.encode(payload.as_bytes());
    Ok(encoded)
}

fn create_hmac_signature(encoded_payload: String, encoded_timestamp: String, secret: String) -> Result<String, String> {
    let salt = "cookie-session";
    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, secret.as_bytes());
    let derived_key = hmac::sign(&key, &salt.as_bytes());
    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, derived_key.as_ref());

    let mut data = Vec::new();
    data.extend_from_slice(encoded_payload.as_bytes());
    data.push(b'.');
    data.extend_from_slice(encoded_timestamp.as_bytes());

    let signature = hmac::sign(&key, &data);
    let encoded = general_purpose::URL_SAFE_NO_PAD.encode(signature.as_ref());
    Ok(encoded) 
}

pub fn sign_cookie(payload: String, timestamp: u32, secret: String) -> Result<String, String> {
    let encoded_payload = encode_payload(payload).unwrap();
    let encoded_timestamp = encode_timestamp(timestamp).unwrap();
    let signature = create_hmac_signature(encoded_payload.clone(), encoded_timestamp.clone(), secret).unwrap();
    let signed = format!("{}.{}.{}", encoded_payload, encoded_timestamp, signature);
    Ok(signed)
}

pub fn sign_and_print_cookie(arg: Sign) {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
    let result = sign_cookie(arg.payload, current_time, arg.secret);
    if result.is_err() {
        println!("{}", result.err().unwrap());
        return;
    }
    let signed = result.unwrap();
    println!("Signed: {}", signed);
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_u32_to_bytes(){
        let timestamp = 1547409093;
        let expected_result = [0x5c, 0x5b, 0x1d, 0x45];
        let result = u32_to_bytes(timestamp);
        assert!(result == expected_result)
    }

    #[test]
    fn test_encode_timestamp(){
        let timestamp = 1547409093;
        let expected_result = "XDuWxQ";
        let result = encode_timestamp(timestamp);
        assert!(result.is_ok());
        let encoded = result.unwrap();
        assert!(encoded == expected_result)
    }

    #[test]
    fn test_encode_payload(){
        let payload = "{\"logged_in\":false}".to_owned();
        let expected_result = "eyJsb2dnZWRfaW4iOmZhbHNlfQ";
        let result = encode_payload(payload);
        assert!(result.is_ok());
        let encoded = result.unwrap();
        assert!(encoded == expected_result)
    }

    #[test]
    fn test_sign_cookie(){
        let payload = "{\"logged_in\":false}".to_owned();
        let timestamp = 1547409093;
        let expected = "E2Pyb6x3w-NODuflHoGnZOEpbH8".to_owned();
        let result = create_hmac_signature(payload, timestamp, "CHANGEME".to_owned());
        assert!(result.is_ok());
        let signature = result.unwrap();
        assert!(signature == expected)
    }
}