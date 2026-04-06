#!/usr/bin/env sh
set -eu

# Cross-platform git hook installer for DTT.
#
# Installs two hooks:
#   1. pre-commit
#      - Verifies the developer has configured signed commits.
#      - Runs `cargo fmt --check` for fast feedback.
#   2. pre-push
#      - Re-verifies signed-commit configuration.
#      - Runs the full quality gate (`make verify`) so broken code
#        never leaves the developer's machine.
#
# Works identically on macOS, Linux, and WSL.

ROOT_DIR=$(git rev-parse --show-toplevel 2>/dev/null) || {
    printf 'Error: not inside a git repository.\n' >&2
    exit 1
}

HOOK_DIR="$ROOT_DIR/.git/hooks"
mkdir -p "$HOOK_DIR"

# ---------------------------------------------------------------------------
# pre-commit
# ---------------------------------------------------------------------------
PRE_COMMIT="$HOOK_DIR/pre-commit"
cat > "$PRE_COMMIT" <<'HOOK'
#!/usr/bin/env sh
# DTT pre-commit hook — enforces signed commits + clean formatting.

set -eu

if [ "$(git config --get commit.gpgsign 2>/dev/null || true)" != "true" ]; then
    printf '\n❌ commit.gpgsign is not enabled.\n'
    printf '   Run:  git config --global commit.gpgsign true\n'
    printf '   See:  CONTRIBUTING.md § 2 (Signed Commits)\n\n'
    exit 1
fi

if [ -z "$(git config --get user.signingkey 2>/dev/null || true)" ]; then
    printf '\n❌ user.signingkey is not configured.\n'
    printf '   See CONTRIBUTING.md § 2 for SSH or GPG setup.\n\n'
    exit 1
fi

# Fast formatting check before allowing the commit through.
exec cargo fmt --all -- --check
HOOK
chmod +x "$PRE_COMMIT"

# ---------------------------------------------------------------------------
# pre-push
# ---------------------------------------------------------------------------
PRE_PUSH="$HOOK_DIR/pre-push"
cat > "$PRE_PUSH" <<'HOOK'
#!/usr/bin/env sh
# DTT pre-push hook — full quality gate before code leaves the machine.
#
# Re-checks signed-commit config and runs the same checks CI will run
# (fmt-check + clippy + test). Aborts the push on any failure.

set -eu

if [ "$(git config --get commit.gpgsign 2>/dev/null || true)" != "true" ]; then
    printf '\n❌ commit.gpgsign is not enabled. Refusing to push.\n'
    printf '   See CONTRIBUTING.md § 2.\n\n'
    exit 1
fi

# Verify the most recent commit on HEAD is signed.
if ! git verify-commit HEAD >/dev/null 2>&1; then
    printf '\n❌ HEAD is not a signed commit. Refusing to push.\n'
    printf '   Re-create your commit with `git commit -S` (or amend it).\n\n'
    exit 1
fi

printf '🔐 Signed-commit check passed.\n'
printf '🔬 Running full quality gate (this is what CI will run)…\n\n'

# `make verify` runs fmt-check + clippy + test. Falls back to raw
# cargo invocations if `make` is not installed (e.g. minimal Windows).
if command -v make >/dev/null 2>&1; then
    exec make verify
else
    cargo fmt --all -- --check
    cargo clippy --all-targets -- -D warnings
    cargo test --all-targets
fi
HOOK
chmod +x "$PRE_PUSH"

printf '✅ Installed pre-commit hook at %s\n' "$PRE_COMMIT"
printf '✅ Installed pre-push   hook at %s\n' "$PRE_PUSH"
printf '\n'
printf 'pre-commit: signed-commit config + cargo fmt --check\n'
printf 'pre-push  : signed-commit config + verify HEAD signature + make verify\n'
