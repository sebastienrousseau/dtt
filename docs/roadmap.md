# DTT Roadmap

This document records the project's planned work, its rejected work, and
the conventions that govern how new work is proposed. It is the
canonical reference for "is X on the roadmap?" questions.

## How to find planned work

Open issues are scoped to a single deliverable and tagged with a
milestone that names the target release:

| Milestone | Focus | Tracking issue |
|-----------|-------|----------------|
| [`v0.0.11`](https://github.com/sebastienrousseau/dtt/milestone/1) | IANA timezone database integration | [#115](https://github.com/sebastienrousseau/dtt/issues/115) |
| [`v0.0.12`](https://github.com/sebastienrousseau/dtt/milestone/2) | Datetime ranges and intervals | [#132](https://github.com/sebastienrousseau/dtt/issues/132) |
| [`v0.0.13`](https://github.com/sebastienrousseau/dtt/milestone/3) | RFC 2822 inbound parser | [#133](https://github.com/sebastienrousseau/dtt/issues/133) |
| [`v0.0.14`](https://github.com/sebastienrousseau/dtt/milestone/4) | Business day arithmetic + holiday calendar | [#134](https://github.com/sebastienrousseau/dtt/issues/134) |
| [`v0.0.15`](https://github.com/sebastienrousseau/dtt/milestone/5) | ISO 8601 extended formats (week-date, ordinal-date) | [#135](https://github.com/sebastienrousseau/dtt/issues/135) |
| [`v0.0.16`](https://github.com/sebastienrousseau/dtt/milestone/6) | Performance pass | [#136](https://github.com/sebastienrousseau/dtt/issues/136), [#137](https://github.com/sebastienrousseau/dtt/issues/137), [#138](https://github.com/sebastienrousseau/dtt/issues/138) |

Each linked issue carries a user story, ≥ 5 acceptance criteria, and a
Definition-of-Done checklist. See
[`.github/ISSUE_TEMPLATE/feature.md`](../.github/ISSUE_TEMPLATE/feature.md)
for the template every new feature request must follow.

## Won't fix — work that conflicts with current design choices

Each item below reverses an explicit decision shipped in v0.0.10. They
are **not on the roadmap** unless a clear new motivation emerges. New
issues advocating any of them must explain how to reconcile with the
listed constraint.

- ❌ **SIMD optimisations** — would require `unsafe`; the crate
  enforces `unsafe_code = "forbid"` crate-wide.
- ❌ **`chrono` compatibility layer** — would pull ~150 transitive
  crates; the v0.0.10 release explicitly moved toward minimal deps
  (optional `serde`, `paste`→`pastey`, dropped `lazy_static`).
- ❌ **SQL types (`sqlx` / `diesel`) integration** — out of scope at
  the crate level. Users can derive `serde` and use the RFC 3339
  string column type.
- ❌ **Web framework helpers (`axum` / `actix`)** — out of scope; same
  `serde`-based workaround.
- ❌ **FFI / C bindings** — would more naturally live in a separate
  `dtt-c` crate.
- ❌ **Leap second handling** — the upstream `time` crate does not
  model leap seconds; out of scope at the dependency layer.
- ❌ **Non-Gregorian calendar systems** (Julian, Hebrew, Islamic, …) —
  entirely new types; out of scope for a Gregorian-focused crate.
- ❌ **Locale-aware formatting** — would pull `icu` (~50 transitive
  crates). Use the existing `format` with explicit format descriptors.
- ❌ **Natural-language duration parsing** — out of scope.
- ❌ **Astronomical datetime calculations** — out of scope.

## Process

- File new work using
  [the feature template](../.github/ISSUE_TEMPLATE/feature.md).
- Increment **version** by `0.0.1` per scoped release pre-1.0
  (e.g. `v0.0.11`, `v0.0.12`, …). Bump the minor digit only when a
  feature changes the public API or carries a deliberate MSRV move
  outside the security exception.
- MSRV bumps are governed by
  [`docs/msrv-policy.md`](msrv-policy.md). They are permitted in any
  release, including PATCH, when required to absorb an upstream
  security advisory; otherwise only in MINOR or MAJOR releases.
- Every PR closes its issue. Every commit is cryptographically signed
  (`git commit -S`).
- The full delivered surface of each release is documented in
  [`CHANGELOG.md`](../CHANGELOG.md).
