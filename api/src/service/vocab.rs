use actix_web::rt::Runtime;
use serde_json::json;

use crate::db::Language;
use crate::models::Vocab;
use crate::{projection, repository, service};

impl service::Vocab {
    pub async fn get(
        &self,
        words: &[String],
        lang: Option<Language>,
    ) -> Result<Vec<Vocab>, &'static str> {
        let vocabs = self.repo.get_by_words(words, lang).await?;
        Ok(vocabs)
    }

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
    ) -> Result<Vec<ProjectedWord>, String> {
        let embedding_repository = repository::Embedding {
            pool: self.pool.clone(),
        };

        let (words, user_vocabs) = embedding_repository
            .get_by_user(user_id, lang)
            .await?
            .into_iter()
            .collect::<(Vec<_>, Vec<_>)>();

        let word_count = words.len();

        if word_count < 3 {
            return Ok(words
                .into_iter()
                .map(|w| ProjectedWord {
                    word: w.word,
                    x: 0.5,
                    y: 0.5,
                })
                .collect());
        }

        let perplexity: f32 = ((word_count as f32).sqrt() - 1.51) * 2.0;

        let (word_list, vec_list): (Vec<String>, Vec<Vec<f32>>) = words
            .into_iter()
            .map(|word| (word.word, word.vector.to_vec()))
            .collect();

        let mut missing_initial_count: i32 = 0;
        /*let initial = user_vocabs
            .iter()
            .map(|vocab| match &vocab.projection {
                Some(projection) => projection.to_vec().try_into().unwrap(),
                None => {
                    missing_initial_count += 1;
                    [0.5; 2]
                }
            })
            .collect();

        let initial = if missing_initial_count < 3 {
            Some(initial)
        } else {
            None
        };*/

        let initial = None;

        let projected: Vec<_> = projection::tsne::<2>(vec_list, initial, perplexity)?;

        /*self.repo
        .update_projection(
            user_id,
            user_vocabs
                .iter()
                .map(|user_vocab| user_vocab.vocab)
                .zip(projected.iter().map(|p| p.as_slice()))
                .collect(),
        )
        .await
        .unwrap();*/

        if let Some(lang) = lang {
            let projection_repository = repository::Projection {
                pool: self.pool.clone(),
            };
            //let projected = projected.clone();
            let json = json!(&projected);
            actix_web::rt::spawn(async move {
                projection_repository.insert(user_id, lang, json).await.ok();
                println!("Inserted!!!!!");
            });
        };

        let result = projected
            .iter()
            .zip(word_list.into_iter())
            .map(|([x, y], word)| ProjectedWord {
                word: word.to_string(),
                x: *x,
                y: *y,
            })
            .collect();

        println!("Returning");

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
            .predict(words, 1, Some(user_id), Some(lang))
            .await?;
        let word_to_add = words_to_add.pop().unwrap().0;

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
