# Build & Run Guide

## 간단한 실행 방법 (권장)

```bash
./run.sh                    # release + arm64
./run.sh debug              # debug + arm64  
./run.sh release x86_64     # release + x86_64
./run.sh debug x86_64       # debug + x86_64
```

## 수동 빌드/실행

### 개발 모드 (Debug)
```bash
./build.sh debug            # Debug 빌드 (기본: arm64)
./build.sh debug x86_64     # Debug 빌드 (Intel)
./output/bin/cpp_app llm    # llm 버전 실행
./output/bin/cpp_app llmrust # llmrust 버전 실행
```

### 릴리즈 모드 (Release)
```bash
./build.sh                  # Release 빌드 (기본: arm64)  
./build.sh x86_64           # Release 빌드 (Intel)
./output/bin/cpp_app llm    # llm 버전 실행
./output/bin/cpp_app llmrust # llmrust 버전 실행
```

### run.sh 사용법
```bash
# 기본 사용법 (Release 모드)
./run.sh                    # clean + build + run with llm (arm64)
./run.sh x86_64             # clean + build + run with llm (Intel)

# Debug 모드 사용법  
./run.sh debug              # clean + debug + run with llm (arm64)
./run.sh debug x86_64       # clean + debug + run with llm (Intel)

# Release 모드 명시적 사용법
./run.sh release            # clean + build + run with llm (arm64)
./run.sh release x86_64     # clean + build + run with llm (Intel)

# 도움말
./run.sh -h                 # 사용법 출력
./run.sh --help             # 사용법 출력
```

## 쉽게 테스트 한 번에 하기
1) Debug 모드
>  % ./build.sh clean --arm64 && ./build.sh debug --arm64 && ./build.sh run --arm64 llm

2) Realase 모드
>  % ./build.sh clean --arm64 && ./build.sh build --arm64 && ./build.sh run --arm64 llm

## 빌드 출력 예시

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

## 명령어 가이드

### 간단한 방법 (run.sh)
| 작업                    | 명령어               | 설명                           |
|-------------------------|---------------------|--------------------------------|
| 전체 파이프라인 (기본)    | `./run.sh`          | clean + build + run llm (arm64) |
| Intel 아키텍처 실행      | `./run.sh x86_64`   | clean + build + run llm (x86_64) |

### 상세 제어 (build.sh)
| 작업         | 명령어 (Apple Silicon)         | 명령어 (Intel)             |
|--------------|-------------------------------|----------------------------|
| 빌드         | `./build.sh build --arm64`    | `./build.sh build --x86_64`|
| 실행 (기본)   | `./build.sh run --arm64`      | `./build.sh run --x86_64`  |
| 실행 (LLM)   | `./build.sh run --arm64 llm`  | `./build.sh run --x86_64 llm`|
| 클린         | `./build.sh clean --arm64`    | `./build.sh clean --x86_64`|
| 디버그       | `./build.sh debug --arm64`     | `./build.sh debug --x86_64` |
| 재설정       | `./build.sh fresh --arm64`    | `./build.sh fresh --x86_64`|
| 재설정(강제) | `./build.sh reconfig --arm64` | `./build.sh reconfig --x86_64`|

> **Tip:** 아키텍처를 변경하거나 Rust 코드를 업데이트할 때는 항상 클린 후 빌드하세요.

---

## 사용법

### run.sh (간단한 파이프라인)
```
Usage: ./run.sh [arch]
  arch: arm64 (default) or x86_64

Examples:
  ./run.sh          # clean + build + run with llm (arm64)
  ./run.sh arm64     # 명시적으로 arm64 지정
  ./run.sh x86_64    # Intel 아키텍처로 실행
```

### build.sh (상세 제어)
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

> **Tip:** 아키텍처를 변경하거나 Rust 코드를 업데이트할 때는 항상 클린 후 빌드하세요.

## 참고사항

### 추천 워크플로우
1. **일반 사용**: `./run.sh` - 모든 과정이 자동으로 진행됩니다
2. **개발 중**: `./build.sh clean --arm64 && ./build.sh build --arm64` 후 테스트
3. **디버깅**: `./build.sh run --arm64 [args...]`로 다양한 인자 테스트

### 주요 정보
- **빌드 결과물**: `output/bin/llmrcpp_app`에 생성
- **아키텍처**: arm64 (Apple Silicon) 기본, x86_64 지원
- **통합 환경**: C++와 Rust가 통합된 하이브리드 프로젝트
- **LLM 기능**: `llm` 인자로 특별한 Rust LLM 함수 호출 가능

### 문제 해결
- 아키텍처 변경 시: 반드시 clean 후 빌드
- Rust 코드 변경 시: clean 후 빌드 권장
- 빌드 오류 시: `./build.sh fresh --arm64`로 완전 재설정
