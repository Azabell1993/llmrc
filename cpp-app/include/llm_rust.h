#ifndef LLM_RUST_H
#define LLM_RUST_H

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

<<<<<<< HEAD
void rust_func(void);
=======
extern "C" {
    void rust_func();
    void rust_llm();
    void llmrust_hello();

    struct CpuInfo {
        uint32_t cores;
        uint32_t logical;
        uint64_t freq_mhz;
        uint8_t  brand[128];
    };

    bool rust_get_cpu_info(CpuInfo* out);
    size_t rust_get_cpu_brand(uint8_t* buf, size_t len);
}
>>>>>>> llm_poc

#endif  /* LLM_RUST_H */
