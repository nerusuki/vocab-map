use bhtsne;

const APPROX_THRESHOLD: f32 = 0.5;
const EPOCHS: usize = 2000;

pub fn tsne2(vec: Vec<Vec<f32>>, perplexity: f32) -> Result<Vec<[f32; 2]>, String> {
    let len = vec.len();
    if len == 0 {
        return Err("Empty vec".to_string());
    }

    let vec: Vec<&[f32]> = vec.iter().map(|v| v.as_slice()).collect();

    let result = bhtsne::tSNE::<f32, &[f32], 2>::new(vec.as_slice())
        .perplexity(perplexity)
        .epochs(EPOCHS)
        .barnes_hut(APPROX_THRESHOLD, self::l2_distance)
        .embedding();

    let result: Vec<[f32; 2]> = unsafe { result.as_chunks_unchecked::<2>().to_vec() };
    Ok(result)
}

fn l2_distance(a: &&[f32], b: &&[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        .sqrt()
}
