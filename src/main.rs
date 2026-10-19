use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Config {
    server_url: String,
    access_key: String,
    service: String,
    host: String,
    environment: String,
    log_file_path: String,
    flush_interval_secs: u64,
    max_batch_patterns: usize,
    max_retry_batches: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let toml_file_path = "./loggate.toml";

    // 1. Read and parse config
    let content = std::fs::read_to_string(toml_file_path)?;
    let config: Config = toml::from_str(&content)?;

    println!("Extracted log file path: {}", config.log_file_path);

    // 2. Open file
    let mut file = loop {
        match File::open(&config.log_file_path) {
            Ok(f) => break f,
            Err(_) => {
                println!("Waiting for {} to be created...", config.log_file_path);
                std::thread::sleep(Duration::from_secs(2));
            }
        }
    };

    // 3. Move pointer to end of file
    let end_pos = file.seek(SeekFrom::End(0))?;
    println!(
        "File opened. Pointer set to end of file at byte: {}",
        end_pos
    );

    // 4. Wrap file in BufReader to read line-by-line
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    println!("Listening for newly added log lines...");

    loop {
        // read_line reads until '\n' or EOF
        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read > 0 {
            if line.ends_with('\n') {
                print!("[NEW] {}", line);
                line.clear(); // Always clear the buffer for the next line
            }
        } else {
            // No new data right now, pause briefly before checking again
            std::thread::sleep(Duration::from_millis(10));

            // If log has been rotated reset the pointer
            let current_pos = file.stream_position()?;
            let file_len = reader.get_ref().metadata()?.len();
            if current_pos < file_len {
                reader.seek(SeekFrom::Start(0))?;
                println!(
                    "Log file rotated/cleared (was at byte {}, file is now {} bytes). Reset pointer to 0.",
                    current_pos, file_len
                );
            }
        }
    }
}
