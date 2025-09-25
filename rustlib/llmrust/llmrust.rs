#[no_mangle]
pub extern "C" fn llmrust_hello() {
    eprintln!("[INFO] Hello from Rust LLM!");
}
