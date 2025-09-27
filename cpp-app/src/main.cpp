#include <iostream>
#include <string>
#include <cstring>
#include <sstream>

/**
 * 
 * Includes necessary headers for Rust integration, command-line argument parsing,
 * logging, and console management. These headers provide the declarations for functions
 * and types used in the main application logic.
 * 
 */
#include "llm_rust.h"
#include "cmd_args.h"
#include "log_cpp.h"
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
    // RAII guard to ensure log thread is properly cleaned up on exit
    logs::LogThreadGuard log_guard;

    // Check if this is a 'llm run' command early to disable verbose output
    bool is_llm_run_command = (argc >= 3 && 
                               std::string(argv[1]) == "llm" && 
                               std::string(argv[2]) == "run");
    
    // Check if this is any 'llm' command to skip parse_args
    bool is_llm_command = (argc >= 2 && std::string(argv[1]) == "llm");
    
    if (!is_llm_run_command) {
#ifdef __APPLE__
        LLMRC_PRINT_I("Running on macOS");
#elif __linux__
        LLMRC_PRINT_I("Running on Linux");
#else
        LLMRC_PRINT_I("Running on an unknown OS");
#endif
        rs_log_info("=== PROGRAM STARTING  ===");

#ifdef _DEBUG
        rs_log_debug("DEBUG MODE ENABLED - Compile-time debug flags are active");
        rs_log_debug("Compile-time debug flags are active");
#else
        rs_log_info("Production mode - debug output disabled");
#endif
    } else {
        // Disable Rust logging for clean animation
        rs_set_logging_enabled(false);
    }


    /**
     * This is the main entry point of the application.
     * It performs initial setup, prints information about the operating system,
     * and logs the program start. Depending on the build configuration,
     * it enables debug logging or switches to production mode.
     * Command-line arguments are parsed and handled, including displaying help information.
     * The program then validates the provided arguments and calls the Rust function
     * to continue execution. All major steps are logged for traceability.
     */
    
    // Skip parse_args for any 'llm' command to avoid "Unknown argument" message
    CmdArgs args;
    if (!is_llm_command) {
        args = parse_args(argc, argv);
    }
    
    // Handle help argument
    bool is_valid = false;
    const char *valid_args[] = {"--run", "-r", "--bench", "-b", "llm", "gguf_list", "config_gen", "config_help", "config_show", "config_validate"};

    // Handle help argument
    if (argc > 1 && (
        std::string(argv[1]) == "--help" ||
        std::string(argv[1]) == "-h" ||
        std::string(argv[1]) == "/?" )) {
        LLMRC_PRINT_I("Usage: %s [options]\n", argv[0]);
        LLMRC_PRINT_I("LlamaRS - Native Rust LLM Backend");
        LLMRC_PRINT_I("A comprehensive Rust implementation replacing llama.cpp dependencies\n");
        LLMRC_PRINT_I("Options:");
        LLMRC_PRINT_I("  --help, -h, /?       Show this help message and exit");
        LLMRC_PRINT_I("  --run, -r            Run in run mode (default)");
        LLMRC_PRINT_I("  llm [subcommand]     Run LLM system with integrated commands\n");
        LLMRC_PRINT_I("Subcommands:");
        LLMRC_PRINT_I("  gguf_list           List all available GGUF models");
        LLMRC_PRINT_I("LLM Subcommands:");
        LLMRC_PRINT_I("  llm run              Start HTTP API server for LLM inference (default)");
        LLMRC_PRINT_I("  llm list             List all available GGUF models");
        LLMRC_PRINT_I("  llm config_gen       Generate and validate model configuration");
        LLMRC_PRINT_I("  llm config_validate  Validate existing model configuration");
        LLMRC_PRINT_I("  llm config_show      Show current model configuration");
        LLMRC_PRINT_I("  llm config_help      Show environment variable configuration help\n");
        LLMRC_PRINT_I("Examples:");
        LLMRC_PRINT_I("  %s llm run          # Start HTTP API server (default)", argv[0]);
        LLMRC_PRINT_I("  %s llm list         # Show available models", argv[0]);
        LLMRC_PRINT_I("  %s llm config_gen   # Generate and validate config", argv[0]);
        LLMRC_PRINT_I("  %s llm config_validate  # Validate existing config", argv[0]);
        LLMRC_PRINT_I("  %s llm config_help  # Show env var help\n", argv[0]);
        LLMRC_PRINT_I("Model Directory: models/");
        LLMRC_PRINT_I("Supported: .gguf format models");
        return 0;
    }

    // Handle GGUF model listing
    if (argc > 1 && std::string(argv[1]) == "gguf_list") {
        rs_log_info("Listing GGUF Models");
        int model_count = list_gguf_models();
        rs_log_info(("Found " + std::to_string(model_count) + " GGUF models").c_str());
        return 0;
    }

    // Handle dynamic configuration generation
    if (argc > 1 && std::string(argv[1]) == "config_gen") {
        rs_log_info("Generating Dynamic Model Configuration");
        int result = rust_generate_and_validate_config();
        if (result == 0) {
            rs_log_info("Configuration generated successfully");
            
            // Perform secondary validation
            rs_log_info("Performing secondary validation in Rust...");
            int validation_result = rust_validate_model_config(nullptr);
            
            if (validation_result == 0) {
                rs_log_info("Secondary validation passed - Configuration is valid");
            } else if (validation_result == 1) {
                rs_log_info("Secondary validation completed with warnings - Check validation file");
            } else {
                rs_log_info("Secondary validation failed - Configuration may have issues");
            }
            
        } else {
            rs_log_info("Failed to generate configuration");
        }
        return result;
    }

    // Handle configuration display
    if (argc > 1 && std::string(argv[1]) == "config_show") {
        rs_log_info("Current Model Configuration");
        const char* config_json = get_model_config_json();
        LLMRC_PRINT_I("%s", config_json);
        return 0;
    }

    // Handle configuration help
    if (argc > 1 && std::string(argv[1]) == "config_help") {
        print_model_config_help();
        return 0;
    }

    // Handle configuration validation
    if (argc > 1 && std::string(argv[1]) == "config_validate") {
        rs_log_info("Validating Model Configuration");
        
        const char* config_path = "models.json";
        if (argc > 2) {
            config_path = argv[2];
        }
        
        int validation_result = rust_validate_model_config(config_path);
        
        if (validation_result == 0) {
            rs_log_info("Configuration validation passed - Configuration is valid");
        } else if (validation_result == 1) {
            rs_log_info("Configuration validation completed with warnings - Check validation file");
        } else {
            rs_log_info("Configuration validation failed - Configuration has errors");
        }
        
        return validation_result;
    }

    // Handle LLM system with integrated subcommands
    if (argc > 1 && std::string(argv[1]) == "llm") {
        if (argc == 2) {
            // Default: run LLM system
            rs_log_info("Running LLM System (default mode)");
            
            call_rsprintln(argc, argv);
            return 0;
        }
        
        std::string subcommand = argv[2];
        
        if (subcommand == "run") {
            rs_log_info("Running LLM System Engine");
            
            // Validate model configuration
            int validation_result = rust_validate_model_config("models.json");
            if (validation_result != 0) {
                rs_log_info("Model configuration validation failed");
                return validation_result;
            }
            rs_log_info("Model configuration validation passed");
            
            // Check model availability
            int model_count = list_gguf_models();
            if (model_count <= 0) {
                rs_log_info("No GGUF models found in models/ directory");
                return 1;
            }
            rs_log_info(("Found " + std::to_string(model_count) + " GGUF models").c_str());
            
            // System environment check
            CpuInfo info{};
            if (rust_get_cpu_info(&info)) {
                std::string cpu_info = std::string(reinterpret_cast<const char*>(info.brand)) + 
                                     " (" + std::to_string(info.cores) + " cores)";
                rs_log_info(("CPU: " + cpu_info).c_str());
            }
            
            rs_log_info("System environment check completed");
            rs_log_info("Starting LLM inference engine...");

            int engine_result = rust_run_llm_engine("models.json");
            if (engine_result != 0) {
                rs_log_info("LLM engine execution failed");
                return engine_result;
            }
            
            rs_log_info("LLM System Engine completed successfully");
            return 0;
        }


        else if (subcommand == "list") {
            rs_log_info("Listing GGUF Models (via LLM command)");
            int model_count = list_gguf_models();
            rs_log_info(("Found " + std::to_string(model_count) + " GGUF models").c_str());
            return 0;
        }


        else if (subcommand == "config_gen") {
            rs_log_info("Generating Dynamic Model Configuration (via LLM command)");
            int result = generate_model_config();
            if (result == 0) {
                rs_log_info("Configuration generated successfully");
                
                // Perform secondary validation using Rust
                rs_log_info("Performing secondary validation in Rust...");
                int validation_result = rust_validate_model_config("models.json");
                
                if (validation_result == 0) {
                    rs_log_info("Secondary validation passed - Configuration is valid");
                } else if (validation_result == 1) {
                    rs_log_info("Secondary validation completed with warnings - Check validation file");
                } else {
                    rs_log_info("Secondary validation failed - Configuration may have issues");
                }
                
                return validation_result;
            } else {
                rs_log_info("Failed to generate configuration");
            }
            return result;
        }


        else if (subcommand == "config_validate") {
            rs_log_info("Validating Model Configuration (via LLM command)");
            
            const char* config_path = "models.json";
            if (argc > 3) {
                config_path = argv[3];
            }
            
            int validation_result = rust_validate_model_config(config_path);
            
            if (validation_result == 0) {
                rs_log_info("Configuration validation passed - Configuration is valid");
            } else if (validation_result == 1) {
                rs_log_info("Configuration validation completed with warnings - Check validation file");
            } else {
                rs_log_info("Configuration validation failed - Configuration has errors");
            }
            
            return validation_result;
        }


        else if (subcommand == "config_show") {
            rs_log_info("Current Model Configuration (via LLM command)");
            const char* config_json = get_model_config_json();
            LLMRC_PRINT_I("%s", config_json);
            return 0;
        }


        else if (subcommand == "config_help") {
            rs_log_info("Model Configuration Help (via LLM command)");
            print_model_config_help();
            return 0;
        }


        else if (subcommand == "--help" || subcommand == "-h") {
            LLMRC_PRINT_I("LLM System Subcommands:\n");
            LLMRC_PRINT_I("Usage: %s llm [subcommand] [options]\n", argv[0]);
            LLMRC_PRINT_I("Subcommands:");
            LLMRC_PRINT_I("  run              Start HTTP API server for LLM inference (default)");
            LLMRC_PRINT_I("  list             List all available GGUF models");
            LLMRC_PRINT_I("  config_gen       Generate and validate model configuration");
            LLMRC_PRINT_I("  config_validate  Validate existing model configuration");
            LLMRC_PRINT_I("                   Optional: specify config file path as next argument");
            LLMRC_PRINT_I("  config_show      Show current model configuration");
            LLMRC_PRINT_I("  config_help      Show environment variable configuration help");
            LLMRC_PRINT_I("  --help, -h       Show this help message\n");
            LLMRC_PRINT_I("Examples:");
            LLMRC_PRINT_I("  %s llm              # Start HTTP API server (default)", argv[0]);
            LLMRC_PRINT_I("  %s llm run          # Start HTTP API server explicitly", argv[0]);
            LLMRC_PRINT_I("  %s llm list         # List available models", argv[0]);
            LLMRC_PRINT_I("  %s llm config_gen   # Generate configuration", argv[0]);
            LLMRC_PRINT_I("  %s llm config_validate  # Validate default config", argv[0]);
            LLMRC_PRINT_I("  %s llm config_validate /path/to/config.json  # Validate custom config", argv[0]);
            return 0;
        }


        else {
            rs_log_info(("Unknown LLM subcommand: " + subcommand).c_str());
            LLMRC_PRINT_W("Use '%s llm --help' to see available subcommands.", argv[0]);
            return 1;
        }

    } ///< if (argc > 1 && std::string(argv[1]) == "llm")

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

    // LogThreadGuard destructor will automatically clean up the log thread
    return 0;
}