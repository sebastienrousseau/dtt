// 2026 wgpu Compute Shader for `dtt` string mapping

struct TimeRecord {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    microsecond: u32,
    offset_seconds: i32,
};

// Input buffer: A contiguous array of u32s representing characters
// Because WGSL byte access is tricky, we treat 4 ASCII chars as 1 u32.
@group(0) @binding(0) var<storage, read> input_data: array<u32>;
// Output buffer
@group(0) @binding(1) var<storage, read_write> output_times: array<TimeRecord>;

fn extract_byte(word: u32, offset: u32) -> u32 {
    let shift = offset * 8u;
    return (word >> shift) & 0xFFu;
}

// Map ASCII char to integer, assume perfect input for this HFT prototype.
fn to_digit(ascii: u32) -> u32 {
    return ascii - 0x30u;
}

@compute @workgroup_size(64)
fn parse_batch(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    
    // Each date string is 24 bytes (e.g. "2023-05-20T15:30:00Z    ")
    // For simplicity of prototype, assume 6 u32 words per string.
    let base_addr = idx * 6u;
    
    // Word 0: '2' '0' '2' '3'
    let w0 = input_data[base_addr];
    let y1 = to_digit(extract_byte(w0, 0u));
    let y2 = to_digit(extract_byte(w0, 1u));
    let y3 = to_digit(extract_byte(w0, 2u));
    let y4 = to_digit(extract_byte(w0, 3u));
    let year = (y1 * 1000u) + (y2 * 100u) + (y3 * 10u) + y4;
    
    // Word 1: '-' '0' '5' '-'
    let w1 = input_data[base_addr + 1u];
    let m1 = to_digit(extract_byte(w1, 1u));
    let m2 = to_digit(extract_byte(w1, 2u));
    let month = (m1 * 10u) + m2;
    
    // Word 2: '2' '0' 'T' '1'
    let w2 = input_data[base_addr + 2u];
    let d1 = to_digit(extract_byte(w2, 0u));
    let d2 = to_digit(extract_byte(w2, 1u));
    let day = (d1 * 10u) + d2;
    
    let h1 = to_digit(extract_byte(w2, 3u));
    
    // Word 3: '5' ':' '3' '0'
    let w3 = input_data[base_addr + 3u];
    let h2 = to_digit(extract_byte(w3, 0u));
    let hour = (h1 * 10u) + h2;
    
    let min1 = to_digit(extract_byte(w3, 2u));
    let min2 = to_digit(extract_byte(w3, 3u));
    let minute = (min1 * 10u) + min2;
    
    // Word 4: ':' '0' '0' 'Z'
    let w4 = input_data[base_addr + 4u];
    let s1 = to_digit(extract_byte(w4, 1u));
    let s2 = to_digit(extract_byte(w4, 2u));
    let second = (s1 * 10u) + s2;
    
    var record: TimeRecord;
    record.year = i32(year);
    record.month = month;
    record.day = day;
    record.hour = hour;
    record.minute = minute;
    record.second = second;
    record.microsecond = 0u;
    record.offset_seconds = 0; // mapped Z
    
    output_times[idx] = record;
}
