# Software Validation Procedure (SVP)

**Scope.** This procedure governs the validation of the `dtt` crate
(DateTime library) as Software of Unknown Pedigree (SOUP) component
under **ISO 13485 § 4.1.6**, **§ 7.3**, and **§ 7.5.6**.

**Objective.** Provide a deterministic, repeatable validation routine
that produces evidence suitable for regulated medical-device audits.

## 1. Software Identification

| Field | Value |
|-------|-------|
| Crate name | `dtt` |
| Versioning | Semantic Versioning 2.0.0 |
| Primary source | <https://github.com/sebastienrousseau/dtt> |
| Build system | Cargo |
| Minimum Supported Rust Version | 1.80.0 |
| Licence | Apache-2.0 OR MIT |
| Cryptographic signing | SSH/Ed25519 or GPG per commit (`git commit -S`) |

## 2. Runtime SOUP Inventory

Runtime dependencies shipped to consumers:

| Crate | Licence | Purpose |
|-------|---------|---------|
| `pastey` | Apache-2.0 OR MIT | Proc-macro for identifier concatenation |
| `serde` | Apache-2.0 OR MIT | Generic (de)serialisation framework |
| `serde_json` | Apache-2.0 OR MIT | JSON (de)serialisation |
| `thiserror` | Apache-2.0 OR MIT | Derive macro for `std::error::Error` |
| `time` | Apache-2.0 OR MIT | Calendar arithmetic primitives |

The full transitive graph is recorded in `Cargo.lock` and is fully
reproducible. Regeneration via `cargo update` is a controlled change
governed by § 5 of this procedure.

## 3. Validation Preconditions

Before any validation run:

1. Working tree matches a signed commit on a feature branch.
2. `git log --show-signature HEAD` reports `Good "git" signature`.
3. Toolchain version matches the MSRV declared in `Cargo.toml`
   (verify with `rustc --version`).

## 4. Validation Routine

The single canonical validation command is:

```bash
make verify
```

This executes, in order:

1. `cargo fmt --all -- --check` — formatting parity with repository style.
2. `cargo clippy --all-targets -- -D warnings` — static analysis with
   pedantic/nursery/cargo lint groups and `unsafe_code = forbid`.
3. `cargo test --all-targets` — unit tests, integration tests,
   property-based tests, and documentation tests.

**Acceptance criteria.**

| Check | Pass condition |
|-------|----------------|
| Formatter | zero diff |
| Clippy | zero warnings |
| Unit + integration tests | ≥ 260 tests, 0 failures |
| Property tests | 5 properties, each ≥ 256 cases, 0 counter-examples |
| Doc tests | 0 failures |

## 5. Continuous Validation in CI

Every push to `feat/**` and every PR to `main` triggers
`.github/workflows/cross-platform.yml`, which executes:

- **Test matrix:** `ubuntu-latest`, `macos-latest`, `windows-latest`
  × Rust `stable` + `1.80.0` MSRV (6 combinations).
- **Coverage gate:** `cargo llvm-cov` with `--fail-under-lines 99
  --fail-under-functions 100`.
- **Security gate:** `cargo audit --deny warnings` against the
  RustSec advisory database.

Merges to `main` require:

- At least one approving review (branch protection).
- Signed commits (`required_signatures = true`).
- All CI checks green (matrix + coverage + audit).

## 6. Risk Controls

| Risk | Control | Evidence |
|------|---------|---------|
| Silent data loss through format/parse drift | Property test `rfc3339_round_trip` + unit tests | `tests/test_properties.rs`, `make verify` |
| Timezone ambiguity producing wrong-answer bugs | Disambiguated IANA-style suffixes (`EST_USA`, `IST_INDIA`, …) | `src/datetime.rs` `TIMEZONE_OFFSETS` |
| Integer overflow in calendar arithmetic | `checked_*` chains on all year/month math | `src/datetime.rs::add_months`, `add_years` |
| Mixed-sign offset coercion | Explicit rejection in `new_with_custom_offset` | `src/datetime.rs` + `test_new_with_custom_offset_mixed_signs_rejected` |
| Non-deterministic `Default` breaking tests | `Default::default()` returns Unix epoch | `src/datetime.rs::Default for DateTime` |
| Unmaintained or vulnerable third-party crates | CI `cargo audit` gate; `deny.toml` licence allowlist | `.github/workflows/cross-platform.yml::audit` |
| Cross-platform drift (CRLF, path case, shell syntax) | `.gitattributes` enforces LF; POSIX shell scripts; Makefile | `.gitattributes`, `tools/*.sh`, `Makefile` |
| Unauthorised code entering `main` | Server-side `required_signatures`, branch protection | GitHub branch protection API |

## 7. Records

Each validation run produces the following records:

| Artefact | Location |
|----------|----------|
| Signed commit history | `git log --show-signature` |
| Test results | `cargo test` exit code + stdout |
| Coverage report | `cargo llvm-cov --summary-only` |
| Dependency audit | `cargo audit` exit code + stdout |
| SBOM source | `Cargo.lock` (checked into the repo) |
| Change log | `CHANGELOG.md` (Keep a Changelog format) |

## 8. Change Control

All changes to this procedure must be submitted as a signed pull
request amending this file and must be approved by the project
maintainer prior to merge.

## 9. Review Cycle

This procedure is reviewed on every MAJOR release and at least
annually.
