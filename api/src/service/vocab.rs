use std::vec;

use crate::db::Language;
use crate::{repository, service};

use linfa::traits::Transformer;
use linfa_tsne::TSneParams;
use ndarray::Array2;

impl service::Vocab {
    pub async fn get_user(
        &self,
        user_id: i32,
        lang: Option<Language>,
    ) -> Result<Vec<String>, &'static str> {
        let words = self.repo.get_by_user(user_id, lang).await?;
        Ok(words.into_iter().map(|vocab| vocab.word).collect())
    }

    pub async fn get_user_projected(
        &self,
        user_id: i32,
        lang: Option<Language>,
    ) -> Result<Vec<ProjectedWord>, &'static str> {
        let embedding_repository = repository::Embedding {
            pool: self.pool.clone(),
        };

        let words = embedding_repository.get_by_user(user_id, lang).await?;

        let word_count = words.len();

        if word_count == 0 {
            return Ok(vec![]);
        }

        let dim = 300;
        let values = words.iter().map(|x| x.vector.to_vec()).flatten().collect();

        let values: Array2<f32> = Array2::from_shape_vec((words.len(), dim), values).unwrap();

        let perplexity: f32 = if word_count > 1 {
            12.0 * (word_count as f32) / 250.0
        } else {
            0.0
        };

        let y_2d = TSneParams::embedding_size(2)
            .perplexity(perplexity)
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

    pub async fn add_user(
        &self,
        word: &str,
        user_id: i32,
        lang: Option<Language>,
    ) -> Result<&'static str, &'static str> {
        let word = self.repo.get_by_word(word, lang).await?;
        self.repo.add_to_user(word.id, user_id).await?;
        Ok("Word added successfully")
    }

    pub async fn add_user_from_words(
        &self,
        words: Vec<String>,
        user_id: i32,
        lang: Option<Language>,
    ) -> Result<String, &'static str> {
        let embedding_service = service::Embedding::new(self.pool.clone());
        let lang = lang.unwrap_or(Language::En);

        let mut words_to_add = embedding_service
            .predict_from_words(words, 1, false, user_id, Some(lang))
            .await?;
        let word_to_add = words_to_add.pop().unwrap();

        let word = match self.repo.get_by_word(&word_to_add, Some(lang)).await {
            Ok(word) => word,
            Err(_) => self.repo.insert(&word_to_add, lang).await?,
        };

        self.repo.add_to_user(word.id, user_id).await?;

        Ok(word_to_add)
    }

    pub async fn delete_user_words(
        &self,
        words: Vec<String>,
        user_id: i32,
        lang: Option<Language>,
    ) -> Result<&'static str, &'static str> {
        let words = self.repo.get_by_words(&words, lang).await?;
        let word_ids = words.iter().map(|word| word.id).collect::<Vec<i32>>();

        self.repo.delete_from_user(&word_ids, user_id).await?;

        Ok("Words deleted successfully")
    }

    pub async fn search(
        &self,
        search: &str,
        lang: Option<Language>,
    ) -> Result<Vec<String>, &'static str> {
        let words = self.repo.search(search, 20, lang).await?;
        let words = words.into_iter().map(|vocab| vocab.word).collect();

        Ok(words)
    }
}

#[derive(serde::Serialize)]
pub struct ProjectedWord {
    pub word: String,
    pub x: f32,
    pub y: f32,
}
