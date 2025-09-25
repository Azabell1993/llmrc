use std::cmp::min;
use std::sync::atomic::{AtomicI32, Ordering};

#[no_mangle]
pub extern "C" fn rust_llm() {
    static GLOBAL_VAR: AtomicI32 = AtomicI32::new(0);

    let old = GLOBAL_VAR.load(Ordering::SeqCst);

    if old == 0 {
        GLOBAL_VAR.store(1, Ordering::SeqCst);
        eprintln!("[INFO] GLOBAL_VAR set to 1");
        eprintln!("[INFO] Hello from Rust LLM!");
    } else {
        GLOBAL_VAR.store(0, Ordering::SeqCst);
        eprintln!("[INFO] GLOBAL_VAR is already set to 1, resetting to 0");
    }

    eprintln!("[INFO] GLOBAL_VAR current value: {}", GLOBAL_VAR.load(Ordering::SeqCst));
}

#[no_mangle]
pub extern "C" fn rust_func() {
    eprintln!("[INFO] Hello from Rust!");
}

#[inline] pub fn checked_add_i64(a: i64, b: i64) -> Option<i64> { a.checked_add(b) }
#[inline] pub fn checked_sub_i64(a: i64, b: i64) -> Option<i64> { a.checked_sub(b) }
#[inline] pub fn checked_mul_i64(a: i64, b: i64) -> Option<i64> { a.checked_mul(b) }
#[inline] pub fn checked_div_i64(a: i64, b: i64) -> Option<i64> {
    if b == 0 { None } else { a.checked_div(b) }
}

#[repr(C)]
pub struct CpuInfo {
    pub cores: u32,
    pub logical: u32,
    pub freq_mhz: u64,
    pub brand: [u8; 128],
}

#[cfg(target_os = "macos")]
pub fn cpu_info_platform() -> CpuInfo {
    let logical = std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(0);
    let cores = logical;    // macOS에서는 논리 코어와 물리 코어를 같다고 가정
    let freq_mhz = 0;       // [TO-DO] 주파수는 나중에 sysctl로 구현
    let brand_str = "macOS CPU";

    let mut info = CpuInfo {
        cores,
        logical,
        freq_mhz,
        brand: [0; 128],
    };
    write_brand(&mut info.brand, brand_str.as_bytes());
    info
}

#[cfg(not(target_os = "macos"))]
pub fn cpu_info_platform() -> CpuInfo {
    let logical = std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(0);
    let cores = logical;
    let freq_mhz = 0;
    let brand_str = "Unknown CPU(portable fallback)";

    let mut info = CpuInfo {
        cores,
        logical,
        freq_mhz,
        brand: [0; 128],
    };
    write_brand(&mut info.brand, brand_str.as_bytes());
    info
}

fn write_brand(dest: &mut [u8; 128], src: &[u8]) {
    let max_copy = dest.len().saturating_sub(1);
    let n = min(max_copy, src.len());
    dest[..n].copy_from_slice(&src[..n]);
    dest[n] = 0; // null-terminate
}

#[no_mangle]
pub extern "C" fn rust_get_cpu_info(out: *mut CpuInfo) -> bool {
    if out.is_null() {
        return false;
    }
    let info = cpu_info_platform();

    let _ = checked_add_i64(info.cores as i64, info.logical as i64)
        .map(|sum| { let _ = sum; })
        .or_else(|| { eprintln!("[ERROR] [rust_get_cpu_info] overflow in cores+logical"); None });

    unsafe {
        std::ptr::write(out, info);
    }
    true
}

#[no_mangle]
pub extern "C" fn rust_get_cpu_brand(buf: *mut u8, buf_len: usize) -> usize {
    if buf.is_null() || buf_len == 0 {
        return 0;
    }
    let info = cpu_info_platform();
    let nul_pos = info.brand.iter().position(|&c| c == 0).unwrap_or(info.brand.len());
    let brand_bytes = &info.brand[..nul_pos];

    let max_copy = buf_len.saturating_sub(1);
    let to_copy = min(max_copy, brand_bytes.len());

    unsafe {
        std::ptr::copy_nonoverlapping(brand_bytes.as_ptr(), buf, to_copy);
        *buf.add(to_copy) = 0; // null-terminate
    }
    to_copy
}