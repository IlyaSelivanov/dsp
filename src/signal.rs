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

        let statistics = Statistics::from_vector(&vector);

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

#[cfg(test)]
mod signal_tests {
    use super::*;

    #[test]
    fn create_new_signal_data_is_empty() {
        let signal = Signal::new();

        assert_eq!(0, signal.data.len());
    }

    #[test]
    fn create_new_signal_statistics_is_empty() {
        let signal = Signal::new();
        let statistics = Statistics::new();

        assert_eq!(statistics, signal.statistics);
    }

    #[test]
    #[should_panic]
    fn create_signal_from_empty_vector() {
        let vector: Vec<i64> = Vec::new();
        let _ = Signal::from_vector(vector);
    }

    #[test]
    fn create_signal_from_vector() {
        let vector = vec![1, 2, 3];
        let signal = Signal::from_vector(vector);

        assert_eq!(3, signal.data.len());
    }
}
