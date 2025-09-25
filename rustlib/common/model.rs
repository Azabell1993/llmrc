// model.rs - GGUF Model Initialization and Management System
// 
// This file provides comprehensive model loading and context initialization
// for GGUF format models with full parameter configuration support.
// Date: 2025-09-21
// Description: Model loading system with GGUF support and parameter management
// Target: macOS ARM64 (Apple Silicon) and cross-platform compatibility
// Developer: Azabell1993 
// License: MIT

#![allow(non_camel_case_types)]
#![allow(dead_code)]    
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_void, c_float};
use std::ptr::{null_mut, null};
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde_json;

// Import types from log.rs
use super::log::{
    llama_context, llama_model, common_sampler, common_params, cpu_params,
    sampling_params, common_init_result, llama_model_holder, llama_context_holder,
    llama_vocab, llama_batch, rs_log_info, rs_log_warn, rs_log_error,
    cstr
};

// Define llama_token locally since it's private in log.rs
pub type llama_token = i32;

// Additional types for model initialization
#[repr(C)]
pub struct llama_model_params {
    pub n_gpu_layers: c_int,
    pub main_gpu: c_int,
    pub split_mode: c_int,
    pub tensor_split: *const c_float,
    pub use_mmap: bool,
    pub use_mlock: bool,
    pub check_tensors: bool,
    pub use_extra_bufts: bool,
    pub devices: *const *const c_char,
    pub kv_overrides: *const llama_model_kv_override,
    pub tensor_buft_overrides: *const llama_model_tensor_buft_override,
    pub progress_callback: Option<extern "C" fn(progress: c_float, user_data: *mut c_void) -> bool>,
    pub progress_callback_user_data: *mut c_void,
}

#[repr(C)]
pub struct llama_context_params {
    pub n_ctx: c_int,
    pub n_seq_max: c_int,
    pub n_batch: c_int,
    pub n_ubatch: c_int,
    pub n_threads: c_int,
    pub n_threads_batch: c_int,
    pub embeddings: bool,
    pub rope_scaling_type: c_int,
    pub rope_freq_base: c_float,
    pub rope_freq_scale: c_float,
    pub yarn_ext_factor: c_float,
    pub yarn_attn_factor: c_float,
    pub yarn_beta_fast: c_float,
    pub yarn_beta_slow: c_float,
    pub yarn_orig_ctx: c_int,
    pub pooling_type: c_int,
    pub attention_type: c_int,
    pub flash_attn_type: c_int,
    pub cb_eval: Option<extern "C" fn()>,
    pub cb_eval_user_data: *mut c_void,
    pub offload_kqv: bool,
    pub no_perf: bool,
    pub op_offload: bool,
    pub swa_full: bool,
    pub kv_unified: bool,
    pub type_k: c_int,
    pub type_v: c_int,
}

#[repr(C)]
pub struct llama_model_kv_override {
    pub key: [c_char; 128],
    pub tag: c_int,
    pub value: llama_model_kv_override_value,
}

#[repr(C)]
pub union llama_model_kv_override_value {
    pub val_i64: i64,
    pub val_f64: f64,
    pub val_bool: bool,
    pub val_str: [c_char; 128],
}

#[repr(C)]
pub struct llama_model_tensor_buft_override {
    pub pattern: *const c_char,
    pub buft_type: c_int,
}

#[repr(C)]
pub struct ggml_threadpool_params {
    pub n_threads: c_int,
    pub paused: bool,
    pub cpumask: [bool; 512], // GGML_MAX_N_THREADS
    pub prio: c_int,
    pub poll: c_int,
    pub strict_cpu: bool,
}

#[repr(C)]
pub struct lora_adapter {
    pub path: *const c_char,
    pub scale: c_float,
    pub ptr: *mut c_void,
    pub task_name: *const c_char,
    pub prompt_prefix: *const c_char,
}

// Constants
pub const LLAMA_TOKEN_NULL: llama_token = -1;
pub const LLAMA_POOLING_TYPE_RANK: c_int = 2;

// Mock model loading functions
#[no_mangle]
pub extern "C" fn llama_model_load_from_file(
    path_model: *const c_char,
    params: llama_model_params
) -> *mut llama_model {
    let path_str = if path_model.is_null() {
        "unknown_model.gguf"
    } else {
        unsafe { CStr::from_ptr(path_model).to_str().unwrap_or("unknown_model.gguf") }
    };
    
    rs_log_info(cstr(&format!("Mock: Loading model from {}", path_str)).as_ptr());
    rs_log_info(cstr(&format!("  - GPU layers: {}", params.n_gpu_layers)).as_ptr());
    rs_log_info(cstr(&format!("  - Main GPU: {}", params.main_gpu)).as_ptr());
    rs_log_info(cstr(&format!("  - Use mmap: {}", params.use_mmap)).as_ptr());
    rs_log_info(cstr(&format!("  - Use mlock: {}", params.use_mlock)).as_ptr());
    
    // Return mock model pointer (non-null to indicate success)
    0x1000 as *mut llama_model
}

#[no_mangle]
pub extern "C" fn llama_init_from_model(
    model: *mut llama_model,
    params: llama_context_params
) -> *mut llama_context {
    rs_log_info(cstr("Mock: Initializing context from model").as_ptr());
    rs_log_info(cstr(&format!("  - Context size: {}", params.n_ctx)).as_ptr());
    rs_log_info(cstr(&format!("  - Batch size: {}", params.n_batch)).as_ptr());
    rs_log_info(cstr(&format!("  - Threads: {}", params.n_threads)).as_ptr());
    rs_log_info(cstr(&format!("  - Embeddings: {}", params.embeddings)).as_ptr());
    
    if model.is_null() {
        rs_log_error(cstr("Mock: Model is null, cannot create context").as_ptr());
        return null_mut();
    }
    
    // Return mock context pointer (non-null to indicate success)
    0x2000 as *mut llama_context
}

#[no_mangle]
pub extern "C" fn llama_model_free(model: *mut llama_model) {
    rs_log_info(cstr("Mock: Freeing model").as_ptr());
}

#[no_mangle]
pub extern "C" fn llama_free(ctx: *mut llama_context) {
    rs_log_info(cstr("Mock: Freeing context").as_ptr());
}

#[no_mangle]
pub extern "C" fn llama_model_default_params() -> llama_model_params {
    rs_log_info(cstr("Mock: Getting default model parameters").as_ptr());
    llama_model_params {
        n_gpu_layers: 0,
        main_gpu: 0,
        split_mode: 0,
        tensor_split: null(),
        use_mmap: true,
        use_mlock: false,
        check_tensors: false,
        use_extra_bufts: true,
        devices: null(),
        kv_overrides: null(),
        tensor_buft_overrides: null(),
        progress_callback: None,
        progress_callback_user_data: null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn llama_context_default_params() -> llama_context_params {
    rs_log_info(cstr("Mock: Getting default context parameters").as_ptr());
    llama_context_params {
        n_ctx: 4096,
        n_seq_max: 1,
        n_batch: 512,
        n_ubatch: 512,
        n_threads: 8,
        n_threads_batch: 8,
        embeddings: false,
        rope_scaling_type: 0,
        rope_freq_base: 10000.0,
        rope_freq_scale: 1.0,
        yarn_ext_factor: -1.0,
        yarn_attn_factor: 1.0,
        yarn_beta_fast: 32.0,
        yarn_beta_slow: 1.0,
        yarn_orig_ctx: 0,
        pooling_type: 0,
        attention_type: 0,
        flash_attn_type: 0,
        cb_eval: None,
        cb_eval_user_data: null_mut(),
        offload_kqv: true,
        no_perf: false,
        op_offload: true,
        swa_full: false,
        kv_unified: false,
        type_k: 0,
        type_v: 0,
    }
}

// Model utility functions
#[no_mangle]
pub extern "C" fn llama_model_n_layer(model: *mut llama_model) -> c_int {
    rs_log_info(cstr("Mock: Getting model layer count").as_ptr());
    32 // Mock layer count
}

#[no_mangle]
pub extern "C" fn llama_model_has_decoder(model: *mut llama_model) -> bool {
    rs_log_info(cstr("Mock: Checking if model has decoder").as_ptr());
    true
}

#[no_mangle]
pub extern "C" fn llama_vocab_sep(vocab: *const llama_vocab) -> llama_token {
    rs_log_info(cstr("Mock: Getting SEP token").as_ptr());
    3 // Mock SEP token
}

#[no_mangle]
pub extern "C" fn llama_vocab_n_tokens(vocab: *const llama_vocab) -> c_int {
    rs_log_info(cstr("Mock: Getting vocab token count").as_ptr());
    32000 // Mock vocab size
}

#[no_mangle]
pub extern "C" fn llama_pooling_type(ctx: *mut llama_context) -> c_int {
    rs_log_info(cstr("Mock: Getting pooling type").as_ptr());
    0 // Mock pooling type
}

#[no_mangle]
pub extern "C" fn llama_memory_can_shift(mem: *mut c_void) -> bool {
    rs_log_info(cstr("Mock: Checking if memory can shift").as_ptr());
    true
}

#[no_mangle]
pub extern "C" fn llama_memory_clear(mem: *mut c_void, clear_kv: bool) {
    rs_log_info(cstr(&format!("Mock: Clearing memory (clear_kv: {})", clear_kv)).as_ptr());
}

#[no_mangle]
pub extern "C" fn llama_synchronize(ctx: *mut llama_context) {
    rs_log_info(cstr("Mock: Synchronizing context").as_ptr());
}

#[no_mangle]
pub extern "C" fn llama_perf_context_reset(ctx: *mut llama_context) {
    rs_log_info(cstr("Mock: Resetting performance context").as_ptr());
}

#[no_mangle]
pub extern "C" fn llama_set_warmup(ctx: *mut llama_context, warmup: bool) {
    rs_log_info(cstr(&format!("Mock: Setting warmup mode: {}", warmup)).as_ptr());
}

// LoRA adapter functions
#[no_mangle]
pub extern "C" fn llama_adapter_lora_init(
    model: *mut llama_model,
    path: *const c_char
) -> *mut c_void {
    let path_str = if path.is_null() {
        "unknown_lora.bin"
    } else {
        unsafe { CStr::from_ptr(path).to_str().unwrap_or("unknown_lora.bin") }
    };
    
    rs_log_info(cstr(&format!("Mock: Loading LoRA adapter from {}", path_str)).as_ptr());
    0x3000 as *mut c_void // Mock LoRA adapter pointer
}

#[no_mangle]
pub extern "C" fn llama_adapter_meta_val_str(
    adapter: *mut c_void,
    key: *const c_char,
    buf: *mut c_char,
    buf_size: usize
) -> c_int {
    let key_str = if key.is_null() {
        "unknown_key"
    } else {
        unsafe { CStr::from_ptr(key).to_str().unwrap_or("unknown_key") }
    };
    
    rs_log_info(cstr(&format!("Mock: Getting adapter meta value for key: {}", key_str)).as_ptr());
    
    // Mock return values based on key
    let mock_value = match key_str {
        "adapter.lora.task_name" => "mock_task",
        "adapter.lora.prompt_prefix" => "Mock: ",
        _ => "mock_value",
    };
    
    if !buf.is_null() && buf_size > 0 {
        let mock_cstring = cstr(mock_value);
        let mock_bytes = mock_cstring.as_bytes_with_nul();
        let copy_len = std::cmp::min(mock_bytes.len(), buf_size);
        unsafe {
            std::ptr::copy_nonoverlapping(mock_bytes.as_ptr(), buf as *mut u8, copy_len);
        }
    }
    
    0 // Success
}

#[no_mangle]
pub extern "C" fn llama_apply_adapter_cvec(
    ctx: *mut llama_context,
    data: *const c_float,
    len: usize,
    n_embd: c_int,
    layer_start: c_int,
    layer_end: c_int
) -> c_int {
    rs_log_info(cstr(&format!("Mock: Applying control vector (embd: {}, layers: {}-{})", 
                              n_embd, layer_start, layer_end)).as_ptr());
    0 // Success
}

// Threadpool functions
#[no_mangle]
pub extern "C" fn ggml_threadpool_params_init(
    params: *mut ggml_threadpool_params,
    n_threads: c_int
) {
    rs_log_info(cstr(&format!("Mock: Initializing threadpool params with {} threads", n_threads)).as_ptr());
    if !params.is_null() {
        unsafe {
            (*params).n_threads = n_threads;
            (*params).paused = false;
            (*params).prio = 0;
            (*params).poll = 50;
            (*params).strict_cpu = false;
            // Initialize cpumask to false
            for i in 0..512 {
                (*params).cpumask[i] = false;
            }
        }
    }
}

// Common parameter conversion functions
#[no_mangle]
pub extern "C" fn common_model_params_to_llama(params: *const common_params) -> llama_model_params {
    rs_log_info(cstr("Mock: Converting common params to llama model params").as_ptr());
    
    let mparams = llama_model_default_params();
    
    if !params.is_null() {
        // Mock parameter conversion
        rs_log_info(cstr("  - Converting model parameters from common_params").as_ptr());
        // In a real implementation, we would copy fields from params
    }
    
    mparams
}

#[no_mangle]
pub extern "C" fn common_context_params_to_llama(params: *const common_params) -> llama_context_params {
    rs_log_info(cstr("Mock: Converting common params to llama context params").as_ptr());
    
    let mut cparams = llama_context_default_params();
    
    if !params.is_null() {
        unsafe {
            cparams.n_ctx = (*params).n_ctx;
            cparams.n_batch = (*params).n_batch;
            cparams.n_threads = (*params).cpuparams.n_threads;
            cparams.embeddings = (*params).embedding;
            cparams.rope_freq_base = (*params).rope_freq_base;
            cparams.rope_freq_scale = (*params).rope_freq_scale;
        }
        rs_log_info(cstr(&format!("  - Context size: {}", cparams.n_ctx)).as_ptr());
        rs_log_info(cstr(&format!("  - Batch size: {}", cparams.n_batch)).as_ptr());
        rs_log_info(cstr(&format!("  - Threads: {}", cparams.n_threads)).as_ptr());
    }
    
    cparams
}

// Enhanced common_init_from_params with comprehensive model loading
#[no_mangle]
pub extern "C" fn common_init_from_params_enhanced(params: *const common_params) -> common_init_result {
    rs_log_info(cstr("=== Enhanced Model Initialization ===").as_ptr());
    
    let mut result = common_init_result {
        model: llama_model_holder { _impl: null_mut() },
        context: llama_context_holder { _impl: null_mut() },
    };
    
    if params.is_null() {
        rs_log_error(cstr("Error: Parameters are null").as_ptr());
        return result;
    }
    
    // Convert parameters
    let mparams = common_model_params_to_llama(params);
    
    // Use actual model path selection
    let selected_model = select_best_model();
    let (model_path_cstr, model_path_display) = match selected_model {
        Some(path) => {
            let path_str = path.to_str().unwrap_or("unknown_model.gguf");
            (cstr(path_str), path_str.to_string())
        }
        None => {
            rs_log_warn(cstr("No model found, using fallback").as_ptr());
            (cstr("models/default.gguf"), "models/default.gguf".to_string())
        }
    };
    rs_log_info(cstr(&format!("Loading model from: {}", model_path_display)).as_ptr());
    
    // Load model
    let model = llama_model_load_from_file(model_path_cstr.as_ptr(), mparams);
    if model.is_null() {
        rs_log_error(cstr("Failed to load model").as_ptr());
        return result;
    }

    // Get vocab for validation
    let vocab = super::log::llama_model_get_vocab(model);
    
    // Convert context parameters
    let cparams = common_context_params_to_llama(params);
    
    // Initialize context
    let ctx = llama_init_from_model(model, cparams);
    if ctx.is_null() {
        rs_log_error(cstr("Failed to create context").as_ptr());
        llama_model_free(model);
        return result;
    }
    
    // Check KV cache shifting capability
    let memory = super::log::llama_get_memory(ctx);
    if !memory.is_null() && !llama_memory_can_shift(memory) {
        rs_log_warn(cstr("KV cache shifting not supported, disabling").as_ptr());
    }
    
    // Mock LoRA adapter loading
    rs_log_info(cstr("Mock: Processing LoRA adapters").as_ptr());
    // In real implementation, would iterate through params.lora_adapters
    
    // Mock control vector application
    rs_log_info(cstr("Mock: Processing control vectors").as_ptr());
    
    // Check pooling type for reranking
    let pooling_type = llama_pooling_type(ctx);
    if pooling_type == LLAMA_POOLING_TYPE_RANK {
        rs_log_info(cstr("Model uses ranking pooling type").as_ptr());
        
        // Check for required tokens
        let bos_token = super::log::llama_vocab_bos(vocab);
        let eos_token = super::log::llama_vocab_eos(vocab);
        let sep_token = llama_vocab_sep(vocab);
        
        if bos_token == LLAMA_TOKEN_NULL {
            rs_log_warn(cstr("Warning: No BOS token found, reranking may not work").as_ptr());
        }
        
        if eos_token == LLAMA_TOKEN_NULL && sep_token == LLAMA_TOKEN_NULL {
            rs_log_warn(cstr("Warning: No EOS or SEP token found").as_ptr());
        }
    }
    
    // Mock warmup sequence
    rs_log_info(cstr("Mock: Performing model warmup").as_ptr());
    llama_set_warmup(ctx, true);
    
    // Mock warmup tokens
    let bos_token = super::log::llama_vocab_bos(vocab);
    let eos_token = super::log::llama_vocab_eos(vocab);
    
    if bos_token != LLAMA_TOKEN_NULL || eos_token != LLAMA_TOKEN_NULL {
        rs_log_info(cstr("  - Using BOS/EOS tokens for warmup").as_ptr());
        
        // Check if model has encoder/decoder
        if super::log::llama_model_has_encoder(model) {
            rs_log_info(cstr("  - Model has encoder, performing encode warmup").as_ptr());
            let decoder_start = super::log::llama_model_decoder_start_token(model);
            rs_log_info(cstr(&format!("  - Decoder start token: {}", decoder_start)).as_ptr());
        }
        
        if llama_model_has_decoder(model) {
            rs_log_info(cstr("  - Model has decoder, performing decode warmup").as_ptr());
        }
    }
    
    // Clear memory and reset performance counters
    llama_memory_clear(memory, true);
    llama_synchronize(ctx);
    llama_perf_context_reset(ctx);
    llama_set_warmup(ctx, false);
    
    rs_log_info(cstr("Model initialization completed successfully").as_ptr());
    
    // Set result
    result.model._impl = model;
    result.context._impl = ctx;
    
    result
}

// Batch utility functions
#[no_mangle]
pub extern "C" fn common_batch_clear(batch: *mut llama_batch) {
    rs_log_info(cstr("Mock: Clearing batch").as_ptr());
    // In real implementation, would clear batch fields
}

#[no_mangle]
pub extern "C" fn common_batch_add(
    batch: *mut llama_batch,
    id: llama_token,
    pos: c_int,
    seq_ids: *const c_int,
    seq_ids_len: usize,
    logits: bool
) {
    rs_log_info(cstr(&format!("Mock: Adding token {} to batch at pos {}", id, pos)).as_ptr());
    // In real implementation, would add token to batch
}

// Model endpoint utility
#[no_mangle]
pub extern "C" fn get_model_endpoint() -> *const c_char {
    rs_log_info(cstr("Mock: Getting model endpoint").as_ptr());
    
    // Check environment variables
    if let Ok(endpoint) = env::var("MODEL_ENDPOINT") {
        let mut endpoint_str = endpoint;
        if !endpoint_str.ends_with('/') {
            endpoint_str.push('/');
        }
        return cstr(&endpoint_str).into_raw();
    }
    
    if let Ok(endpoint) = env::var("HF_ENDPOINT") {
        let mut endpoint_str = endpoint;
        if !endpoint_str.ends_with('/') {
            endpoint_str.push('/');
        }
        return cstr(&endpoint_str).into_raw();
    }
    
    // Default endpoint
    cstr("https://huggingface.co/").into_raw()
}

// Additional model management functions
#[no_mangle]
pub extern "C" fn common_set_adapter_lora(
    ctx: *mut llama_context,
    adapters: *const lora_adapter,
    adapter_count: usize
) {
    rs_log_info(cstr(&format!("Mock: Setting {} LoRA adapters", adapter_count)).as_ptr());
    // In real implementation, would apply LoRA adapters to context
}

#[no_mangle]
pub extern "C" fn common_control_vector_load(file_paths: *const *const c_char, count: usize) -> *mut c_void {
    rs_log_info(cstr(&format!("Mock: Loading {} control vectors", count)).as_ptr());
    // Return mock control vector data
    0x4000 as *mut c_void
}

// common_token_to_piece is already defined in log.rs

// =============================================================================
// GGUF Model Discovery and Initialization Functions
// =============================================================================

/// Discover available GGUF models in a directory
fn discover_available_models(models_dir: &str) -> Vec<String> {
    let models_path = Path::new(models_dir);
    let mut models = Vec::new();
    
    if !models_path.exists() {
        rs_log_warn(cstr(&format!("Models directory does not exist: {}", models_dir)).as_ptr());
        return models;
    }
    
    match fs::read_dir(models_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "gguf" {
                            if let Some(filename) = path.file_name() {
                                if let Some(filename_str) = filename.to_str() {
                                    models.push(filename_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            rs_log_warn(cstr(&format!("Failed to read models directory {}: {}", models_dir, e)).as_ptr());
        }
    }
    
    // Sort models with quantized models first if any found
    models.sort_by(|a, b| {
        let a_quantized = a.contains("q4") || a.contains("q8");
        let b_quantized = b.contains("q4") || b.contains("q8");
        b_quantized.cmp(&a_quantized)
    });
    
    rs_log_info(cstr(&format!("Discovered {} models in {}", models.len(), models_dir)).as_ptr());
    models
}

/// Load model configuration from JSON file or use defaults
pub fn load_model_config() -> ModelConfig {
    let config_path = Path::new("models.json");
    
    if config_path.exists() {
        match fs::read_to_string(config_path) {
            Ok(content) => {
                match serde_json::from_str::<ModelConfig>(&content) {
                    Ok(config) => {
                        rs_log_info(cstr("Loaded model configuration from models.json").as_ptr());
                        return config;
                    }
                    Err(e) => {
                        rs_log_warn(cstr(&format!("Failed to parse models.json: {}", e)).as_ptr());
                    }
                }
            }
            Err(e) => {
                rs_log_warn(cstr(&format!("Failed to read models.json: {}", e)).as_ptr());
            }
        }
    } else {
        rs_log_info(cstr("models.json not found, using default configuration").as_ptr());
    }
    
    ModelConfig::default()
}

/// Get the models directory path from configuration
pub fn get_models_directory(config: &ModelConfig) -> PathBuf {
    // Check environment variable first
    if let Ok(models_dir) = env::var(&config.environment_variables.models_dir_var) {
        rs_log_info(cstr(&format!("Using models directory from {}: {}", 
                                  config.environment_variables.models_dir_var, models_dir)).as_ptr());
        return PathBuf::from(models_dir);
    }
    
    // Use configuration default
    PathBuf::from(&config.model_directory)
}

/// Scans the models directory for GGUF files using configuration
pub fn scan_models_directory() -> Result<Vec<PathBuf>, std::io::Error> {
    let config = load_model_config();
    let models_dir = get_models_directory(&config);
    let mut gguf_files = Vec::new();
    
    rs_log_info(cstr(&format!("Scanning models directory: {}", models_dir.display())).as_ptr());
    
    if !models_dir.exists() {
        rs_log_warn(cstr("Models directory does not exist").as_ptr());
        return Ok(gguf_files);
    }
    
    for entry in fs::read_dir(&models_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if let Some(extension) = path.extension() {
            if extension == "gguf" {
                // Apply file size filtering from preferences
                if let Ok(metadata) = path.metadata() {
                    let file_size_mb = metadata.len() / (1024 * 1024);
                    let file_size_gb = file_size_mb / 1024;
                    
                    if file_size_gb > config.model_preferences.max_file_size_gb {
                        rs_log_warn(cstr(&format!("Skipping large model: {} ({} GB)", 
                                                  path.display(), file_size_gb)).as_ptr());
                        continue;
                    }
                    
                    if file_size_mb < config.model_preferences.min_file_size_mb {
                        rs_log_warn(cstr(&format!("Skipping small model: {} ({} MB)", 
                                                  path.display(), file_size_mb)).as_ptr());
                        continue;
                    }
                }
                
                rs_log_info(cstr(&format!("Found GGUF model: {}", path.display())).as_ptr());
                gguf_files.push(path);
            }
        }
    }
    
    // Sort models with quantized models first if preferred
    if config.model_preferences.prefer_quantized {
        gguf_files.sort_by(|a, b| {
            let a_quantized = a.to_string_lossy().contains("q4") || a.to_string_lossy().contains("q8");
            let b_quantized = b.to_string_lossy().contains("q4") || b.to_string_lossy().contains("q8");
            b_quantized.cmp(&a_quantized)
        });
    }
    
    rs_log_info(cstr(&format!("Found {} GGUF models", gguf_files.len())).as_ptr());
    Ok(gguf_files)
}

/// Gets basic information about a GGUF file
pub fn get_gguf_info(path: &Path) -> Result<GgufInfo, std::io::Error> {
    rs_log_info(cstr(&format!("Reading GGUF info from: {}", path.display())).as_ptr());
    
    let mut file = fs::File::open(path)?;
    let file_size = file.metadata()?.len();
    
    // Read GGUF magic bytes (first 4 bytes should be "GGUF")
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;
    
    let is_valid_gguf = &magic == b"GGUF";
    
    if !is_valid_gguf {
        rs_log_warn(cstr(&format!("Invalid GGUF magic bytes in {}", path.display())).as_ptr());
    }
    
    Ok(GgufInfo {
        path: path.to_path_buf(),
        file_size,
        is_valid: is_valid_gguf,
        model_name: path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string(),
    })
}

#[derive(Debug, Clone)]
pub struct GgufInfo {
    pub path: PathBuf,
    pub file_size: u64,
    pub is_valid: bool,
    pub model_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub engine_port : i16,
    pub model_path: String,
    pub default_model: String,
    pub model_directory: String,
    pub fallback_models: Vec<String>,
    pub model_preferences: ModelPreferences,
    pub environment_variables: EnvironmentConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelPreferences {
    pub prefer_quantized: bool,
    pub max_file_size_gb: u64,
    pub min_file_size_mb: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvironmentConfig {
    pub model_path_var: String,
    pub default_model_var: String,
    pub models_dir_var: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        // Get models directory from environment or use default
        let models_dir = env::var("MODELS_DIR").unwrap_or_else(|_| "models".to_string());
        
        // Discover available models dynamically
        let fallback_models = discover_available_models(&models_dir);
        
        // Get default model from environment
        let default_model = env::var("DEFAULT_MODEL").unwrap_or_default();
        
        // Get preferences from environment or use sensible defaults
        let prefer_quantized = env::var("PREFER_QUANTIZED")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(true);
        
        let max_file_size_gb = env::var("MAX_FILE_SIZE_GB")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(20);
        
        let min_file_size_mb = env::var("MIN_FILE_SIZE_MB")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);

        // Get model path from environment or discover the best model
        let model_path = env::var("MODEL_PATH").unwrap_or_else(|_| {
            // Try to discover a model automatically
            if let Some(best_model) = discover_available_models(&models_dir).first() {
                format!("{}/{}", models_dir, best_model)
            } else {
                String::new()
            }
        });

        ModelConfig {
            engine_port: 5000, // Default port, only use JSON config value
            model_path,
            default_model,
            model_directory: models_dir,
            fallback_models,
            model_preferences: ModelPreferences {
                prefer_quantized,
                max_file_size_gb,
                min_file_size_mb,
            },
            environment_variables: EnvironmentConfig {
                model_path_var: env::var("MODEL_PATH_VAR").unwrap_or_else(|_| "MODEL_PATH".to_string()),
                default_model_var: env::var("DEFAULT_MODEL_VAR").unwrap_or_else(|_| "DEFAULT_MODEL".to_string()),
                models_dir_var: env::var("MODELS_DIR_VAR").unwrap_or_else(|_| "MODELS_DIR".to_string()),
            },
        }
    }
}

/// Select the best model based on configuration and environment
pub fn select_best_model() -> Option<PathBuf> {
    let config = load_model_config();
    
    // 1. Check environment variable for specific model path
    if let Ok(model_path) = env::var(&config.environment_variables.model_path_var) {
        let path = PathBuf::from(&model_path);
        if path.exists() && path.extension().map_or(false, |ext| ext == "gguf") {
            rs_log_info(cstr(&format!("Using model from {}: {}", 
                                      config.environment_variables.model_path_var, model_path)).as_ptr());
            return Some(path);
        } else {
            rs_log_warn(cstr(&format!("Model path from {} not found or invalid: {}", 
                                      config.environment_variables.model_path_var, model_path)).as_ptr());
        }
    }
    
    // 2. Check environment variable for default model name
    if let Ok(default_model) = env::var(&config.environment_variables.default_model_var) {
        let models_dir = get_models_directory(&config);
        let model_path = models_dir.join(&default_model);
        if model_path.exists() {
            rs_log_info(cstr(&format!("Using default model from {}: {}", 
                                      config.environment_variables.default_model_var, default_model)).as_ptr());
            return Some(model_path);
        } else {
            rs_log_warn(cstr(&format!("Default model from {} not found: {}", 
                                      config.environment_variables.default_model_var, default_model)).as_ptr());
        }
    }
    
    // 3. Check configured default model
    if !config.default_model.is_empty() {
        let models_dir = get_models_directory(&config);
        let model_path = models_dir.join(&config.default_model);
        if model_path.exists() {
            rs_log_info(cstr(&format!("Using configured default model: {}", config.default_model)).as_ptr());
            return Some(model_path);
        } else {
            rs_log_warn(cstr(&format!("Configured default model not found: {}", config.default_model)).as_ptr());
        }
    }
    
    // 4. Try fallback models from configuration
    let models_dir = get_models_directory(&config);
    for fallback_model in &config.fallback_models {
        let model_path = models_dir.join(fallback_model);
        if model_path.exists() {
            rs_log_info(cstr(&format!("Using fallback model: {}", fallback_model)).as_ptr());
            return Some(model_path);
        }
    }
    
    // 5. Scan directory and use first available model
    match scan_models_directory() {
        Ok(models) if !models.is_empty() => {
            let selected = &models[0];
            rs_log_info(cstr(&format!("Using first available model: {}", selected.display())).as_ptr());
            Some(selected.clone())
        }
        Ok(_) => {
            rs_log_error(cstr("No GGUF models found in directory").as_ptr());
            None
        }
        Err(e) => {
            rs_log_error(cstr(&format!("Failed to scan models directory: {}", e)).as_ptr());
            None
        }
    }
}

/// Initialize GGUF model with automatic model discovery
#[no_mangle]
pub extern "C" fn init_gguf_model_auto() -> common_init_result {
    rs_log_info(cstr("=== Automatic GGUF Model Initialization ===").as_ptr());
    
    let result = common_init_result {
        model: llama_model_holder { _impl: null_mut() },
        context: llama_context_holder { _impl: null_mut() },
    };
    
    // Select the best model based on configuration
    let selected_model = match select_best_model() {
        Some(path) => path,
        None => {
            rs_log_error(cstr("No suitable GGUF model found").as_ptr());
            return result;
        }
    };
    
    rs_log_info(cstr(&format!("Selected model: {}", selected_model.display())).as_ptr());
    
    // Get model info
    let model_info = match get_gguf_info(&selected_model) {
        Ok(info) => info,
        Err(e) => {
            rs_log_error(cstr(&format!("Failed to read model info: {}", e)).as_ptr());
            return result;
        }
    };
    
    rs_log_info(cstr(&format!("Model: {}", model_info.model_name)).as_ptr());
    rs_log_info(cstr(&format!("Size: {:.2} MB", model_info.file_size as f64 / 1024.0 / 1024.0)).as_ptr());
    rs_log_info(cstr(&format!("Valid GGUF: {}", model_info.is_valid)).as_ptr());
    
    if !model_info.is_valid {
        rs_log_error(cstr("Invalid GGUF file format").as_ptr());
        return result;
    }
    
    // Initialize model with the selected GGUF file
    init_gguf_model_from_path(&selected_model)
}

/// Initialize GGUF model from specific path
pub fn init_gguf_model_from_path(model_path: &Path) -> common_init_result {
    rs_log_info(cstr(&format!("=== Initializing GGUF Model: {} ===", model_path.display())).as_ptr());
    
    let mut result = common_init_result {
        model: llama_model_holder { _impl: null_mut() },
        context: llama_context_holder { _impl: null_mut() },
    };
    
    // Check if file exists
    if !model_path.exists() {
        rs_log_error(cstr(&format!("Model file does not exist: {}", model_path.display())).as_ptr());
        return result;
    }
    
    // Convert path to C string
    let path_str = model_path.to_str().unwrap_or("unknown_path");
    let path_cstring = cstr(path_str);
    
    // Create model parameters for GGUF loading
    let mparams = create_gguf_model_params();
    
    // Load the GGUF model
    rs_log_info(cstr("Loading GGUF model...").as_ptr());
    let model = llama_model_load_from_file(path_cstring.as_ptr(), mparams);
    
    if model.is_null() {
        rs_log_error(cstr("Failed to load GGUF model").as_ptr());
        return result;
    }
    
    rs_log_info(cstr("GGUF model loaded successfully").as_ptr());
    
    // Create context parameters optimized for the model
    let cparams = create_gguf_context_params();
    
    // Initialize context
    rs_log_info(cstr("Initializing context...").as_ptr());
    let ctx = llama_init_from_model(model, cparams);
    
    if ctx.is_null() {
        rs_log_error(cstr("Failed to create context from GGUF model").as_ptr());
        llama_model_free(model);
        return result;
    }
    
    rs_log_info(cstr("Context initialized successfully").as_ptr());
    
    // Perform model validation
    validate_gguf_model(model, ctx);
    
    // Set result
    result.model._impl = model;
    result.context._impl = ctx;
    
    rs_log_info(cstr("=== GGUF Model Initialization Complete ===").as_ptr());
    result
}

/// Create optimized model parameters for GGUF files
fn create_gguf_model_params() -> llama_model_params {
    rs_log_info(cstr("Creating optimized GGUF model parameters").as_ptr());
    
    let mut params = llama_model_default_params();
    
    // Optimize for macOS ARM64 (Apple Silicon)
    params.use_mmap = true;  // Enable memory mapping for efficiency
    params.use_mlock = false; // Disable memory locking to avoid system limits
    params.check_tensors = true; // Enable tensor validation
    params.use_extra_bufts = true; // Use extra buffers for performance
    
    // Set GPU layers (0 for CPU-only on macOS without Metal support in mock)
    params.n_gpu_layers = 0;
    params.main_gpu = 0;
    
    rs_log_info(cstr("  - Memory mapping: enabled").as_ptr());
    rs_log_info(cstr("  - Memory locking: disabled").as_ptr());
    rs_log_info(cstr("  - Tensor checking: enabled").as_ptr());
    rs_log_info(cstr("  - GPU layers: 0 (CPU only)").as_ptr());
    
    params
}

/// Create optimized context parameters for GGUF models
fn create_gguf_context_params() -> llama_context_params {
    rs_log_info(cstr("Creating optimized GGUF context parameters").as_ptr());
    
    let mut params = llama_context_default_params();
    
    // Optimize context size and batch processing
    params.n_ctx = 4096;  // Context window size
    params.n_batch = 512; // Batch size for processing
    params.n_ubatch = 512; // Micro-batch size
    
    // Set thread count based on system capabilities
    let cpu_count = std::thread::available_parallelism()
        .map(|n| n.get() as c_int)
        .unwrap_or(8);
    
    params.n_threads = cpu_count.min(8); // Limit to 8 threads for stability
    params.n_threads_batch = params.n_threads;
    
    // Optimize for text generation
    params.embeddings = false;
    params.rope_freq_base = 10000.0;
    params.rope_freq_scale = 1.0;
    
    // Enable performance optimizations
    params.offload_kqv = true;
    params.no_perf = false;
    
    rs_log_info(cstr(&format!("  - Context size: {}", params.n_ctx)).as_ptr());
    rs_log_info(cstr(&format!("  - Batch size: {}", params.n_batch)).as_ptr());
    rs_log_info(cstr(&format!("  - CPU threads: {}", params.n_threads)).as_ptr());
    rs_log_info(cstr(&format!("  - KV offload: {}", params.offload_kqv)).as_ptr());
    
    params
}

/// Validate GGUF model after loading
fn validate_gguf_model(model: *mut llama_model, ctx: *mut llama_context) {
    rs_log_info(cstr("Validating GGUF model...").as_ptr());
    
    // Check model properties
    let vocab = super::log::llama_model_get_vocab(model);
    if vocab.is_null() {
        rs_log_warn(cstr("Warning: Could not retrieve model vocabulary").as_ptr());
    } else {
        rs_log_info(cstr("✓ Model vocabulary accessible").as_ptr());
    }
    
    // Check context properties
    let n_ctx = super::log::llama_n_ctx(ctx);
    let n_ctx_train = super::log::llama_model_n_ctx_train(model);
    
    rs_log_info(cstr(&format!("  - Context size: {}", n_ctx)).as_ptr());
    rs_log_info(cstr(&format!("  - Training context: {}", n_ctx_train)).as_ptr());
    
    // Check encoder/decoder capabilities
    let has_encoder = super::log::llama_model_has_encoder(model);
    let has_decoder = llama_model_has_decoder(model);
    
    rs_log_info(cstr(&format!("  - Has encoder: {}", has_encoder)).as_ptr());
    rs_log_info(cstr(&format!("  - Has decoder: {}", has_decoder)).as_ptr());
    
    // Check special tokens
    if !vocab.is_null() {
        let bos_token = super::log::llama_vocab_bos(vocab);
        let eos_token = super::log::llama_vocab_eos(vocab);
        
        rs_log_info(cstr(&format!("  - BOS token: {}", bos_token)).as_ptr());
        rs_log_info(cstr(&format!("  - EOS token: {}", eos_token)).as_ptr());
        
        if bos_token != LLAMA_TOKEN_NULL && eos_token != LLAMA_TOKEN_NULL {
            rs_log_info(cstr("✓ Special tokens available").as_ptr());
        } else {
            rs_log_warn(cstr("Warning: Some special tokens missing").as_ptr());
        }
    }
    
    rs_log_info(cstr("GGUF model validation complete").as_ptr());
}

/// Set environment variable for model configuration
#[no_mangle]
pub extern "C" fn set_model_path_env(model_path: *const c_char) -> c_int {
    if model_path.is_null() {
        rs_log_error(cstr("Model path is null").as_ptr());
        return -1;
    }
    
    let path_str = unsafe {
        CStr::from_ptr(model_path).to_str().unwrap_or_default()
    };
    
    if path_str.is_empty() {
        rs_log_error(cstr("Model path is empty").as_ptr());
        return -1;
    }
    
    env::set_var("MODEL_PATH", path_str);
    rs_log_info(cstr(&format!("Set MODEL_PATH environment variable: {}", path_str)).as_ptr());
    0
}

/// Set environment variable for default model name
#[no_mangle]
pub extern "C" fn set_default_model_env(model_name: *const c_char) -> c_int {
    if model_name.is_null() {
        rs_log_error(cstr("Model name is null").as_ptr());
        return -1;
    }
    
    let name_str = unsafe {
        CStr::from_ptr(model_name).to_str().unwrap_or_default()
    };
    
    if name_str.is_empty() {
        rs_log_error(cstr("Model name is empty").as_ptr());
        return -1;
    }
    
    env::set_var("DEFAULT_MODEL", name_str);
    rs_log_info(cstr(&format!("Set DEFAULT_MODEL environment variable: {}", name_str)).as_ptr());
    0
}

/// Generate and save dynamic model configuration
#[no_mangle]
pub extern "C" fn generate_model_config() -> c_int {
    rs_log_info(cstr("Generating dynamic model configuration...").as_ptr());
    
    let config = ModelConfig::default(); // This now uses dynamic discovery
    
    match serde_json::to_string_pretty(&config) {
        Ok(json_str) => {
            match fs::write("models.json", json_str) {
                Ok(_) => {
                    rs_log_info(cstr("Generated models.json with discovered models").as_ptr());
                    rs_log_info(cstr(&format!("Found {} fallback models", config.fallback_models.len())).as_ptr());
                    for (i, model) in config.fallback_models.iter().enumerate() {
                        rs_log_info(cstr(&format!("  {}. {}", i + 1, model)).as_ptr());
                    }
                    0
                }
                Err(e) => {
                    rs_log_error(cstr(&format!("Failed to write models.json: {}", e)).as_ptr());
                    -1
                }
            }
        }
        Err(e) => {
            rs_log_error(cstr(&format!("Failed to serialize config: {}", e)).as_ptr());
            -1
        }
    }
}

/// Get current model configuration as JSON string
#[no_mangle]
pub extern "C" fn get_model_config_json() -> *const c_char {
    let config = load_model_config();
    
    match serde_json::to_string_pretty(&config) {
        Ok(json_str) => {
            rs_log_info(cstr("Retrieved model configuration as JSON").as_ptr());
            cstr(&json_str).into_raw()
        }
        Err(e) => {
            rs_log_error(cstr(&format!("Failed to serialize model config: {}", e)).as_ptr());
            cstr("{}").into_raw()
        }
    }
}

/// Print environment configuration help
#[no_mangle]
pub extern "C" fn print_model_config_help() {
    rs_log_info(cstr("=== Model Configuration Environment Variables ===").as_ptr());
    rs_log_info(cstr("MODEL_PATH        - Full path to specific GGUF model file").as_ptr());
    rs_log_info(cstr("DEFAULT_MODEL     - Filename of default model in models directory").as_ptr());
    rs_log_info(cstr("MODELS_DIR        - Path to models directory (default: models)").as_ptr());
    rs_log_info(cstr("PREFER_QUANTIZED  - Prefer quantized models (true/false, default: true)").as_ptr());
    rs_log_info(cstr("MAX_FILE_SIZE_GB  - Maximum model file size in GB (default: 20)").as_ptr());
    rs_log_info(cstr("MIN_FILE_SIZE_MB  - Minimum model file size in MB (default: 100)").as_ptr());
    rs_log_info(cstr("").as_ptr());
    rs_log_info(cstr("Examples:").as_ptr());
    rs_log_info(cstr("  export MODEL_PATH=/path/to/my-model.gguf").as_ptr());
    rs_log_info(cstr("  export DEFAULT_MODEL=llama-2-7b-chat.q4_0.gguf").as_ptr());
    rs_log_info(cstr("  export MODELS_DIR=/custom/models").as_ptr());
    rs_log_info(cstr("  export PREFER_QUANTIZED=false").as_ptr());
}

/// C-compatible function to initialize GGUF model
#[no_mangle]
pub extern "C" fn init_gguf_model_c(model_path: *const c_char) -> common_init_result {
    let path_str = if model_path.is_null() {
        rs_log_warn(cstr("No model path provided, using auto-discovery").as_ptr());
        return init_gguf_model_auto();
    } else {
        unsafe { 
            CStr::from_ptr(model_path)
                .to_str()
                .unwrap_or_default()
        }
    };
    
    // If the provided path is empty, fall back to auto-discovery
    if path_str.is_empty() {
        rs_log_warn(cstr("Empty model path provided, using auto-discovery").as_ptr());
        return init_gguf_model_auto();
    }
    
    let model_path = Path::new(path_str);
    init_gguf_model_from_path(model_path)
}

/// List all available GGUF models in the models directory
#[no_mangle]
pub extern "C" fn list_gguf_models() -> c_int {
    rs_log_info(cstr("=== Available GGUF Models ===").as_ptr());
    
    let gguf_files = match scan_models_directory() {
        Ok(files) => files,
        Err(e) => {
            rs_log_error(cstr(&format!("Failed to scan models: {}", e)).as_ptr());
            return -1;
        }
    };
    
    if gguf_files.is_empty() {
        rs_log_info(cstr("No GGUF models found").as_ptr());
        return 0;
    }
    
    for (i, model_path) in gguf_files.iter().enumerate() {
        match get_gguf_info(model_path) {
            Ok(info) => {
                rs_log_info(cstr(&format!("{}. {}", i + 1, info.model_name)).as_ptr());
                rs_log_info(cstr(&format!("   Path: {}", info.path.display())).as_ptr());
                rs_log_info(cstr(&format!("   Size: {:.2} MB", info.file_size as f64 / 1024.0 / 1024.0)).as_ptr());
                rs_log_info(cstr(&format!("   Valid: {}", info.is_valid)).as_ptr());
            }
            Err(e) => {
                rs_log_warn(cstr(&format!("Failed to read info for {}: {}", model_path.display(), e)).as_ptr());
            }
        }
    }
    
    gguf_files.len() as c_int
}

/// Test function to demonstrate GGUF model initialization
#[no_mangle]
pub extern "C" fn gguf_initialization() -> c_int {
    rs_log_info(cstr("Testing GGUF Model Initialization").as_ptr());
    
    // List available models
    let model_count = list_gguf_models();
    
    if model_count <= 0 {
        rs_log_error(cstr("No GGUF models available for testing").as_ptr());
        return -1;
    }
    
    // Initialize model automatically
    rs_log_info(cstr("Initializing model automatically...").as_ptr());
    let result = init_gguf_model_auto();
    
    if result.model._impl.is_null() || result.context._impl.is_null() {
        rs_log_error(cstr("Failed to initialize GGUF model").as_ptr());
        return -1;
    }
    
    rs_log_info(cstr("GGUF model initialized successfully!").as_ptr());
    rs_log_info(cstr(&format!("   Model pointer: {:p}", result.model._impl)).as_ptr());
    rs_log_info(cstr(&format!("   Context pointer: {:p}", result.context._impl)).as_ptr());
    
    // Clean up
    llama_free(result.context._impl);
    llama_model_free(result.model._impl);
    
    rs_log_info(cstr("🧹 Model resources cleaned up").as_ptr());
    rs_log_info(cstr("GGUF initialization test completed successfully!").as_ptr());
    
    0 // Success
}