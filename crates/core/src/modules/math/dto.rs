//! Data transfer objects and input validation for math operations.

const MAX_OPERAND_ABS: i64 = 1_000_000_000;

/// Request payload for addition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddRequest {
    /// Left addend.
    pub a: i64,
    /// Right addend.
    pub b: i64,
}

impl AddRequest {
    /// Validates operand bounds before domain execution.
    ///
    /// # Errors
    /// Returns an error string when an operand exceeds allowed bounds.
    pub const fn validate(self) -> Result<(), &'static str> {
        if self.a.saturating_abs() > MAX_OPERAND_ABS || self.b.saturating_abs() > MAX_OPERAND_ABS {
            return Err("operand exceeds safe template bounds");
        }
        Ok(())
    }
}

/// Response payload returned after addition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddResponse {
    /// Sum of `a` and `b`.
    pub sum: i64,
    /// Number of calculations recorded after this operation.
    pub history_count: usize,
}
