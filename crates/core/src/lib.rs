//! Minimal workspace core library with layered modular domains.

pub mod modules;

/// Returns the crate identifier for smoke verification.
#[must_use]
pub const fn crate_name() -> &'static str {
    "template-core"
}

#[cfg(test)]
mod tests {
    use super::crate_name;
    use super::modules::math::MathModule;

    #[test]
    fn returns_expected_name() {
        assert_eq!(crate_name(), "template-core");
    }

    #[test]
    fn math_module_adds_and_tracks_history() {
        let mut module = MathModule::new();
        let res = module.controller.handle_add(15, 27).expect("valid addition");
        assert_eq!(res.sum, 42);
        assert_eq!(res.history_count, 1);
    }
}
