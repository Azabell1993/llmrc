#!/usr/bin/env bash
set -e

# Simple pipeline: clean -> build -> run with llm argument
# Usage: ./run.sh [mode] [arch]
#   mode: debug or release (default: release)
#   arch: arm64 (default) or x86_64

# Parse arguments
MODE="${1:-release}"
ARCH="${2:-arm64}"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_SH="${ROOT_DIR}/build.sh"

# Validate mode
case "$MODE" in
  debug|release) ;;
  arm64|x86_64) 
    # First argument is architecture, not mode
    ARCH="$MODE"
    MODE="release"
    ;;
  --help|-h)
    echo "Usage: $0 [mode] [arch]"
    echo "  mode: debug or release (default: release)"
    echo "  arch: arm64 (default) or x86_64"
    echo ""
    echo "Examples:"
    echo "  $0                    # release build, arm64"
    echo "  $0 debug              # debug build, arm64"
    echo "  $0 release x86_64     # release build, x86_64"
    echo "  $0 debug x86_64       # debug build, x86_64"
    exit 0
    ;;
  *) 
    echo "ERROR: Invalid mode: $MODE (use debug or release)"
    echo "   Use --help for usage information"
    exit 1
    ;;
esac

# Validate architecture
case "$ARCH" in
  arm64|x86_64) ARCH_FLAG="--${ARCH}" ;;
  *) echo "ERROR: Invalid arch: $ARCH (use arm64 or x86_64)"; exit 1 ;;
esac

# Check if build.sh exists
[[ -f "$BUILD_SH" ]] || { echo "ERROR: Missing build.sh"; exit 1; }

# Display mode information
if [[ "$MODE" == "debug" ]]; then
  echo "DEBUG Mode: Detailed logging enabled"
else
  echo "RELEASE Mode: Optimized build"
fi

echo "🧹 Cleaning..."
"$BUILD_SH" clean "$ARCH_FLAG"

if [[ "$MODE" == "debug" ]]; then
  echo "🔨 Building (Debug)..."
  "$BUILD_SH" debug "$ARCH_FLAG"
else
  echo "🔨 Building (Release)..."
  "$BUILD_SH" build "$ARCH_FLAG"
fi

echo "🏃 Running with llm argument..."
"$BUILD_SH" run "$ARCH_FLAG" llm