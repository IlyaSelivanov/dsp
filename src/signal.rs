mod statistics;

pub use crate::signal::statistics::*;

pub struct Signal {
    data: Vec<i64>,
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
    pub fn from_vector(vector: Vec<i64>) -> Self {
        if vector.is_empty() {
            panic!("Not empty data needed.")
        }

        let statistics = Statistics::form_vector(&vector);

        Signal {
            data: vector,
            statistics: statistics,
        }
    }

    /// Returns a reference to the data of this [`Signal`].
    pub fn data(&self) -> &[i64] {
        self.data.as_ref()
    }

    /// Returns a reference to the statistics of this [`Signal`].
    pub fn statistics(&self) -> &Statistics {
        &self.statistics
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self::new()
    }
}
