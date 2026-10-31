use crate::db::Language;
use crate::models::Vocab;
use crate::{repository, service, tsne};

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

        let words = embedding_repository.get_by_user(user_id, lang).await?;

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

        let result: Vec<_> = tsne::tsne2(
            words.iter().map(|word| word.vector.to_vec()).collect(),
            perplexity,
        )?
        .into_iter()
        .zip(words.into_iter())
        .map(|([x, y], w)| ProjectedWord { word: w.word, x, y })
        .collect();

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
            .predict_from_words(words, 1, user_id, Some(lang))
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
