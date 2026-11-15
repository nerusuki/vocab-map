use std::time::Instant;

use crate::db::{DbConn, DbPool};

mod embedding;
mod projection;
mod user;
mod vocab;

pub struct Embedding {
    pub pool: DbPool,
}

impl Repository for Embedding {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

pub struct Projection {
    pub pool: DbPool,
}

impl Repository for Projection {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

pub struct User {
    pub pool: DbPool,
}

impl Repository for User {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

pub struct Vocab {
    pub pool: DbPool,
}

impl Repository for Vocab {
    fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}

trait Repository {
    fn get_pool(&self) -> &DbPool;

    async fn get_conn(&self) -> DbConn {
        let now = Instant::now();

        let pool = self.get_pool();
        let status = pool.status();

        println!("{:?}", status);

        let conn = pool.get().await.unwrap();

        let elapsed_time = now.elapsed();
        println!("get_conn() took {} ms", elapsed_time.as_millis());

        conn
    }
}
