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
