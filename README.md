# LlamaRS - Native Rust LLM Backend
> **LlamaCore-RS**: A comprehensive Rust implementation replacing llama.cpp dependencies

[![macOS](https://img.shields.io/badge/macOS-ARM64%20%7C%20x86__64-blue.svg)](https://www.apple.com/macos/)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![C++](https://img.shields.io/badge/C%2B%2B-17-blue.svg)](https://isocpp.org/)

## Developer Information
- **Contact:** [![Email](https://img.shields.io/badge/Email-azabell1993%40gmail.com-blue?logo=gmail)](mailto:azabell1993@gmail.com)  
- **GitHub:** [![GitHub](https://img.shields.io/badge/GitHub-Repository-black?logo=github)](https://github.com/Azabell1993)

For questions, issues, or contributions, please open an issue or pull request on GitHub.

## Project Overview

A macOS-exclusive project converting LLaMa to pure Rust implementation.

This project **completely eliminates external llama.cpp dependency** by implementing a comprehensive mock LLM backend in Rust. Originally developed to solve compilation warnings and linking issues, it has evolved into a standalone, production-ready alternative that maintains full API compatibility while providing superior build reliability.

### Why This Project?
- **Dependency Hell Solution**: No more complex llama.cpp build configurations
- **Zero Warnings**: Clean compilation output for professional development
- **Native Performance**: Direct Rust implementation without C++ overhead
- **Development Efficiency**: Instant builds without external library compilation

### Key Features
- **Pure Rust Implementation**: 150+ LLM functions reimplemented from scratch
- **Zero External Dependencies**: Self-contained with no llama.cpp requirement
- **Warning-Free Build**: Eliminates all cbindgen and rustc compile warnings
- **C++ FFI Compatibility**: Drop-in replacement maintaining full API compatibility
- **Cross-Platform Support**: Native ARM64 (Apple Silicon) and x86_64 builds
- **Debug Mode**: Conditional compilation with detailed logging and diagnostics
- **Fast Builds**: Sub-second incremental compilation without external dependencies

### Technical Architecture

![LlamaRS Architecture](llmrust.png)

#### Core System Architecture
```
┌──────────────────────────────────────────────────────────────────────────────┐
│                           LlamaRS Ecosystem                                  │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────┐    ┌──────────────────┐    ┌──────────────────────────┐ │
│  │   C++ Frontend  │◄──►│  Rust LLM Core   │◄──►│    Rust Extensions       │ │
│  │   (main.cpp)    │    │  (150+ Mocks)    │    │   • Native Crates        │ │
│  │                 │    │                  │    │   • FFI Bindings         │ │
│  │  ┌─────────────┐│    │ ┌──────────────┐ │    │   • Cargo Features       │ │
│  │  │Debug/Release││    │ │State Manager │ │    │   • Procedural Macros    │ │
│  │  │Conditional  ││    │ │Memory Pool   │ │    │   • Trait Implementations│ │
│  │  │Compilation  ││    │ │Token Handler │ │    │                          │ │
│  │  └─────────────┘│    │ └──────────────┘ │    └──────────────────────────┘ │
│  └─────────────────┘    └──────────────────┘               │                 │
│          │                        │                        │                 │
│          ▼                        ▼                        ▼                 │
│  ┌─────────────────┐    ┌──────────────────┐    ┌─────────────────────────┐  │
│  │  CMake Build    │    │   cbindgen FFI   │    │    Performance Layer    │  │
│  │  System         │    │   Auto Header    │    │   • Zero-Copy Memory    │  │
│  │                 │    │   Generation     │    │   • SIMD Optimization   │  │
│  │ ARM64 | x86_64  │    │                  │    │   • Async Processing    │  │
│  │ Debug | Release │    │  Warning-Free    │    │   • Concurrent Safety   │  │
│  └─────────────────┘    └──────────────────┘    └─────────────────────────┘  │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

# Build & Run Guide

## Quick Start (Recommended)

```bash
./run.sh                    # release + arm64
./run.sh debug              # debug + arm64  
./run.sh release x86_64     # release + x86_64
./run.sh debug x86_64       # debug + x86_64
```

## Manual Build/Run

### Development Mode (Debug)
```bash
./build.sh debug            # Debug build (default: arm64)
./build.sh debug x86_64     # Debug build (Intel)
./output/bin/cpp_app llm    # Run llm version
./output/bin/cpp_app llmrust # Run llmrust version
```

### Release Mode
```bash
./build.sh                  # Release build (default: arm64)  
./build.sh x86_64           # Release build (Intel)
./output/bin/cpp_app llm    # Run llm version
./output/bin/cpp_app llmrust # Run llmrust version
```

### run.sh Usage
```bash
# Basic usage (Release mode)
./run.sh                    # clean + build + run with llm (arm64)
./run.sh x86_64             # clean + build + run with llm (Intel)

# Debug mode usage  
./run.sh debug              # clean + debug + run with llm (arm64)
./run.sh debug x86_64       # clean + debug + run with llm (Intel)

# Explicit release mode usage
./run.sh release            # clean + build + run with llm (arm64)
./run.sh release x86_64     # clean + build + run with llm (Intel)

# Help
./run.sh -h                 # Show usage
./run.sh --help             # Show usage
```

## One-Command Testing
1) Debug Mode
>  % ./build.sh clean --arm64 && ./build.sh debug --arm64 && ./build.sh run --arm64 llm

2) Release Mode
>  % ./build.sh clean --arm64 && ./build.sh build --arm64 && ./build.sh run --arm64 llm

## Build Output Example

```
==> Building project [arm64]...
[ 50%] Building CXX object CMakeFiles/cpp_objs.dir/src/hello.cpp.o
[ 50%] Building Rust crate: llm_rust
[ 50%] Generating C header via cbindgen: .../output/include/llm_rust
  Compiling llm_rust v0.1.0 (.../rustlib)
   Finished `release` profile [optimized] target(s) in 0.12s
Export rust staticlib -> .../output/lib
[100%] Linking CXX executable .../output/bin/llmrcpp_app
[100%] Built target cpp_app
```

## Command Guide

### Simple Method (run.sh)
| Task                     | Command             | Description                    |
|--------------------------|---------------------|--------------------------------|
| Full Pipeline (Default) | `./run.sh`          | clean + build + run llm (arm64) |
| Intel Architecture Run  | `./run.sh x86_64`   | clean + build + run llm (x86_64) |

### Detailed Control (build.sh)
| Task         | Command (Apple Silicon)        | Command (Intel)            |
|--------------|-------------------------------|----------------------------|
| Build        | `./build.sh build --arm64`    | `./build.sh build --x86_64`|
| Run (Basic)  | `./build.sh run --arm64`      | `./build.sh run --x86_64`  |
| Run (LLM)    | `./build.sh run --arm64 llm`  | `./build.sh run --x86_64 llm`|
| Clean        | `./build.sh clean --arm64`    | `./build.sh clean --x86_64`|
| Debug        | `./build.sh debug --arm64`    | `./build.sh debug --x86_64` |
| Reset        | `./build.sh fresh --arm64`    | `./build.sh fresh --x86_64`|
| Force Reset  | `./build.sh reconfig --arm64` | `./build.sh reconfig --x86_64`|

> **Tip:** Always clean before build when switching architectures or updating Rust code.

---

## Usage

### run.sh (Simple Pipeline)
```
Usage: ./run.sh [arch]
  arch: arm64 (default) or x86_64

Examples:
  ./run.sh          # clean + build + run with llm (arm64)
  ./run.sh arm64     # explicitly specify arm64
  ./run.sh x86_64    # run on Intel architecture
```

### build.sh (Detailed Control)
```
Usage: ./build.sh [build|run|clean|reconfig|fresh] <--arm64|--x86_64> [additional_args...]
  build     Configure and build with CMake (Rust build is triggered inside CMake)
  run       Run the built binary (pass additional args after arch flag)
  clean     Clean CMake build dir + Rust targets (clean-all)
  debug     Build with debug symbols and output for troubleshooting
  reconfig  Force reconfigure
  fresh     Remove build dir and reconfigure from scratch
  --arm64   Force build for Apple Silicon (Rust + CMake)
  --x86_64  Force build for Intel (Rust + CMake)

Examples:
  ./build.sh build --arm64
  ./build.sh debug --arm64                # Build with debug output
  ./build.sh run --arm64
  ./build.sh run --arm64 llm              # Run with 'llm' argument
  ./build.sh run --arm64 arg1 arg2        # Run with multiple arguments
```

---

> **Tip:** Always clean before build when switching architectures or updating Rust code.

## Additional Information

### Recommended Workflows
1. **General Use**: `./run.sh` - All processes run automatically
2. **During Development**: Run `./build.sh clean --arm64 && ./build.sh build --arm64` then test
3. **Debugging**: Test with various arguments using `./build.sh run --arm64 [args...]`

### Key Information
- **Build Output**: Generated in `output/bin/llmrcpp_app`
- **Architecture**: arm64 (Apple Silicon) default, x86_64 supported
- **Integrated Environment**: Hybrid project combining C++ and Rust
- **LLM Features**: Special Rust LLM function calls available with `llm` argument

### Troubleshooting
- When changing architecture: Always clean before build
- When updating Rust code: Clean before build recommended
- On build errors: Complete reset with `./build.sh fresh --arm64`
