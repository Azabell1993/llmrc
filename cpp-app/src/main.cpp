#include <iostream>
#include <string>
#include <cstring>

/**
 * 
 * Includes necessary headers for Rust integration, command-line argument parsing,
 * and utility functions. These headers provide the declarations for functions
 * and types used in the main application logic.
 * 
 */
#include "llm_rust.h"
#include "cmd_args.h"
#include "rust_utils.h"


/**
 * 
 * Main function of the C++ application.
 * It initializes the application, processes command-line arguments,
 * and calls the Rust functions to perform the main logic.
 * Extensive logging is performed to trace execution flow and state.
 * @return Exit code from the Rust function or 0 on success.
 * 
 */
int main(int argc, char* argv[]) {
    

#ifdef __APPLE__
    std::cout << "Running on macOS" << std::endl;
#elif __linux__
    std::cout << "Running on Linux" << std::endl;
#else
    std::cout << "Running on an unknown OS" << std::endl;
#endif
    rs_log_info("=== PROGRAM STARTING  ===");



#ifdef _DEBUG
    rs_log_debug("🐛 DEBUG MODE ENABLED 🐛");
    rs_log_debug("Compile-time debug flags are active");
#else
    rs_log_info("Production mode - debug output disabled");
#endif


    /**
     * This is the main entry point of the application.
     * It performs initial setup, prints information about the operating system,
     * and logs the program start. Depending on the build configuration,
     * it enables debug logging or switches to production mode.
     * Command-line arguments are parsed and handled, including displaying help information.
     * The program then validates the provided arguments and calls the Rust function
     * to continue execution. All major steps are logged for traceability.
     */
    CmdArgs args = parse_args(argc, argv);
    
    // Handle help argument
    bool is_valid = false;
    const char *valid_args[] = {"--run", "-r", "--bench", "-b"};

    // Handle help argument
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

    /**
     * Validate command-line arguments and log errors if invalid.
     * If invalid arguments are detected, display an error message and exit.
     */
    call_rsprintln(argc, argv);

    return 0;
}