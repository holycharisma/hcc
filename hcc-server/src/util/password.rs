extern crate bcrypt;

use std::str;

use bcrypt::{hash, verify, DEFAULT_COST};

pub struct PasswordUtil {}

impl PasswordUtil {
    pub fn into_password_hash(_plaintext: &str) -> Vec<u8> {
        match hash(_plaintext, DEFAULT_COST) {
            Ok(hash) => String::into_bytes(hash),
            _ => vec![]
        }
    }

    pub fn verify_hashed_bytes(attempt: &str, hash: &[u8]) -> bool {
        match verify(attempt, str::from_utf8(hash).unwrap()) {
            Ok(b) => b,
            _ => false
        }
    }
}
