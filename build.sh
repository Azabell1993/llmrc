#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

# ==========================
#   Rust (staticlib) + C++
#   Build & Run Orchestrator
# ==========================

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="$ROOT_DIR/cpp-app"
BUILD_DIR="$SRC_DIR/build"
BIN_NAME="llmrcpp_app"   # 실행파일명
APP_EXE="${ROOT_DIR}/output/bin/${BIN_NAME}"

# -------- UI helpers --------
BOLD=$'\033[1m'; GRN=$'\033[32m'; RED=$'\033[31m'; YLW=$'\033[33m'; CYA=$'\033[36m'; RST=$'\033[0m'

usage() {
  cat <<USAGE
Usage: $0 [build|run|clean|reconfig|fresh|debug] <--arm64|--x86_64> [additional_args...]
  build     Configure and build with CMake (Rust build is triggered inside CMake)
  debug     Build with debug output enabled (_DEBUG flag)
  run       Run the built binary (pass additional args after arch flag)
  clean     Clean CMake build dir + Rust targets (clean-all)
  reconfig  Force reconfigure
  fresh     Remove build dir and reconfigure from scratch
  --arm64   Force build for Apple Silicon (Rust + CMake)
  --x86_64  Force build for Intel (Rust + CMake)

Examples:
  $0 build --arm64
  $0 debug --arm64                # Build with debug output
  $0 run --arm64
  $0 run --arm64 llm              # Run with 'llm' argument
  $0 run --arm64 arg1 arg2        # Run with multiple arguments
USAGE
}

# -------- Parse args (command and arch required, additional args optional) --------
cmd="${1:-}"; arch_flag="${2:-}"
if [[ -z "${cmd}" || -z "${arch_flag}" ]]; then
  echo "${RED}✘ Missing required arguments.${RST}"; usage; exit 1
fi

# Collect additional arguments for run command
shift 2  # Remove first two arguments (cmd and arch_flag)
additional_args=("$@")  # Collect remaining arguments

case "${cmd}" in
  build|run|clean|reconfig|fresh|debug) ;; 
  *) echo "${RED}✘ Invalid command: ${cmd}${RST}"; usage; exit 1;;
esac

case "${arch_flag}" in
  --arm64)
    ARCH="arm64"
    export CARGO_BUILD_TARGET=aarch64-apple-darwin
    CMAKE_OPTS="-DCMAKE_OSX_ARCHITECTURES=arm64"
    ;;
  --x86_64)
    ARCH="x86_64"
    export CARGO_BUILD_TARGET=x86_64-apple-darwin
    CMAKE_OPTS="-DCMAKE_OSX_ARCHITECTURES=x86_64"
    ;;
  *)
    echo "${RED}✘ Must specify --arm64 or --x86_64${RST}"; usage; exit 1;;
esac

# -------- Helpers (define BEFORE use) --------
ensure_config() {
  mkdir -p "${BUILD_DIR}"
  
  # Check if debug mode is requested
  DEBUG_FLAG=""
  if [[ "${cmd}" == "debug" ]]; then
    DEBUG_FLAG="-DENABLE_DEBUG_OUTPUT=ON -DCMAKE_BUILD_TYPE=Debug"
    echo "${CYA}==> Debug mode enabled${RST}"
  fi
  
  if [[ ! -f "${BUILD_DIR}/CMakeCache.txt" ]]; then
    echo "${BOLD}==> Configuring (first time)...${RST}"
    cmake -S "${SRC_DIR}" -B "${BUILD_DIR}" ${CMAKE_OPTS} ${DEBUG_FLAG}
  elif ! grep -q '^CMAKE_PROJECT_NAME:' "${BUILD_DIR}/CMakeCache.txt" 2>/dev/null; then
    echo "${YLW}==> Cache broken. Reconfiguring...${RST}"
    cmake -S "${SRC_DIR}" -B "${BUILD_DIR}" ${CMAKE_OPTS} ${DEBUG_FLAG}
  fi
}

print_banner() {
  # Rainbow banner (signature)
  local COLORS=($'\033[31m' $'\033[32m' $'\033[33m' $'\033[34m' $'\033[35m' $'\033[36m')
  local i=0
  while IFS= read -r line; do
    local color="${COLORS[$(( i % ${#COLORS[@]} ))]}"
    printf "%b%s%b\n" "$color" "$line" "$RST"
    ((i++))
  done <<'ART'

$$\      $$\ $$\                 $$\ $$\ $$\           $$\       
$$$\    $$$ |\__|                $$ |$$ |\__|          $$ |      
$$$$\  $$$$ |$$\ $$$$$$$\   $$$$$$$ |$$ |$$\ $$$$$$$\  $$ |  $$\ 
$$\$$\$$ $$ |$$ |$$  __$$\ $$  __$$ |$$ |$$ |$$  __$$\ $$ | $$  |
$$ \$$$  $$ |$$ |$$ |  $$ |$$ /  $$ |$$ |$$ |$$ |  $$ |$$$$$$  / 
$$ |\$  /$$ |$$ |$$ |  $$ |$$ |  $$ |$$ |$$ |$$ |  $$ |$$  _$$<  
$$ | \_/ $$ |$$ |$$ |  $$ |\$$$$$$$ |$$ |$$ |$$ |  $$ |$$ | \$$\ 
\__|     \__|\__|\__|  \__| \_______|\__|\__|\__|  \__|\__|  \__|

ART
}

final_guide() {
  echo
  echo "${BOLD}Next steps ${RST}"
  cat <<EOF
✔ To rebuild:   $0 build ${arch_flag}
✔ To run:       $0 run ${arch_flag}
✔ To clean:     $0 clean ${arch_flag}
✔ To reconfig:  $0 reconfig ${arch_flag}
✔ To reset:     $0 fresh ${arch_flag}

Tip: Always clean before build if you switch architectures or update Rust code.
EOF
}

# -------- Always show banner once --------
print_banner

# -------- Commands --------
case "${cmd}" in
  build)
    echo "${YLW}⚠ It is recommended to run '$0 clean ${arch_flag}' before build to avoid stale objects.${RST}"
    ensure_config
    echo "${BOLD}==> Building project [${ARCH}]...${RST}"
    # ⚠️ Rust 빌드는 CMakeLists.txt의 add_custom_command/target에서 cargo로 수행됨
    cmake --build "${BUILD_DIR}" -j
    ;;

  debug)
    echo "${CYA}==> Building with debug output enabled [${ARCH}]...${RST}"
    ensure_config
    echo "${BOLD}==> Building debug project [${ARCH}]...${RST}"
    cmake --build "${BUILD_DIR}" -j
    ;;

  run)
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
      # CMake build 디렉터리 정리
      if [[ -d "${BUILD_DIR}" ]]; then
        cmake --build "${BUILD_DIR}" --target clean-all || true
        rm -rf "${BUILD_DIR}"
      fi

      # Rust 쪽도 안전하게 clean
      if command -v cargo >/dev/null 2>&1; then
        ( cd "${ROOT_DIR}/rustlib" && cargo clean ) || true
      fi

      # output 디렉터리 전체 제거
      if [[ -d "${ROOT_DIR}/output" ]]; then
        rm -rf "${ROOT_DIR}/output"
        echo "✔ Removed output directory"
      fi
      ;;

  reconfig)
    echo "${BOLD}==> Force reconfigure [${ARCH}]${RST}"
    rm -f "${BUILD_DIR}/CMakeCache.txt"
    ensure_config
    ;;

  fresh)
    echo "${BOLD}==> Fresh configure (wipe build dir) [${ARCH}]${RST}"
    rm -rf "${BUILD_DIR}"
    ensure_config
    ;;
esac

final_guide