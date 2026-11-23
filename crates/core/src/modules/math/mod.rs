//! Module container wiring the math domain layers.

pub mod controller;
pub mod dto;
pub mod entity;
pub mod repository;
pub mod service;

use controller::MathController;
use repository::InMemoryMathRepository;
use service::MathService;

/// Assembled math module exposing the domain controller.
#[derive(Debug)]
pub struct MathModule {
    /// Controller handling math domain operations.
    pub controller: MathController,
}

impl MathModule {
    /// Wires `InMemoryMathRepository` -> `MathService` -> `MathController`.
    #[must_use]
    pub fn new() -> Self {
        let repo = Box::new(InMemoryMathRepository::new());
        let service = MathService::new(repo);
        let controller = MathController::new(service);
        Self { controller }
    }
}

impl Default for MathModule {
    fn default() -> Self {
        Self::new()
    }
}
