#!/usr/bin/env sh
set -eu

# Cross-platform git hook installer for DTT.
#
# Installs a pre-commit hook that:
#   1. Verifies the developer has configured signed commits
#      (commit.gpgsign = true and a user.signingkey set).
#   2. Runs `cargo fmt --check` so unformatted code never lands.
#
# Works identically on macOS, Linux, and WSL.

ROOT_DIR=$(git rev-parse --show-toplevel 2>/dev/null) || {
    printf 'Error: not inside a git repository.\n' >&2
    exit 1
}

HOOK_DIR="$ROOT_DIR/.git/hooks"
HOOK_FILE="$HOOK_DIR/pre-commit"

mkdir -p "$HOOK_DIR"

cat > "$HOOK_FILE" <<'HOOK'
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

# Run a fast formatting sanity check before allowing the commit through.
exec cargo fmt --all -- --check
HOOK

chmod +x "$HOOK_FILE"

printf '✅ Installed pre-commit hook at %s\n' "$HOOK_FILE"
printf '   It checks signed-commit config and runs cargo fmt --check.\n'
