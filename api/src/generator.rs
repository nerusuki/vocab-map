use fastembed::{EmbeddingModel, Error, TextEmbedding, TextInitOptions};
use futures::future;

use crate::{db::create_connection_pool, repository};

const MODEL: EmbeddingModel = EmbeddingModel::BGEM3;
const CHUNK_SIZE: usize = 64;

pub fn generate_from_words(words: &[String]) -> Result<Vec<Vec<f32>>, Error> {
    let mut model =
        TextEmbedding::try_new(TextInitOptions::new(MODEL).with_show_download_progress(true))?;

    let embeddings = model.embed(words, None)?;

    Ok(embeddings)
}

pub fn generate_missing() -> Result<(), &'static str> {
    let pool = create_connection_pool();
    let vocab_repo = repository::Vocab { pool: pool.clone() };
    let embedding_repo = repository::Embedding { pool };

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let words = runtime.block_on(vocab_repo.get_without_embedding())?;
    let total_words = words.len();

    println!("Generating embeddings for {} words", total_words);

    let mut generated = 0;
    runtime.block_on(async {
        for word_chunk in words
            .into_iter()
            .map(|vocab| vocab.word)
            .collect::<Vec<String>>()
            .chunks(CHUNK_SIZE)
        {
            let mut vecs = generate_from_words(word_chunk)
                .or(Err("Could not generate embeddings"))
                .unwrap()
                .into_iter();

            generated += vecs.len();

            let results = future::join_all(
                word_chunk
                    .iter()
                    .map(|word| embedding_repo.insert(word, vecs.next().unwrap())),
            )
            .await;

            for result in results {
                if let Err(err) = result {
                    println!("Error: {}", err);
                    continue;
                };
            }

            let progress = generated as f32 / total_words as f32 * 100.0;
            println!("Progress: {} / {} = {}%", generated, total_words, progress);
        }
    });

    Ok(())
}
