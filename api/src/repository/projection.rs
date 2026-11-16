use crate::db::Language;
use crate::repository;
use crate::repository::Repository;
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
    ) -> Result<(), &'static str> {
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
            .execute(conn)
            .await
            .or(Err("Could not insert vocab"))?;

        Ok(())
    }

    pub async fn get_projection(&self, user_id: i32, lang: Language) -> Option<serde_json::Value> {
        let conn = &mut self.get_conn().await;

        projection::table
            .filter(projection::user.eq(user_id))
            .filter(projection::lang.eq(lang))
            .select(projection::projection_)
            .get_result(conn)
            .await
            .ok()
    }
}
