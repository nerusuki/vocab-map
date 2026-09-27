use crate::repository;

pub async fn predict_from_word(
    word: &str,
    count: i64,
    vocab_only: bool,
) -> Result<Vec<String>, &str> {
    let embedding = repository::embedding::get_by_word(word).await?;
    return repository::embedding::get_closest_words(
        embedding.vector.as_slice(),
        count,
        vocab_only,
        None,
    )
    .await;
}

pub async fn predict_from_words(
    words: Vec<String>,
    count: i64,
    vocab_only: bool,
    user_id: i32,
) -> Result<Vec<String>, &'static str> {
    let mut vecs = vec![];

    for word in words.iter() {
        let embedding = repository::embedding::get_by_word(word).await?;
        vecs.push(embedding.vector.to_vec());
    }

    let sum = self::sum_vecs(&vecs);

    return repository::embedding::get_closest_words(&sum, count, vocab_only, Some(user_id)).await;
}

fn sum_vecs(vecs: &[Vec<f32>]) -> Vec<f32> {
    vecs.into_iter()
        .map(|vec| vec.to_vec())
        .reduce(|a, b| a.iter().zip(b.iter()).map(|(&a, &b)| a + b).collect())
        .unwrap()
}
