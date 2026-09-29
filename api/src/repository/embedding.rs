use crate::models::Embedding;
use crate::repository;
use crate::repository::Repository;
use crate::schema::{embedding, user_vocab, vocab};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use pgvector::{Vector, VectorExpressionMethods};

impl repository::Embedding {
    pub async fn get_by_word(&self, word: &str) -> Result<Embedding, &'static str> {
        let mut conn = self.pool.get().await.unwrap();

        embedding::table
            .filter(embedding::word.eq(word))
            .select(Embedding::as_select())
            .first(&mut conn)
            .await
            .or(Err("Word not found"))
    }

    pub async fn get_closest_words(
        &self,
        vec: &[f32],
        count: i64,
        vocab_only: bool,
        user_id: Option<i32>,
    ) -> Result<Vec<String>, &'static str> {
        let conn = &mut self.get_conn().await;

        let mut query = embedding::table
            .left_join(vocab::table.on(embedding::word.ilike(vocab::word)))
            .left_join(
                user_vocab::table.on(vocab::id
                    .eq(user_vocab::vocab)
                    .and(user_vocab::user.eq(user_id.unwrap_or(0)))),
            )
            .order_by(embedding::vector.l2_distance(Vector::from(vec.to_vec())))
            .limit(count)
            .select(embedding::word)
            .into_boxed();

        if vocab_only {
            query = query.filter(vocab::id.is_not_null());
        }

        if user_id.is_some() {
            query = query.filter(user_vocab::user.is_null());
        }

        query.load(conn).await.or(Err("Could not find words"))
    }

    pub async fn get_by_user(&self, user_id: i32) -> Result<Vec<Embedding>, &'static str> {
        let conn = &mut self.get_conn().await;

        embedding::table
            .inner_join(vocab::table.on(embedding::word.eq(vocab::word)))
            .inner_join(user_vocab::table.on(vocab::id.eq(user_vocab::vocab)))
            .filter(user_vocab::user.eq(user_id))
            .select(Embedding::as_select())
            .load(conn)
            .await
            .or(Err("Could not load vocab"))
    }
}
