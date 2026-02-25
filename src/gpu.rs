// SPDX-License-Identifier: MIT OR Apache-2.0

//! Native GPU Compute Offloading for `dtt`.
//! 
//! Exposes a brutalist `wgpu` translation layer specifically optimized to stream 
//! massive arrays of ISO-8601 strings into WebGPU/CUDA/Metal shaders for sub-nanosecond 
//! parallel execution.

#![cfg(feature = "gpu")]

use crate::DateTime;
use std::mem::size_of;
use wgpu::util::DeviceExt;

/// Interface representation mirroring the WGSL output `TimeRecord`
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TimeRecord {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    microsecond: u32,
    offset_seconds: i32,
}

/// The context environment for offloading datetime processing to the GPU.
#[derive(Debug)]
pub struct DttCompute {
    device: wgpu::Device,
    queue: wgpu::Queue,
    compute_pipeline: wgpu::ComputePipeline,
}

impl DttCompute {
    /// Initializes the WGPU context asynchronously.
    /// Maps directly to the optimal system backend (Vulkan/Metal/DX12).
    pub async fn new() -> Option<Self> {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .ok()?;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Dtt Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("dtt_parse.wgsl").into()),
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Dtt Compute Pipeline"),
            layout: None,
            module: &shader,
            entry_point: Some("parse_batch"),
            compilation_options: Default::default(),
            cache: None,
        });

        Some(Self {
            device,
            queue,
            compute_pipeline,
        })
    }

    /// Converts a contiguous array of byte-strings into an array of `DateTime`
    /// objects utilizing massive hardware parallelization.
    ///
    /// The input must be a multiple of 24 bytes (representing fixed strings).
    pub async fn parse_matrix(&self, raw_buffer: &[u32]) -> Vec<DateTime> {
        let count = (raw_buffer.len() / 6) as u32; // Assuming 6 words per string as per shader
        if count == 0 {
            return vec![];
        }

        let output_size = (count as usize * size_of::<TimeRecord>()) as wgpu::BufferAddress;

        // Create GPU buffers
        let input_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Input Buffer"),
            contents: bytemuck::cast_slice(raw_buffer),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: output_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Create bind group
        let bind_group_layout = self.compute_pipeline.get_bind_group_layout(0);
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Dtt Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: input_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output_buffer.as_entire_binding(),
                },
            ],
        });

        // Dispatch
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            let workgroups = (count + 63) / 64; // Divide by workgroup size
            cpass.dispatch_workgroups(workgroups, 1, 1);
        }

        // Setup Readback
        let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: output_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, output_size);
        let _ = self.queue.submit(Some(encoder.finish()));

        // Await GPU sync
        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        
        // Wait for device to finish
        let _ = self.device.poll(wgpu::Maintain::Wait);
        receiver.recv().unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        let records: &[TimeRecord] = bytemuck::cast_slice(&data);

        // Map back to standard DateTime elements
        let mut results = Vec::with_capacity(count as usize);
        for record in records {
            if let Ok(dt) = DateTime::from_components(
                record.year,
                record.month as u8,
                record.day as u8,
                record.hour as u8,
                record.minute as u8,
                record.second as u8,
                time::UtcOffset::UTC,
            ) {
                results.push(dt);
            }
        }

        drop(data);
        staging_buffer.unmap();
        results
    }
}
