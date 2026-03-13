#!/usr/bin/env bash
# Build Linux release binaries and package as tarballs for GitHub Releases.
# Used by consumers (e.g. hachi-worker Dockerfile) with DVCLI_RELEASE_BASE_URL.
#
# Prerequisites (for Linux targets from macOS):
#   cargo install cross
#
# Usage:
#   ./scripts/build-release-binaries.sh [version]
#   If version is omitted, read from Cargo.toml.
#
# Output:
#   dist/dvcli-<version>-x86_64-unknown-linux-gnu.tar.gz
#   dist/dvcli-<version>-aarch64-unknown-linux-gnu.tar.gz
# Each tarball contains a single binary named "dvcli" at the top level.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# Version from arg or Cargo.toml
VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
  VERSION=$(grep -E '^version\s*=' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
  if [[ -z "$VERSION" ]]; then
    echo "Could not read version from Cargo.toml"
    exit 1
  fi
fi

TARGETS=(
  x86_64-unknown-linux-gnu
  aarch64-unknown-linux-gnu
)

if command -v cross &>/dev/null; then
  CARGO_CMD=cross
  echo "Using 'cross' for Linux targets"
else
  CARGO_CMD=cargo
  echo "Using 'cargo' (install 'cross' for Linux targets when not on Linux)"
fi

mkdir -p dist

for target in "${TARGETS[@]}"; do
  echo "Building $target..."
  $CARGO_CMD build --release --target "$target"

  binary="$REPO_ROOT/target/$target/release/dvcli"
  if [[ ! -f "$binary" ]]; then
    echo "Binary not found: $binary"
    exit 1
  fi

  tarball="dist/dvcli-${VERSION}-${target}.tar.gz"
  tmpdir=$(mktemp -d)
  cp "$binary" "$tmpdir/dvcli"
  (cd "$tmpdir" && tar czf "$REPO_ROOT/$tarball" dvcli)
  rm -rf "$tmpdir"

  echo "  -> $tarball"
done

echo ""
echo "Done. Upload to a GitHub release:"
echo "  gh release create v${VERSION} dist/dvcli-${VERSION}-*.tar.gz"
echo ""
echo "Consumers can then build with:"
echo "  docker build --build-arg DVCLI_RELEASE_BASE_URL=https://github.com/breskeby/develocity-cli-client/releases/download/v${VERSION} ..."
