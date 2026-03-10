use std::collections::BTreeMap;

use actix_web::http::header::{HeaderMap, HeaderName};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

pub fn get_token(headers: &HeaderMap) -> Option<BTreeMap<String, String>> {
    let Some(authorization) = headers.get(HeaderName::from_static("authorization")) else {
        return None;
    };

    let authorization = authorization.to_str().unwrap();

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let token_str = authorization.replace("Bearer ", "");
    return token_str.verify_with_key(&key).unwrap_or(None);
}

pub fn get_user_id(headers: &HeaderMap) -> Option<i32> {
    let Some(mut token) = get_token(headers) else {
        return None;
    };
    let Some(sub) = token.remove("sub") else {
        return None;
    };
    let Ok(user_id) = sub.parse() else {
        return None;
    };

    return Some(user_id);
}
