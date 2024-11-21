use candid::CandidType;
use serde::{Deserialize, Serialize};

pub type StorageKey = [u8; 32];  
pub type FixedString = [u8; 32];  
pub type FixedContent = [u8; 1024];  

#[derive(Clone, Debug)]
pub struct Convertible<T: Clone> {
    pub data: T,
}

pub fn string_to_storage_key(s: &str) -> StorageKey {
    let mut key = [0u8; 32];
    let bytes = s.as_bytes();
    let len = bytes.len().min(32);
    key[..len].copy_from_slice(&bytes[..len]);
    key
}

pub fn storage_key_to_string(key: &StorageKey) -> String {
    String::from_utf8(
        key.iter()
            .take_while(|&&x| x != 0)
            .copied()
            .collect()
    ).unwrap_or_default()
}

pub fn string_to_fixed(s: &str) -> FixedString {
    let mut fixed = [0u8; 32];
    let bytes = s.as_bytes();
    let len = bytes.len().min(32);
    fixed[..len].copy_from_slice(&bytes[..len]);
    fixed
}

pub fn fixed_to_string(fixed: &FixedString) -> String {
    String::from_utf8(
        fixed.iter()
            .take_while(|&&x| x != 0)
            .copied()
            .collect()
    ).unwrap_or_default()
}

pub fn string_to_content(s: &str) -> FixedContent {
    let mut content = [0u8; 1024];
    let bytes = s.as_bytes();
    let len = bytes.len().min(1024);
    content[..len].copy_from_slice(&bytes[..len]);
    content
}

pub fn content_to_string(content: &FixedContent) -> String {
    String::from_utf8(
        content.iter()
            .take_while(|&&x| x != 0)
            .copied()
            .collect()
    ).unwrap_or_default()
}