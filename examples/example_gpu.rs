// SPDX-License-Identifier: MIT OR Apache-2.0

//! Example: Native GPU Compute Offloading
//!
//! This example demonstrates how to process millions of ISO-8601 strings
//! concurrently using WebGPU/Metal compute shaders.

#[cfg(feature = "gpu")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        use dtt::gpu::DttCompute;
        use std::time::Instant;

    println!("Initializing DTT GPU Compute Engine...");
    
    // Fallback if no GPU adapter is found on this machine
    let compute = match DttCompute::new().await {
        Some(c) => c,
        None => {
            println!("No suitable GPU adapter found on this system. Exiting gracefully.");
            return Ok(());
        }
    };

    println!("GPU Engine Loaded. Generating payload matrix...");

    // Generate 1 million ISO-8601 datetime strings
    // The shader expects 24-byte padded chunks per string: "YYYY-MM-DDTHH:MM:SSZ    "
    let base_string = b"2026-10-15T14:30:00Z    ";
    let count = 1_000_000;
    
    // We convert the bytes to u32 words as expected by the dtt_parse.wgsl shader.
    // 24 bytes = 6 u32 words.
    let mut raw_buffer: Vec<u32> = Vec::with_capacity(count * 6);
    
    let w0 = u32::from_le_bytes(base_string[0..4].try_into().unwrap());
    let w1 = u32::from_le_bytes(base_string[4..8].try_into().unwrap());
    let w2 = u32::from_le_bytes(base_string[8..12].try_into().unwrap());
    let w3 = u32::from_le_bytes(base_string[12..16].try_into().unwrap());
    let w4 = u32::from_le_bytes(base_string[16..20].try_into().unwrap());
    let w5 = u32::from_le_bytes(base_string[20..24].try_into().unwrap());

    for _ in 0..count {
        raw_buffer.push(w0);
        raw_buffer.push(w1);
        raw_buffer.push(w2);
        raw_buffer.push(w3);
        raw_buffer.push(w4);
        raw_buffer.push(w5);
    }

    println!("Payload Size: {} bytes ({} records)", raw_buffer.len() * 4, count);
    
    println!("Dispatching to Compute Shader...");
    let start = Instant::now();
    
    // Execute massive parallel parse
    let datetimes = compute.parse_matrix(&raw_buffer).await;
    
    let duration = start.elapsed();
    
    println!("Shader Execution Complete!");
    println!("Parsed {} records in {:?}", datetimes.len(), duration);
    
    if let Some(dt) = datetimes.first() {
        println!("Sample output: {:?}", dt);
    }

    Ok(())
    })
}

#[cfg(not(feature = "gpu"))]
fn main() {
    println!("Please run this example with the `gpu` feature enabled:");
    println!("cargo run --example example_gpu --features gpu");
}
