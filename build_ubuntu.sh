#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

# ==========================
#   Rust (staticlib) + C++
#   Build & Run Orchestrator (Ubuntu)
# ==========================

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="$ROOT_DIR/cpp-app"
BUILD_DIR="$SRC_DIR/build"
BIN_NAME="llmrcpp_app"
APP_EXE="${ROOT_DIR}/output/bin/${BIN_NAME}"

# -------- UI helpers --------
BOLD=$'\033[1m'; GRN=$'\033[32m'; RED=$'\033[31m'; YLW=$'\033[33m'; CYA=$'\033[36m'; RST=$'\033[0m'

usage() {
  cat <<USAGE
${BOLD}${CYA}LLM Rust - Ubuntu Build & Model Management System${RST}
${BOLD}===================================================${RST}

${BOLD}SYNOPSIS${RST}
    $0 [COMMAND] <ARCH> [ADDITIONAL_ARGS...]

${BOLD}COMMANDS${RST}
    ${GRN}build${RST}     Run CMake configure and build (Rust build auto-triggered)
    ${GRN}debug${RST}     Build in debug mode (symbols/logs enabled)
    ${GRN}run${RST}       Run built binary (can pass additional args)
    ${GRN}clean${RST}     Clean CMake/Rust/output
    ${GRN}reconfig${RST}  Reconfigure CMake cache only
    ${GRN}fresh${RST}     Delete build dir and reconfigure

${BOLD}ARCH FLAGS${RST}
    ${YLW}--x86_64${RST}  Host or cross build: x86_64-unknown-linux-gnu
    ${YLW}--arm64${RST}   Host or cross build: aarch64-unknown-linux-gnu

${BOLD}ENV VARS (same)${RST}
    MODEL_PATH, DEFAULT_MODEL, MODELS_DIR, PREFER_QUANTIZED, MAX_FILE_SIZE_GB, MIN_FILE_SIZE_MB

${BOLD}EXAMPLES${RST}
    $0 build --x86_64
    $0 debug --arm64
    $0 run --x86_64 gguf_list
    MODELS_DIR=/data/models $0 run --arm64 config_show

${BOLD}NOTES${RST}
    • Assumes Ubuntu / Debian based system
    • For cross build, aarch64-linux-gnu-gcc/g++ (or x86_64-linux-gnu-*) required
    • Rust target auto-configured (rustup target add will be prompted if needed)
USAGE
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" || "${1:-}" == "help" ]]; then
  usage; exit 0
fi

cmd="${1:-}"; arch_flag="${2:-}"
if [[ -z "${cmd}" || -z "${arch_flag}" ]]; then
  echo "${RED}Missing required arguments.${RST}"; usage; exit 1
fi

shift 2
additional_args=("$@")

case "${cmd}" in
  build|run|clean|reconfig|fresh|debug) ;; 
  *) echo "${RED}Invalid command: ${cmd}${RST}"; usage; exit 1;;
esac

# -------- Arch & toolchain detection --------
TARGET_TRIPLE=""
CMAKE_TOOLCHAIN_FILE=""
CROSS_REQUIRED="false"

make_toolchain_file() {
  local tcfile="$1"
  local cc="$2"
  local cxx="$3"
  cat > "${tcfile}" <<TC
set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR ${ARCH})
set(CMAKE_C_COMPILER   ${cc})
set(CMAKE_CXX_COMPILER ${cxx})
TC
}

case "${arch_flag}" in
  --x86_64)
    ARCH="x86_64"
    TARGET_TRIPLE="x86_64-unknown-linux-gnu"
    # Host가 aarch64인데 x86_64 타깃을 원하면 크로스
    if [[ "$(uname -m)" != "x86_64" ]]; then
      CROSS_REQUIRED="true"
      CC_BIN="$(command -v x86_64-linux-gnu-gcc || true)"
      CXX_BIN="$(command -v x86_64-linux-gnu-g++ || true)"
      if [[ -z "${CC_BIN}" || -z "${CXX_BIN}" ]]; then
        echo "${RED}Need x86_64 cross toolchain (x86_64-linux-gnu-gcc/g++)${RST}"
        echo "  sudo apt-get install gcc-x86-64-linux-gnu g++-x86-64-linux-gnu"
        exit 1
      fi
      CMAKE_TOOLCHAIN_FILE="${BUILD_DIR}/toolchain-x86_64.cmake"
    fi
    ;;
  --arm64)
    ARCH="arm64"
    TARGET_TRIPLE="aarch64-unknown-linux-gnu"
    # Host가 x86_64인데 arm64 타깃을 원하면 크로스
    if [[ "$(uname -m)" != "aarch64" ]]; then
      CROSS_REQUIRED="true"
      CC_BIN="$(command -v aarch64-linux-gnu-gcc || true)"
      CXX_BIN="$(command -v aarch64-linux-gnu-g++ || true)"
      if [[ -z "${CC_BIN}" || -z "${CXX_BIN}" ]]; then
        echo "${RED}Need ARM64 cross toolchain (aarch64-linux-gnu-gcc/g++)${RST}"
        echo "  sudo apt-get install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu"
        exit 1
      fi
      CMAKE_TOOLCHAIN_FILE="${BUILD_DIR}/toolchain-aarch64.cmake"
    fi
    ;;
  --x86|--x86_64_typo|--x86-*)
    echo "${RED}Invalid architecture flag: ${arch_flag}${RST}"
    echo "${YLW}💡 Did you mean ${BOLD}--x86_64${RST}${YLW}?${RST}"
    echo ""
    usage; exit 1;;
  --arm|--aarch64|--arm-64|--arm_64)
    echo "${RED}Invalid architecture flag: ${arch_flag}${RST}"
    echo "${YLW}💡 Did you mean ${BOLD}--arm64${RST}${YLW}?${RST}"
    echo ""
    usage; exit 1;;
  *)
    echo "${RED}Unknown architecture flag: ${arch_flag}${RST}"
    echo "${YLW}💡 Supported flags: ${BOLD}--arm64${RST}${YLW} or ${BOLD}--x86_64${RST}"
    echo ""
    usage; exit 1;;
esac

export CARGO_BUILD_TARGET="${TARGET_TRIPLE}"

# -------- Helpers --------
ensure_rust_target() {
  if ! rustc -vV >/dev/null 2>&1; then
    echo "${RED}Rust toolchain not found. Install rustup/rustc first.${RST}"
    exit 1
  fi
  if ! rustup target list --installed | grep -q "^${TARGET_TRIPLE}\$"; then
    echo "${YLW}==> Adding Rust target: ${TARGET_TRIPLE}${RST}"
    rustup target add "${TARGET_TRIPLE}"
  fi
}

ensure_cbindgen() {
  if ! command -v cbindgen >/dev/null 2>&1; then
    echo "${YLW}==> cbindgen not found. Installing via cargo...${RST}"
    cargo install cbindgen
    echo "${GRN}cbindgen installed successfully${RST}"
  else
    echo "${GRN}cbindgen found: $(cbindgen --version)${RST}"
  fi
}

ensure_config() {
  mkdir -p "${BUILD_DIR}"

  local DEBUG_FLAG=""
  if [[ "${cmd}" == "debug" ]]; then
    DEBUG_FLAG="-DENABLE_DEBUG_OUTPUT=ON -DCMAKE_BUILD_TYPE=Debug"
    echo "${CYA}==> Debug mode enabled${RST}"
  else
    # build/run/reconfig/fresh default is Release
    [[ "${cmd}" == "build" || "${cmd}" == "run" || "${cmd}" == "reconfig" || "${cmd}" == "fresh" ]] \
      && DEBUG_FLAG="-DCMAKE_BUILD_TYPE=Release"
  fi

  # Generate cross compile toolchain file if needed
  local TC_OPT=""
  if [[ "${CROSS_REQUIRED}" == "true" ]]; then
    if [[ ! -f "${CMAKE_TOOLCHAIN_FILE}" ]]; then
      echo "${BOLD}==> Generating CMake toolchain file (${CMAKE_TOOLCHAIN_FILE})...${RST}"
      make_toolchain_file "${CMAKE_TOOLCHAIN_FILE}" "${CC_BIN}" "${CXX_BIN}"
    fi
    TC_OPT="-DCMAKE_TOOLCHAIN_FILE=${CMAKE_TOOLCHAIN_FILE}"
  fi

  if [[ ! -f "${BUILD_DIR}/CMakeCache.txt" ]]; then
    echo "${BOLD}==> Configuring (first time) [${ARCH}]...${RST}"
    cmake -S "${SRC_DIR}" -B "${BUILD_DIR}" ${TC_OPT} ${DEBUG_FLAG} \
      -DTARGET_TRIPLE="${TARGET_TRIPLE}" \
      -DOUTPUT_DIR="${ROOT_DIR}/output"
  elif ! grep -q '^CMAKE_PROJECT_NAME:' "${BUILD_DIR}/CMakeCache.txt" 2>/dev/null; then
    echo "${YLW}==> Cache broken. Reconfiguring...${RST}"
    cmake -S "${SRC_DIR}" -B "${BUILD_DIR}" ${TC_OPT} ${DEBUG_FLAG} \
      -DTARGET_TRIPLE="${TARGET_TRIPLE}" \
      -DOUTPUT_DIR="${ROOT_DIR}/output"
  fi
}

print_banner() {
  echo "${BOLD}${CYA}================================================${RST}"
  echo "${BOLD}${CYA}      LLM Rust - Ubuntu Build System           ${RST}"
  echo "${BOLD}${CYA}      Cross-Platform Rust + C++ Integration    ${RST}"
  echo "${BOLD}${CYA}================================================${RST}"
}

final_guide() {
  echo
  echo "${BOLD}Next steps ${RST}"
  cat <<EOF
${GRN}Build Commands:${RST}
To rebuild:   $0 build ${arch_flag}
To debug:     $0 debug ${arch_flag}
To clean:     $0 clean ${arch_flag}
To reconfig:  $0 reconfig ${arch_flag}
To reset:     $0 fresh ${arch_flag}

${CYA}LLM System Commands:${RST}
Run LLM:      $0 run ${arch_flag} llm
Start Server: $0 run ${arch_flag} llm run
List models:  $0 run ${arch_flag} llm list
Gen config:   $0 run ${arch_flag} llm config_gen
Validate:     $0 run ${arch_flag} llm config_validate
Show config:  $0 run ${arch_flag} llm config_show
Config help:  $0 run ${arch_flag} llm config_help
LLM help:     $0 run ${arch_flag} llm --help
EOF
}

print_banner

case "${cmd}" in
  build)
    echo "${YLW}WARNING: Run '$0 clean ${arch_flag}' before build if switching arch or Rust code changed.${RST}"
    ensure_rust_target
    ensure_cbindgen
    ensure_config
    echo "${BOLD}==> Building project [${ARCH}/${TARGET_TRIPLE}]...${RST}"
    cmake --build "${BUILD_DIR}" -j
    ;;

  debug)
    echo "${CYA}==> Building with debug output enabled [${ARCH}/${TARGET_TRIPLE}]...${RST}"
    ensure_rust_target
    ensure_cbindgen
    # Force reconfigure for debug mode to update CMAKE_BUILD_TYPE
    rm -f "${BUILD_DIR}/CMakeCache.txt"
    ensure_config
    echo "${BOLD}==> Building debug project [${ARCH}]...${RST}"
    cmake --build "${BUILD_DIR}" -j
    ;;

  run)
    ensure_rust_target
    exe="${APP_EXE}"
    if [[ ! -x "${exe}" ]]; then
      echo "${YLW}Binary missing → building first...${RST}"
      "$0" build "${arch_flag}"
    fi
    if [[ ${#additional_args[@]} -gt 0 ]]; then
      echo "${BOLD}==> Running ${BIN_NAME} [${ARCH}] with args: ${CYA}${additional_args[*]}${RST}"
      "${exe}" "${additional_args[@]}"
    else
      echo "${BOLD}==> Running ${BIN_NAME} [${ARCH}]${RST}"
      "${exe}"
    fi
    ;;

  clean)
    echo "${BOLD}==> Cleaning (CMake + Rust targets + output) [${ARCH}]${RST}"
    if [[ -d "${BUILD_DIR}" ]]; then
      cmake --build "${BUILD_DIR}" --target clean || true
      rm -rf "${BUILD_DIR}"
    fi
    if command -v cargo >/dev/null 2>&1; then
      ( cd "${ROOT_DIR}/rustlib" && cargo clean --target "${TARGET_TRIPLE}" ) || true
    fi
    if [[ -d "${ROOT_DIR}/output" ]]; then
      rm -rf "${ROOT_DIR}/output"
      echo "Removed output directory"
    fi
    ;;

  reconfig)
    echo "${BOLD}==> Force reconfigure [${ARCH}]${RST}"
    rm -f "${BUILD_DIR}/CMakeCache.txt"
    ensure_rust_target
    ensure_config
    ;;

  fresh)
    echo "${BOLD}==> Fresh configure (wipe build dir) [${ARCH}]${RST}"
    rm -rf "${BUILD_DIR}"
    ensure_rust_target
    ensure_config
    ;;
esac

final_guide

