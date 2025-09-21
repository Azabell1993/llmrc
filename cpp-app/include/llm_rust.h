/**
 * @file llm_rust.h
 * @brief LLM Rust - Comprehensive GGUF Model Management System C API
 * @author Azabell1993
 * @version 1.0
 * @date 2024
 * 
 * This header provides C bindings for the Rust-based LLM system with dynamic
 * GGUF model discovery, configuration management, and comprehensive build system.
 */

#ifndef LLM_RUST_H
#define LLM_RUST_H

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/** @brief Default pooling type rank for LLAMA models */
#define LLAMA_POOLING_TYPE_RANK 2

/**
 * @brief CPU Information structure
 * 
 * Contains detailed information about the system's CPU including
 * core count, logical processors, frequency, and brand information.
 */
typedef struct CpuInfo {
  uint32_t cores;        ///< Physical CPU cores count
  uint32_t logical;      ///< Logical processors count (including hyperthreading)
  uint64_t freq_mhz;     ///< CPU frequency in MHz
  uint8_t brand[128];    ///< CPU brand name as null-terminated string
} CpuInfo;

/**
 * @brief Opaque LLaMA model structure
 * 
 * Represents a loaded LLaMA model with private implementation details.
 * This structure should only be manipulated through provided API functions.
 */
typedef struct llama_model {
  uint8_t _private[0];  ///< Private implementation data (do not access directly)
} llama_model;

/**
 * @brief LLaMA model holder wrapper
 * 
 * Container structure that holds a pointer to the actual llama_model implementation.
 * Provides an additional layer of abstraction for model management.
 */
typedef struct llama_model_holder {
  struct llama_model *_impl;  ///< Pointer to the actual model implementation
} llama_model_holder;

/**
 * @brief Opaque LLaMA context structure
 * 
 * Represents the execution context for a LLaMA model instance.
 * Contains all runtime state and configuration for model inference.
 */
typedef struct llama_context {
  uint8_t _private[0];  ///< Private implementation data (do not access directly)
} llama_context;

/**
 * @brief LLaMA context holder wrapper
 * 
 * Container structure that holds a pointer to the actual llama_context implementation.
 * Provides an additional layer of abstraction for context management.
 */
typedef struct llama_context_holder {
  struct llama_context *_impl;  ///< Pointer to the actual context implementation
} llama_context_holder;

/**
 * @brief Common initialization result structure
 * 
 * Contains both model and context holders returned from initialization functions.
 * This structure represents a complete, ready-to-use LLaMA model setup.
 */
typedef struct common_init_result {
  struct llama_model_holder model;    ///< Initialized model holder
  struct llama_context_holder context; ///< Initialized context holder
} common_init_result;

/**
 * @brief CPU configuration parameters
 * 
 * Defines CPU-related settings for model execution including
 * thread count and process priority.
 */
typedef struct cpu_params {
  int n_threads;  ///< Number of threads to use for computation
  int priority;   ///< Process priority level
} cpu_params;

/**
 * @brief Sampling parameters for text generation
 * 
 * Configuration structure for controlling text sampling behavior
 * during model inference and generation.
 */
typedef struct sampling_params {
  int _placeholder;  ///< Placeholder field for future sampling parameters
} sampling_params;

/**
 * @brief Comprehensive model configuration parameters
 * 
 * Contains all configuration options for model initialization and runtime behavior.
 * This structure encompasses interactive settings, performance parameters,
 * chat configuration, and various optimization flags.
 */
typedef struct common_params {
  bool interactive;                    ///< Enable interactive mode
  bool interactive_first;              ///< Start in interactive mode
  int conversation_mode;               ///< Conversation mode setting
  bool enable_chat_template;           ///< Enable chat template processing
  bool single_turn;                    ///< Single turn conversation mode
  bool simple_io;                      ///< Use simple input/output mode
  bool use_color;                      ///< Enable colored output
  bool embedding;                      ///< Enable embedding mode
  int n_ctx;                          ///< Context window size
  float rope_freq_base;               ///< RoPE frequency base parameter
  float rope_freq_scale;              ///< RoPE frequency scale parameter
  int numa;                           ///< NUMA configuration
  struct cpu_params cpuparams;        ///< CPU parameters for general computation
  struct cpu_params cpuparams_batch;  ///< CPU parameters for batch processing
  int n_batch;                        ///< Batch size for processing
  int n_predict;                      ///< Number of tokens to predict
  int n_keep;                         ///< Number of tokens to keep in context
  int n_print;                        ///< Number of tokens to print
  bool ctx_shift;                     ///< Enable context shifting
  bool display_prompt;                ///< Display the prompt
  bool verbose_prompt;                ///< Enable verbose prompt output
  bool input_prefix_bos;              ///< Add BOS token to input prefix
  const char *input_prefix;           ///< Input prefix string
  const char *input_suffix;           ///< Input suffix string
  int antiprompt_count;               ///< Number of antiprompts
  bool escape;                        ///< Enable escape sequence processing
  bool prompt_cache_all;              ///< Cache all prompts
  bool prompt_cache_ro;               ///< Read-only prompt cache
  const char *path_prompt_cache;      ///< Path to prompt cache file
  bool special;                       ///< Enable special token processing
  const char *default_template_kwargs; ///< Default template keyword arguments
  bool use_jinja;                     ///< Use Jinja template engine
  void *call_log_res;                 ///< Call log resource pointer
  struct sampling_params sampling;    ///< Sampling parameters
  const char *prompt;                 ///< Initial prompt text
  const char *system_prompt;          ///< System prompt text
  const char *chat_template;          ///< Chat template string
} common_params;

/**
 * @brief Opaque common sampler structure
 * 
 * Handles text generation sampling algorithms and token selection strategies.
 * Used for controlling randomness and quality of generated text.
 */
typedef struct common_sampler {
  uint8_t _private[0];  ///< Private implementation data (do not access directly)
} common_sampler;

/**
 * @brief Opaque LLaMA vocabulary structure
 * 
 * Contains the model's vocabulary mapping between tokens and text.
 * Handles tokenization and detokenization operations.
 */
typedef struct llama_vocab {
  uint8_t _private[0];  ///< Private implementation data (do not access directly)
} llama_vocab;

/**
 * @brief LLaMA token type
 * 
 * Represents a single token ID used by the model for text processing.
 * Tokens are the basic units of text that the model operates on.
 */
typedef int32_t llama_token;

/**
 * @brief Applied chat template result
 * 
 * Contains the processed prompt after applying chat templates.
 * Used for structured conversation formatting.
 */
typedef struct common_applied_template {
  const char *prompt;  ///< Processed prompt string after template application
} common_applied_template;

/**
 * @brief Token list structure
 * 
 * Represents a sequence of tokens with associated length information.
 * Used for passing tokenized text between functions.
 */
typedef struct token_list {
  llama_token *data;  ///< Array of token IDs
  uintptr_t len;      ///< Number of tokens in the array
} token_list;

/**
 * @brief Opaque LLaMA batch structure
 * 
 * Represents a batch of tokens for efficient parallel processing.
 * Used for batched inference operations to improve performance.
 */
typedef struct llama_batch {
  uint8_t _private[0];  ///< Private implementation data (do not access directly)
} llama_batch;

/**
 * @brief Opaque GGML backend device structure
 * 
 * Represents a computational device (CPU, GPU, etc.) available for
 * model execution through the GGML backend system.
 */
typedef struct ggml_backend_device {
  uint8_t _private[0];  ///< Private implementation data (do not access directly)
} ggml_backend_device;

/**
 * @brief Opaque GGML backend registry structure
 * 
 * Registry for managing and discovering available GGML backend devices
 * and their associated capabilities.
 */
typedef struct ggml_backend_registry {
  uint8_t _private[0];  ///< Private implementation data (do not access directly)
} ggml_backend_registry;

/**
 * @brief GGML thread pool parameters
 * 
 * Configuration parameters for thread pool management in GGML operations.
 * Controls threading behavior for parallel computation.
 */
typedef struct ggml_threadpool_params {
  int n_threads;  ///< Number of threads in the pool
  bool paused;    ///< Whether the thread pool is paused
} ggml_threadpool_params;

/**
 * @brief Opaque GGML thread pool structure
 * 
 * Manages a pool of worker threads for parallel GGML operations.
 * Provides efficient thread management and work distribution.
 */
typedef struct ggml_threadpool {
  uint8_t _private[0];  ///< Private implementation data (do not access directly)
} ggml_threadpool;

/**
 * @brief Union for model key-value override values
 * 
 * Supports multiple data types for overriding model metadata values.
 * Allows flexible configuration of model parameters at runtime.
 */
typedef union llama_model_kv_override_value {
  int64_t val_i64;      ///< 64-bit integer value
  double val_f64;       ///< 64-bit floating-point value
  bool val_bool;        ///< Boolean value
  char val_str[128];    ///< String value (max 128 characters)
} llama_model_kv_override_value;

/**
 * @brief Model key-value override structure
 * 
 * Allows overriding specific model metadata key-value pairs.
 * Used for runtime customization of model behavior and parameters.
 */
typedef struct llama_model_kv_override {
  char key[128];                              ///< Key name (max 128 characters)
  int tag;                                    ///< Value type tag
  union llama_model_kv_override_value value;  ///< Override value
} llama_model_kv_override;

/**
 * @brief Model tensor buffer type override
 * 
 * Allows overriding tensor buffer types for specific tensor patterns.
 * Used for optimizing memory layout and device placement.
 */
typedef struct llama_model_tensor_buft_override {
  const char *pattern;  ///< Tensor name pattern to match
  int buft_type;        ///< Buffer type to use for matching tensors
} llama_model_tensor_buft_override;
/**
 * @brief LLaMA model parameters
 * 
 * Configuration structure for loading a LLaMA model, including device settings,
 * memory mapping options, tensor overrides, and progress callbacks.
 */
typedef struct llama_model_params {
  int n_gpu_layers;    ///< Number of layers to offload to GPU
  int main_gpu;        ///< Main GPU device index
  int split_mode;      ///< Tensor split mode
  const float *tensor_split; ///< Array specifying tensor split ratios
  bool use_mmap;       ///< Enable memory-mapped file loading
  bool use_mlock;      ///< Lock model memory in RAM
  bool check_tensors;  ///< Enable tensor integrity checks
  bool use_extra_bufts;///< Use extra buffer types for tensors
  const char *const *devices; ///< Array of device names for model execution
  const struct llama_model_kv_override *kv_overrides; ///< Key-value overrides for model metadata
  const struct llama_model_tensor_buft_override *tensor_buft_overrides; ///< Tensor buffer type overrides
  bool (*progress_callback)(float progress, void *user_data); ///< Progress callback function
  void *progress_callback_user_data; ///< User data for progress callback
} llama_model_params;

/**
 * @brief LLaMA context parameters
 * 
 * Configuration structure for initializing a LLaMA context, including
 * context window size, batch settings, threading, RoPE parameters, pooling,
 * attention types, and evaluation callbacks.
 */
typedef struct llama_context_params {
  int n_ctx;           ///< Context window size
  int n_seq_max;       ///< Maximum number of sequences
  int n_batch;         ///< Batch size
  int n_ubatch;        ///< Micro-batch size
  int n_threads;       ///< Number of threads for computation
  int n_threads_batch; ///< Number of threads for batch processing
  bool embeddings;     ///< Enable embedding mode
  int rope_scaling_type; ///< RoPE scaling type
  float rope_freq_base; ///< RoPE frequency base
  float rope_freq_scale;///< RoPE frequency scale
  float yarn_ext_factor;///< Yarn extension factor
  float yarn_attn_factor;///< Yarn attention factor
  float yarn_beta_fast; ///< Yarn fast beta parameter
  float yarn_beta_slow; ///< Yarn slow beta parameter
  int yarn_orig_ctx;    ///< Original context size for Yarn
  int pooling_type;     ///< Pooling type
  int attention_type;   ///< Attention type
  int flash_attn_type;  ///< Flash attention type
  void (*cb_eval)(void);///< Evaluation callback function
  void *cb_eval_user_data; ///< User data for evaluation callback
  bool offload_kqv;     ///< Enable offloading of KQV tensors
  bool no_perf;         ///< Disable performance metrics
  bool op_offload;      ///< Enable operator offloading
  bool swa_full;        ///< Enable full SWA (Stochastic Weight Averaging)
  bool kv_unified;      ///< Use unified KV cache
  int type_k;           ///< Data type for K tensor
  int type_v;           ///< Data type for V tensor
} llama_context_params;

/**
 * @brief LoRA (Low-Rank Adaptation) adapter configuration
 * 
 * Configuration structure for LoRA adapters that modify model behavior
 * for specific tasks without retraining the entire model.
 */
typedef struct lora_adapter {
  const char *path;         ///< Path to the LoRA adapter file
  float scale;              ///< Scaling factor for adapter influence
  void *ptr;                ///< Pointer to adapter implementation
  const char *task_name;    ///< Name of the task this adapter is for
  const char *prompt_prefix; ///< Prefix to add to prompts when using this adapter
} lora_adapter;

/** @brief Null token constant representing an invalid or empty token */
#define LLAMA_TOKEN_NULL -1

#ifdef __cplusplus
extern "C" 
#endif // __cplusplus

///@name Core Rust Interface Functions
///@{

/**
 * @brief Initialize and run the main LLM system
 * 
 * Entry point for the primary LLM functionality. Initializes the system
 * and prepares it for model operations.
 */
void rust_llm(void);

/**
 * @brief Generic Rust function call
 * 
 * General-purpose function for testing Rust integration and
 * basic functionality verification.
 */
void rust_func(void);

/**
 * @brief Retrieve system CPU information
 * 
 * Gathers detailed information about the system's CPU including
 * core count, frequency, and brand information.
 * 
 * @param[out] out Pointer to CpuInfo structure to fill with CPU data
 * @return true if CPU information was successfully retrieved, false otherwise
 */
bool rust_get_cpu_info(struct CpuInfo *out);

/**
 * @brief Get CPU brand string
 * 
 * Retrieves the CPU brand name as a string.
 * 
 * @param[out] buf Buffer to store the CPU brand string
 * @param[in] buf_len Maximum length of the buffer
 * @return Number of bytes written to buffer, or 0 on error
 */
uintptr_t rust_get_cpu_brand(uint8_t *buf, uintptr_t buf_len);

/**
 * @brief Print hello message from LLM Rust system
 * 
 * Simple greeting function to verify system functionality
 * and basic integration between C++ and Rust components.
 */
void llmrust_hello(void);

///@}
///@name Logging Functions
///@{
/**
 * @brief General purpose logging function
 * 
 * @param[in] _fmt Format string for log message
 */
void LOG(const char *_fmt);

/**
 * @brief Log informational message
 * 
 * @param[in] _fmt Format string for informational log message
 */
void LOG_INF(const char *_fmt);

/**
 * @brief Log warning message
 * 
 * @param[in] _fmt Format string for warning log message
 */
void LOG_WRN(const char *_fmt);

/**
 * @brief Log error message
 * 
 * @param[in] _fmt Format string for error log message
 */
void LOG_ERR(const char *_fmt);

/**
 * @brief Log debug message
 * 
 * @param[in] _fmt Format string for debug log message
 */
void LOG_DBG(const char *_fmt);

/**
 * @brief Log counter/statistics message
 * 
 * @param[in] _fmt Format string for counter log message
 */
void LOG_CNT(const char *_fmt);
///@}
///@name Console Management Functions
///@{

/**
 * @brief Initialize console subsystem
 * 
 * Sets up the console for interactive input/output operations.
 * 
 * @param[in] _simple_io Enable simple I/O mode (no advanced features)
 * @param[in] _use_color Enable colored output
 */
void console_init(bool _simple_io, bool _use_color);

/**
 * @brief Cleanup console resources
 * 
 * Properly shuts down the console subsystem and releases resources.
 */
void console_cleanup(void);

/**
 * @brief Set console display mode
 * 
 * Configures the console display behavior and formatting.
 * 
 * @param[in] _mode Display mode identifier
 */
void console_set_display(int _mode);

/**
 * @brief Read line from console input
 * 
 * Reads a complete line from console input with optional multiline support.
 * 
 * @param[out] _out_line Pointer to store the allocated line string
 * @param[in] _multiline Enable multiline input mode
 * @return true if line was successfully read, false on error or EOF
 */
bool console_readline(char **_out_line, bool _multiline);

/**
 * @brief Display console prompt
 * 
 * Shows the interactive prompt to the user.
 * 
 * @return Status code (0 on success)
 */
int console_prompt(void);

/**
 * @brief Reset console state
 * 
 * Resets the console to its initial state.
 * 
 * @return Status code (0 on success)
 */
int console_reset(void);

/**
 * @brief Handle user input
 * 
 * Processes user input from the console.
 * 
 * @return Status code (0 on success)
 */
int console_user_input(void);

/**
 * @brief Handle console error state
 * 
 * Manages console error conditions and recovery.
 * 
 * @return Error code
 */
int console_error(void);
///@}
///@name Common Initialization Functions
///@{

/**
 * @brief Initialize common subsystems
 * 
 * Performs basic initialization of common LLM subsystems.
 */
void common_init(void);

/**
 * @brief Initialize model and context from parameters
 * 
 * Creates and initializes both model and context using the provided parameters.
 * 
 * @param[in] _params Configuration parameters for initialization
 * @return Initialization result containing model and context holders
 */
struct common_init_result common_init_from_params(struct common_params _params);

/**
 * @brief Free initialization result resources
 * 
 * Properly releases resources allocated during initialization.
 * 
 * @param[in,out] _r Pointer to initialization result to free
 */
void common_init_result_free(struct common_init_result *_r);

/**
 * @brief Print performance statistics
 * 
 * Displays performance metrics for the given context and sampler.
 * 
 * @param[in] _ctx LLaMA context to analyze
 * @param[in] _smpl Common sampler to analyze
 */
void common_perf_print(struct llama_context *_ctx, struct common_sampler *_smpl);

/**
 * @brief Initialize main logging system
 * 
 * Sets up the primary logging infrastructure.
 * 
 * @return Pointer to logging system handle
 */
void *common_log_main(void);

/**
 * @brief Pause logging system
 * 
 * Temporarily suspends logging operations.
 * 
 * @param[in] _ptr Logging system handle
 */
void common_log_pause(void *_ptr);
///@}
///@name LLaMA Backend Functions
///@{

/**
 * @brief Initialize LLaMA backend system
 * 
 * Initializes the core LLaMA backend infrastructure required
 * for model loading and operation.
 */
void llama_backend_init(void);

/**
 * @brief Free LLaMA backend resources
 * 
 * Cleans up and releases all LLaMA backend system resources.
 */
void llama_backend_free(void);

/**
 * @brief Initialize NUMA (Non-Uniform Memory Access) support
 * 
 * Configures NUMA memory allocation strategy for optimal performance
 * on multi-socket systems.
 * 
 * @param[in] _mode NUMA configuration mode
 */
void llama_numa_init(int _mode);

/**
 * @brief Get model vocabulary
 * 
 * Retrieves the vocabulary associated with the given model.
 * 
 * @param[in] _model LLaMA model to query
 * @return Pointer to model vocabulary, or NULL on error
 */
const struct llama_vocab *llama_model_get_vocab(struct llama_model *_model);

/**
 * @brief Get context memory handle
 * 
 * Retrieves the memory management handle for the given context.
 * 
 * @param[in] _ctx LLaMA context to query
 * @return Pointer to memory handle
 */
void *llama_get_memory(struct llama_context *_ctx);

/**
 * @brief Get model training context size
 * 
 * Returns the context window size the model was trained with.
 * 
 * @param[in] _model LLaMA model to query
 * @return Training context size in tokens
 */
int llama_model_n_ctx_train(struct llama_model *_model);

/**
 * @brief Get context window size
 * 
 * Returns the current context window size for the given context.
 * 
 * @param[in] _ctx LLaMA context to query
 * @return Context size in tokens
 */
int llama_n_ctx(struct llama_context *_ctx);

/**
 * @brief Check if model has encoder
 * 
 * Determines whether the model includes an encoder component.
 * 
 * @param[in] _model LLaMA model to check
 * @return true if model has encoder, false otherwise
 */
bool llama_model_has_encoder(struct llama_model *_model);

/**
 * @brief Get decoder start token
 * 
 * Returns the special token used to start decoder sequences.
 * 
 * @param[in] _model LLaMA model to query
 * @return Decoder start token ID
 */
llama_token llama_model_decoder_start_token(struct llama_model *_model);

///@}
///@name Vocabulary and Token Management Functions
///@{

/**
 * @brief Check if vocabulary adds BOS (Beginning of Sequence) token
 * 
 * Determines whether the model's vocabulary configuration automatically
 * adds a BOS token at the beginning of sequences.
 * 
 * @param[in] _vocab Vocabulary to query
 * @return true if BOS token is automatically added, false otherwise
 */
bool llama_vocab_get_add_bos(const struct llama_vocab *_vocab);

/**
 * @brief Check if vocabulary adds EOS (End of Sequence) token
 * 
 * Determines whether the model's vocabulary configuration automatically
 * adds an EOS token at the end of sequences.
 * 
 * @param[in] _vocab Vocabulary to query
 * @return true if EOS token is automatically added, false otherwise
 */
bool llama_vocab_get_add_eos(const struct llama_vocab *_vocab);

/**
 * @brief Get BOS (Beginning of Sequence) token ID
 * 
 * Retrieves the token ID used to mark the beginning of a sequence.
 * 
 * @param[in] _vocab Vocabulary to query
 * @return BOS token ID, or LLAMA_TOKEN_NULL if not available
 */
llama_token llama_vocab_bos(const struct llama_vocab *_vocab);

/**
 * @brief Get EOS (End of Sequence) token ID
 * 
 * Retrieves the token ID used to mark the end of a sequence.
 * 
 * @param[in] _vocab Vocabulary to query
 * @return EOS token ID, or LLAMA_TOKEN_NULL if not available
 */
llama_token llama_vocab_eos(const struct llama_vocab *_vocab);

/**
 * @brief Get EOT (End of Turn/Text) token ID
 * 
 * Retrieves the token ID used to mark the end of a turn or text segment.
 * 
 * @param[in] _vocab Vocabulary to query
 * @return EOT token ID, or LLAMA_TOKEN_NULL if not available
 */
llama_token llama_vocab_eot(const struct llama_vocab *_vocab);

/**
 * @brief Check if token is End-of-Generation (EOG)
 * 
 * Determines whether the given token represents an end-of-generation marker,
 * which indicates that text generation should stop.
 * 
 * @param[in] _vocab Vocabulary containing token definitions
 * @param[in] _tok Token ID to check
 * @return true if token is EOG, false otherwise
 */
bool llama_vocab_is_eog(const struct llama_vocab *_vocab, llama_token _tok);

///@}
///@name Chat Template Functions
///@{

/**
 * @brief Initialize chat template system
 * 
 * Sets up the chat template processing system for the given model.
 * Chat templates are used to format conversations in a model-specific way.
 * 
 * @param[in] _model LLaMA model to initialize templates for
 * @param[in] _user_template Optional user-provided template string, or NULL for default
 * @return Pointer to initialized chat template handle, or NULL on error
 */
void *common_chat_templates_init(struct llama_model *_model, const char *_user_template);

/**
 * @brief Check if chat template was explicitly provided
 * 
 * Determines whether the chat template was explicitly provided by the user
 * or if the system is using a default template.
 * 
 * @param[in] _ptr Chat template handle from common_chat_templates_init()
 * @return true if template was explicitly provided, false if using default
 */
bool common_chat_templates_was_explicit(void *_ptr);

/**
 * @brief Generate chat format example
 * 
 * Creates an example of how the chat template formats conversations,
 * useful for documentation and debugging purposes.
 * 
 * @param[in] _ptr Chat template handle
 * @param[in] _use_jinja Whether to use Jinja template engine
 * @param[in] _default_kwargs Default keyword arguments for template
 * @return Example chat format string, or NULL on error
 */
const char *common_chat_format_example(void *_ptr, bool _use_jinja, const char *_default_kwargs);

/**
 * @brief Format single chat message
 * 
 * Applies the chat template to format a single message in the context
 * of an ongoing conversation.
 * 
 * @param[in] _ptr Chat template handle
 * @param[in] _msgs_json JSON string containing conversation history
 * @param[in] _new_msg_json JSON string containing the new message
 * @param[in] _is_user Whether the new message is from the user (true) or assistant (false)
 * @param[in] _use_jinja Whether to use Jinja template engine
 * @return Formatted message string, or NULL on error
 */
const char *common_chat_format_single(void *_ptr,
                                      const char *_msgs_json,
                                      const char *_new_msg_json,
                                      bool _is_user,
                                      bool _use_jinja);

/**
 * @brief Apply chat template to conversation
 * 
 * Processes the entire conversation through the chat template system
 * to produce a properly formatted prompt for the model.
 * 
 * @param[in] _ptr Chat template handle
 * @return Applied template result containing formatted prompt
 */
struct common_applied_template common_chat_templates_apply(void *_ptr);

///@}
///@name Tokenization Functions
///@{

/**
 * @brief Tokenize text string
 * 
 * Converts a text string into a sequence of tokens that can be processed
 * by the model. Supports various tokenization options for different use cases.
 * 
 * @param[in] _ctx LLaMA context containing tokenizer
 * @param[in] _text Text string to tokenize
 * @param[in] _add_special Whether to add special tokens (BOS, EOS, etc.)
 * @param[in] _parse_special Whether to parse special token syntax in text
 * @return Token list containing tokenized sequence
 */
struct token_list common_tokenize(struct llama_context *_ctx,
                                  const char *_text,
                                  bool _add_special,
                                  bool _parse_special);

/**
 * @brief Convert tokens back to string
 * 
 * Detokenizes a sequence of tokens back into human-readable text.
 * This is the inverse operation of tokenization.
 * 
 * @param[in] _ctx LLaMA context containing vocabulary
 * @param[in] _toks Token list to convert to string
 * @return Detokenized string, or NULL on error
 */
const char *string_from(struct llama_context *_ctx, struct token_list _toks);

/**
 * @brief Convert single token to text piece
 * 
 * Converts a single token ID to its corresponding text representation.
 * Useful for debugging and incremental text generation.
 * 
 * @param[in] _ctx LLaMA context containing vocabulary
 * @param[in] _tok Token ID to convert
 * @param[in] _special Whether to include special token formatting
 * @return Text piece for the token, or NULL on error
 */
const char *common_token_to_piece(struct llama_context *_ctx, llama_token _tok, bool _special);

///@}
///@name Text Generation Sampling Functions
///@{

/**
 * @brief Initialize text generation sampler
 * 
 * Creates and configures a sampler for controlling text generation behavior.
 * The sampler manages various sampling strategies like temperature, top-k, top-p, etc.
 * 
 * @param[in] _model LLaMA model to create sampler for
 * @param[in] _params Sampling parameters configuration
 * @return Pointer to initialized sampler, or NULL on error
 */
struct common_sampler *common_sampler_init(struct llama_model *_model,
                                           struct sampling_params _params);

/**
 * @brief Free sampler resources
 * 
 * Releases all resources associated with the given sampler.
 * 
 * @param[in] _s Sampler to free (must not be NULL)
 */
void common_sampler_free(struct common_sampler *_s);

/**
 * @brief Get sampler random seed
 * 
 * Retrieves the random seed currently used by the sampler for
 * reproducible text generation.
 * 
 * @param[in] _s Sampler to query
 * @return Current random seed value
 */
unsigned int common_sampler_get_seed(struct common_sampler *_s);

/**
 * @brief Get sampler configuration string
 * 
 * Returns a human-readable description of the sampler's current
 * configuration and parameters.
 * 
 * @param[in] _s Sampler to describe
 * @return Configuration string, or NULL on error
 */
const char *common_sampler_print(struct common_sampler *_s);

/**
 * @brief Accept or reject a sampled token
 * 
 * Informs the sampler whether the previously sampled token was accepted
 * or rejected, allowing it to update its internal state and grammar.
 * 
 * @param[in] _s Sampler to update
 * @param[in] _tok Token that was sampled
 * @param[in] _accept_grammar Whether to accept the token for grammar purposes
 */
void common_sampler_accept(struct common_sampler *_s, llama_token _tok, bool _accept_grammar);

/**
 * @brief Sample next token from model logits
 * 
 * Uses the configured sampling strategy to select the next token
 * from the model's output logits.
 * 
 * @param[in] _s Sampler to use for token selection
 * @param[in] _ctx LLaMA context containing model state
 * @param[in] _seq_id Sequence ID for multi-sequence generation
 * @return Selected token ID
 */
llama_token common_sampler_sample(struct common_sampler *_s,
                                  struct llama_context *_ctx,
                                  int _seq_id);

/**
 * @brief Get previous tokens as string
 * 
 * Retrieves a string representation of the last N tokens that were
 * processed by the sampler.
 * 
 * @param[in] _s Sampler containing token history
 * @param[in] _ctx LLaMA context for token-to-text conversion
 * @param[in] _n_prev Number of previous tokens to include
 * @return String representation of previous tokens, or NULL on error
 */
const char *common_sampler_prev_str(struct common_sampler *_s,
                                    struct llama_context *_ctx,
                                    int _n_prev);

/**
 * @brief Get last sampled token
 * 
 * Returns the most recently sampled token ID.
 * 
 * @param[in] _s Sampler to query
 * @return Last sampled token ID, or LLAMA_TOKEN_NULL if none
 */
llama_token common_sampler_last(struct common_sampler *_s);

/**
 * @brief Reset sampler state
 * 
 * Resets the sampler to its initial state, clearing history and
 * internal state while preserving configuration.
 * 
 * @param[in] _s Sampler to reset
 */
void common_sampler_reset(struct common_sampler *_s);

///@}
///@name Model Inference Functions
///@{

/**
 * @brief Encode tokens through model
 * 
 * Processes a batch of tokens through the model encoder, updating
 * the model's internal state with the encoded representations.
 * 
 * @param[in] _ctx LLaMA context containing model state
 * @param[in] _batch Batch of tokens to encode
 * @return 0 on success, negative value on error
 */
int llama_encode(struct llama_context *_ctx, struct llama_batch _batch);

/**
 * @brief Decode tokens and generate logits
 * 
 * Processes a batch of tokens through the model decoder to generate
 * output logits for next token prediction.
 * 
 * @param[in] _ctx LLaMA context containing model state
 * @param[in] _batch Batch of tokens to decode
 * @return 0 on success, negative value on error
 */
int llama_decode(struct llama_context *_ctx, struct llama_batch _batch);

/**
 * @brief Create batch from token array
 * 
 * Creates a simple batch containing a sequence of tokens.
 * Useful for processing single sequences through the model.
 * 
 * @param[in] _data Array of token IDs
 * @param[in] _n Number of tokens in the array
 * @return Batch structure containing the tokens
 */
struct llama_batch llama_batch_get_one(const llama_token *_data, int _n);

///@}
///@name State Management Functions
///@{

/**
 * @brief Load model state from file
 * 
 * Restores the model's internal state from a previously saved file,
 * including KV cache and token history. Enables resuming conversations
 * or continuing from specific checkpoints.
 * 
 * @param[in] _ctx LLaMA context to load state into
 * @param[in] _path Path to the state file
 * @param[out] _out_tokens Buffer to store loaded tokens
 * @param[in] _capacity Maximum number of tokens the buffer can hold
 * @param[out] out_count Number of tokens actually loaded
 * @return true on successful load, false on error
 */
bool llama_state_load_file(struct llama_context *_ctx,
                           const char *_path,
                           llama_token *_out_tokens,
                           uintptr_t _capacity,
                           uintptr_t *out_count);

/**
 * @brief Save model state to file
 * 
 * Saves the model's current internal state to a file, including
 * KV cache and token history. Enables checkpointing and resuming
 * conversations later.
 * 
 * @param[in] _ctx LLaMA context to save state from
 * @param[in] _path Path where to save the state file
 * @param[in] _tokens Array of tokens to save with the state
 * @param[in] _count Number of tokens in the array
 * @return true on successful save, false on error
 */
bool llama_state_save_file(struct llama_context *_ctx,
                           const char *_path,
                           const llama_token *_tokens,
                           uintptr_t _count);

///@}
///@name Memory Sequence Management Functions
///@{

/**
 * @brief Remove memory sequence range
 * 
 * Removes tokens from a specific sequence within the specified position range.
 * Used for memory management and context window optimization.
 * 
 * @param[in] _mem Memory handle from llama_get_memory()
 * @param[in] _seq_id Sequence ID to modify
 * @param[in] _p0 Start position (inclusive)
 * @param[in] _p1 End position (exclusive)
 */
void llama_memory_seq_rm(void *_mem, int _seq_id, uintptr_t _p0, int _p1);

/**
 * @brief Add offset to memory sequence range
 * 
 * Shifts token positions in a sequence by adding a delta value.
 * Useful for inserting content or adjusting sequence positions.
 * 
 * @param[in] _mem Memory handle from llama_get_memory()
 * @param[in] _seq_id Sequence ID to modify
 * @param[in] _p0 Start position for the operation
 * @param[in] _p1 End position for the operation
 * @param[in] _delta Value to add to positions in the range
 */
void llama_memory_seq_add(void *_mem, int _seq_id, uintptr_t _p0, int _p1, int _delta);

/**
 * @brief Divide memory sequence positions
 * 
 * Divides token positions in a sequence by a divisor value.
 * Used for position scaling and memory optimization.
 * 
 * @param[in] _mem Memory handle from llama_get_memory()
 * @param[in] _seq_id Sequence ID to modify
 * @param[in] _p0 Start position for the operation
 * @param[in] _p1 End position for the operation
 * @param[in] _div Divisor value for position scaling
 */
void llama_memory_seq_div(void *_mem, int _seq_id, uintptr_t _p0, uintptr_t _p1, int _div);

///@}
///@name GGML Backend Management Functions
///@{

/**
 * @brief Get backend device by type
 * 
 * Retrieves a backend device handle for the specified device type.
 * Used for managing different computational backends (CPU, GPU, etc.).
 * 
 * @param[in] _dev_type Device type identifier (use GGML_BACKEND_DEVICE_TYPE_*)
 * @return Pointer to backend device, or NULL if not available
 */
struct ggml_backend_device *ggml_backend_dev_by_type(int _dev_type);

/**
 * @brief Get backend registry for device
 * 
 * Retrieves the backend registry associated with a specific device.
 * The registry contains information about available operations and capabilities.
 * 
 * @param[in] _dev Backend device to query
 * @return Pointer to backend registry, or NULL on error
 */
struct ggml_backend_registry *ggml_backend_dev_backend_reg(struct ggml_backend_device *_dev);

/**
 * @brief Get procedure address from backend registry
 * 
 * Retrieves a function pointer for a named procedure from the backend registry.
 * Enables dynamic loading of backend-specific operations.
 * 
 * @param[in] _reg Backend registry to search
 * @param[in] _name Name of the procedure to find
 * @return Function pointer, or NULL if procedure not found
 */
void *ggml_backend_reg_get_proc_address(struct ggml_backend_registry *_reg, const char *_name);

///@}
///@name Thread Pool Management Functions
///@{

/**
 * @brief Create thread pool parameters from CPU parameters
 * 
 * Converts CPU configuration parameters into thread pool parameters
 * suitable for GGML operations.
 * 
 * @param[in] p CPU parameters to convert
 * @return Thread pool parameters structure
 */
struct ggml_threadpool_params ggml_threadpool_params_from_cpu_params(struct cpu_params p);

/**
 * @brief Compare thread pool parameters
 * 
 * Checks if two thread pool parameter structures are identical.
 * Useful for determining if thread pool reconfiguration is needed.
 * 
 * @param[in] _a First parameter structure to compare
 * @param[in] _b Second parameter structure to compare
 * @return true if parameters match, false otherwise
 */
bool ggml_threadpool_params_match(const struct ggml_threadpool_params *_a,
                                  const struct ggml_threadpool_params *_b);

/**
 * @brief Attach thread pools to LLaMA context
 * 
 * Associates thread pools with a LLaMA context for parallel computation.
 * Separate thread pools can be used for different types of operations.
 * 
 * @param[in] _ctx LLaMA context to attach thread pools to
 * @param[in] _tp_default Default thread pool for general operations
 * @param[in] _tp_batch Thread pool specifically for batch operations
 */
void llama_attach_threadpool(struct llama_context *_ctx,
                             struct ggml_threadpool *_tp_default,
                             struct ggml_threadpool *_tp_batch);

///@}
///@name System Utility Functions
///@{

/**
 * @brief Set process priority
 * 
 * Adjusts the operating system priority of the current process.
 * Higher priority can improve performance but may affect system responsiveness.
 * 
 * @param[in] _priority Priority level (system-dependent scale)
 */
void set_process_priority(int _priority);

/**
 * @brief Get system information string
 * 
 * Retrieves comprehensive system information including hardware details,
 * configuration parameters, and runtime settings.
 * 
 * @param[in] _params Configuration parameters to include in system info
 * @return System information string, or NULL on error
 */
const char *common_params_get_system_info(struct common_params _params);

/**
 * @brief Get CPU device type constant
 * 
 * Returns the constant value representing CPU device type for backend operations.
 * 
 * @return CPU device type identifier
 */
int GGML_BACKEND_DEVICE_TYPE_CPU(void);

/**
 * @brief Get common vector string length
 * 
 * Returns the length of common vector string structures.
 * Used for memory allocation and string handling.
 * 
 * @return Length value for vector strings
 */
uintptr_t common_vec_str_len(void);

///@}
///@name Rust Logging Functions
///@{

/**
 * @brief Log informational message (rs_log variant)
 * 
 * Logs an informational message using the Rust logging infrastructure.
 * 
 * @param[in] msg Message string to log
 */
void rs_log_info(const char *msg);

/**
 * @brief Log warning message (rs_log variant)
 * 
 * Logs a warning message using the Rust logging infrastructure.
 * 
 * @param[in] msg Warning message string to log
 */
void rs_log_warn(const char *msg);

/**
 * @brief Log error message (rs_log variant)
 * 
 * Logs an error message using the Rust logging infrastructure.
 * 
 * @param[in] msg Error message string to log
 */
void rs_log_error(const char *msg);

/**
 * @brief Log debug message (rs_log variant)
 * 
 * Logs a debug message using the Rust logging infrastructure.
 * Debug messages are typically only shown in debug builds.
 * 
 * @param[in] msg Debug message string to log
 */
void rs_log_debug(const char *msg);

/**
 * @brief Log trace message (rs_log variant)
 * 
 * Logs a trace message using the Rust logging infrastructure.
 * Trace messages provide the most detailed logging information.
 * 
 * @param[in] msg Trace message string to log
 */
void rs_log_trace(const char *msg);

/**
 * @brief Log informational message (rslog variant)
 * 
 * Alternative informational logging function using Rust infrastructure.
 * 
 * @param[in] msg Message string to log
 */
void rslog_info(const char *msg);

/**
 * @brief Log warning message (rslog variant)
 * 
 * Alternative warning logging function using Rust infrastructure.
 * 
 * @param[in] msg Warning message string to log
 */
void rslog_warn(const char *msg);

/**
 * @brief Log error message (rslog variant)
 * 
 * Alternative error logging function using Rust infrastructure.
 * 
 * @param[in] msg Error message string to log
 */
void rslog_error(const char *msg);

/**
 * @brief Log debug message (rslog variant)
 * 
 * Alternative debug logging function using Rust infrastructure.
 * 
 * @param[in] msg Debug message string to log
 */
void rslog_debug(const char *msg);

/**
 * @brief Log trace message (rslog variant)
 * 
 * Alternative trace logging function using Rust infrastructure.
 * 
 * @param[in] msg Trace message string to log
 */
void rslog_trace(const char *msg);

///@}
///@name Application Control Functions
///@{

/**
 * @brief Signal interrupt handler
 * 
 * Handles interrupt signals (like Ctrl+C) for graceful application shutdown.
 * 
 * @param[in] signo Signal number that was received
 */
void sigint_handler(int signo);

/**
 * @brief Print application usage information
 * 
 * Displays command-line usage help and available options.
 * 
 * @param[in] argc Argument count from main()
 * @param[in] argv Argument vector from main()
 */
void print_usage(int argc, char **argv);

/**
 * @brief Rust application entry point
 * 
 * Main entry point for Rust-side application logic.
 * Processes command-line arguments and coordinates application execution.
 * 
 * @param[in] argc Argument count
 * @param[in] argv Argument vector
 * @return Exit code (0 for success, non-zero for error)
 */
int32_t rust_entry(int32_t argc, char **argv);

/**
 * @brief Call Rust logging system
 * 
 * Invokes the Rust logging system initialization and setup.
 */
void call_log_rs(void);

/**
 * @brief Call Rust logging with parameters
 * 
 * Invokes the Rust logging system with specific parameter configuration.
 * 
 * @param[in] _params_ptr Pointer to common parameters for logging configuration
 */
void call_log_rs_real(struct common_params *_params_ptr);

///@}
///@name Core LLaMA Model Functions
///@{

/**
 * @brief Load LLaMA model from file
 * 
 * Loads a LLaMA model from a GGUF file with the specified parameters.
 * This is the primary function for model initialization.
 * 
 * @param[in] path_model Path to the GGUF model file
 * @param[in] params Model loading parameters and configuration
 * @return Pointer to loaded model, or NULL on error
 */
struct llama_model *llama_model_load_from_file(const char *path_model,
                                               struct llama_model_params params);

/**
 * @brief Initialize context from model
 * 
 * Creates an execution context for the given model with specified parameters.
 * The context manages the runtime state for inference operations.
 * 
 * @param[in] model Loaded LLaMA model
 * @param[in] params Context initialization parameters
 * @return Pointer to initialized context, or NULL on error
 */
struct llama_context *llama_init_from_model(struct llama_model *model,
                                            struct llama_context_params params);

/**
 * @brief Free LLaMA model resources
 * 
 * Releases all memory and resources associated with a LLaMA model.
 * 
 * @param[in] model Model to free (must not be NULL)
 */
void llama_model_free(struct llama_model *model);

/**
 * @brief Free LLaMA context resources
 * 
 * Releases all memory and resources associated with a LLaMA context.
 * 
 * @param[in] ctx Context to free (must not be NULL)
 */
void llama_free(struct llama_context *ctx);

/**
 * @brief Get default model parameters
 * 
 * Returns a model parameters structure initialized with default values.
 * These defaults provide a good starting point for most use cases.
 * 
 * @return Default model parameters structure
 */
struct llama_model_params llama_model_default_params(void);

/**
 * @brief Get default context parameters
 * 
 * Returns a context parameters structure initialized with default values.
 * These defaults provide a good starting point for most use cases.
 * 
 * @return Default context parameters structure
 */
struct llama_context_params llama_context_default_params(void);

///@}
///@name Model Information Functions
///@{

/**
 * @brief Get number of model layers
 * 
 * Returns the number of transformer layers in the loaded model.
 * 
 * @param[in] model LLaMA model to query
 * @return Number of layers in the model
 */
int llama_model_n_layer(struct llama_model *model);

/**
 * @brief Check if model has decoder
 * 
 * Determines whether the model includes a decoder component.
 * Most LLaMA models are decoder-only architectures.
 * 
 * @param[in] model LLaMA model to check
 * @return true if model has decoder, false otherwise
 */
bool llama_model_has_decoder(struct llama_model *model);

/**
 * @brief Get vocabulary separator token
 * 
 * Returns the token ID used as a separator in the vocabulary.
 * 
 * @param[in] vocab Vocabulary to query
 * @return Separator token ID, or LLAMA_TOKEN_NULL if not available
 */
llama_token llama_vocab_sep(const struct llama_vocab *vocab);

/**
 * @brief Get vocabulary size
 * 
 * Returns the total number of tokens in the model's vocabulary.
 * 
 * @param[in] vocab Vocabulary to query
 * @return Number of tokens in vocabulary
 */
int llama_vocab_n_tokens(const struct llama_vocab *vocab);

/**
 * @brief Get context pooling type
 * 
 * Returns the pooling method used by the context for sequence processing.
 * 
 * @param[in] ctx LLaMA context to query
 * @return Pooling type identifier
 */
int llama_pooling_type(struct llama_context *ctx);

///@}
///@name Memory and Performance Management Functions
///@{

/**
 * @brief Check if memory can be shifted
 * 
 * Determines whether the memory system supports shifting operations
 * for context window management.
 * 
 * @param[in] mem Memory handle from llama_get_memory()
 * @return true if shifting is supported, false otherwise
 */
bool llama_memory_can_shift(void *mem);

/**
 * @brief Clear context memory
 * 
 * Clears the context's memory, optionally including the KV cache.
 * Used for resetting conversation state or freeing memory.
 * 
 * @param[in] mem Memory handle from llama_get_memory()
 * @param[in] clear_kv Whether to also clear the KV cache
 */
void llama_memory_clear(void *mem, bool clear_kv);

/**
 * @brief Synchronize context operations
 * 
 * Ensures all pending operations on the context are completed.
 * Useful for performance measurement and synchronization.
 * 
 * @param[in] ctx LLaMA context to synchronize
 */
void llama_synchronize(struct llama_context *ctx);

/**
 * @brief Reset performance context counters
 * 
 * Resets all performance measurement counters and timers
 * associated with the context.
 * 
 * @param[in] ctx LLaMA context to reset counters for
 */
void llama_perf_context_reset(struct llama_context *ctx);

/**
 * @brief Set context warmup mode
 * 
 * Enables or disables warmup mode for the context, which can improve
 * performance measurement accuracy by pre-loading operations.
 * 
 * @param[in] ctx LLaMA context to configure
 * @param[in] warmup Whether to enable warmup mode
 */
void llama_set_warmup(struct llama_context *ctx, bool warmup);

///@}
///@name Adapter and Fine-tuning Functions
///@{

/**
 * @brief Initialize LoRA adapter
 * 
 * Loads and initializes a LoRA (Low-Rank Adaptation) adapter for the model.
 * LoRA adapters allow fine-tuning without modifying the base model weights.
 * 
 * @param[in] model LLaMA model to attach adapter to
 * @param[in] path Path to the LoRA adapter file
 * @return Pointer to initialized adapter, or NULL on error
 */
void *llama_adapter_lora_init(struct llama_model *model, const char *path);

/**
 * @brief Get adapter metadata value as string
 * 
 * Retrieves metadata from an adapter as a string value.
 * Useful for querying adapter properties and configuration.
 * 
 * @param[in] adapter Adapter handle from llama_adapter_lora_init()
 * @param[in] key Metadata key to retrieve
 * @param[out] buf Buffer to store the string value
 * @param[in] buf_size Size of the output buffer
 * @return Length of the retrieved string, or negative on error
 */
int llama_adapter_meta_val_str(void *adapter, const char *key, char *buf, uintptr_t buf_size);

/**
 * @brief Apply control vector to context
 * 
 * Applies a control vector to influence model behavior across specified layers.
 * Control vectors can be used to steer model outputs in desired directions.
 * 
 * @param[in] ctx LLaMA context to apply vector to
 * @param[in] data Control vector data array
 * @param[in] len Length of the control vector data
 * @param[in] n_embd Embedding dimension size
 * @param[in] layer_start First layer to apply the vector to
 * @param[in] layer_end Last layer to apply the vector to
 * @return 0 on success, negative on error
 */
int llama_apply_adapter_cvec(struct llama_context *ctx,
                             const float *data,
                             uintptr_t len,
                             int n_embd,
                             int layer_start,
                             int layer_end);

///@}
///@name Parameter Conversion and Enhanced Initialization Functions
///@{

/**
 * @brief Initialize thread pool parameters
 * 
 * Initializes a thread pool parameters structure with the specified
 * number of threads and default settings.
 * 
 * @param[out] params Thread pool parameters structure to initialize
 * @param[in] n_threads Number of threads for the pool
 */
void ggml_threadpool_params_init(struct ggml_threadpool_params *params, int n_threads);

/**
 * @brief Convert common parameters to LLaMA model parameters
 * 
 * Converts high-level common parameters into LLaMA-specific model parameters.
 * This function bridges the gap between user-friendly configuration and
 * internal model requirements.
 * 
 * @param[in] params Common parameters to convert
 * @return LLaMA model parameters structure
 */
struct llama_model_params common_model_params_to_llama(const struct common_params *params);

/**
 * @brief Convert common parameters to LLaMA context parameters
 * 
 * Converts high-level common parameters into LLaMA-specific context parameters.
 * This function bridges the gap between user-friendly configuration and
 * internal context requirements.
 * 
 * @param[in] params Common parameters to convert
 * @return LLaMA context parameters structure
 */
struct llama_context_params common_context_params_to_llama(const struct common_params *params);

/**
 * @brief Enhanced initialization from parameters
 * 
 * Advanced initialization function that provides enhanced error handling
 * and configuration options compared to the basic initialization.
 * 
 * @param[in] params Common parameters for initialization
 * @return Initialization result with model and context, or error state
 */
struct common_init_result common_init_from_params_enhanced(const struct common_params *params);

///@}
///@name Batch Processing Functions
///@{

/**
 * @brief Clear batch contents
 * 
 * Resets a batch structure to empty state, clearing all tokens
 * and associated metadata.
 * 
 * @param[in,out] batch Batch structure to clear
 */
void common_batch_clear(struct llama_batch *batch);

/**
 * @brief Add token to batch
 * 
 * Adds a token with associated metadata to a batch for processing.
 * Supports multi-sequence batching with position and logit control.
 * 
 * @param[in,out] batch Batch to add token to
 * @param[in] id Token ID to add
 * @param[in] pos Position of the token in the sequence
 * @param[in] seq_ids Array of sequence IDs this token belongs to
 * @param[in] seq_ids_len Number of sequence IDs in the array
 * @param[in] logits Whether to compute logits for this token
 */
void common_batch_add(struct llama_batch *batch,
                      llama_token id,
                      int pos,
                      const int *seq_ids,
                      uintptr_t seq_ids_len,
                      bool logits);

///@}
///@name Model Endpoint and Adapter Management Functions
///@{

/**
 * @brief Get model endpoint information
 * 
 * Returns endpoint information for the currently loaded model.
 * Used for API integration and model identification.
 * 
 * @return Model endpoint string, or NULL if not available
 */
const char *get_model_endpoint(void);

/**
 * @brief Set LoRA adapters for context
 * 
 * Applies multiple LoRA adapters to a context for multi-task fine-tuning.
 * Allows combining different adapters for complex model behavior.
 * 
 * @param[in] ctx LLaMA context to apply adapters to
 * @param[in] adapters Array of LoRA adapter configurations
 * @param[in] adapter_count Number of adapters in the array
 */
void common_set_adapter_lora(struct llama_context *ctx,
                             const struct lora_adapter *adapters,
                             uintptr_t adapter_count);

/**
 * @brief Load control vectors from files
 * 
 * Loads control vectors from multiple files for behavior steering.
 * Control vectors can influence model outputs in specific directions.
 * 
 * @param[in] file_paths Array of file paths containing control vectors
 * @param[in] count Number of file paths in the array
 * @return Handle to loaded control vectors, or NULL on error
 */
void *common_control_vector_load(const char *const *file_paths, uintptr_t count);

///@}
///@name Dynamic Model Management Functions
///@{

/**
 * @brief Initialize GGUF model with automatic discovery
 * 
 * Automatically discovers available GGUF models in the models directory
 * and initializes the most suitable one based on system configuration
 * and environment variables.
 * 
 * This function implements the core dynamic model management feature,
 * eliminating the need for hardcoded model paths.
 * 
 * @return Initialization result containing model and context, or error state
 * @see generate_model_config()
 * @see list_gguf_models()
 */
struct common_init_result init_gguf_model_auto(void);

/**
 * @brief Set model path environment variable
 * 
 * Configures the MODEL_PATH environment variable for direct model specification.
 * When set, this overrides automatic model discovery.
 * 
 * @param[in] model_path Full path to the GGUF model file
 * @return 0 on success, negative value on error
 * @note This function provides runtime model configuration without code changes
 */
int set_model_path_env(const char *model_path);

/**
 * @brief Set default model environment variable
 * 
 * Configures the DEFAULT_MODEL environment variable to specify which model
 * in the models directory should be used as the default choice.
 * 
 * @param[in] model_name Filename of the model in the models directory
 * @return 0 on success, negative value on error
 * @note Used in conjunction with MODELS_DIR for flexible model selection
 */
int set_default_model_env(const char *model_name);

/**
 * @brief Generate dynamic model configuration
 * 
 * Scans the models directory for available GGUF files and generates
 * a comprehensive models.json configuration file. This file includes
 * model metadata, preferences, and fallback options.
 * 
 * This is a key function in the dynamic configuration system, providing
 * the foundation for automatic model management.
 * 
 * @return 0 on success, negative value on error
 * @see get_model_config_json()
 * @see list_gguf_models()
 */
int generate_model_config(void);

/**
 * @brief Get current model configuration as JSON
 * 
 * Returns the current model configuration in JSON format, including
 * all discovered models, preferences, and environment variable settings.
 * 
 * @return JSON string representation of current configuration, or NULL on error
 * @note The returned string should not be freed by the caller
 * @see generate_model_config()
 */
const char *get_model_config_json(void);

/**
 * @brief Print environment configuration help
 * 
 * Displays comprehensive help information about all supported environment
 * variables for model configuration, including examples and usage patterns.
 * 
 * This function provides user-friendly documentation for the dynamic
 * configuration system.
 * 
 * @see Environment variables: MODEL_PATH, DEFAULT_MODEL, MODELS_DIR, etc.
 */
void print_model_config_help(void);

/**
 * @brief Initialize GGUF model from specific path
 * 
 * C-compatible wrapper for initializing a GGUF model from a specific file path.
 * Provides direct model loading without automatic discovery.
 * 
 * @param[in] model_path Full path to the GGUF model file to load
 * @return Initialization result containing model and context, or error state
 * @note This function bypasses automatic discovery for direct model specification
 */
struct common_init_result init_gguf_model_c(const char *model_path);

/**
 * @brief List all available GGUF models
 * 
 * Scans the models directory and displays detailed information about all
 * discovered GGUF model files, including file sizes, validation status,
 * and metadata when available.
 * 
 * This function implements the `gguf_list` command functionality.
 * 
 * @return Number of models found, or negative value on error
 * @note Output includes model names, sizes, and validation status
 * @see generate_model_config()
 */
int list_gguf_models(void);

/**
 * @brief Test GGUF model initialization
 * 
 * Demonstration function that tests the GGUF model initialization process.
 * Used for verifying system functionality and debugging model loading issues.
 * 
 * @return 0 on successful test, negative value on failure
 * @note This is primarily a development and testing function
 */
int gguf_initialization(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* LLM_RUST_H */
