use crate::db::Language;
use crate::models::Vocab;
use crate::repository;
use crate::repository::Repository;
use crate::schema::{user_vocab, vocab};

use diesel::dsl::{delete, insert_into};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

impl repository::Vocab {
    pub async fn insert(&self, word: &str, lang: Language) -> Result<Vocab, &'static str> {
        let conn = &mut self.get_conn().await;

        insert_into(vocab::table)
            .values((vocab::word.eq(word), vocab::lang.eq(lang)))
            .get_result(conn)
            .await
            .or(Err("Could not insert vocab"))
    }

    pub async fn get_by_user(&self, user_id: i32) -> Result<Vec<Vocab>, &'static str> {
        let conn = &mut self.get_conn().await;

        vocab::table
            .inner_join(user_vocab::table.on(vocab::id.eq(user_vocab::vocab)))
            .filter(user_vocab::user.eq(user_id))
            .select(Vocab::as_select())
            .load(conn)
            .await
            .or(Err("Could not load vocab"))
    }

    pub async fn get_by_word(&self, word: &str) -> Result<Vocab, &'static str> {
        let conn = &mut self.get_conn().await;

        vocab::table
            .filter(vocab::word.eq(word))
            .select(Vocab::as_select())
            .first(conn)
            .await
            .or(Err("Could not find word"))
    }

    pub async fn get_by_words(&self, words: &[String]) -> Result<Vec<Vocab>, &'static str> {
        let conn = &mut self.get_conn().await;

        vocab::table
            .filter(vocab::word.eq_any(words))
            .select(Vocab::as_select())
            .load(conn)
            .await
            .or(Err("Could not find words"))
    }

    pub async fn add_to_user(&self, word_id: i32, user_id: i32) -> Result<usize, &'static str> {
        let conn = &mut self.get_conn().await;

        insert_into(user_vocab::table)
            .values((user_vocab::vocab.eq(word_id), user_vocab::user.eq(user_id)))
            .execute(conn)
            .await
            .or(Err("Could not add word"))
    }

    pub async fn delete_from_user(
        &self,
        word_ids: &[i32],
        user_id: i32,
    ) -> Result<usize, &'static str> {
        let conn = &mut self.get_conn().await;

        return delete(user_vocab::table)
            .filter(user_vocab::vocab.eq_any(word_ids))
            .filter(user_vocab::user.eq(user_id))
            .execute(conn)
            .await
            .or(Err("Could not delete words"));
    }

    pub async fn search(&self, search: &str, limit: i64) -> Result<Vec<Vocab>, &'static str> {
        let conn = &mut self.get_conn().await;

        return vocab::table
            .filter(vocab::word.ilike(format!("{}%", search)))
            .limit(limit)
            .select(Vocab::as_select())
            .load(conn)
            .await
            .or(Err("Could not load vocab"));
    }
}
