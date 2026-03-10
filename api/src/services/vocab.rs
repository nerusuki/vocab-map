use std::vec;

use crate::models::{Embedding, Vocab};
use crate::schema::user_vocab::{self};
use crate::{db, schema};

use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};
use linfa::traits::Transformer;
use linfa_tsne::TSneParams;
use ndarray::Array2;

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

#[derive(serde::Serialize)]
pub struct ProjectedWord {
    pub word: String,
    pub x: f32,
    pub y: f32,
}

pub fn get_user_projected(user_id: i32) -> Result<Vec<ProjectedWord>, &'static str> {
    use self::schema::embeddings::dsl::*;

    let connection = &mut db::establish_connection();

    let Ok(words): Result<Vec<Embedding>, _> = embeddings
        .inner_join(schema::vocab::table.on(word.eq(schema::vocab::word)))
        .inner_join(user_vocab::table.on(schema::vocab::id.eq(user_vocab::vocab)))
        .filter(user_vocab::user.eq(user_id))
        .select(Embedding::as_select())
        .load(connection)
    else {
        return Err("Could not load vocab");
    };

    let dim = 300;
    let values = words.iter().map(|x| x.vector.to_vec()).flatten().collect();

    let values: Array2<f32> = Array2::from_shape_vec((words.len(), dim), values).unwrap();

    let y_2d = TSneParams::embedding_size(2)
        .perplexity(10.0)
        .approx_threshold(0.3)
        .transform(values)
        .unwrap();

    let mut result: Vec<ProjectedWord> = vec![];
    let mut y_2d_iter = y_2d.outer_iter().into_iter();
    for w in words {
        let y = y_2d_iter.next().unwrap();
        result.push(ProjectedWord {
            word: w.word,
            x: y[0],
            y: y[1],
        });
    }

    Ok(result)
}
