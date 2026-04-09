use std::{env, thread};
use std::time::Duration;

fn main() {
    // 1. Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let program_name = args
        .first()
        .map(|arg| {
            std::path::Path::new(arg)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(arg.as_str())
        })
        .unwrap_or("ResourceHog");

    if args.len() < 3 {
        println!("Usage: {} <memory_mb> <cpu_cores>", program_name);
        println!("Example: {} 512 4", program_name);
        return;
    }

    let target_mb: usize = args[1].parse().expect("memory_mb must be a valid integer");
    let target_cores: usize = args[2].parse().expect("cpu_cores must be a valid integer");

    println!(
        "Allocating {} MB of memory and starting {} CPU worker threads",
        target_mb, target_cores
    );

    // 2. Allocate memory
    // Use a Rust Vec<u8> to reserve and touch the requested memory.
    // 1 MB = 1024 * 1024 bytes
    let mut _memory_buffer: Vec<u8> = Vec::with_capacity(target_mb * 1024 * 1024);

    // Touch each megabyte to ensure the operating system commits the pages.
    for _ in 0..target_mb {
        _memory_buffer.extend_from_slice(&[1u8; 1024 * 1024]);
    }
    println!("Memory allocation complete.");

    // 3. Saturate CPU
    let mut handles = vec![];
    for i in 0..target_cores {
        let handle = thread::spawn(move || {
            println!("Worker {} is now consuming CPU...", i);
            loop {
                // Keep the thread busy with a tight loop.
                let mut _x = 0;
                for _ in 0..1000000 {
                    _x += 1;
                }
            }
        });
        handles.push(handle);
    }

    println!("Resource load is running. Press Ctrl+C to stop.");

    // Keep the main thread alive.
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
