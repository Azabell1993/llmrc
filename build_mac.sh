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
${BOLD}${CYA}LLM Rust - Comprehensive Build & Model Management System${RST}
${BOLD}===========================================================${RST}

${BOLD}SYNOPSIS${RST}
    $0 [COMMAND] <ARCHITECTURE> [ADDITIONAL_ARGS...]

${BOLD}DESCRIPTION${RST}
    Unified build system for Rust-based LLM backend with GGUF model support.
    Combines Rust static libraries with C++ frontend, featuring dynamic model
    discovery and configuration management.

${BOLD}COMMANDS${RST}
    ${GRN}build${RST}     Configure and build with CMake (Rust build triggered automatically)
               Creates optimized release binary with full GGUF support
    
    ${GRN}debug${RST}     Build with debug output enabled (_DEBUG flag)
               Enables verbose logging and debug symbols for development
    
    ${GRN}run${RST}       Run the built binary (pass additional args after arch flag)
               Execute the LLM system with specified arguments
    
    ${GRN}clean${RST}     Clean CMake build dir + Rust targets + output directory
               Complete cleanup of all build artifacts and cached files
    
    ${GRN}reconfig${RST}  Force reconfigure CMake cache
               Refresh build configuration without removing build directory
    
    ${GRN}fresh${RST}     Remove build dir and reconfigure from scratch
               Nuclear option - complete rebuild from zero state

${BOLD}ARCHITECTURE FLAGS${RST}
    ${YLW}--arm64${RST}   Build for Apple Silicon (M1/M2/M3) processors
               Sets: CARGO_BUILD_TARGET=aarch64-apple-darwin
    
    ${YLW}--x86_64${RST}  Build for Intel processors
               Sets: CARGO_BUILD_TARGET=x86_64-apple-darwin

${BOLD}MODEL MANAGEMENT COMMANDS${RST}
    After building, use these commands with the binary:
    
    ${CYA}gguf_list${RST}     List all available GGUF models in models/ directory
                  Shows model names, file sizes, and validation status
    
    ${CYA}config_gen${RST}    Generate dynamic model configuration
                  Scans models/ and creates models.json with discovered models
    
    ${CYA}config_show${RST}   Display current model configuration as JSON
                  Shows active configuration including environment overrides
    
    ${CYA}config_help${RST}   Show environment variable configuration help
                  Lists all supported environment variables for model management

${BOLD}ENVIRONMENT VARIABLES${RST}
    ${YLW}MODEL_PATH${RST}        Full path to specific GGUF model file
    ${YLW}DEFAULT_MODEL${RST}     Filename of default model in models directory  
    ${YLW}MODELS_DIR${RST}        Path to models directory (default: models)
    ${YLW}PREFER_QUANTIZED${RST}  Prefer quantized models (true/false, default: true)
    ${YLW}MAX_FILE_SIZE_GB${RST}  Maximum model file size in GB (default: 20)
    ${YLW}MIN_FILE_SIZE_MB${RST}  Minimum model file size in MB (default: 100)

${BOLD}EXAMPLES${RST}
    ${GRN}Basic Build:${RST}
        $0 build --arm64                    # Build for Apple Silicon
        $0 debug --x86_64                   # Debug build for Intel
    
    ${GRN}Model Management:${RST}
        $0 run --arm64 gguf_list            # List available models
        $0 run --arm64 config_gen           # Generate dynamic config
        $0 run --arm64 config_help          # Show env var help
    
    ${GRN}Custom Environment:${RST}
        MODELS_DIR=/custom/path $0 run --arm64 gguf_list
        DEFAULT_MODEL=my-model.gguf $0 run --arm64 config_show
    
    ${GRN}System Operations:${RST}
        $0 run --arm64 llm                  # Run LLM system
        $0 clean --arm64                    # Clean all build artifacts
        $0 fresh --arm64                    # Complete rebuild

${BOLD}PROJECT STRUCTURE${RST}
    models/          GGUF model files (.gguf format)
    models.json      Dynamic model configuration (auto-generated)
    rustlib/         Rust backend implementation
    cpp-app/         C++ frontend application
    output/          Build artifacts and executables

${BOLD}NOTES${RST}
    • Always specify architecture flag (--arm64 or --x86_64)
    • Use 'clean' before switching architectures
    • GGUF models must be placed in models/ directory
    • Configuration is auto-discovered at runtime
    • Environment variables override JSON configuration

${BOLD}VERSION & COMPATIBILITY${RST}
    Target: macOS ARM64 (Apple Silicon) and Intel x86_64
    Rust: Latest stable (edition 2021)
    CMake: 3.15+ required
    Models: GGUF format support
USAGE
}

# -------- Early help handling --------
if [[ "${1:-}" == "--help" || "${1:-}" == "-h" || "${1:-}" == "help" ]]; then
  usage; exit 0
fi

# -------- Parse args (command and arch required, additional args optional) --------
cmd="${1:-}"; arch_flag="${2:-}"

if [[ -z "${cmd}" || -z "${arch_flag}" ]]; then
  echo "${RED}Missing required arguments.${RST}"; usage; exit 1
fi

# Collect additional arguments for run command
shift 2  # Remove first two arguments (cmd and arch_flag)
additional_args=("$@")  # Collect remaining arguments

case "${cmd}" in
  build|run|clean|reconfig|fresh|debug) ;; 
  *) echo "${RED}Invalid command: ${cmd}${RST}"; usage; exit 1;;
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
    echo "${RED}Must specify --arm64 or --x86_64${RST}"; usage; exit 1;;
esac

# -------- Helpers (define BEFORE use) --------
ensure_cbindgen() {
  if ! command -v cbindgen >/dev/null 2>&1; then
    echo "${YLW}==> cbindgen not found. Installing via cargo...${RST}"
    if ! command -v cargo >/dev/null 2>&1; then
      echo "${RED}Rust/Cargo not found. Please install Rust first:${RST}"
      echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
      exit 1
    fi
    cargo install cbindgen
    echo "${GRN}cbindgen installed successfully${RST}"
  else
    echo "${GRN}cbindgen found: $(cbindgen --version)${RST}"
  fi
}

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
${GRN}Build Commands:${RST}
To rebuild:   $0 build ${arch_flag}
To debug:     $0 debug ${arch_flag}
To clean:     $0 clean ${arch_flag}
To reconfig:  $0 reconfig ${arch_flag}
To reset:     $0 fresh ${arch_flag}

${CYA}LLM System Commands:${RST}
Run LLM:      $0 run ${arch_flag} llm
Start Server  $0 run ${arch_flag} llm run
List models:  $0 run ${arch_flag} llm list
Gen config:   $0 run ${arch_flag} llm config_gen
Validate:     $0 run ${arch_flag} llm config_validate
Show config:  $0 run ${arch_flag} llm config_show
Config help:  $0 run ${arch_flag} llm config_help
LLM help:     $0 run ${arch_flag} llm --help

${YLW}Development Tips:${RST}
• Always clean before build if you switch architectures or update Rust code
• Use debug build for development: $0 debug ${arch_flag}
• Check model directory: ls -la models/
• View validation results: cat models/*.validation
EOF
}

# -------- Always show banner once --------
print_banner

# -------- Commands --------
case "${cmd}" in
  build)
    echo "${YLW}WARNING: It is recommended to run '$0 clean ${arch_flag}' before build to avoid stale objects.${RST}"
    ensure_cbindgen
    ensure_config
    echo "${BOLD}==> Building project [${ARCH}]...${RST}"
    # Note: Rust build is performed by cargo in CMakeLists.txt's add_custom_command/target
    cmake --build "${BUILD_DIR}" -j
    ;;

  debug)
    echo "${CYA}==> Building with debug output enabled [${ARCH}]...${RST}"
    ensure_cbindgen
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
        echo "Removed output directory"
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