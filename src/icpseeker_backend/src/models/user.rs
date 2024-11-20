use candid::{CandidType, Decode, Encode};
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use std::borrow::Cow;
use ic_stable_structures::{Storable, BoundedStorable};

pub type FixedString = [u8; 32];

fn string_to_fixed(s: &str) -> FixedString {
    let mut fixed = [0u8; 32];
    let bytes = s.as_bytes();
    let len = bytes.len().min(32);
    fixed[..len].copy_from_slice(&bytes[..len]);
    fixed
}

fn fixed_to_string(fixed: &FixedString) -> String {
    String::from_utf8(
        fixed.iter()
            .take_while(|&&x| x != 0)
            .copied()
            .collect()
    ).unwrap_or_default()
}

#[derive(Clone, Debug)]
pub struct StableUserProfile {
    pub id: FixedString,
    pub name: FixedString,
    pub email: FixedString,
    pub phone_number: FixedString,
    pub city: FixedString,
    pub country: FixedString,
    pub education_id: Option<FixedString>,
    pub bank_info_id: Option<FixedString>,
    pub status: u8,
    pub profile_completion: u8,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Storable for StableUserProfile {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id);
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&self.email);
        bytes.extend_from_slice(&self.phone_number);
        bytes.extend_from_slice(&self.city);
        bytes.extend_from_slice(&self.country);
        
        if let Some(edu_id) = &self.education_id {
            bytes.push(1);
            bytes.extend_from_slice(edu_id);
        } else {
            bytes.push(0);
            bytes.extend_from_slice(&[0u8; 32]);
        }
        
        if let Some(bank_id) = &self.bank_info_id {
            bytes.push(1);
            bytes.extend_from_slice(bank_id);
        } else {
            bytes.push(0);
            bytes.extend_from_slice(&[0u8; 32]);
        }

        bytes.push(self.status);
        bytes.push(self.profile_completion);
        bytes.extend_from_slice(&self.created_at.to_be_bytes());
        bytes.extend_from_slice(&self.updated_at.to_be_bytes());
        
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let mut pos = 0;
        let mut next_fixed_str = || {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&bytes[pos..pos + 32]);
            pos += 32;
            arr
        };

        let id = next_fixed_str();
        let name = next_fixed_str();
        let email = next_fixed_str();
        let phone_number = next_fixed_str();
        let city = next_fixed_str();
        let country = next_fixed_str();

        let education_id = if bytes[pos] == 1 {
            pos += 1;
            Some(next_fixed_str())
        } else {
            pos += 33; 
            None
        };

        let bank_info_id = if bytes[pos] == 1 {
            pos += 1;
            Some(next_fixed_str())
        } else {
            pos += 33; 
            None
        };

        let status = bytes[pos];
        pos += 1;
        let profile_completion = bytes[pos];
        pos += 1;

        let created_at = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let updated_at = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());

        Self {
            id,
            name,
            email,
            phone_number,
            city,
            country,
            education_id,
            bank_info_id,
            status,
            profile_completion,
            created_at,
            updated_at,
        }
    }
}

impl BoundedStorable for StableUserProfile {
    const MAX_SIZE: u32 = (32 * 8) + 2 + 16; 
    const IS_FIXED_SIZE: bool = true;
}