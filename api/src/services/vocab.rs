use crate::models::Vocab;
use crate::schema::user_vocab::{self};
use crate::{db, schema};

use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};

pub fn get_user(user_id: i32) -> Result<Vec<String>, &'static str> {
    use self::schema::vocab::dsl::*;

    let connection = &mut db::establish_connection();

    let Ok(mut words) = vocab
        .inner_join(user_vocab::table.on(id.eq(user_vocab::vocab)))
        .filter(user_vocab::user.eq(user_id))
        .select(Vocab::as_select())
        .load(connection)
    else {
        return Err("Could not load vocab");
    };

    let mut result_words = vec![];
    while let Some(w) = words.pop() {
        result_words.push(w.word);
    }
    result_words.reverse();

    Ok(result_words)
}
