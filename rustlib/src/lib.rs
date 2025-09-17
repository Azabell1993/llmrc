mod rust_llm;

// llmrust 디렉터리를 모듈로 선언
#[path = "../llmrust/llmrust.rs"]
mod llmrust;

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

// llmrust 모듈에서 함수 export
pub use llmrust::llmrust_hello;
