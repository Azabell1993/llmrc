mod rust_llm;

#[path = "../llmrust/llmrust.rs"]
mod llmrust;

#[path = "../common/mod.rs"]
mod common;

#[path = "../engine/mod.rs"]
mod engine;



pub use rust_llm::rust_llm;
pub use rust_llm::rust_func;
pub use rust_llm::CpuInfo;
pub use rust_llm::checked_add_i64;
pub use rust_llm::checked_sub_i64;
pub use rust_llm::checked_mul_i64;
pub use rust_llm::checked_div_i64;
pub use rust_llm::rust_get_cpu_info;
pub use rust_llm::rust_get_cpu_brand;
pub use rust_llm::cpu_info_platform;

pub use llmrust::llmrust_hello;

pub use common::log::*;
pub use common::utils::*;
pub use common::model::{
    llama_model_load_from_file, llama_init_from_model, llama_model_free, llama_free,
    llama_model_default_params, llama_context_default_params, common_init_from_params_enhanced,
    common_model_params_to_llama, common_context_params_to_llama, get_model_endpoint,
    llama_model_params, llama_context_params, ggml_threadpool_params, lora_adapter,
    // GGUF-specific functions
    init_gguf_model_auto, init_gguf_model_c, list_gguf_models, scan_models_directory,
    get_gguf_info, GgufInfo, llama_token, gguf_initialization
};

pub use engine::*;
