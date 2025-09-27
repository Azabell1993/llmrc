# 🦀 LLM Rust - Comprehensive GGUF Model Management System
> **Advanced Rust-based LLM Backend with Dynamic Model Discovery and Configuration**

![LlamaRS Architecture](llmrust.png)

[![macOS](https://img.shields.io/badge/macOS-ARM64%20%7C%20x86__64-success.svg?logo=apple)](https://www.apple.com/macos/)
[![Ubuntu](https://img.shields.io/badge/Ubuntu-x86__64%20%7C%20ARM64-orange.svg?logo=ubuntu)](https://ubuntu.com/)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![C++](https://img.shields.io/badge/C%2B%2B-17-blue.svg?logo=cplusplus)](https://isocpp.org/)
[![GGUF](https://img.shields.io/badge/GGUF-Model%20Support-green.svg)](https://github.com/ggerganov/ggml)
[![MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Developer Information
- **Developer:** Azabell1993
- **Contact:** [![Email](https://img.shields.io/badge/Email-azabell1993%40gmail.com-blue?logo=gmail)](mailto:azabell1993@gmail.com)  
- **GitHub:** [![GitHub](https://img.shields.io/badge/GitHub-Repository-black?logo=github)](https://github.com/Azabell1993/llmrc)
- **Branch:** `llama-rs`

For questions, issues, or contributions, please open an issue or pull request on GitHub.

## Project Overview

**LLM Rust** is a comprehensive Large Language Model management system designed for **macOS and Ubuntu**, featuring dynamic GGUF model discovery, configuration management, and a powerful cross-platform build system. This project provides a complete replacement for traditional llama.cpp dependencies while offering superior performance and reliability across different operating systems.

### Why Choose LLM Rust?

- **Cross-Platform Support**: Native builds for macOS (ARM64/x86_64) and Ubuntu (x86_64/ARM64)
- **Zero Configuration Hassles**: Automatic model discovery and configuration
- **Self-Contained System**: No external llama.cpp dependencies required
- **Lightning Fast Builds**: Optimized Rust compilation with minimal overhead
- **Dynamic Configuration**: Runtime environment variable support
- **Professional Build System**: Comprehensive CMake + Cargo integration with platform detection
- **Smart Model Management**: Automatic GGUF validation and metadata extraction

## Key Features

### Dynamic Model Management
- **Auto-Discovery**: Automatic scanning and validation of GGUF models
- **Smart Configuration**: Dynamic `models.json` generation based on available models
- **Environment Integration**: Full environment variable configuration support
- **Flexible Filtering**: Size-based model filtering and quantization preferences

### Advanced Build System
- **Cross-Architecture**: Native ARM64 (Apple Silicon) and x86_64 support
- **Debug Mode**: Comprehensive logging and debug symbol generation
- **Clean Operations**: Intelligent build artifact cleanup
- **Fresh Rebuilds**: Complete environment reset capabilities

### HTTP API Server
- **OpenAI Compatible**: Standard REST API endpoints for chat completions
- **Real-time Logging**: Comprehensive request/response logging with timestamps
- **Engine Integration**: Periodic metadata transmission and system monitoring
- **Graceful Shutdown**: Immediate server termination via `/stop` endpoint
- **Multi-threaded**: Concurrent request handling with thread-safe operations

### Model Configuration System
```json
{
  "default_model": "",
  "model_directory": "models",
  "fallback_models": ["auto-discovered"],
  "model_preferences": {
    "prefer_quantized": true,
    "max_file_size_gb": 20,
    "min_file_size_mb": 100
  }
}
```

### **Command Interface**
- **Model Listing**: `gguf_list` - Display all available models with metadata
- **Config Generation**: `config_gen` - Create dynamic configuration files
- **Config Display**: `config_show` - View current configuration as JSON
- **Help System**: `config_help` - Environment variable documentation
- **HTTP API Server**: `llm run` - Start HTTP API server with real-time logging and graceful shutdown

## System Architecture

```
┌───────────────────────────────────────────────────────────────────────────────────┐
│                               LLM Rust Ecosystem                                  │
├───────────────────────────────────────────────────────────────────────────────────┤
│                                                                                   │
│  ┌───────────────────┐   ┌─────────────────────┐   ┌────────────────────────────┐ │
│  │   Build System    │◄─►│   Rust LLM Core     │◄─►│     Model Management       │ │
│  │   • CMake + Cargo │   │   • GGUF Support    │   │   • Auto-Discovery         │ │
│  │   • Cross-Platform│   │   • 150+ Functions  │   │   • Dynamic Config         │ │
│  │   • macOS + Ubuntu│   │   • Mock Backend    │   │   • Environment Variables  │ │
│  └───────────────────┘   └─────────────────────┘   └────────────────────────────┘ │
│                                                                                   │
│  ┌───────────────────┐   ┌─────────────────────┐   ┌────────────────────────────┐ │
│  │      HTTP API     │◄─►│.    Engine System   │◄─►│    Configuration System    │ │
│  │   • REST Endpoints│   │   • Metadata Tx     │   │   • JSON Config            │ │
│  │   • OpenAI Compat │   │   • Async Runtime   │   │   • Env Variables          │ │
│  │   • Graceful Stop │   │   • Real-time Log   │   │   • Runtime Setup          │ │
│  └───────────────────┘   └─────────────────────┘   └────────────────────────────┘ │
│                                                                                   │
│  ┌───────────────────┐   ┌─────────────────────┐   ┌────────────────────────────┐ │
│  │     C++ Engine    │◄─►│   Logging System    │◄─►│        GGUF Models         │ │
│  │   • CLI Interface │   │   • Thread-Safe     │   │   • Automatic Validation   │ │
│  │   • Command Args  │   │   • Multi-Level     │   │   • Metadata Extraction    │ │
│  │   • Help System   │   │   • Error Tracking  │   │   • Smart Filtering        │ │
│  └───────────────────┘   └─────────────────────┘   └────────────────────────────┘ │
└───────────────────────────────────────────────────────────────────────────────────┘
```

## Quick Start

### Prerequisites
- **macOS** (ARM64 or x86_64) or **Ubuntu/Debian** (x86_64 or ARM64)
- **Rust** 1.70+ (stable)
- **CMake** 3.15+
- **C++ Compiler**: Clang/C++ 17+ (macOS) or GCC 11+ (Ubuntu)

### Installation & Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Azabell1993/llmrc.git
   cd llmrc
   git checkout ubuntu_poc  # Latest cross-platform branch
   ```

2. **Place GGUF models:**
   ```bash
   mkdir -p models
   # Copy your .gguf model files to the models/ directory
   cp your-model.gguf models/
   ```

3. **Build the system:**
   
   **macOS:**
   ```bash
   ./build.sh build --arm64    # For Apple Silicon
   ./build.sh build --x86_64   # For Intel Macs
   ```
   
   **Ubuntu/Debian:**
   ```bash
   ./build_ubuntu.sh build --x86_64    # For x86_64 systems
   ./build_ubuntu.sh build --arm64     # For ARM64 systems
   ```

4. **List available models:**
   
   **macOS:**
   ```bash
   ./build.sh run --arm64 gguf_list
   ```
   
   **Ubuntu/Debian:**
   ```bash
   ./build_ubuntu.sh run --x86_64 gguf_list
   ```

## Usage Guide

### Build System Commands

**macOS:**
```bash
# Basic Operations
./build.sh build --arm64                    # Build for Apple Silicon  
./build.sh debug --arm64                    # Debug build with logging
./build.sh clean --arm64                    # Clean all build artifacts
./build.sh fresh --arm64                    # Complete rebuild from scratch

# Model Management
./build.sh run --arm64 gguf_list            # List available models
./build.sh run --arm64 config_gen           # Generate dynamic config
./build.sh run --arm64 config_show          # Show current config
./build.sh run --arm64 config_help          # Environment variable help

# HTTP API Server Operations  
./build.sh run --arm64 llm run              # Start HTTP API server with Engine integration
./build.sh --help                           # Show comprehensive help
```

**Ubuntu/Debian:**
```bash
# Basic Operations
./build_ubuntu.sh build --x86_64            # Build for x86_64 systems
./build_ubuntu.sh debug --x86_64            # Debug build with logging
./build_ubuntu.sh clean --x86_64            # Clean all build artifacts
./build_ubuntu.sh fresh --x86_64            # Complete rebuild from scratch

# Model Management
./build_ubuntu.sh run --x86_64 gguf_list    # List available models
./build_ubuntu.sh run --x86_64 config_gen   # Generate dynamic config
./build_ubuntu.sh run --x86_64 config_show  # Show current config
./build_ubuntu.sh run --x86_64 config_help  # Environment variable help

# HTTP API Server Operations  
./build_ubuntu.sh run --x86_64 llm run      # Start HTTP API server with Engine integration
./build_ubuntu.sh --help                    # Show comprehensive help
```

### Model Management

#### Automatic Model Discovery
The system automatically discovers and validates GGUF models in the `models/` directory:

```bash
# Generate dynamic configuration based on available models
./build.sh run --arm64 config_gen

# View discovered models and their metadata
./build.sh run --arm64 gguf_list
```

**Example Output:**
```
[INFO] === Available GGUF Models ===
[INFO] 1. deepseek-coder-v2-lite-instruct-q4_k_m
[INFO]    Path: models/deepseek-coder-v2-lite-instruct-q4_k_m.gguf
[INFO]    Size: 9884.28 MB
[INFO]    Valid: true
```

### HTTP API Server

Start the comprehensive HTTP API server with Engine integration and real-time logging:

**macOS:**
```bash
./build.sh run --arm64 llm run
```

**Ubuntu/Debian:**
```bash
./build_ubuntu.sh run --x86_64 llm run
```

**Available Endpoints:**
- `GET /health` - Server health check
- `GET /v1/models` - List available models
- `POST /v1/chat/completions` - Chat completions (OpenAI-compatible)
- `POST /stop` - Graceful server shutdown

**Server Features:**
- **Real-time Logging**: All requests and responses logged with timestamps
- **Engine Integration**: Periodic metadata transmission every 1 second
- **Graceful Shutdown**: Immediate termination via `/stop` endpoint
- **Multi-threaded**: Concurrent request handling
- **OpenAI Compatible**: Standard API format support
- **Cross-Platform**: Works identically on macOS and Ubuntu

**Example Usage:**
```bash
# Start the server (choose your platform)
./build.sh run --arm64 llm run              # macOS
./build_ubuntu.sh run --x86_64 llm run      # Ubuntu

# Health check
curl http://localhost:8080/health

# List models
curl http://localhost:8080/v1/models

# Chat completion
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "deepseek-coder", "messages": [{"role": "user", "content": "Hello"}]}'

# Stop server
curl -X POST http://localhost:8080/stop
```

## Configuration System

### Environment Variables

The system supports comprehensive environment variable configuration:

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `MODEL_PATH` | Full path to specific GGUF model file | - | `/path/to/model.gguf` |
| `DEFAULT_MODEL` | Filename of default model in models directory | - | `llama-2-7b-chat.q4_0.gguf` |
| `MODELS_DIR` | Path to models directory | `models` | `/custom/models` |
| `PREFER_QUANTIZED` | Prefer quantized models | `true` | `false` |
| `MAX_FILE_SIZE_GB` | Maximum model file size in GB | `20` | `50` |
| `MIN_FILE_SIZE_MB` | Minimum model file size in MB | `100` | `500` |

### Configuration Examples

```bash
# Use specific model (macOS)
export MODEL_PATH="/path/to/my-model.gguf"
./build.sh run --arm64 config_show

# Use specific model (Ubuntu)
export MODEL_PATH="/path/to/my-model.gguf"
./build_ubuntu.sh run --x86_64 config_show

# Set default model in models directory (macOS)
export DEFAULT_MODEL="deepseek-coder-v2-lite-instruct-q4_k_m.gguf"
./build.sh run --arm64 gguf_list

# Set default model in models directory (Ubuntu)
export DEFAULT_MODEL="deepseek-coder-v2-lite-instruct-q4_k_m.gguf"
./build_ubuntu.sh run --x86_64 gguf_list

# Custom models directory (macOS)
export MODELS_DIR="/custom/path/to/models"
./build.sh run --arm64 config_gen

# Custom models directory (Ubuntu)
export MODELS_DIR="/custom/path/to/models"
./build_ubuntu.sh run --x86_64 config_gen

# Customize preferences (works on both platforms)
export PREFER_QUANTIZED=false MAX_FILE_SIZE_GB=50
./build.sh run --arm64 config_show              # macOS
./build_ubuntu.sh run --x86_64 config_show      # Ubuntu
```

### Dynamic Configuration File

The system generates `models.json` automatically:

```json
{
  "engine_port": 18080,
  "model_path": "models/deepseek-coder-v2-lite-instruct-q4_k_m.gguf",
  "default_model": "",
  "model_directory": "models",
  "fallback_models": [
    "deepseek-coder-v2-lite-instruct-q4_k_m.gguf"
  ],
  "model_preferences": {
    "prefer_quantized": true,
    "max_file_size_gb": 20,
    "min_file_size_mb": 100
  },
  "environment_variables": {
    "model_path_var": "MODEL_PATH",
    "default_model_var": "DEFAULT_MODEL",
    "models_dir_var": "MODELS_DIR"
  }
}
```

## Advanced Features

### Debug Mode with Comprehensive Logging

**macOS:**
```bash
./build.sh debug --arm64
# Enables:
# - Verbose compilation output
# - Runtime debug logging  
# - Debug symbols for debugging
# - Memory safety checks
```

**Ubuntu/Debian:**
```bash
./build_ubuntu.sh debug --x86_64
# Enables:
# - Verbose compilation output
# - Runtime debug logging  
# - Debug symbols for debugging
# - Memory safety checks
```

### Cross-Platform and Cross-Architecture Building

**macOS:**
```bash
# Apple Silicon (M1/M2/M3)
./build.sh build --arm64
export CARGO_BUILD_TARGET=aarch64-apple-darwin

# Intel Macs
./build.sh build --x86_64  
export CARGO_BUILD_TARGET=x86_64-apple-darwin
```

**Ubuntu/Debian:**
```bash
# x86_64 Systems (Intel/AMD)
./build_ubuntu.sh build --x86_64
export CARGO_BUILD_TARGET=x86_64-unknown-linux-gnu

# ARM64 Systems (Raspberry Pi, ARM servers)
./build_ubuntu.sh build --arm64
export CARGO_BUILD_TARGET=aarch64-unknown-linux-gnu
```

### Model Filtering and Preferences
```bash
# Filter models by size (works on both platforms)
export MAX_FILE_SIZE_GB=10 MIN_FILE_SIZE_MB=500

# macOS
./build.sh run --arm64 config_gen

# Ubuntu
./build_ubuntu.sh run --x86_64 config_gen

# Prefer non-quantized models
export PREFER_QUANTIZED=false

# macOS
./build.sh run --arm64 gguf_list

# Ubuntu
./build_ubuntu.sh run --x86_64 gguf_list
```

### HTTP API Server Advanced Features
```bash
# Start server with custom port (default: 8080)
# macOS
./build.sh run --arm64 llm run

# Ubuntu
./build_ubuntu.sh run --x86_64 llm run

# Server automatically includes:
# - Engine metadata transmission (every 1 second)
# - Real-time request/response logging
# - Thread-safe multi-client support
# - Immediate shutdown capability
# - OpenAI-compatible API format

# Example API interactions:
curl http://localhost:8080/health
curl http://localhost:8080/v1/models
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "deepseek-coder", "messages": [{"role": "user", "content": "Hello"}]}'

# Graceful shutdown
curl -X POST http://localhost:8080/stop
```

## Development & Debugging

### Build System Features
- **Incremental Builds**: Only rebuilds changed components
- **Clean Operations**: `clean`, `reconfig`, `fresh` options
- **Warning-Free**: Eliminates all compiler warnings
- **Cross-Platform**: macOS (ARM64/x86_64) and Ubuntu (x86_64/ARM64) support
- **Auto-Dependencies**: Automatic installation of cbindgen and other tools

### Rust Implementation Details
- **150+ Mock Functions**: Complete LLM API compatibility
- **Memory Management**: Safe Rust memory handling
- **FFI Safety**: Proper C++ integration without undefined behavior
- **Modular Design**: Separate logging, model management, and core systems

### C++ Frontend Features  
- **Command-Line Interface**: Comprehensive argument parsing
- **Error Handling**: Graceful error reporting and recovery
- **Help System**: Built-in documentation and usage examples

## Contributing

### Development Workflow
1. **Fork the repository** and create a feature branch
2. **Make changes** following Rust and C++ best practices
3. **Test thoroughly** on both macOS (ARM64/x86_64) and Ubuntu (x86_64/ARM64)
4. **Update documentation** including README.md changes
5. **Submit pull request** with detailed description

### Code Standards
- **Rust**: Follow `rustfmt` and `clippy` recommendations
- **C++**: Use C++17 standards with RAII principles
- **Cross-Platform**: Ensure compatibility across macOS and Ubuntu
- **Documentation**: Maintain comprehensive inline comments
- **Testing**: Include both unit and integration tests

## Version Information

- **Current Version**: Development Branch `ubuntu_poc` (Latest cross-platform)
- **Stable Branches**: `llama-rs`, `llm_poc`
- **Rust Edition**: 2021
- **C++ Standard**: C++17
- **CMake Requirement**: 3.15+
- **Supported Platforms**: macOS (ARM64, x86_64), Ubuntu/Debian (x86_64, ARM64)
- **Target Platforms**: macOS (ARM64, x86_64), Ubuntu/Debian (x86_64, ARM64)

## Related Projects

- [GGML](https://github.com/ggerganov/ggml) - Machine learning tensor library
- [llama.cpp](https://github.com/ggerganov/llama.cpp) - Original C++ LLaMA implementation  
- [Rust ML](https://www.arewelearningyet.com/) - Rust machine learning ecosystem

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---