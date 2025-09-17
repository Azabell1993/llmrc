# Build & Run Guide (C++/Rust Integration)

## Build Output Example

```
-- Check for working CXX compiler: /usr/bin/c++ - skipped
-- Detecting CXX compile features
-- Detecting CXX compile features - done
-- Configuring done (1.9s)
-- Generating done (0.0s)
-- Build files have been written to: /Users/mac/Desktop/workspace/llm_rust/cpp-app/build
==> Building project [arm64]...
[ 50%] Building Rust crate: llm_rust
[ 50%] Generating C header via cbindgen: /Users/mac/Desktop/workspace/llm_rust/cpp-app/include/llm_rust.h
    Compiling llm_rust v0.1.0 (/Users/mac/Desktop/workspace/llm_rust/rustlib)
     Finished `release` profile [optimized] target(s) in 0.17s
[ 50%] Built target llm_rust_rust
[ 75%] Building CXX object CMakeFiles/cpp_app.dir/src/hello.cpp.o
[100%] Linking CXX executable run_cpp_rust
[100%] Built target cpp_app
```

## 실행 예시

```
==> Running run_cpp_rust [arm64]
Hello C++!
Hello Rust!
```

---

## 빌드/실행/클린/재설정 명령어

| 작업         | 명령어 (Apple Silicon)         | 명령어 (Intel)             |
|--------------|-------------------------------|----------------------------|
| 빌드         | `./build.sh build --arm64`    | `./build.sh build --x86_64`|
| 실행         | `./build.sh run --arm64`      | `./build.sh run --x86_64`  |
| 클린         | `./build.sh clean --arm64`    | `./build.sh clean --x86_64`|
| 재설정       | `./build.sh fresh --arm64`    | `./build.sh fresh --x86_64`|
| 재설정(강제) | `./build.sh reconfig --arm64` | `./build.sh reconfig --x86_64`|

> **Tip:** 아키텍처를 변경하거나 Rust 코드를 업데이트할 때는 항상 클린 후 빌드하세요.

---

## 사용법

```
Usage: ./build.sh [build|run|clean|reconfig|fresh] <--arm64|--x86_64>
  build     Configure and build with CMake (Rust build is triggered inside CMake)
  run       Run the built binary
  clean     Clean CMake build dir + Rust targets (clean-all)
  reconfig  Force reconfigure
  fresh     Remove build dir and reconfigure from scratch
  --arm64   Force build for Apple Silicon (Rust + CMake)
  --x86_64  Force build for Intel (Rust + CMake)
```

---

## 참고

- 빌드 전 `./build.sh clean --arm64` 또는 `./build.sh clean --x86_64` 실행을 권장합니다.
- 빌드 결과물은 `output` 디렉토리에 생성됩니다.
- C++와 Rust가 통합된 프로젝트입니다.

