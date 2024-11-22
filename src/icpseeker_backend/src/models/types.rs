use candid::{CandidType, Decode, Encode};
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use std::borrow::Cow;
use ic_stable_structures::{Storable, BoundedStorable};
use crate::models::StableUserAPIUsage;
use crate::models::UserAPIUsage;

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

impl Storable for StableUserAPIUsage {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.user_id);
        bytes.extend_from_slice(&self.daily_requests.to_be_bytes());
        bytes.extend_from_slice(&self.last_reset.to_be_bytes());
        bytes.extend_from_slice(&self.total_requests.to_be_bytes());
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let mut pos = 0;
        
        let mut user_id = [0u8; 32];
        user_id.copy_from_slice(&bytes[pos..pos + 32]);
        pos += 32;

        let daily_requests = u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap());
        pos += 4;

        let last_reset = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;

        let total_requests = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());

        Self {
            user_id,
            daily_requests,
            last_reset,
            total_requests,
        }
    }
}

impl BoundedStorable for StableUserAPIUsage {
    const MAX_SIZE: u32 = 32 + 4 + 8 + 8;
    const IS_FIXED_SIZE: bool = true;
}

impl From<UserAPIUsage> for StableUserAPIUsage {
    fn from(usage: UserAPIUsage) -> Self {
        Self {
            user_id: string_to_fixed(&usage.user_id),
            daily_requests: usage.daily_requests,
            last_reset: usage.last_reset,
            total_requests: usage.total_requests,
        }
    }
}

impl From<StableUserAPIUsage> for UserAPIUsage {
    fn from(usage: StableUserAPIUsage) -> Self {
        Self {
            user_id: fixed_to_string(&usage.user_id),
            daily_requests: usage.daily_requests,
            last_reset: usage.last_reset,
            total_requests: usage.total_requests,
        }
    }
}

