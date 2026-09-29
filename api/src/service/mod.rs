use crate::{db::DbPool, repository};

mod embedding;
mod user;
mod vocab;

pub struct Embedding {
    pool: DbPool,
    repo: repository::Embedding,
}

impl Embedding {
    pub fn new(pool: DbPool) -> Self {
        Self {
            pool: pool.clone(),
            repo: repository::Embedding { pool },
        }
    }
}

pub struct User {
    pool: DbPool,
    repo: repository::User,
}

impl User {
    pub fn new(pool: DbPool) -> Self {
        Self {
            pool: pool.clone(),
            repo: repository::User { pool },
        }
    }
}

pub struct Vocab {
    pool: DbPool,
    repo: repository::Vocab,
}

impl Vocab {
    pub fn new(pool: DbPool) -> Self {
        Self {
            pool: pool.clone(),
            repo: repository::Vocab { pool },
        }
    }
}
