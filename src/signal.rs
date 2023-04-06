mod statistics;

pub use crate::signal::statistics::*;

pub struct Signal {
    data: Vec<i32>,
    statistics: Statistics,
}

impl Signal {
    /// Creates a new [`Signal`].
    pub fn new() -> Self {
        Signal {
            data: Vec::new(),
            statistics: Statistics::new(),
        }
    }

    /// Creates a new [`Signal`] with data provided.
    pub fn from_i32_vector(vector: Vec<i32>) -> Self {
        if vector.is_empty() {
            panic!("Not empty data needed.")
        }

        let statistics = Statistics::form_i32_vector(&vector);

        Signal {
            data: vector,
            statistics: statistics,
        }
    }

    /// Returns a reference to the data of this [`Signal`].
    pub fn data(&self) -> &[i32] {
        self.data.as_ref()
    }

    /// Prints info of this [`Signal`].
    pub fn print_info(&self) {
        println!("signal data length: {}", self.data().len());
        println!("signal mean: {:.3}", self.statistics.mean());
        println!("signal variance: {:.3}", self.statistics.variance());
        println!("signal st. deviation: {:.3}", self.statistics.standard_deviation());
        match self.statistics.median() {
            Median::Even(first, second) => println!("signal median = {first}, {second}"),
            Median::Odd(median) => println!("signal median = {median}"),
            _ => println!("signal median = empty"),
        };
        println!("signal histogram: {:?}", self.statistics.histogram());
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self::new()
    }
}
