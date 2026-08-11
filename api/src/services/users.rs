use crate::models::User;
use crate::{db, schema, utils};

use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};
use jwt::SignWithKey;
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

    let mut claims = BTreeMap::new();
    claims.insert("sub", user_entity.id.to_string());
    claims.insert("name", user_entity.name);
    let token_str = claims.sign_with_key(&utils::token::get_key()).unwrap();

    Ok(token_str)
}
