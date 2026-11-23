//! Architecture boundary tests verifying crate isolation.

use std::fs;
use std::path::Path;

#[test]
fn core_never_depends_on_xtask() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let content = fs::read_to_string(manifest).expect("read core Cargo.toml");
    assert!(
        !content.contains("xtask"),
        "core crate must never depend on xtask"
    );
}

#[test]
fn workspace_enforces_forbid_unsafe_code() {
    let root_manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .join("Cargo.toml");
    let content = fs::read_to_string(root_manifest).expect("read workspace Cargo.toml");
    assert!(
        content.contains("unsafe_code = \"forbid\""),
        "workspace must forbid unsafe_code"
    );
}

#[test]
fn workspace_locks_immutable_lint_policy() {
    let root_manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .join("Cargo.toml");
    let content = fs::read_to_string(root_manifest).expect("read workspace Cargo.toml");

    assert!(
        !content.contains("unsafe_code = \"warn\"") && !content.contains("unsafe_code = \"allow\""),
        "unsafe_code must never be weakened to warn or allow"
    );

    let required_entries = [
        "rust-version = \"1.85\"",
        "unsafe_code = \"forbid\"",
        "missing_debug_implementations = \"warn\"",
        "missing_docs = \"warn\"",
        "unreachable_pub = \"warn\"",
        "rust_2018_idioms = { level = \"warn\", priority = -1 }",
        "all = { level = \"warn\", priority = -1 }",
        "pedantic = { level = \"warn\", priority = -1 }",
        "unwrap_used = \"warn\"",
        "expect_used = \"allow\"",
        "panic_in_result_fn = \"warn\"",
        "missing_panics_doc = \"warn\"",
        "missing_errors_doc = \"warn\"",
        "missing_assert_message = \"warn\"",
        "allow_attributes = \"warn\"",
        "allow_attributes_without_reason = \"warn\"",
        "undocumented_unsafe_blocks = \"warn\"",
        "multiple_unsafe_ops_per_block = \"warn\"",
        "fn_params_excessive_bools = \"warn\"",
        "struct_excessive_bools = \"warn\"",
        "clone_on_ref_ptr = \"warn\"",
        "redundant_clone = \"warn\"",
        "rc_buffer = \"warn\"",
        "exhaustive_enums = \"warn\"",
        "string_add = \"warn\"",
        "wildcard_imports = \"warn\"",
        "dbg_macro = \"warn\"",
        "todo = \"warn\"",
        "print_stdout = \"warn\"",
        "print_stderr = \"warn\"",
        "as_conversions = \"warn\"",
        "cast_possible_truncation = \"warn\"",
        "cast_sign_loss = \"warn\"",
    ];

    for entry in required_entries {
        assert!(
            content.contains(entry),
            "workspace Cargo.toml missing required lint policy entry: {entry}"
        );
    }
}

#[test]
fn workspace_locks_clippy_toml() {
    let clippy_toml = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .join("clippy.toml");
    let content = fs::read_to_string(clippy_toml).expect("read workspace clippy.toml");

    let required_entries = [
        "max-fn-params-bools = 0",
        "allow-unwrap-in-tests = true",
        "allow-expect-in-tests = true",
        "allow-print-in-tests = true",
    ];

    for entry in required_entries {
        assert!(
            content.contains(entry),
            "clippy.toml missing required policy setting: {entry}"
        );
    }
}
