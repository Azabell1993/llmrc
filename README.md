# 🦀 LLM Rust - Comprehensive GGUF Model Management System
> **Advanced Rust-based LLM Backend with Dynamic Model Discovery and Configuration**

[![macOS](https://img.shields.io/badge/macOS-ARM64%20%7C%20x86__64-success.svg?logo=apple)](https://www.apple.com/macos/)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![C++](https://img.shields.io/badge/C%2B%2B-17-blue.svg?logo=cplusplus)](https://isocpp.org/)
[![GGUF](https://img.shields.io/badge/GGUF-Model%20Support-green.svg)](https://github.com/ggerganov/ggml)
[![MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## 👨‍💻 Developer Information
- **Developer:** Azabell1993
- **Contact:** [![Email](https://img.shields.io/badge/Email-azabell1993%40gmail.com-blue?logo=gmail)](mailto:azabell1993@gmail.com)  
- **GitHub:** [![GitHub](https://img.shields.io/badge/GitHub-Repository-black?logo=github)](https://github.com/Azabell1993/llmrc)
- **Branch:** `llama-rs`

For questions, issues, or contributions, please open an issue or pull request on GitHub.

## 🌟 Project Overview

**LLM Rust** is a comprehensive Large Language Model management system designed for macOS, featuring dynamic GGUF model discovery, configuration management, and a powerful build system. This project provides a complete replacement for traditional llama.cpp dependencies while offering superior performance and reliability.

### 🎯 Why Choose LLM Rust?

- **🔧 Zero Configuration Hassles**: Automatic model discovery and configuration
- **📦 Self-Contained System**: No external llama.cpp dependencies required
- **⚡ Lightning Fast Builds**: Optimized Rust compilation with minimal overhead
- **🎛️ Dynamic Configuration**: Runtime environment variable support
- **🏗️ Professional Build System**: Comprehensive CMake + Cargo integration
- **🔍 Smart Model Management**: Automatic GGUF validation and metadata extraction

## ✨ Key Features

### 🚀 **Dynamic Model Management**
- **Auto-Discovery**: Automatic scanning and validation of GGUF models
- **Smart Configuration**: Dynamic `models.json` generation based on available models
- **Environment Integration**: Full environment variable configuration support
- **Flexible Filtering**: Size-based model filtering and quantization preferences

### 🔧 **Advanced Build System**
- **Cross-Architecture**: Native ARM64 (Apple Silicon) and x86_64 support
- **Debug Mode**: Comprehensive logging and debug symbol generation
- **Clean Operations**: Intelligent build artifact cleanup
- **Fresh Rebuilds**: Complete environment reset capabilities

### 📊 **Model Configuration System**
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

### 🎮 **Command Interface**
- **Model Listing**: `gguf_list` - Display all available models with metadata
- **Config Generation**: `config_gen` - Create dynamic configuration files
- **Config Display**: `config_show` - View current configuration as JSON
- **Help System**: `config_help` - Environment variable documentation

## 🏗️ System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                          LLM Rust Ecosystem                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌──────────────────┐   ┌─────────────────────┐   ┌────────────────────────────┐ │
│  │   Build System   │◄─►│   Rust LLM Core     │◄─►│     Model Management       │ │
│  │   • CMake + Cargo│   │   • GGUF Support    │   │   • Auto-Discovery         │ │
│  │   • Cross-Arch   │   │   • 150+ Functions  │   │   • Dynamic Config         │ │
│  │   • Debug Mode   │   │   • Mock Backend    │   │   • Environment Variables  │ │
│  └──────────────────┘   └─────────────────────┘   └────────────────────────────┘ │
│                                                                                 │
│  ┌──────────────────┐   ┌─────────────────────┐   ┌────────────────────────────┐ │
│  │   C++ Frontend   │◄─►│   Configuration     │◄─►│        GGUF Models         │ │
│  │   • CLI Interface│   │   • JSON Config     │   │   • Automatic Validation   │ │
│  │   • Command Args │   │   • Env Variables   │   │   • Metadata Extraction    │ │
│  │   • Help System  │   │   • Runtime Setup   │   │   • Smart Filtering        │ │
│  └──────────────────┘   └─────────────────────┘   └────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites
- **macOS** (ARM64 or x86_64)
- **Rust** 1.70+ (stable)
- **CMake** 3.15+
- **Clang/C++** 17+

### Installation & Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Azabell1993/llmrc.git
   cd llmrc
   git checkout llama-rs
   ```

2. **Place GGUF models:**
   ```bash
   mkdir -p models
   # Copy your .gguf model files to the models/ directory
   cp your-model.gguf models/
   ```

3. **Build the system:**
   ```bash
   ./build.sh build --arm64    # For Apple Silicon
   ./build.sh build --x86_64   # For Intel Macs
   ```

4. **List available models:**
   ```bash
   ./build.sh run --arm64 gguf_list
   ```

## 📖 Usage Guide

### Build System Commands

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

# System Operations  
./build.sh run --arm64 llm                  # Run LLM system
./build.sh --help                           # Show comprehensive help
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

## ⚙️ Configuration System

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
# Use specific model
export MODEL_PATH="/path/to/my-model.gguf"
./build.sh run --arm64 config_show

# Set default model in models directory
export DEFAULT_MODEL="deepseek-coder-v2-lite-instruct-q4_k_m.gguf"
./build.sh run --arm64 gguf_list

# Custom models directory
export MODELS_DIR="/custom/path/to/models"
./build.sh run --arm64 config_gen

# Customize preferences
export PREFER_QUANTIZED=false MAX_FILE_SIZE_GB=50
./build.sh run --arm64 config_show
```

### Dynamic Configuration File

The system generates `models.json` automatically:

```json
{
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

## 📁 Project Structure

```
llm_rust/
├── 📁 models/                      # GGUF model files (.gguf format)
│   └── deepseek-coder-v2-lite-instruct-q4_k_m.gguf
├── 📄 models.json                  # Dynamic model configuration (auto-generated)
├── 📁 rustlib/                     # Rust backend implementation
│   ├── 📁 src/
│   │   ├── lib.rs                  # Main library entry point
│   │   └── 📁 common/
│   │       ├── log.rs              # Logging and utility functions
│   │       └── model.rs            # GGUF model management system
│   └── Cargo.toml                  # Rust dependencies and configuration
├── 📁 cpp-app/                     # C++ frontend application
│   ├── 📁 src/
│   │   └── main.cpp                # Main application entry point
│   ├── 📁 include/
│   │   ├── llm_rust.h              # Auto-generated C header (cbindgen)
│   │   ├── cmd_args.h              # Command-line argument parsing
│   │   └── rust_utils.h            # Rust utility function declarations
│   └── CMakeLists.txt              # CMake build configuration
├── 📁 output/                      # Build artifacts and executables
│   ├── 📁 bin/
│   │   └── llmrcpp_app             # Final executable
│   ├── 📁 lib/                     # Static libraries
│   ├── 📁 obj/                     # Object files
│   └── 📁 include/                 # Generated headers
├── 🔧 build.sh                     # Comprehensive build system
├── 🚀 run.sh                       # Quick run script
├── ⚙️ cbindgen.toml                # C header generation configuration
└── 📖 README.md                    # This documentation
```

## 🔧 Advanced Features

### Debug Mode with Comprehensive Logging
```bash
./build.sh debug --arm64
# Enables:
# - Verbose compilation output
# - Runtime debug logging  
# - Debug symbols for debugging
# - Memory safety checks
```

### Cross-Architecture Building
```bash
# Apple Silicon (M1/M2/M3)
./build.sh build --arm64
export CARGO_BUILD_TARGET=aarch64-apple-darwin

# Intel Macs
./build.sh build --x86_64  
export CARGO_BUILD_TARGET=x86_64-apple-darwin
```

### Model Filtering and Preferences
```bash
# Filter models by size
export MAX_FILE_SIZE_GB=10 MIN_FILE_SIZE_MB=500
./build.sh run --arm64 config_gen

# Prefer non-quantized models
export PREFER_QUANTIZED=false
./build.sh run --arm64 gguf_list
```

## 🔍 Development & Debugging

### Build System Features
- **Incremental Builds**: Only rebuilds changed components
- **Clean Operations**: `clean`, `reconfig`, `fresh` options
- **Warning-Free**: Eliminates all compiler warnings
- **Cross-Platform**: ARM64 and x86_64 support

### Rust Implementation Details
- **150+ Mock Functions**: Complete LLM API compatibility
- **Memory Management**: Safe Rust memory handling
- **FFI Safety**: Proper C++ integration without undefined behavior
- **Modular Design**: Separate logging, model management, and core systems

### C++ Frontend Features  
- **Command-Line Interface**: Comprehensive argument parsing
- **Error Handling**: Graceful error reporting and recovery
- **Help System**: Built-in documentation and usage examples

## 🤝 Contributing

### Development Workflow
1. **Fork the repository** and create a feature branch
2. **Make changes** following Rust and C++ best practices
3. **Test thoroughly** on both ARM64 and x86_64
4. **Update documentation** including README.md changes
5. **Submit pull request** with detailed description

### Code Standards
- **Rust**: Follow `rustfmt` and `clippy` recommendations
- **C++**: Use C++17 standards with RAII principles
- **Documentation**: Maintain comprehensive inline comments
- **Testing**: Include both unit and integration tests

## 📝 Version Information

- **Current Version**: Development Branch `llama-rs`
- **Rust Edition**: 2021
- **C++ Standard**: C++17
- **CMake Requirement**: 3.15+
- **Target Platforms**: macOS ARM64, macOS x86_64

## 🔗 Related Projects

- [GGML](https://github.com/ggerganov/ggml) - Machine learning tensor library
- [llama.cpp](https://github.com/ggerganov/llama.cpp) - Original C++ LLaMA implementation  
- [Rust ML](https://www.arewelearningyet.com/) - Rust machine learning ecosystem

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---