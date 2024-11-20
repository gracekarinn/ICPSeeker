use candid::{CandidType, Decode, Encode};
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use std::borrow::Cow;
use ic_stable_structures::{Storable, BoundedStorable};

pub type FixedString = [u8; 32];

pub fn string_to_fixed(s: &str) -> FixedString {
    let mut fixed = [0u8; 32];
    let bytes = s.as_bytes();
    let len = bytes.len().min(32);
    fixed[..len].copy_from_slice(&bytes[..len]);
    fixed
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub city: String,
    pub country: String,
    pub education_id: Option<String>,
    pub bank_info_id: Option<String>,
    pub status: u8,
    pub profile_completion: u8,
    pub created_at: u64,
    pub updated_at: u64,
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

pub fn fixed_to_string(fixed: &FixedString) -> String {
    String::from_utf8(
        fixed.iter()
            .take_while(|&&x| x != 0)
            .copied()
            .collect()
    ).unwrap_or_default()
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
        fn read_fixed_string(bytes: &[u8], start: usize) -> FixedString {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&bytes[start..start + 32]);
            arr
        }

        let mut pos = 0;
        
        let id = read_fixed_string(&bytes, pos);
        pos += 32;
        
        let name = read_fixed_string(&bytes, pos);
        pos += 32;
        
        let email = read_fixed_string(&bytes, pos);
        pos += 32;
        
        let phone_number = read_fixed_string(&bytes, pos);
        pos += 32;
        
        let city = read_fixed_string(&bytes, pos);
        pos += 32;
        
        let country = read_fixed_string(&bytes, pos);
        pos += 32;

        let education_id = if bytes[pos] == 1 {
            pos += 1;
            let edu_id = read_fixed_string(&bytes, pos);
            pos += 32;
            Some(edu_id)
        } else {
            pos += 33;
            None
        };

        let bank_info_id = if bytes[pos] == 1 {
            pos += 1;
            let bank_id = read_fixed_string(&bytes, pos);
            pos += 32;
            Some(bank_id)
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

impl UserProfile {
    pub fn new(
        id: String,
        name: String,
        email: String,
        phone_number: String,
        city: String,
        country: String,
    ) -> Self {
        let timestamp = time();
        Self {
            id,
            name,
            email,
            phone_number,
            city,
            country,
            education_id: None,
            bank_info_id: None,
            status: 0,
            profile_completion: 0,
            created_at: timestamp,
            updated_at: timestamp,
        }
    }
}

impl From<StableUserProfile> for UserProfile {
    fn from(profile: StableUserProfile) -> Self {
        Self {
            id: fixed_to_string(&profile.id),
            name: fixed_to_string(&profile.name),
            email: fixed_to_string(&profile.email),
            phone_number: fixed_to_string(&profile.phone_number),
            city: fixed_to_string(&profile.city),
            country: fixed_to_string(&profile.country),
            education_id: profile.education_id.map(|id| fixed_to_string(&id)),
            bank_info_id: profile.bank_info_id.map(|id| fixed_to_string(&id)),
            status: profile.status,
            profile_completion: profile.profile_completion,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        }
    }
}

impl From<UserProfile> for StableUserProfile {
    fn from(profile: UserProfile) -> Self {
        Self {
            id: string_to_fixed(&profile.id),
            name: string_to_fixed(&profile.name),
            email: string_to_fixed(&profile.email),
            phone_number: string_to_fixed(&profile.phone_number),
            city: string_to_fixed(&profile.city),
            country: string_to_fixed(&profile.country),
            education_id: profile.education_id.map(|id| string_to_fixed(&id)),
            bank_info_id: profile.bank_info_id.map(|id| string_to_fixed(&id)),
            status: profile.status,
            profile_completion: profile.profile_completion,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        }
    }
}