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
    const char *valid_args[] = {"--run", "-r", "--bench", "-b", "llm", "gguf_list", "config_gen", "config_help", "config_show"};

    // Handle help argument
    if (argc > 1 && (
        std::string(argv[1]) == "--help" ||
        std::string(argv[1]) == "-h" ||
        std::string(argv[1]) == "/?" )) {
        std::cout << "Usage: " << argv[0] << " [options]\n\n";
        std::cout << "LlamaRS - Native Rust LLM Backend\n";
        std::cout << "A comprehensive Rust implementation replacing llama.cpp dependencies\n\n";
        std::cout << "Options:\n";
        std::cout << "  --help, -h, /?       Show this help message and exit\n";
        std::cout << "  --run, -r            Run in run mode (default)\n";
        std::cout << "  llm                  Run LLM system execution\n\n";
        std::cout << "GGUF Model Commands:\n";
        std::cout << "  gguf_list            List all available GGUF models in models/ directory\n";
        std::cout << "  config_gen           Generate dynamic model configuration\n";
        std::cout << "  config_show          Show current model configuration\n";
        std::cout << "  config_help          Show environment variable configuration help\n\n";
        std::cout << "Examples:\n";
        std::cout << "  " << argv[0] << " gguf_list      # Show available models\n";
        std::cout << "  " << argv[0] << " config_gen     # Generate dynamic config\n";
        std::cout << "  " << argv[0] << " config_help    # Show env var help\n";
        std::cout << "  " << argv[0] << " llm            # Run LLM system\n\n";
        std::cout << "Model Directory: models/\n";
        std::cout << "Supported: .gguf format models\n";
        return 0;
    }

    // Handle GGUF model listing
    if (argc > 1 && std::string(argv[1]) == "gguf_list") {
        rs_log_info("📋 Listing GGUF Models");
        int model_count = list_gguf_models();
        rs_log_info(("Found " + std::to_string(model_count) + " GGUF models").c_str());
        return 0;
    }

    // Handle dynamic configuration generation
    if (argc > 1 && std::string(argv[1]) == "config_gen") {
        rs_log_info("🔧 Generating Dynamic Model Configuration");
        int result = generate_model_config();
        if (result == 0) {
            rs_log_info("✅ Configuration generated successfully");
        } else {
            rs_log_info("❌ Failed to generate configuration");
        }
        return result;
    }

    // Handle configuration display
    if (argc > 1 && std::string(argv[1]) == "config_show") {
        rs_log_info("📄 Current Model Configuration");
        const char* config_json = get_model_config_json();
        std::cout << config_json << std::endl;
        return 0;
    }

    // Handle configuration help
    if (argc > 1 && std::string(argv[1]) == "config_help") {
        print_model_config_help();
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