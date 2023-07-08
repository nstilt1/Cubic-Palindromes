use std::fs::File;
use std::io::Write;

use crate::RADIX;

/**
 * Helper function for writing a file
 */
pub fn write_file(min: u32, max: u32, content: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("./results/base{}_threaded_big_ranged_{}_{}.txt", RADIX, &min.to_string(), &max.to_string()))?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

/**
 * Just a helper function to write a file when exclusively checking palindromes.
 */
pub fn write_file_exclusive(min: u32, max: u32, content: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("./results/exclusive_check_{}_{}.txt", &min.to_string(), &max.to_string()))?;
    file.write(content.as_bytes())?;
    file.sync_all()?;
    Ok(())
}