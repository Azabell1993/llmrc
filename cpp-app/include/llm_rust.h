#ifndef LLM_RUST_H
#define LLM_RUST_H

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct CpuInfo {
  uint32_t cores;
  uint32_t logical;
  uint64_t freq_mhz;
  uint8_t brand[128];
} CpuInfo;

// Test function to verify Rust integration
void rust_llm(void);
void rust_func(void);
bool rust_get_cpu_info(struct CpuInfo *out);
uintptr_t rust_get_cpu_brand(uint8_t *buf, uintptr_t buf_len);
void llmrust_hello(void);


// Logging functions
void rs_log_info(const char *msg);
void rs_log_warn(const char *msg);
void rs_log_error(const char *msg);
void rs_log_debug(const char *msg);
void rs_log_trace(const char *msg);
int rust_entry(int argc, char **argv);



#ifdef __cplusplus
}
#endif

#endif  /* LLM_RUST_H */
