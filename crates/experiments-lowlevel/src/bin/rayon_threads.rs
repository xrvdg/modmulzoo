use num_cpus;
use rayon::prelude::*;
use std::thread;
use std::sync::Mutex;
use std::collections::HashSet;
#[macro_use]
extern crate lazy_static;

// Create a global structure to track which cores have been initialized
lazy_static! {
    // Track cores by core ID when available (primarily for Linux)
    static ref INITIALIZED_CORES: Mutex<HashSet<usize>> = Mutex::new(HashSet::new());
    // Track threads by thread ID when core ID is not available (for macOS/Windows)
    static ref INITIALIZED_THREADS: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
}

// Get the actual CPU core for Linux or None for macOS
#[cfg(target_os = "linux")]
fn get_cpu_core() -> Option<usize> {
    // Use sched_getcpu() from libc
    unsafe {
        let cpu = libc::sched_getcpu();
        if cpu >= 0 {
            Some(cpu as usize)
        } else {
            None
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(never)]
/// Set the floating point rounding mode to round to zero
///
/// inline(never) to prevent to compiler from reordering
pub fn set_round_to_zero() -> u64 {
    let fpcr: u64;
    unsafe {
        // Set RMode (bits 22-23) to 0b11 for round toward zero
        core::arch::asm!(
            "mrs {fpcr}, fpcr",             // Read current FPCR
            "orr {tmp}, {fpcr}, #0b11<<22", // Set RMode bits to 11 using bit shift notation
            "msr fpcr, {tmp}",             // Write back to FPCR
            tmp = out(reg) _,
            fpcr = out(reg) fpcr,
        );
    }

    // Defense-in-depth but can't be relied on
    // From the documentation:
    // Programs cannot rely on black_box for correctness, beyond it behaving as the identity function. As such, it must not be relied upon to control critical program behavior.
    std::hint::black_box(fpcr)
}

#[cfg(not(target_arch = "aarch64"))]
pub fn set_round_to_zero() -> u64 {
    // No-op or panic depending on your needs for non-ARM platforms
    unimplemented!("Round to zero is only implemented for ARM64");
}

#[cfg(not(target_os = "linux"))]
fn get_cpu_core() -> Option<usize> {
    // Return None for non-Linux platforms (including macOS)
    None
}

// Function to read the floating point rounding mode from FPCR
#[cfg(target_arch = "aarch64")]
fn get_fpcr_rounding_mode() -> u64 {
    let fpcr: u64;
    unsafe {
        core::arch::asm!(
            "mrs {tmp}, fpcr",
            tmp = out(reg) fpcr
        );
    }
    // Extract RMode bits (bits 22-23)
    (fpcr >> 22) & 0b11
}

#[cfg(not(target_arch = "aarch64"))]
fn get_fpcr_rounding_mode() -> u64 {
    // Return a placeholder value for non-ARM platforms
    todo!("Implement reading rounding mode for non-ARM platforms")
}

// Function to interpret the rounding mode value
fn rounding_mode_to_string(mode: u64) -> &'static str {
    match mode {
        0b00 => "Round to Nearest, ties to Even",
        0b01 => "Round towards Plus Infinity",
        0b10 => "Round towards Minus Infinity",
        0b11 => "Round towards Zero",
        _ => unreachable!("Invalid rounding mode"),
    }
}

// Function to get a numeric representation of the current thread ID
fn get_thread_id_numeric() -> u64 {
    // Get thread ID and convert it to a number for storage in a HashSet
    // This is a hack but works for our demonstration purposes
    let thread_id = format!("{:?}", thread::current().id());
    let clean_id = thread_id.replace("ThreadId(", "").replace(")", "");
    clean_id.parse::<u64>().unwrap_or(0)
}

// Function to set rounding mode to zero if not already set for the current core/thread
fn ensure_round_to_zero_on_core() -> bool {
    if let Some(core_id) = get_cpu_core() {
        // We have a core ID (likely on Linux), use it for tracking
        let mut initialized_cores = INITIALIZED_CORES.lock().unwrap();
        if !initialized_cores.contains(&core_id) {
            // First time running on this core, set rounding mode
            set_round_to_zero();
            initialized_cores.insert(core_id);
            return true;
        }
    } else {
        // We can't determine the core ID (likely on macOS), use thread ID instead
        let thread_id = get_thread_id_numeric();
        let mut initialized_threads = INITIALIZED_THREADS.lock().unwrap();
        if !initialized_threads.contains(&thread_id) {
            // First time running on this thread, set rounding mode
            set_round_to_zero();
            initialized_threads.insert(thread_id);
            return true;
        }
    }
    false
}

fn main() {
    // Get current thread pool configuration
    println!(
        "Rayon thread pool is using {} threads with {} CPU cores available",
        rayon::current_num_threads(),
        num_cpus::get()
    );
    
    // Set floating point rounding mode to zero in the main thread
    ensure_round_to_zero_on_core();
    
    // Print main thread's rounding mode
    let main_thread_mode = get_fpcr_rounding_mode();
    println!(
        "Main thread set floating point rounding mode: {} ({})",
        main_thread_mode,
        rounding_mode_to_string(main_thread_mode)
    );

    // Print main thread ID and core ID
    let main_thread_id = thread::current().id();
    let main_thread_id_numeric = get_thread_id_numeric();
    println!("Main thread ID: {:?} (numeric: {})", main_thread_id, main_thread_id_numeric);
    
    // Initialize tracking for main thread
    {
        let mut initialized_threads = INITIALIZED_THREADS.lock().unwrap();
        initialized_threads.insert(main_thread_id_numeric);
    }
    
    // Print core ID only on Linux
    #[cfg(target_os = "linux")]
    {
        if let Some(core_id) = get_cpu_core() {
            println!("Main thread running on CPU core: {}", core_id);
            // Initialize first core in our tracking structure
            let mut initialized_cores = INITIALIZED_CORES.lock().unwrap();
            initialized_cores.insert(core_id);
        } else {
            println!("Main thread CPU core unknown");
        }
    }

    // Create a vector of tasks to process
    let tasks: Vec<usize> = (0..20).collect();

    // Process tasks in parallel
    tasks.par_iter().for_each(|&task_id| {
        // Ensure rounding mode is set to zero on this core/thread (only if needed)
        let newly_initialized = ensure_round_to_zero_on_core();
        
        // Get the current thread ID
        let thread_id = thread::current().id();
        let thread_id_numeric = get_thread_id_numeric();
        
        // Read the floating point rounding mode for this thread
        let fp_mode = get_fpcr_rounding_mode();

        // Simulate some work
        let result = perform_task(task_id);

        // Format output based on platform
        #[cfg(target_os = "linux")]
        {
            if let Some(core_id) = get_cpu_core() {
                println!(
                    "Task {} processed by thread {:?} (#{}) on CPU core {}, using fp rounding mode: {} ({}), {}result: {}",
                    task_id, thread_id, thread_id_numeric, core_id, fp_mode, rounding_mode_to_string(fp_mode), 
                    if newly_initialized { "NEWLY INITIALIZED, " } else { "" },
                    result
                );
            } else {
                println!(
                    "Task {} processed by thread {:?} (#{}) (core unknown), using fp rounding mode: {} ({}), {}result: {}",
                    task_id, thread_id, thread_id_numeric, fp_mode, rounding_mode_to_string(fp_mode),
                    if newly_initialized { "NEWLY INITIALIZED, " } else { "" },
                    result
                );
            }
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            println!(
                "Task {} processed by thread {:?} (#{}) using fp rounding mode: {} ({}), {}result: {}",
                task_id, thread_id, thread_id_numeric, fp_mode, rounding_mode_to_string(fp_mode),
                if newly_initialized { "NEWLY INITIALIZED, " } else { "" },
                result
            );
        }
    });
}

// Function to simulate work being done in each task
fn perform_task(id: usize) -> usize {
    // Simulate CPU-intensive work
    let mut sum: usize = 0;
    for i in 0..(id * 100_000_000 + 1) {
        sum = sum.wrapping_add(i);
    }
    sum
}
