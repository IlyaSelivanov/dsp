use std::collections::BTreeMap;

use rand::prelude::*;

pub struct Statistics {
    mean: f64,
    variance: f64,
    standard_deviation: f64,
    median: Median,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            mean: 0f64,
            variance: 0f64,
            standard_deviation: 0f64,
            median: Median::None,
        }
    }

    /// Generates random i32 vector with a specified length in a speciied range.
    pub fn generate_random_i32_vector(length: usize, range: (i32, i32)) -> Option<Vec<i32>> {
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

    /// Creates [`Statistics`] instance with pre-calculated values.
    ///
    /// # Panics
    ///
    /// Panics if provided vector is empty.
    pub fn form_i32_vector(vector: &Vec<i32>) -> Self {
        if vector.is_empty() {
            panic!("Not empty data needed.")
        }

        let mean = calc_vector_mean(&vector);
        let variance = calc_vector_variance(&vector);
        let med = match get_median(&vector) {
            Median::Even(f, s) => Median::Even(f, s),
            Median::Odd(m) => Median::Odd(m),
            _ => Median::None,
        };

        Statistics {
            mean: mean,
            standard_deviation: variance.sqrt(),
            variance: variance,
            median: med,
        }
    }

    /// Returns the mean of this [`Statistics`].
    pub fn mean(&self) -> f64 {
        self.mean
    }

    /// Returns the variance of this [`Statistics`].
    pub fn variance(&self) -> f64 {
        self.variance
    }

    /// Returns the standard deviation of this [`Statistics`].
    pub fn standard_deviation(&self) -> f64 {
        self.standard_deviation
    }

    /// Returns a reference to the median of this [`Statistics`].
    pub fn median(&self) -> &Median {
        &self.median
    }
}

/// Calculates mean value for provded vector.
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

/// Calculates variance for provded vector.
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

/// Gets median value(s) of a provded vector.
pub fn get_median(vec: &Vec<i32>) -> Median {
    if vec.is_empty() {
        return Median::None;
    }

    //Copying input vector to do not modify original.
    //Think it is a bad design from performance perspective.
    //TODO: replace with an adequate algorithm.
    let mut mut_vec: Vec<i32> = Vec::new();
    for item in vec {
        mut_vec.push(*item);
    }
    mut_vec.sort_unstable();
    let idx = mut_vec.len() / 2;

    //For debug purposes only.
    //TODO: remove when done.
    println!("get_median mut_vec: {:?}", mut_vec);

    if vec.len() % 2 == 0 {
        return Median::Even(mut_vec[idx - 1], mut_vec[idx]);
    } else {
        return Median::Odd(mut_vec[idx]);
    }
}

///Builds a BTreeMap histogram of a provided vector.
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
