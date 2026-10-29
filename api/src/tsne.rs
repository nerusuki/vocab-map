use linfa::traits::Transformer;
use linfa_tsne::TSneParams;
use ndarray::Array2;

const APPROX_THRESHOLD: f32 = 0.3;

pub fn tsne2(vec: Vec<Vec<f32>>, perplexity: f32) -> Result<Vec<[f32; 2]>, String> {
    let len = vec.len();
    if len == 0 {
        return Err("Empty vec".to_string());
    }

    let dimensions = vec[0].len();
    let values: Array2<f32> =
        Array2::from_shape_vec((len, dimensions), vec.into_iter().flatten().collect())
            .map_err(|err| err.to_string())?;

    let result = TSneParams::embedding_size(2)
        .perplexity(perplexity)
        .approx_threshold(APPROX_THRESHOLD)
        .transform(values)
        .map_err(|err| err.to_string())?;

    Ok(result
        .outer_iter()
        .into_iter()
        .map(|v| [v[0], v[1]])
        .collect())
}
