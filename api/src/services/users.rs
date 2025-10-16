use crate::models::User;
use crate::{db, schema};

use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn auth(username: &str, password: &str) -> Result<String, &'static str> {
    use self::schema::user::dsl::*;

    let connection = &mut db::establish_connection();

    let Ok(user_entity) = user
        .filter(name.eq(username))
        .select(User::as_select())
        .first(connection)
    else {
        return Err("User not found");
    };

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", user_entity.id.to_string());
    claims.insert("name", user_entity.name);
    let token_str = claims.sign_with_key(&key).unwrap();

    Ok(token_str)
}
