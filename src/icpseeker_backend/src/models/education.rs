use candid::{CandidType, Decode, Encode};
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use ic_stable_structures::{Storable, BoundedStorable};
use std::borrow::Cow;

use super::user::FixedString;
use super::user::{string_to_fixed, fixed_to_string};

#[derive(Clone, Debug)]
pub struct StableEducationRecord {
    pub id: FixedString,
    pub user_id: FixedString,
    pub school_name: FixedString,
    pub track: FixedString,
    pub university_name: FixedString,
    pub major: FixedString,
    pub city: FixedString,
    pub country: FixedString,
    pub education_level: u8,
    pub status: u8,
    pub start_year: u32,
    pub end_year: Option<u32>,
    pub gpa: Option<u32>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Storable for StableEducationRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id);
        bytes.extend_from_slice(&self.user_id);
        bytes.extend_from_slice(&self.school_name);
        bytes.extend_from_slice(&self.track);
        bytes.extend_from_slice(&self.university_name);
        bytes.extend_from_slice(&self.major);
        bytes.extend_from_slice(&self.city);
        bytes.extend_from_slice(&self.country);
        bytes.push(self.education_level);
        bytes.push(self.status);
        bytes.extend_from_slice(&self.start_year.to_be_bytes());
        
        if let Some(end_year) = self.end_year {
            bytes.push(1);
            bytes.extend_from_slice(&end_year.to_be_bytes());
        } else {
            bytes.push(0);
            bytes.extend_from_slice(&[0u8; 4]);
        }
        
        if let Some(gpa) = self.gpa {
            bytes.push(1);
            bytes.extend_from_slice(&gpa.to_be_bytes());
        } else {
            bytes.push(0);
            bytes.extend_from_slice(&[0u8; 4]);
        }

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
        let user_id = next_fixed_str();
        let school_name = next_fixed_str();
        let track = next_fixed_str();
        let university_name = next_fixed_str();
        let major = next_fixed_str();
        let city = next_fixed_str();
        let country = next_fixed_str();
        
        let education_level = bytes[pos];
        pos += 1;
        let status = bytes[pos];
        pos += 1;
        
        let start_year = u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap());
        pos += 4;

        let end_year = if bytes[pos] == 1 {
            pos += 1;
            Some(u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap()))
        } else {
            pos += 5;
            None
        };
        
        let gpa = if bytes[pos] == 1 {
            pos += 1;
            Some(u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap()))
        } else {
            pos += 5;
            None
        };

        let created_at = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let updated_at = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());

        Self {
            id,
            user_id,
            school_name,
            track,
            university_name,
            major,
            city,
            country,
            education_level,
            status,
            start_year,
            end_year,
            gpa,
            created_at,
            updated_at,
        }
    }
}

impl BoundedStorable for StableEducationRecord {
    const MAX_SIZE: u32 = (32 * 8) + 2 + 4 + 5 + 5 + 16; 
    const IS_FIXED_SIZE: bool = true;
}