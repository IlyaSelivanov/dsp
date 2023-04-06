use std::collections::BTreeMap;

use rand::prelude::*;

pub fn generate_random_vector(length: usize, range: (i32, i32)) -> Option<Vec<i32>> {
    if length <= 0 || range.0 > range.1 {
        return None;
    }

    let mut vec: Vec<i32> = Vec::new();
    let mut rng = thread_rng();

    for _ in 0..length {
        vec.push(rng.gen_range(range.0..=range.1));
    }

    Some(vec)
}

pub fn calc_vector_mean(vec: &Vec<i32>) -> f64 {
    if vec.is_empty() {
        return 0.0;
    }

    let mut sum: f64 = 0.0;

    for item in vec.iter() {
        sum += *item as f64;
    }

    sum / (vec.len() as f64)
}

pub fn calc_vector_variance(vec: &Vec<i32>) -> f64 {
    if vec.is_empty() {
        return 0.0;
    }

    let mean = calc_vector_mean(vec);

    let mut variance = 0.0;
    for item in vec.iter() {
        variance += (*item as f64 - mean).powf(2.0);
    }

    variance / (vec.len() - 1) as f64
}

pub enum Median {
    Even(i32, i32),
    Odd(i32),
    None,
}

pub fn get_median(vec: &mut Vec<i32>) -> Median {
    if vec.is_empty() {
        return Median::None;
    }

    vec.sort_unstable();
    let idx = vec.len() / 2;

    if vec.len() % 2 == 0 {
        return Median::Even(vec[idx - 1], vec[idx]);
    } else {
        return Median::Odd(vec[idx]);
    }
}

pub fn build_btree_map_histogram(vec: &Vec<i32>) -> Option<BTreeMap<i32, u32>> {
    if vec.is_empty() {
        return None;
    }

    let mut map: BTreeMap<i32, u32> = BTreeMap::new();

    for item in vec {
        let count = map.entry(*item).or_insert(0);
        *count += 1;
    }

    Some(map)
}
