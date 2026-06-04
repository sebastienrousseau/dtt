# Minimum Supported Rust Version (MSRV) Policy

## The rule

`dtt` supports the **stable Rust release that was current six months
ago**. Pre-1.0, the floor is exactly **`1.88.0`** — pinned by the
`time = 0.3.47` security upgrade, whose `time-core = =0.1.8`
transitive dependency uses Cargo's `edition2024` feature.

## How the floor moves

- The MSRV may be raised in any **MINOR** release.
- The MSRV is **not** raised in a PATCH release.
- An MSRV bump is documented under `### Changed` in `CHANGELOG.md`
  with a brief justification (typically: "required by upstream
  dependency X" or "uses stable feature Y").

## How the floor is enforced

| Layer | Mechanism |
|-------|-----------|
| `Cargo.toml` | `rust-version = "1.88.0"` field. Cargo refuses to compile on older toolchains. |
| `build.rs` | `version_check::is_min_version("1.88")` produces a clear error message. |
| CI matrix | Every PR runs the full test suite on the declared MSRV in addition to `stable`. See [`.github/workflows/cross-platform.yml`](../.github/workflows/cross-platform.yml). |
| CHANGELOG | Every MSRV bump is recorded under `### Changed`. |

## Why this floor

The MSRV is governed by the **most security-relevant upstream
dependency**, currently `time`:

- `time 0.3.47` carries the upstream fix for `RUSTSEC` stack-exhaustion
  DoS (affects `>=0.3.6, <0.3.47`, medium severity, Dependabot alert #3).
- `time 0.3.47` pins `time-core = =0.1.8`, which declares
  `rust-version = "1.88.0"` and uses Cargo's `edition2024` feature.
- No earlier `time` release contains the security fix; downgrading
  would reintroduce the vulnerability.

`dtt` also relies on `std::sync::LazyLock` (stable since 1.80.0) for the
timezone-offset table, which is already covered by the 1.88.0 floor.

## How a downstream user pins to a known-good MSRV

```toml
# In your project's Cargo.toml
[dependencies]
dtt = "0.1.0"            # already constrained to >= 1.88
```

Or, to lock to the lowest verified-working toolchain:

```bash
rustup install 1.88.0
cargo +1.88.0 build
```

## History of MSRV bumps

| Release | MSRV | Reason |
|---------|------|--------|
| `0.1.0` | `1.88.0` | `time 0.3.47` security upgrade pins `time-core =0.1.8` which uses `edition2024`. |
| `0.0.10` | `1.80.0` | `std::sync::LazyLock` stabilisation. |

## Future direction

Once `dtt` reaches `1.0.0`, the policy moves to **rolling
"stable − 6 months"**: every MINOR release floors at the stable
toolchain that was current six months prior to the release tag. This
matches the policy used by `tokio`, `serde`, and other ecosystem
anchor crates.
