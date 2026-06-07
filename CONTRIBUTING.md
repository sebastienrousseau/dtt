# Contributing to `DateTime (DTT)`

Welcome! This guide explains how to set up your environment, follow our git
conventions, and submit a pull request that lands cleanly.

## Table of Contents

1. [Quick Setup](#1-quick-setup)
2. [Signed Commits (required)](#2-signed-commits-required)
3. [Commit Message Convention](#3-commit-message-convention)
4. [Pull Request Checklist](#4-pull-request-checklist)
5. [Project Layout](#5-project-layout)
6. [Reporting Bugs & Feature Requests](#6-reporting-bugs--feature-requests)
7. [Code of Conduct](#7-code-of-conduct)

---

## 1. Quick Setup

```bash
# Clone (use SSH if you have it set up)
git clone https://github.com/sebastienrousseau/dtt.git
cd dtt

# Install Rust 1.80+ (skip if already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Verify everything builds and tests cleanly (one command, all 3 OSes)
make verify
```

The `make verify` target runs `cargo fmt --check`, `cargo clippy
--all-targets -- -D warnings`, and `cargo test --all-targets`.
Run `make help` for the full list of tasks.

If `make` is unavailable, the equivalent raw commands are:

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Platform notes:

- **macOS:** prefer `brew install rustup-init && rustup-init -y`.
- **Linux / WSL:** the `curl ... | sh` command above works unchanged.
- **Windows:** download `rustup-init.exe` from [rustup.rs](https://rustup.rs/).

If your `cargo build` fails with a `LazyLock` error, run `rustup update stable`.

### Optional: install the signed-commit pre-commit hook

```bash
make install-hooks
```

This installs a portable `pre-commit` hook (POSIX shell, works on
macOS/Linux/WSL) that fails the commit if `commit.gpgsign` or
`user.signingkey` is not configured, and also runs `cargo fmt --check`.

---

## 2. Signed Commits (required)

**All commits to `main` and to any feature branch must be cryptographically signed.**
Unsigned commits will be rejected by branch protection. We accept SSH-signed
or GPG-signed commits.

### One-time setup — SSH signing (recommended)

```bash
git config --global gpg.format ssh
git config --global user.signingkey ~/.ssh/id_ed25519.pub
git config --global commit.gpgsign true
```

The same SSH key can be uploaded to GitHub under
**Settings → SSH and GPG keys → New SSH key → Key type: Signing Key**.

### One-time setup — GPG signing

```bash
# Generate a key if you don't have one
gpg --full-generate-key

# Tell git to use it
git config --global user.signingkey <YOUR_GPG_KEY_ID>
git config --global commit.gpgsign true
```

Upload the public key to GitHub under
**Settings → SSH and GPG keys → New GPG key**.

### On every commit

```bash
git commit -S -m "feat(dtt): your message"
```

Verify locally:

```bash
git log --show-signature -1
# expect: Good "git" signature for <your-email>
```

---

## 3. Commit Message Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```text
<type>(dtt): <short description>

<optional longer body explaining motivation, not implementation>
```

Common types:

| Type | When to use |
|------|-------------|
| `feat` | New user-facing functionality |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `refactor` | Code change that does not alter behavior |
| `test` | Adding or updating tests |
| `chore` | Tooling, dependencies, repo hygiene |
| `ci` | Changes to GitHub Actions / CI |
| `build` | Build-system or `Cargo.toml` changes |
| `perf` | Performance improvements |

Example:

```text
fix(dtt): reject mixed-sign offsets in new_with_custom_offset

The time crate silently coerces (5, -30) to +05:30, which is rarely
the caller's intent. Validate that hours and minutes share a sign
(unless one is zero) before constructing the offset.
```

---

## 4. Pull Request Checklist

Before opening a PR, run locally:

```bash
make verify        # one command: fmt-check + lint + test
cargo doc --no-deps
```

Your PR should:

- [ ] Have a clear title following Conventional Commits.
- [ ] Be based on an up-to-date `main`.
- [ ] Pass `cargo test` (240+ tests should be green).
- [ ] Pass `cargo clippy --all-targets -- -D warnings`.
- [ ] Pass `cargo fmt --check`.
- [ ] Include tests for any new behavior.
- [ ] Update `README.md` and rustdoc if the public API changes.
- [ ] Update `CHANGELOG.md` under `## [Unreleased]` if user-visible.
- [ ] All commits signed (`git log --show-signature` shows `Good signature`).

PRs that change public API must include a rationale for the change in the
description, and ideally a benchmark or test demonstrating the improvement.

---

## 5. Project Layout

| Path | Purpose |
|------|---------|
| `src/lib.rs` | Crate root, re-exports, `run()` entry point |
| `src/datetime.rs` | `DateTime`, `DateTimeBuilder`, validators, calendar helpers |
| `src/error.rs` | `AppError`, `DateTimeError` |
| `src/macros.rs` | User-facing `dtt_*` macros |
| `src/main.rs` | CLI binary entry point |
| `tests/` | Integration tests (one file per concern) |
| `benches/criterion.rs` | Criterion micro-benchmarks |
| `examples/dtt.rs` | End-to-end usage demo |
| `build.rs` | MSRV check |
| `tools/check_dependencies.sh` | Helper script that flags unused deps in `Cargo.toml` |
| `tools/install-hooks.sh` | Installs the signed-commit pre-commit hook (cross-platform) |
| `Makefile` | Cross-platform task runner (`make verify`, `make help`) |
| `.gitattributes` | Enforces LF line endings on shell, Rust, and config files |
| `.github/workflows/ci.yml` | Reusable CI pipeline (lint, test, coverage, security) |
| `.github/workflows/cross-platform.yml` | macOS/Linux/Windows test matrix on stable + MSRV |
| `deny.toml` | `cargo-deny` policy (licenses, advisories, bans) |
| `rustfmt.toml` | Formatting rules |
| `.deepsource.toml` | DeepSource analyzer config |

To check whether any declared dependency is unused:

```bash
./tools/check_dependencies.sh
```

---

## 6. Reporting Bugs & Feature Requests

Open an issue at <https://github.com/sebastienrousseau/dtt/issues/new>.

### Bug reports should include

- A short description of the expected vs. actual behavior
- A minimal reproduction (ideally a failing `#[test]`)
- Output of `rustc --version` and `cargo --version`
- Operating system

### Feature requests should include

- The use case (what are you trying to do?)
- Why existing APIs are insufficient
- Any alternatives you've considered

---

## 7. Code of Conduct

This project follows the [Code of Conduct](.github/CODE-OF-CONDUCT.md).
By participating, you agree to uphold it.

---

[1]: https://github.com/sebastienrousseau/dtt
[2]: https://github.com/sebastienrousseau/dtt/issues/new
