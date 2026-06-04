# DTT — cross-platform task runner.
#
# Works identically on macOS, Linux, and WSL with any POSIX-compliant `make`.
# Run `make help` for the command list.

.PHONY: help all build test lint fmt fmt-check doc bench example verify coverage audit sbom clean install-hooks check-deps

help:
	@echo "DTT — common tasks (works on macOS, Linux, WSL)"
	@echo ""
	@echo "  make build         Build the library and binary"
	@echo "  make test          Run all tests (240+)"
	@echo "  make lint          cargo clippy --all-targets -- -D warnings"
	@echo "  make fmt           cargo fmt"
	@echo "  make fmt-check     cargo fmt --check"
	@echo "  make doc           cargo doc --no-deps --open"
	@echo "  make bench         cargo bench"
	@echo "  make example       cargo run --example dtt"
	@echo "  make verify        fmt-check + lint + test (PR-ready gate)"
	@echo "  make coverage      Run cargo llvm-cov and print summary"
	@echo "  make audit         Run cargo-audit against RustSec database"
	@echo "  make sbom          Generate CycloneDX SBOM at dtt-sbom.cdx.json"
	@echo "  make check-deps    Run tools/check_dependencies.sh"
	@echo "  make install-hooks Install signed-commit pre-commit hook"
	@echo "  make clean         cargo clean"

all: verify

build:
	cargo build --all-targets

test:
	cargo test --all-targets

lint:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

doc:
	cargo doc --no-deps --open

bench:
	cargo bench

example:
	cargo run --example dtt

verify: fmt-check lint test
	@echo ""
	@echo "✅ Ready to commit. Don't forget: git commit -S"

coverage:
	@command -v cargo-llvm-cov >/dev/null 2>&1 || { \
	    echo "Installing cargo-llvm-cov…"; \
	    cargo install cargo-llvm-cov; \
	}
	cargo llvm-cov --summary-only

audit:
	@command -v cargo-audit >/dev/null 2>&1 || { \
	    echo "Installing cargo-audit…"; \
	    cargo install cargo-audit; \
	}
	cargo-audit audit --deny warnings

sbom:
	@command -v cargo-cyclonedx >/dev/null 2>&1 || { \
	    echo "Installing cargo-cyclonedx…"; \
	    cargo install cargo-cyclonedx; \
	}
	cargo cyclonedx --format json --override-filename dtt-sbom
	@mv -f dtt-sbom.json dtt-sbom.cdx.json
	@echo "✅ SBOM written to dtt-sbom.cdx.json"

check-deps:
	./tools/check_dependencies.sh

install-hooks:
	./tools/install-hooks.sh

clean:
	cargo clean
