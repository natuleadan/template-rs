//! Pure domain entity for math calculation history.

/// Recorded addition operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalculationEntry {
    /// Left addend.
    pub a: i64,
    /// Right addend.
    pub b: i64,
    /// Resulting sum.
    pub sum: i64,
}
