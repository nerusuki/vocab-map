use std::{
    fs::File,
    io::{Read, Write},
};

use bhtsne;

const APPROX_THRESHOLD: f32 = 0.5;
const EPOCHS: usize = 2000;

pub fn tsne<const D: usize>(vec: Vec<Vec<f32>>, perplexity: f32) -> Result<Vec<[f32; D]>, String> {
    let len = vec.len();
    if len == 0 {
        return Err("Empty vec".to_string());
    }

    let vec: Vec<&[f32]> = vec.iter().map(|v| v.as_slice()).collect();

    let initial = retrieve();
    dbg!(initial.is_ok());

    let mut tsne = bhtsne::tSNE::<f32, &[f32], 2>::new(vec.as_slice());

    tsne.perplexity(perplexity).epochs(EPOCHS);

    if let Ok(initial) = initial {
        tsne.initial_embedding(initial);
    }

    let result = tsne
        .barnes_hut(APPROX_THRESHOLD, self::l2_distance)
        .embedding();

    let result: Vec<[f32; D]> = unsafe { result.as_chunks_unchecked::<D>().to_vec() };

    dbg!(store(&result).is_ok());

    Ok(result)
}

fn l2_distance(a: &&[f32], b: &&[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        .sqrt()
}

fn store<const D: usize>(vec: &Vec<[f32; D]>) -> std::io::Result<()> {
    let mut file = File::create(".cache/projection")?;
    let buf: String = vec
        .iter()
        .flatten()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    let buf: &[u8] = buf.as_bytes();

    file.write_all(buf)?;
    Ok(())
}

fn retrieve() -> std::io::Result<Vec<f32>> {
    let mut file = File::open(".cache/projection")?;
    let mut buf: String = String::new();
    unsafe {
        file.read_to_end(buf.as_mut_vec())?;
    };
    let vec = String::from(buf)
        .split("\n")
        .map(|value| value.parse::<f32>().unwrap())
        .collect::<Vec<_>>();

    Ok(vec)
}
