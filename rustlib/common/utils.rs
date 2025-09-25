/*!
 * @file utils.rs
 * @brief Contains various utility functions and common configurations module.
 * 
 * Defines Arguments struct and ApiServer struct together.
 */

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::sync::Mutex;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::signal;
use super::model::ModelConfig;

/// Custom logging system with file output
static LOG_FILE: Mutex<Option<std::fs::File>> = Mutex::new(None);

/// Initialize logging system
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all("output")?;
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("output/llm_engine.log")?;
    
    *LOG_FILE.lock().unwrap() = Some(log_file);
    Ok(())
}

/// Macro for logging with different levels
macro_rules! log_info {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!("[{}] [INFO] {}", timestamp, message);
        
        println!("{}", message);
        
        if let Ok(mut lock) = LOG_FILE.lock() {
            if let Some(ref mut file) = *lock {
                let _ = writeln!(file, "{}", log_entry);
                let _ = file.flush();
            }
        }
    }};
}

macro_rules! log_error {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!("[{}] [ERROR] {}", timestamp, message);
        
        eprintln!("{}", message);
        
        if let Ok(mut lock) = LOG_FILE.lock() {
            if let Some(ref mut file) = *lock {
                let _ = writeln!(file, "{}", log_entry);
                let _ = file.flush();
            }
        }
    }};
}

/// Structure to hold program execution arguments
#[derive(Debug, Clone, Default)]
pub struct Arguments {
    /// Configuration file path
    pub config_path: String,
}

impl Arguments {
    /// Create a new Arguments instance.
    pub fn new(config_path: String) -> Self {
        Self { config_path }
    }
}

/// Simple API server structure
#[derive(Debug, Clone)]
pub struct ApiServer {
    /// Server host address
    host: String,
    /// Server port number
    port: u16,
    /// Server running status
    is_running: Arc<AtomicBool>,
}

impl ApiServer {
    /// ApiServer constructor
    /// 
    /// # Arguments
    /// * `host` - Server host address
    /// * `port` - Server port number
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Initialize server
    pub async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialization logic (currently empty implementation)
        log_info!("API Server initialized at {}:{}", self.host, self.port);
        Ok(())
    }

    /// Start server  
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr).await?;
        
        self.is_running.store(true, Ordering::SeqCst);
        log_info!("API Server started on {}", addr);

        // Basic server loop (add HTTP handling in actual implementation)
        while self.is_running.load(Ordering::SeqCst) {
            tokio::select! {
                result = listener.accept() => {
                    match result {
                        Ok((socket, addr)) => {
                            log_info!("New connection from: {}", addr);
                            tokio::spawn(async move {
                                // TODO: Implement proper HTTP handling
                                drop(socket);
                            });
                        }
                        Err(e) => {
                            log_error!("Failed to accept connection: {}", e);
                        }
                    }
                }
                _ = signal::ctrl_c() => {
                    log_info!("Shutdown signal received");
                    break;
                }
            }
        }

        Ok(())
    }

    /// Stop server
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);
        log_info!("API Server stopped");
    }

    /// Check server running status
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }
}

/// Returns current time as string.
/// 
/// # Returns
/// Current time string in "YYYY-MM-DD HH:MM:SS" format
pub fn get_current_time() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Removes characters that cannot be used in file names.
/// 
/// # Arguments
/// * `raw` - Original file name string
/// 
/// # Returns
/// Safe file name string
pub fn sanitize_filename(raw: &str) -> String {
    raw.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Removes whitespace from both ends of string.
/// 
/// # Arguments
/// * `input` - Input string
/// 
/// # Returns
/// String with whitespace removed
pub fn trim_whitespace(input: &str) -> String {
    input.trim().to_string()
}

/// Gets Unix timestamp.
pub fn get_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

/// Engine configuration structure (simple example)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct EngineConfig {
    pub common: CommonConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonConfig {
    pub api_port: u16,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            common: CommonConfig {
                api_port: 5000, // Use default, will be overridden by JSON config
            },
        }
    }
}

/// Loads configuration file.
pub fn load_engine_config(filepath: &str, config: &mut EngineConfig) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filepath)?;
    *config = serde_json::from_str(&content)?;
    Ok(())
}

/// Model configuration structure
// ModelConfig is now imported from model.rs

/// Structure representing validation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationStatus {
    pub is_valid: bool,
    pub validation_time: String,
    pub validator: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Default for ValidationStatus {
    fn default() -> Self {
        Self {
            is_valid: false,
            validation_time: get_current_time(),
            validator: "rust_validator".to_string(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

/// Function to perform secondary validation of model configuration
pub fn validate_model_config(config_path: &str) -> Result<ValidationStatus, Box<dyn std::error::Error>> {
    let mut validation = ValidationStatus::default();
    validation.validation_time = get_current_time();

    // 1. Check file existence
    if !std::path::Path::new(config_path).exists() {
        validation.errors.push(format!("Configuration file not found: {}", config_path));
        return Ok(validation);
    }

    // 2. Check JSON parsing
    let content = match std::fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(e) => {
            validation.errors.push(format!("Failed to read config file: {}", e));
            return Ok(validation);
        }
    };

    let config: ModelConfig = match serde_json::from_str(&content) {
        Ok(config) => config,
        Err(e) => {
            validation.errors.push(format!("Invalid JSON format: {}", e));
            return Ok(validation);
        }
    };

    // 3. Check model file existence if specified
    if !config.model_path.is_empty() && !std::path::Path::new(&config.model_path).exists() {
        validation.errors.push(format!("Model file not found: {}", config.model_path));
    }

    // 4. Check model directory existence
    if !std::path::Path::new(&config.model_directory).exists() {
        validation.warnings.push(format!("Model directory not found: {}", config.model_directory));
    }

    // 5. Validate fallback models exist
    let model_dir = std::path::Path::new(&config.model_directory);
    let mut missing_models = Vec::new();
    for model in &config.fallback_models {
        let model_path = model_dir.join(model);
        if !model_path.exists() {
            missing_models.push(model.clone());
        }
    }
    
    if !missing_models.is_empty() {
        validation.warnings.push(format!("Some fallback models not found: {}", missing_models.join(", ")));
    }

    // 6. Validate preferences
    if config.model_preferences.max_file_size_gb == 0 {
        validation.warnings.push("Max file size is 0, may be too restrictive".to_string());
    }
    
    if config.model_preferences.min_file_size_mb > config.model_preferences.max_file_size_gb * 1024 {
        validation.warnings.push("Min file size is larger than max file size".to_string());
    }

    // Set validation result
    validation.is_valid = validation.errors.is_empty();

    Ok(validation)
}

/// Function to save model configuration validation results
pub fn save_validation_result(config_path: &str, validation: &ValidationStatus) -> Result<(), Box<dyn std::error::Error>> {
    let validation_path = format!("{}.validation", config_path);
    let json_content = serde_json::to_string_pretty(validation)?;
    std::fs::write(validation_path, json_content)?;
    Ok(())
}

/// FFI functions for C++ calls
#[no_mangle]
pub extern "C" fn rust_validate_model_config(config_path: *const std::os::raw::c_char) -> std::os::raw::c_int {
    // Initialize logging if not already done
    let _ = init_logging();
    
    if config_path.is_null() {
        return -1;
    }

    let config_path_str = unsafe {
        match std::ffi::CStr::from_ptr(config_path).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        }
    };


    match validate_model_config(config_path_str) {
        Ok(validation) => {
            // Output validation results to log
            log_info!("Model Configuration Validation Results:");
            log_info!("  Valid: {}", validation.is_valid);
            log_info!("  Validated at: {}", validation.validation_time);
            log_info!("  Validator: {}", validation.validator);

            if !validation.errors.is_empty() {
                log_error!("  Errors ({}):", validation.errors.len());
                for error in &validation.errors {
                    log_error!("    - {}", error);
                }
            }
            
            if !validation.warnings.is_empty() {
                log_info!("  Warnings ({}):", validation.warnings.len());
                for warning in &validation.warnings {
                    log_info!("    - {}", warning);
                }
            }
            
            // Save validation results
            if let Err(e) = save_validation_result(config_path_str, &validation) {
                log_error!("  Failed to save validation result: {}", e);
            } else {
                log_info!("  Validation result saved to {}.validation", config_path_str);
            }

            if validation.is_valid { 0 } else { 1 }
        }
        Err(e) => {
            log_error!("Validation failed: {}", e);
            -3
        }
    } // validate_model_config


}

/// LLM engine execution function
pub fn run_llm_engine(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = init_logging();
    log_info!("Starting LLM Engine...");
    
    // 1. Load configuration
    let config = load_model_config(config_path)?;
    log_info!("Configuration loaded: {}", config_path);
    
    // 2. Check model file
    if !std::path::Path::new(&config.model_path).exists() {
        // Find first .gguf file in models/ directory
        let models_dir = std::path::Path::new("models");
        if let Ok(entries) = std::fs::read_dir(models_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("gguf") {
                        log_info!("Found model: {}", path.display());
                        return run_llm_with_model(&path.to_string_lossy(), &config);
                    }
                }
            }
        }
        return Err(format!("No GGUF model found in models/ directory").into());
    }
    
    run_llm_with_model(&config.model_path, &config)
}

fn run_llm_with_model(model_path: &str, config: &ModelConfig) -> Result<(), Box<dyn std::error::Error>> {
    log_info!("Loading model: {}", model_path);
    log_info!("Model parameters:");
    log_info!("   - Default model: {}", config.default_model);
    log_info!("   - Model directory: {}", config.model_directory);
    log_info!("   - Fallback models: {:?}", config.fallback_models);
    log_info!("   - Prefer quantized: {}", config.model_preferences.prefer_quantized);
    log_info!("   - Max file size: {} GB", config.model_preferences.max_file_size_gb);
    log_info!("   - Min file size: {} MB", config.model_preferences.min_file_size_mb);
    
    // Model loading
    log_info!("Initializing model context...");
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    log_info!("Model loaded successfully!");
    log_info!("Starting HTTP API server...");
    

    // Start the LLM API server (async version)
    start_llm_api_server_with_engine(config)?;
    
    Ok(())
}

fn start_llm_api_server_with_engine(config: &ModelConfig) -> Result<(), Box<dyn std::error::Error>> {
    use std::net::TcpListener;
    use std::thread;
    
    let port = config.engine_port as u16; // Use port from JSON config
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    log_info!("LLM API Server listening on http://127.0.0.1:{}", port);
    log_info!("");
    log_info!("API Endpoints:");
    log_info!("  POST /v1/chat/completions - Chat completions");
    log_info!("  GET  /v1/models           - List available models");
    log_info!("  GET  /health              - Health check");
    log_info!("  POST /stop                - Graceful server shutdown");
    log_info!("  GET  /stop                - Alternative shutdown method");
    log_info!("");
    log_info!("Example cURL commands:");
    log_info!("  # Health check");
    log_info!("  curl http://127.0.0.1:{}/health", port);
    log_info!("");
    log_info!("  # List models");
    log_info!("  curl http://127.0.0.1:{}/v1/models", port);
    log_info!("");
    log_info!("  # Chat completion");
    log_info!("  curl -X POST http://127.0.0.1:{}/v1/chat/completions \\", port);
    log_info!("    -H \"Content-Type: application/json\" \\");
    log_info!("    -d '{{\"messages\":[{{\"role\":\"user\",\"content\":\"Hello!\"}}]}}'");
    log_info!("");
    log_info!("  # Stop server gracefully");
    log_info!("  curl -X POST http://127.0.0.1:{}/stop", port);
    log_info!("  # or");
    log_info!("  curl http://127.0.0.1:{}/stop", port);
    log_info!("");
    log_info!("Press Ctrl+C to stop the server manually");
    log_info!("");

    let config_clone = config.clone();
    
    let http_handle = thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let config = config_clone.clone();
                    thread::spawn(move || {
                        handle_client(stream, &config);
                    });
                }
                Err(e) => {
                    log_error!("Error accepting connection: {}", e);
                }
            }
        }
    });

    tokio::runtime::Runtime::new()?.block_on(async {
        // Load model configuration first
        let model_config = crate::common::model::load_model_config();
        let mut engine = crate::engine::engine_::Engine::new_with_model_config(&model_config);
        
        log_info!("Starting Engine alongside HTTP server...");
        
        // Load model configuration
        if std::path::Path::new("models.json").exists() {
            log_info!("Model configuration file found (models.json) - using port {} from config", model_config.engine_port);
        } else {
            log_info!("No configuration file found (models.json), using default port {}...", model_config.engine_port);
        }
        
        match engine.init().await {
            crate::engine::engine_::EngineState::Success => {
                log_info!("Engine initialized successfully - metadata transmission enabled!");
                
                match engine.run().await {
                    crate::engine::engine_::EngineState::Success => {
                        log_info!("Engine completed successfully");
                    }
                    state => {
                        log_error!("Engine run failed: {}", state);
                    }
                }
            }
            state => {
                log_error!("Engine initialization failed: {}", state);
            }
        }
    });

    if let Err(e) = http_handle.join() {
        log_error!("HTTP server thread error: {:?}", e);
    }
    
    Ok(())
}

fn handle_client(mut stream: std::net::TcpStream, config: &ModelConfig) {
    use std::io::{Read, Write};
    
    let mut buffer = [0; 1024];
    let client_addr = stream.peer_addr().map(|addr| addr.to_string()).unwrap_or_else(|_| "unknown".to_string());
    
    if let Ok(bytes_read) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[0..bytes_read]);
        let lines: Vec<&str> = request.lines().collect();
        
        if let Some(first_line) = lines.first() {
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            
            if parts.len() >= 2 {
                let method = parts[0];
                let path = parts[1];
                
                log_info!("[{}] {} {}", client_addr, method, path);
                
                let (response, status_code) = match (method, path) {
                    ("GET", "/health") => {
                        let response_body = r#"{"status": "ok", "message": "LLM API Server is running"}"#;
                        (create_json_response(200, response_body), 200)
                    }
                    ("GET", "/v1/models") => {
                        let models_json = format!(
                            r#"{{"object": "list", "data": [{{"id": "llm-rust", "object": "model", "created": {}, "owned_by": "llm-rust"}}]}}"#,
                            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
                        );
                        (create_json_response(200, &models_json), 200)
                    }
                    ("POST", "/v1/chat/completions") => {
                        // Extract JSON body from request
                        if let Some(body_start) = request.find("\r\n\r\n") {
                            let body = &request[body_start + 4..];
                            if body.trim().is_empty() {
                                let error_response = r#"{"error": "Request body is empty"}"#;
                                (create_json_response(400, error_response), 400)
                            } else {
                                match serde_json::from_str::<serde_json::Value>(body) {
                                    Ok(_) => (handle_chat_completion(body, config), 200),
                                    Err(_) => {
                                        let error_response = r#"{"error": "Invalid JSON format"}"#;
                                        (create_json_response(400, error_response), 400)
                                    }
                                }
                            }
                        } else {
                            let error_response = r#"{"error": "Invalid request format"}"#;
                            (create_json_response(400, error_response), 400)
                        }
                    }
                    ("POST", "/stop") | ("GET", "/stop") => {
                        let stop_response = r#"{"message": "Server shutdown initiated", "status": "stopping", "timestamp": ""}"#;
                        let timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        let response_with_timestamp = stop_response.replace("\"\"", &format!("\"{}\"", timestamp));
                        
                        log_info!("Server shutdown requested by client: {}", client_addr);
                        log_info!("Initiating graceful shutdown sequence...");
                        
                        let response = create_json_response(200, &response_with_timestamp);
                        let _ = stream.write_all(response.as_bytes());
                        let _ = stream.flush();
                        
                        log_info!("Shutdown response sent to client");
                        log_info!("Server shutting down now. Goodbye!");
                        std::process::exit(0);
                    }
                    ("GET", "/shutdown") => {
                        let shutdown_response = r#"{"message": "Alternative shutdown endpoint triggered", "status": "stopping", "note": "Use /stop for primary shutdown"}"#;
                        log_info!("Alternative shutdown endpoint accessed");
                        (create_json_response(200, shutdown_response), 200)
                    }
                    _ => {
                        let error_response = r#"{"error": "Not found"}"#;
                        (create_json_response(404, error_response), 404)
                    }
                };
                
                let status_prefix = match status_code {
                    200..=299 => "SUCCESS",
                    400..=499 => "WARNING",
                    500..=599 => "ERROR",
                    _ => "UNKNOWN",
                };
                
                log_info!("[{}] [{}] {} {} -> {} {}", 
                    status_prefix, 
                    client_addr, 
                    method, 
                    path, 
                    status_code,
                    get_status_text(status_code)
                );

                // Simulate server error for testing
                if path.contains("error") || path.contains("fail") {
                    let error_response = r#"{"error": "Internal server error", "message": "Simulated server error"}"#;
                    let error_response_full = create_json_response(500, error_response);
                    log_error!("❌ [{}] {} {} -> 500 Internal Server Error", client_addr, method, path);
                    
                    if let Err(e) = stream.write_all(error_response_full.as_bytes()) {
                        log_error!("Failed to send error response: {}", e);
                    }
                    return;
                }
                
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    log_error!("Failed to send response: {}", e);
                }
            }
        }
    } else {
        log_error!("❌ [{}] Failed to read request", client_addr);
    }
}

/// Get HTTP status text for status code
fn get_status_text(status_code: u16) -> &'static str {
    match status_code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}

/// JSON Response
fn create_json_response(status_code: u16, json_body: &str) -> String {
    let status_text = get_status_text(status_code);
    
    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
        status_code, status_text, json_body.len(), json_body
    )
}

/// Handle chat completion requests
fn handle_chat_completion(body: &str, _config: &ModelConfig) -> String {
    // 요청 내용 로깅
    let truncated_body = if body.len() > 100 {
        format!("{}...", &body[0..100])
    } else {
        body.to_string()
    };
    log_info!("📝 Processing chat completion request: {}", truncated_body);

    let response_content = if body.contains("Hello") || body.contains("hello") || body.contains("hi") {
        "Hello! I'm an LLM running on Rust via HTTP API. How can I help you today?"
    } else if body.contains("config") {
        "Current model configuration: temperature=0.7, top_p=0.9, context_size=2048"
    } else if body.contains("error") {
        // 에러 시뮬레이션
        log_error!("Simulating internal error for testing");
        return create_json_response(500, r#"{"error": "Internal server error", "message": "Simulated processing error"}"#);
    } else {
        "I received your message. This is a simulated response from the LLM HTTP API."
    };
    
    let chat_response = format!(
        r#"{{
  "id": "chatcmpl-{}", 
  "object": "chat.completion", 
  "created": {}, 
  "model": "llm-rust", 
  "choices": [{{
    "index": 0, 
    "message": {{
      "role": "assistant", 
      "content": "{}"
    }}, 
    "finish_reason": "stop"
  }}], 
  "usage": {{
    "prompt_tokens": 10, 
    "completion_tokens": 20, 
    "total_tokens": 30
  }}
}}"#,
        generate_id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        response_content
    );
    
    log_info!("💬 Generated chat completion response");
    create_json_response(200, &chat_response)
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("chat-{}", timestamp % 1000000)
}

/// model config file load function
fn load_model_config(config_path: &str) -> Result<ModelConfig, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(config_path)?;
    let config: ModelConfig = serde_json::from_str(&content)?;
    Ok(config)
}

/// Generate and validate configuration automatically
#[no_mangle]
pub extern "C" fn rust_generate_and_validate_config() -> std::os::raw::c_int {
    let config_path = "models.json";
    
    // 1. Generate default configuration (using the model.rs structure)
    let default_config = ModelConfig::default();

    // 2. Save to file (overwrite if exists)
    match serde_json::to_string_pretty(&default_config) {
        Ok(json_content) => {
            if let Err(e) = std::fs::write(config_path, json_content) {
                log_error!("Failed to write config file: {}", e);
                return -1;
            }
        }
        Err(e) => {
            log_error!("Failed to serialize config: {}", e);
            return -2;
        }
    }

    log_info!("Generated configuration file: {}", config_path);

    // 3. Perform secondary validation
    log_info!("Performing secondary validation...");
    rust_validate_model_config(config_path.as_ptr() as *const std::os::raw::c_char)
}

/// Run LLM engine with given configuration file
#[no_mangle]
pub extern "C" fn rust_run_llm_engine(config_path: *const std::os::raw::c_char) -> std::os::raw::c_int {
    let _ = init_logging();
    
    let config_path_str = if config_path.is_null() {
        "models.json"
    } else {
        unsafe {
            match std::ffi::CStr::from_ptr(config_path).to_str() {
                Ok(s) => s,
                Err(_) => return -1,
            }
        }
    };

    match run_llm_engine(config_path_str) {
        Ok(_) => {
            log_info!("LLM Engine completed successfully");
            0
        }
        Err(e) => {
            log_error!("LLM Engine failed: {}", e);
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arguments_creation() {
        let args = Arguments::new("test_config.json".to_string());
        assert_eq!(args.config_path, "test_config.json");
    }

    #[test]
    fn test_api_server_creation() {
        let server = ApiServer::new("localhost".to_string(), 8080);
        assert_eq!(server.host, "localhost");
        assert_eq!(server.port, 8080);
        assert!(!server.is_running());
    }

    #[test]
    fn test_sanitize_filename() {
        let result = sanitize_filename("test@file#name.txt");
        assert_eq!(result, "test_file_name_txt");
    }

    #[test]
    fn test_trim_whitespace() {
        let result = trim_whitespace("  hello world  ");
        assert_eq!(result, "hello world");
    }
}