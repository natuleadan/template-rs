# Project Template (CHANGE ME)

Production-ready Rust workspace template featuring a pure-Cargo `xtask` validation suite, layered modular domain architecture, strict Git hygiene, supply-chain cooldown enforcement, and automated GitHub Actions pipelines.

## Key Features

- **100% Native Rust Tooling**: Powered by Cargo (`crates/xtask`) on **Rust `1.85+`** (2024 edition) with zero Node.js, `pnpm`, or external script dependencies.
- **Strict Workspace Lint Policy**: `unsafe_code = "forbid"`, strict `[workspace.lints]` rules, and `clippy.toml` (`max-fn-params-bools = 0`) locked by immutable tripwires in `crates/core/tests/architecture.rs`.
- **Layered Modular Architecture**: Reference domain module (`crates/core/src/modules/math/`) separating `entity`, `dto`, `repository`, `service`, `controller`, and `mod` layers with automated boundary tests.
- **Eight Standardized Validation Gates**: `lint`, `typecheck`, `unit tests`, `integration and architecture tests`, `cooldown`, `hygiene`, `notices`, and `release`.
- **Supply-Chain Hardening**: 3-day dependency publish-age cooldown (`resolver.min-publish-age="3 days"`), CodeQL static analysis, and automated SBOM/dependency inventory artifacts.
- **Zero Formatter Drift**: Automated formatters (`cargo fmt`, `prettier`) are intentionally excluded from hooks and CI so diffs stay focused on logic.

## Repository Layout

```text
.
├── .githooks/                   # Versioned commit-msg, pre-commit, and pre-push hooks
├── .github/
│   ├── CODEOWNERS               # Code ownership rules (CHANGE ME)
│   ├── dependabot.yml           # Cargo and GitHub Actions updates with 3-day cooldown
│   └── workflows/               # validate.yml, nightly-tag.yml, security.yml
├── crates/
│   ├── core/
│   │   ├── src/modules/math/      # Layered domain module (entity, dto, repository, service, controller, mod)
│   │   ├── tests/architecture.rs  # Crate isolation and immutable lint/MSRV policy tripwires
│   │   └── tests/integration.rs   # End-to-end integration and layer boundary tripwires
│   └── xtask/                     # Native automation CLI (cargo xtask <cmd>)
├── Cargo.toml                     # Workspace manifest (rust-version = "1.85", [workspace.lints])
├── clippy.toml                    # Strict Clippy settings (max-fn-params-bools = 0)
├── ARCHITECTURE.md                # Modular architecture, lint policy, and boundary guide
├── AGENTS.md                    # Autonomous agent operating rules
├── CONTRIBUTING.md              # Contributor workflow and commit grammar
├── SECURITY.md                  # Supply-chain controls and vulnerability reporting
├── LICENSE                      # Apache-2.0 License (Copyright (c) 2026 CHANGE_ME)
└── ThirdPartyNotices.txt        # Generated dependency notice inventory
```

## Quickstart

Activate the repository hooks and run the full 8-gate validation suite:

```sh
git config core.hooksPath .githooks
cargo xtask validate
```

Other `xtask` commands:

```sh
cargo xtask notices          # Regenerate ThirdPartyNotices.txt from cargo metadata
cargo xtask release --check  # Verify Cargo.toml version matches CHANGELOG.md
```

## Validation Gates

| Gate | Implementation | Purpose |
| :--- | :--- | :--- |
| `lint` | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Strict zero-warning static lints (`[workspace.lints]` + `clippy.toml`) |
| `typecheck` | `cargo check --workspace --all-targets` | Workspace type verification |
| `unit tests` | `cargo test --workspace --lib` | Fast unit test execution |
| `integration and architecture tests` | `cargo test --workspace --test '*' --doc` | Integration flows and layer boundaries |
| `cooldown` | `cargo --config 'resolver.min-publish-age="3 days"'` | 3-day package release cooldown |
| `hygiene` | `crates/xtask/src/hygiene.rs` | 500-line limit, anti-leak, shell syntax |
| `notices` | `crates/xtask/src/notices.rs` | License inventory parity |
| `release` | `crates/xtask/src/release.rs` | SemVer and `CHANGELOG.md` consistency |

## Branching & Release Workflow

- **`dev`**: Active integration branch protected against deletion. Every push updates the rolling `dev` tag, `dev-<short_sha>` tag, and `dev` pre-release snapshot.
- **`main`**: Protected release branch. Accepts rebase-only pull requests from `dev` after all status checks pass. Merging triggers automatic `vX.Y.Z` tagging, GitHub Release creation, and `dev` fast-forward sync.

## Template Customization (CHANGE ME)

When bootstrapping a new repository from this template, update the following placeholders:

- **`README.md`**: Replace `# Project Template (CHANGE ME)` with your project title and description.
- **`LICENSE` and `NOTICE`**: Replace `Copyright (c) 2026 CHANGE_ME` with your copyright holder.
- **`.github/CODEOWNERS`**: Uncomment and set your GitHub handle or team (`* @your-handle`, CHANGE ME).
- **Package Manifest**: Update `[workspace.package]` and `crates/core/Cargo.toml` package `name` (`CHANGE ME`).
- **Git Identity**: Configure `git config user.name` and a GitHub noreply address (`<id>+<handle>@users.noreply.github.com`) locally before committing.

## License

Licensed under the [Apache-2.0 License](LICENSE). Copyright (c) 2026 CHANGE_ME.
