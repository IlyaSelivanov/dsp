mod statistics;

pub use crate::signal::statistics::*;

pub struct Signal {
    data: Vec<i32>,
    is_sorted: bool,
    mean: f64,
    variance: f64,
    standard_deviation: f64,
    median: Median,
}

impl Signal {
    /// Creates a new [`Signal`].
    pub fn new() -> Self {
        Signal {
            data: Vec::new(),
            is_sorted: false,
            mean: 0f64,
            standard_deviation: 0f64,
            variance: 0f64,
            median: Median::None,
        }
    }

    /// Creates a new [`Signal`] with data provided.
    pub fn form_data(mut data: Vec<i32>) -> Self {
        if data.is_empty() {
            panic!("Not empty data needed.")
        }

        let mean = calc_vector_mean(&data);
        let variance = calc_vector_variance(&data);
        let med = match get_median(&mut data) {
            Median::Even(f, s) => Median::Even(f, s),
            Median::Odd(m) => Median::Odd(m),
            _ => Median::None,
        };

        Signal {
            data: data,
            is_sorted: true,
            mean: mean,
            standard_deviation: variance.sqrt(),
            variance: variance,
            median: med,
        }
    }

    /// Returns a reference to the data of this [`Signal`].
    pub fn data(&self) -> &[i32] {
        self.data.as_ref()
    }

    /// Returns if this [`Signal`] is sorted.
    pub fn is_sorted(&self) -> bool {
        self.is_sorted
    }

    /// Returns the mean of this [`Signal`].
    pub fn mean(&self) -> f64 {
        self.mean
    }

    /// Returns the variance of this [`Signal`].
    pub fn variance(&self) -> f64 {
        self.variance
    }

    /// Returns the standard deviation of this [`Signal`].
    pub fn standard_deviation(&self) -> f64 {
        self.standard_deviation
    }

    /// Returns a reference to the median of this [`Signal`].
    pub fn median(&self) -> &Median {
        &self.median
    }

    /// Prints info of this [`Signal`].
    pub fn print_info(&self) {
        println!("signal data length: {}", self.data().len());
        println!("signal sorted: {}", self.is_sorted());
        println!("signal mean: {:.3}", self.mean());
        println!("signal variance: {:.3}", self.variance());
        println!("signal st. deviation: {:.3}", self.standard_deviation());
        match self.median() {
            Median::Even(first, second) => println!("signal median = {first}, {second}"),
            Median::Odd(median) => println!("signal median = {median}"),
            _ => println!("signal median = empty"),
        }
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self::new()
    }
}
