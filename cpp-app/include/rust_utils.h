#ifndef RISCV_NTLH_H
#define RISCV_NTLH_H

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string>
#include "cmd_args.h"
#include "llm_rust.h"

// Forward declarations for functions used
extern void rs_log_info(const char*);
extern void rs_log_debug(const char*);
extern void rs_log_error(const char*);
extern int rust_entry(int argc, char* argv[]);
extern int rust_check_make(int argc, char* argv[]);

/**
 * @brief Call Rust println function
 * * This function initializes the Rust environment and calls the main Rust entry point.
 * * It also performs argument validation and logging.
 * @param[in] argc Argument count
 * @param[in] argv Argument vector
 * @return Exit code from Rust function or 0 on success
 *  @retval 0 Success
 */
#ifdef __cplusplus
extern "C" {
#endif

inline int call_rsprintln(int argc, char* argv[]) {

    // Log function entry
    rs_log_info("Entering call_rsprintln()");
    std::string log_msg = std::string(__func__) + ": ** Engine init **";
    rs_log_info(log_msg.c_str());

    std::string log_msg_init = std::string(__func__) + ": Initializing logging system...";
    rs_log_info(log_msg_init.c_str());

#ifdef _DEBUG
    std::string debug_msg = std::string(__func__) + ": Debug mode active";
    rs_log_debug(debug_msg.c_str());
#endif

    if (argc > 1 && std::string(argv[1]) == "llm") {
        std::string log_msg2 = std::string(__func__) + ": Starting full LLM system";
        rs_log_info(log_msg2.c_str());

        rs_log_info("   ** Listing GGUF Models");
        int model_count = list_gguf_models();
        rs_log_info(("Found " + std::to_string(model_count) + " GGUF models").c_str());
        
        int result = rust_entry(argc, argv);
        rs_log_info("   ** LLM system execution completed");
        return result;

    } else if (argc > 1 && std::string(argv[1]) == "llmrust") { // Basic test command
        rs_log_info(" [*] Running basic llmrust test command");
        llmrust_hello();
    } else { // No LLM system, basic mode
        //< -- Basic mode without LLM system
        rs_log_info(" [*] No LLM system command detected");
        rs_log_info(" [!!] Running in basic mode - no LLM system execution");
    }

    rs_log_debug("Fetching CPU info from Rust...");
    CpuInfo info{};
    if (rust_get_cpu_info(&info)) {
        rs_log_info("[CPU INFO]");
        std::string cores_msg = "  Cores: " + std::to_string(info.cores);
        rs_log_info(cores_msg.c_str());
        std::string logical_msg = "  Logical: " + std::to_string(info.logical);
        rs_log_info(logical_msg.c_str());
        std::string freq_msg = "  Freq: " + std::to_string(info.freq_mhz) + " MHz";
        rs_log_info(freq_msg.c_str());
        std::string brand_msg = "  Brand: " + std::string(reinterpret_cast<const char*>(info.brand));
        rs_log_info(brand_msg.c_str());
    } else {
        rs_log_error("Failed to get CPU info from Rust");
    }

    char brand_buf[64] = {0};
    size_t n = rust_get_cpu_brand(reinterpret_cast<uint8_t*>(brand_buf), sizeof(brand_buf));
    if (n > 0) {
        std::string brand_short_msg = "[CPU BRAND SHORT] " + std::string(brand_buf, n) + " (" + std::to_string(n) + " bytes)";
        rs_log_info(brand_short_msg.c_str());
    }

#ifdef _DEBUG
    rs_log_debug("Exiting call_rsprintln()");
#endif
    return 0;
}

#ifdef __cplusplus
}
#endif

#endif // RISCV_NTLH_H
