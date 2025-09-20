mod rust_llm;

#[path = "../llmrust/llmrust.rs"]
mod llmrust;

#[path = "../common/mod.rs"]
mod common;

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
