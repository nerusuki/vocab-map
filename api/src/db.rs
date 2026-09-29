use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{
        AsyncDieselConnectionManager,
        deadpool::{Object, Pool},
    },
};
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<AsyncPgConnection>;
pub type DbConn = Object<AsyncPgConnection>;

pub fn create_connection_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config)
        .max_size(20)
        .build()
        .expect("Unable to create connection pool");

    pool
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[db_enum(existing_type_path = "crate::schema::sql_types::Language")]
pub enum Language {
    En,
}
