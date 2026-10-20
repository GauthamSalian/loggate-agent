use crate::processor;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReducedRecord {
    pub pattern: String,
    pub sample_lines: Vec<String>, // Holds up to 3 samples
    pub raw_line_count: u64,
    pub bytes_represented: u64,
}

pub static LOG_LINES_COUNT: LazyLock<Mutex<u64>> = LazyLock::new(|| Mutex::new(0));
pub static LOG_PATTERN_COUNT: LazyLock<Mutex<u64>> = LazyLock::new(|| Mutex::new(0));

// A global, thread-safe cache that maps a normalized log pattern to its reduced record.
pub static PATTERN_CACHE: LazyLock<Mutex<HashMap<String, ReducedRecord>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn process_and_batch(log_line: &str) {
    // 1. Mask the log line using the existing processor
    let masked_line = processor::masking(log_line);

    // 2. Lock the cache (exclusive mutable access)
    let mut cache = PATTERN_CACHE.lock().unwrap();

    // 3. Fetch existing entry or initialize a new one with the entry API
    let record = cache.entry(masked_line.clone()).or_insert_with(|| {
        let mut pattern_count = LOG_PATTERN_COUNT.lock().unwrap();
        *pattern_count += 1;

        ReducedRecord {
            pattern: masked_line,
            sample_lines: Vec::new(),
            raw_line_count: 0,
            bytes_represented: 0,
        }
    });

    // 4. Update count and bytes
    let mut line_count = LOG_LINES_COUNT.lock().unwrap();
    *line_count += 1;
    record.raw_line_count += 1;
    record.bytes_represented += log_line.len() as u64;

    // 5. Retain up to 3 sample lines
    if record.sample_lines.len() < 3 {
        record.sample_lines.push(log_line.to_string());
    }
}
