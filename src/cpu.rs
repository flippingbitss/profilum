use core::arch::x86_64::_rdtsc;

#[cfg(target_arch = "x86_64")]
pub fn read_tsc() -> u64 {
    unsafe { _rdtsc() }
}

// simulates
pub fn get_cpu_freq(duration: std::time::Duration) -> u64 {
    // freq = ticks per unit time
    // Run at the CPU freq for 1 sec
    let start = std::time::Instant::now();
    let start_ts = read_tsc();
    while start.elapsed() < duration {}
    let end_ts = read_tsc();

    end_ts - start_ts
}
