use crate::models::User;
use crate::repository;
use crate::repository::Repository;
use crate::schema::user;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

impl repository::User {
    pub async fn get_by_name(&self, name: &str) -> Result<User, &'static str> {
        let conn = &mut self.get_conn().await;

        user::table
            .filter(user::name.eq(name))
            .select(User::as_select())
            .first(conn)
            .await
            .or(Err("User not found"))
    }
}
