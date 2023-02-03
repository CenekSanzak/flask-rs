use flask_rs::decode::{Decode, decode_cookie};
use flask_rs::sign::{sign_cookie};

#[test]
fn test_decode_cookie(){
    let cookie = "eyJsb2dnZWRfaW4iOmZhbHNlfQ.XDuWxQ.E2Pyb6x3w-NODuflHoGnZOEpbH8";
    let expected_result = Ok(("{\"logged_in\":false}".to_owned(), 1547409093));
    let args = Decode{
        cookie: cookie.to_string()
    };
    let result = decode_cookie(args);
    assert!(result == expected_result)
}

fn test_sign_cookie(){
    let payload = "{\"logged_in\":false}";
    let timestamp = 1547409093;
    let expected_result = Ok("eyJsb2dnZWRfaW4iOmZhbHNlfQ.XDuWxQ.E2Pyb6x3w-NODuflHoGnZOEpbH8".to_owned());
    let result = sign_cookie(payload.to_string(), timestamp);
    assert!(result == expected_result)
}