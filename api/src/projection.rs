use bhtsne;

const APPROX_THRESHOLD: f32 = 0.5;
const EPOCHS: usize = 2000;

pub fn tsne<const D: usize>(
    vec: Vec<Vec<f32>>,
    initial: Option<Vec<[f32; D]>>,
    perplexity: f32,
) -> Result<Vec<[f32; D]>, String> {
    let len = vec.len();
    if len == 0 {
        return Err("Empty vec".to_string());
    }

    let vec: Vec<&[f32]> = vec.iter().map(|v| v.as_slice()).collect();

    let mut tsne = bhtsne::tSNE::<f32, &[f32], 2>::new(vec.as_slice());

    if let Some(initial) = initial {
        tsne.initial_embedding(initial.into_iter().flatten().collect::<Vec<f32>>())
            .stop_lying_epoch(0);
    }

    let result = tsne
        .perplexity(perplexity)
        .epochs(EPOCHS)
        .barnes_hut(APPROX_THRESHOLD, self::l2_distance)
        .embedding();

    let result: Vec<[f32; D]> = unsafe { result.as_chunks_unchecked::<D>().to_vec() };
    Ok(result)
}

fn l2_distance(a: &&[f32], b: &&[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        .sqrt()
}

pub fn to_json_map(ids: &[i32], points: &[[f32; 2]]) -> serde_json::Value {
    serde_json::Map::from_iter(
        ids.iter().map(|id| id.to_string()).zip(
            points
                .iter()
                .map(|point| serde_json::to_value(point).unwrap()),
        ),
    )
    .into()
}

pub fn from_json_map(ids: &[i32], mut json: serde_json::Value) -> Vec<[f32; 2]> {
    ids.iter()
        .map(|id| serde_json::from_value(json[id.to_string()].take()).unwrap_or_default())
        .collect::<Vec<_>>()
}
