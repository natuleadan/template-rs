//! Transport-facing controller for the math domain.

use super::dto::{AddRequest, AddResponse};
use super::service::MathService;

/// Entrypoint adapter delegating validated requests to `MathService`.
#[derive(Debug)]
pub struct MathController {
    service: MathService,
}

impl MathController {
    /// Constructs a controller from a configured `MathService`.
    #[must_use]
    pub const fn new(service: MathService) -> Self {
        Self { service }
    }

    /// Handles an addition request and returns the formatted response.
    ///
    /// # Errors
    /// Propagates validation or arithmetic errors from `MathService`.
    pub fn handle_add(&mut self, a: i64, b: i64) -> Result<AddResponse, &'static str> {
        self.service.add(AddRequest { a, b })
    }
}
