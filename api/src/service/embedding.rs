use crate::{db::Language, service};

impl service::Embedding {
    pub async fn predict(
        &self,
        words: Vec<String>,
        count: i64,
        user_id: Option<i32>,
        lang: Option<Language>,
    ) -> Result<Vec<String>, &'static str> {
        let embeddings = self.repo.get_by_words(&words).await?;
        let vecs = embeddings
            .into_iter()
            .map(|embedding| embedding.vector.to_vec())
            .collect::<Vec<_>>();

        let sum = sum_vecs(&vecs);

        return self
            .repo
            .get_closest_words(&sum, count, user_id, lang)
            .await;
    }
}

fn sum_vecs(vecs: &[Vec<f32>]) -> Vec<f32> {
    vecs.into_iter()
        .map(|vec| vec.to_vec())
        .reduce(|a, b| a.iter().zip(b.iter()).map(|(&a, &b)| a + b).collect())
        .unwrap()
}
