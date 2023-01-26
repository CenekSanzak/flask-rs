use flask_rs::decode::{Decode, decode_cookie};

#[test]
fn test_decode_cookie(){
    let cookie = "eyJhZG1pbiI6ZmFsc2UsInVpZCI6ImNlbmVrc2FuemFrIn0.Y8LELg.Be7MYQQSD-rm0xm4XGDk6IJ4aWQ";
    let expected_result = "{\"admin\":false,\"uid\":\"ceneksanzak\"}";
    let args = Decode{
        cookie: cookie.to_string()
    };
    let result = decode_cookie(args);
    assert!(result == expected_result);

}