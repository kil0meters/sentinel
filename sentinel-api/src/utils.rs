use std::fs;
use std::cmp;
use std::path::Path;
use std::os::unix::fs::MetadataExt;
use std::env;

pub fn pretty_bytes(num: f64) -> String {
    let negative = if num.is_sign_positive() { "" } else { "-" };
    let num = num.abs(); //sets num to absolute value
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB"]; // probably enough for now
    if num < 1_f64 {
        return format!("{}{} {}", negative, num, "B");
    }
    let delimiter = 1000_f64;
    let exponent = cmp::min(
        (num.ln() / delimiter.ln()).floor() as i32,
        (units.len() - 1) as i32,
    );
    let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent))
        .parse::<f64>()
        .unwrap() * 1_f64;
    let unit = units[exponent as usize];
    format!("{}{}{}", negative, pretty_bytes, unit)
}

// Basically a clone of the thing above.
// I should probably try to minimize redundancy.
pub fn pretty_number(num: f64) -> String {
    let units = ["", "K", "M", "B", "T", "Q"]; // probably enough for now
    let delimiter = 1000_f64;
    let exponent = cmp::min(
        (num.ln() / delimiter.ln()).floor() as i32,
        (units.len() - 1) as i32,
    );
    let pretty_number = format!("{:.0}", num / delimiter.powi(exponent))
        .parse::<f64>()
        .unwrap() * 1_f64;
    let unit = units[exponent as usize];
    format!("{}{}", pretty_number, unit)
}

pub fn get_config_dir() -> String {
    format!("{}/.config/sentinel", env::var("HOME").unwrap())
}

pub fn dir_size_recursive(path: &Path) -> u64 {
    let mut size: u64 = 0;
    match fs::read_dir(&path) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                let meta = entry.metadata().unwrap();

                if meta.is_file() {
                    size += meta.size();
                } else {
                    size += 4096;
                    size += dir_size_recursive(&path);
                }
            }
        }
        Err(e) => eprintln!("Error reading {:?}, caused by I/O error: {}", path, e),
    }
    size
}
