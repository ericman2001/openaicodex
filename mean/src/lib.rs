//! Library helpers for computing the mean of numbers from various sources.
//!
//! The core logic is exposed via [`mean_from_reader`], which operates on any
//! type implementing [`std::io::BufRead`].

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Compute the mean of numbers coming from any `BufRead` source.
///
/// Returns an error if no numbers are found or if any line cannot be
/// parsed as a floating point number.
pub fn mean_from_reader<R: BufRead>(reader: R) -> Result<f64, String> {
    let mut sum = 0.0_f64;
    let mut count = 0_u64;

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let value: f64 = trimmed
            .parse()
            .map_err(|e: std::num::ParseFloatError| e.to_string())?;
        sum += value;
        count += 1;
    }

    if count == 0 {
        Err("No numbers found".into())
    } else {
        Ok(sum / count as f64)
    }
}

/// Convenience function to compute the mean from a file on disk.
///
/// The provided path is opened and then passed to `mean_from_reader`.
pub fn mean_from_file<P: AsRef<Path>>(path: P) -> Result<f64, String> {
    let file = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);
    mean_from_reader(reader)
}
