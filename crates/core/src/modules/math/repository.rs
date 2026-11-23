//! Persistence port and in-memory adapter for math calculations.

use super::entity::CalculationEntry;

/// Storage abstraction for calculation records.
pub trait MathRepository: std::fmt::Debug {
    /// Persists a completed calculation entry.
    fn save(&mut self, entry: CalculationEntry);
    /// Returns the total number of stored entries.
    fn count(&self) -> usize;
    /// Returns all stored calculation entries in insertion order.
    fn list(&self) -> &[CalculationEntry];
}

/// Thread-local in-memory calculation store.
#[derive(Debug, Default)]
pub struct InMemoryMathRepository {
    entries: Vec<CalculationEntry>,
}

impl InMemoryMathRepository {
    /// Creates an empty in-memory calculation repository.
    #[must_use]
    pub const fn new() -> Self {
        Self { entries: Vec::new() }
    }
}

impl MathRepository for InMemoryMathRepository {
    fn save(&mut self, entry: CalculationEntry) {
        self.entries.push(entry);
    }

    fn count(&self) -> usize {
        self.entries.len()
    }

    fn list(&self) -> &[CalculationEntry] {
        &self.entries
    }
}
