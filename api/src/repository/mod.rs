use crate::db::{DbConn, DbPool};

mod embedding;
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
        self.get_pool().get().await.unwrap()
    }
}
