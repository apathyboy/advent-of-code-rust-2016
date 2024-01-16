pub mod template;

// Use this file to add helper functions and additional modules.

pub fn make_secret(input: &str, suffix: u32) -> String {
    format!("{}{}", input.trim(), suffix)
}

pub fn make_hash(input: &str) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}
