#include <iostream>
#include <string>
#include <cstring>

#include "llm_rust.h"
#include "cmd_args.h"

int call_rust_func(int argc, char* argv[]) {
#ifdef _DEBUG
    std::cout << "[DEBUG] Entering call_rust_func()" << std::endl;

    std::cout << "Hello C++!" << std::endl;
    rust_func();
#endif

    if (argc > 1 && std::string(argv[1]) == "llm") {
        rust_llm();
    } else if (argc > 1 && std::string(argv[1]) == "llmrust") {
        llmrust_hello();
    } else {
        std::cout << "Skipping rust_llm function call." << std::endl;
    }

#ifdef _DEBUG
    std::cout << "[DEBUG] Fetching CPU info from Rust..." << std::endl;
    CpuInfo info{};
    if (rust_get_cpu_info(&info)) {
        std::cout << "[CPU INFO]" << std::endl;
        std::cout << "  Cores:   " << info.cores << std::endl;
        std::cout << "  Logical: " << info.logical << std::endl;
        std::cout << "  Freq:    " << info.freq_mhz << " MHz" << std::endl;
        std::cout << "  Brand:   " << reinterpret_cast<const char*>(info.brand) << std::endl;
    } else {
        std::cerr << "Failed to get CPU info from Rust" << std::endl;
    }
#endif

    char brand_buf[64];
    size_t n = rust_get_cpu_brand(reinterpret_cast<uint8_t*>(brand_buf), sizeof(brand_buf));
    if (n > 0) {
        std::cout << "[CPU BRAND SHORT] " << brand_buf << " (" << n << " bytes)" << std::endl;
    }

#ifdef _DEBUG
    std::cout << "[DEBUG] Exiting call_rust_func()" << std::endl;
#endif
    return 0;
}

int main(int argc, char* argv[]) {
#ifdef __APPLE__
    std::cout << "Running on macOS" << std::endl;
#elif __linux__
    std::cout << "Running on Linux" << std::endl;
#else
    std::cout << "Running on an unknown OS" << std::endl;
#endif

#ifdef _DEBUG
    start_log_thread();
    atexit(stop_log_thread);
#endif

    CmdArgs args = parse_args(argc, argv);

    // 도움말 인자 처리
    if (argc > 1 && (
        std::string(argv[1]) == "--help" ||
        std::string(argv[1]) == "-h" ||
        std::string(argv[1]) == "/?" )) {
        std::cout << "Usage: " << argv[0] << " [options]\n";
        std::cout << "Options:\n";
        std::cout << "  --help, -h, /?       Show this help message and exit\n";
        std::cout << "  --run, -r            Run in run mode (default)\n";
        return 0;
    }

    const char *valid_args[] = {"--run", "-r", "--bench", "-b"};
    bool is_valid = false;
    for (const char* valid_arg : valid_args) {
        if (argc > 1 && std::string(argv[1]) == valid_arg) {
            is_valid = true;
            break;
        }
        if(!is_valid && argc > 2 && std::string(argv[1]) == valid_arg) {
            is_valid = true;
            break;
        }
    }

    call_rust_func(argc, argv);

    return 0;
}