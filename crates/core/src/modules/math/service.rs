//! Business logic service for math operations.

use super::dto::{AddRequest, AddResponse};
use super::entity::CalculationEntry;
use super::repository::MathRepository;

/// Orchestrates validated addition and history persistence.
#[derive(Debug)]
pub struct MathService {
    repo: Box<dyn MathRepository>,
}

impl MathService {
    /// Constructs a service backed by a repository implementation.
    #[must_use]
    pub fn new(repo: Box<dyn MathRepository>) -> Self {
        Self { repo }
    }

    /// Validates, sums, and records an addition request.
    ///
    /// # Errors
    /// Returns an error if validation fails or integer overflow occurs.
    pub fn add(&mut self, req: AddRequest) -> Result<AddResponse, &'static str> {
        req.validate()?;
        let sum = req.a.checked_add(req.b).ok_or("integer overflow")?;
        self.repo.save(CalculationEntry {
            a: req.a,
            b: req.b,
            sum,
        });
        Ok(AddResponse {
            sum,
            history_count: self.repo.count(),
        })
    }
}
