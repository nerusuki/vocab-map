use crate::db::Language;
use crate::models::Projection;
use crate::repository::{self, Repository};
use crate::schema::projection;

use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

impl repository::Projection {
    pub async fn insert(
        &self,
        user_id: i32,
        lang: Language,
        projection: serde_json::Value,
    ) -> Result<Projection, &'static str> {
        let conn = &mut self.get_conn().await;

        insert_into(projection::table)
            .values((
                projection::user.eq(user_id),
                projection::lang.eq(lang),
                projection::projection_.eq(&projection),
            ))
            .on_conflict(projection::table.primary_key())
            .do_update()
            .set(projection::projection_.eq(&projection))
            .get_result(conn)
            .await
            .or(Err("Could not insert vocab"))
    }

    pub async fn get(&self, user_id: i32, lang: Language) -> Option<Projection> {
        let conn = &mut self.get_conn().await;

        projection::table
            //.inner_join(user_vocab::table.on(vocab::id.eq(user_vocab::vocab)))
            .filter(projection::user.eq(user_id))
            .filter(projection::lang.eq(lang))
            .select(Projection::as_select())
            .get_result(conn)
            .await
            .ok()
    }

    /*
    pub async fn update_projections(
        &self,
        user_id: i32,
        projections: Vec<(i32, &[f32])>,
    ) -> Result<(), String> {
        let conn = &mut self.get_conn().await;

        fn update_projection(
            user_id: i32,
            vocab_id: i32,
            projection: Vec<f32>,
            mut conn: &AsyncPgConnection,
        ) -> Pin<Box<dyn Future<Output = Result<usize, Error>> + '_>> {
            println!("Running update!");

            update(user_vocab::table)
                .filter(user_vocab::user.eq(user_id))
                .filter(user_vocab::vocab.eq(vocab_id))
                .set(user_vocab::projection.eq(Vector::from(projection)))
                .execute(&mut conn)
        }

        let results: Vec<_> =
            FuturesUnordered::from_iter(projections.iter().map(|(vocab_id, projection)| {
                update_projection(user_id, *vocab_id, projection.to_vec(), &mut &conn)
                //self.update_projection(user_id, *vocab_id, projection.to_vec())

                /*update(user_vocab::table)
                .filter(user_vocab::user.eq(user_id))
                .filter(user_vocab::vocab.eq(vocab_id))
                .set(user_vocab::projection.eq(Vector::from(projection.to_vec())))
                .execute(&mut conn)
                .await*/
            }))
            .collect()
            .await;

        for result in results {
            println!("Getting result");
            result.map_err(|err| err.to_string())?;
        }

        Ok(())
    }

    pub async fn search(
        &self,
        search: &str,
        limit: i64,
        lang: Option<Language>,
    ) -> Result<Vec<Vocab>, &'static str> {
        let conn = &mut self.get_conn().await;

        let mut query = vocab::table
            .filter(vocab::word.ilike(format!("{}%", search)))
            .limit(limit)
            .select(Vocab::as_select())
            .into_boxed();

        if let Some(lang) = lang {
            query = query.filter(vocab::lang.eq(lang));
        }

        query.load(conn).await.or(Err("Could not load vocab"))
    }

    pub async fn get_without_embedding(&self) -> Result<Vec<Vocab>, &'static str> {
        let conn = &mut self.get_conn().await;

        vocab::table
            .left_join(embedding::table.on(embedding::word.eq(vocab::word)))
            .filter(embedding::word.is_null())
            .select(Vocab::as_select())
            .load(conn)
            .await
            .or(Err("Could not load vocab"))
    }*/
}
