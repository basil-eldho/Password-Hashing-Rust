use tracing::error;
pub mod args;

pub fn max_threads(s: &str) -> Result<usize, String> {
    let threads = s.parse_value("Failed to parse the thread count as a number")?;

    let max_available = std::thread::available_parallelism()
        .map_err(|_| "Failed to determine the number of available CPU cores".to_string())?
        .get();

    // Validate threads
    if threads == 0 {
        error!("Error: Number of threads must be greater than zero.");
        return Err("Error: Number of threads must be greater than zero.".to_string());
    }

    if threads > max_available {
        return Err(format!(
            "Requested thread count ({}) exceeds the available CPU threads ({})",
            threads, max_available
        ));
    }
    Ok(threads)
}

pub fn max_num(s: &str) -> Result<usize, String> {
    let num = s.parse_value("Failed to parse the given number as a number")?;
    Ok(num)
}