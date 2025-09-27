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

#[cfg(target_os = "linux")]
pub fn cpu_info_platform() -> CpuInfo {
    use std::fs;
    
    let logical = std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(0);
    
    // Linux에서 /proc/cpuinfo를 읽어서 실제 CPU 정보 가져오기
    let (cores, brand_str, freq_mhz) = read_linux_cpu_info();
    
    let mut info = CpuInfo {
        cores: cores.unwrap_or(logical), // 물리 코어 수, 실패시 논리 코어 수 사용
        logical,
        freq_mhz: freq_mhz.unwrap_or(0),
        brand: [0; 128],
    };
    
    write_brand(&mut info.brand, brand_str.as_bytes());
    info
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn cpu_info_platform() -> CpuInfo {
    let logical = std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(0);
    let cores = logical;
    let freq_mhz = 0;
    let brand_str = "Unknown CPU (portable fallback)";

    let mut info = CpuInfo {
        cores,
        logical,
        freq_mhz,
        brand: [0; 128],
    };
    write_brand(&mut info.brand, brand_str.as_bytes());
    info
}

#[cfg(target_os = "linux")]
fn read_linux_cpu_info() -> (Option<u32>, String, Option<u64>) {
    use std::fs;
    use std::collections::HashSet;
    
    let mut physical_cores = None;
    let mut brand_name = "Linux CPU".to_string();
    let mut max_freq_mhz = None;
    
    // /proc/cpuinfo 읽기
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        let mut core_ids = HashSet::new();
        let mut current_physical_id = None;
        let mut current_core_id = None;
        
        for line in content.lines() {
            let line = line.trim();
            
            // CPU 모델명 추출
            if line.starts_with("model name") {
                if let Some(colon_pos) = line.find(':') {
                    brand_name = line[colon_pos + 1..].trim().to_string();
                }
            }
            // 물리적 프로세서 ID
            else if line.starts_with("physical id") {
                if let Some(colon_pos) = line.find(':') {
                    if let Ok(id) = line[colon_pos + 1..].trim().parse::<u32>() {
                        current_physical_id = Some(id);
                    }
                }
            }
            // 코어 ID
            else if line.starts_with("core id") {
                if let Some(colon_pos) = line.find(':') {
                    if let Ok(id) = line[colon_pos + 1..].trim().parse::<u32>() {
                        current_core_id = Some(id);
                    }
                }
            }
            // CPU 주파수 (MHz)
            else if line.starts_with("cpu MHz") {
                if let Some(colon_pos) = line.find(':') {
                    if let Ok(freq) = line[colon_pos + 1..].trim().parse::<f64>() {
                        let freq_mhz = freq as u64;
                        max_freq_mhz = Some(max_freq_mhz.unwrap_or(0).max(freq_mhz));
                    }
                }
            }
            
            // 빈 줄이면 현재 프로세서 정보 처리
            if line.is_empty() {
                if let (Some(phy_id), Some(core_id)) = (current_physical_id, current_core_id) {
                    core_ids.insert((phy_id, core_id));
                }
                current_physical_id = None;
                current_core_id = None;
            }
        }
        
        // 마지막 프로세서 정보 처리
        if let (Some(phy_id), Some(core_id)) = (current_physical_id, current_core_id) {
            core_ids.insert((phy_id, core_id));
        }
        
        if !core_ids.is_empty() {
            physical_cores = Some(core_ids.len() as u32);
        }
    }
    
    // /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq에서 최대 주파수 확인
    if max_freq_mhz.is_none() {
        if let Ok(freq_str) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq") {
            if let Ok(freq_khz) = freq_str.trim().parse::<u64>() {
                max_freq_mhz = Some(freq_khz / 1000); // kHz를 MHz로 변환
            }
        }
    }
    
    (physical_cores, brand_name, max_freq_mhz)
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