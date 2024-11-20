use candid::{CandidType, Decode, Encode};
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use std::borrow::Cow;
use ic_stable_structures::{Storable, BoundedStorable};

pub type FixedString = [u8; 32];

pub fn fixed_to_string(fixed: &FixedString) -> String {
    String::from_utf8(
        fixed.iter()
            .take_while(|&&x| x != 0)
            .copied()
            .collect()
    ).unwrap_or_default()
}

fn string_to_fixed(s: &str) -> FixedString {
    let mut fixed = [0u8; 32];
    let bytes = s.as_bytes();
    let len = bytes.len().min(32);
    fixed[..len].copy_from_slice(&bytes[..len]);
    fixed
}

#[derive(Clone, Debug)]
pub struct StableBankInformation {
    pub id: FixedString,
    pub user_id: FixedString,
    pub account_holder_name: FixedString,
    pub bank_name: FixedString,
    pub swift_code: FixedString,
    pub account_number: FixedString,
    pub bank_country: FixedString,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct BankInformation {
    pub id: String,
    pub user_id: String,
    pub account_holder_name: String,
    pub bank_name: String,
    pub swift_code: String,
    pub account_number: String,
    pub bank_country: String,
    pub bank_branch: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Storable for StableBankInformation {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id);
        bytes.extend_from_slice(&self.user_id);
        bytes.extend_from_slice(&self.account_holder_name);
        bytes.extend_from_slice(&self.bank_name);
        bytes.extend_from_slice(&self.swift_code);
        bytes.extend_from_slice(&self.account_number);
        bytes.extend_from_slice(&self.bank_country);
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
        let account_holder_name = next_fixed_str();
        let bank_name = next_fixed_str();
        let swift_code = next_fixed_str();
        let account_number = next_fixed_str();
        let bank_country = next_fixed_str();

        let created_at = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let updated_at = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());

        Self {
            id,
            user_id,
            account_holder_name,
            bank_name,
            swift_code,
            account_number,
            bank_country,
            created_at,
            updated_at,
        }
    }
}

impl BoundedStorable for StableBankInformation {
    const MAX_SIZE: u32 = 32 * 7 + 8 + 8; 
    const IS_FIXED_SIZE: bool = true;
}

impl From<BankInformation> for StableBankInformation {
    fn from(info: BankInformation) -> Self {
        Self {
            id: string_to_fixed(&info.id),
            user_id: string_to_fixed(&info.user_id),
            account_holder_name: string_to_fixed(&info.account_holder_name),
            bank_name: string_to_fixed(&info.bank_name),
            swift_code: string_to_fixed(&info.swift_code),
            account_number: string_to_fixed(&info.account_number),
            bank_country: string_to_fixed(&info.bank_country),
            created_at: info.created_at,
            updated_at: info.updated_at,
        }
    }
}

impl BankInformation {
    pub fn new(
        id: String,
        user_id: String,
        account_holder_name: String,
        bank_name: String,
        swift_code: String,
        account_number: String,
        bank_country: String,
        bank_branch: Option<String>,
    ) -> Self {
        let timestamp = time();
        Self {
            id,
            user_id,
            account_holder_name,
            bank_name,
            swift_code,
            account_number,
            bank_country,
            bank_branch,
            created_at: timestamp,
            updated_at: timestamp,
        }
    }
}

impl From<StableBankInformation> for BankInformation {
    fn from(info: StableBankInformation) -> Self {
        Self {
            id: fixed_to_string(&info.id),
            user_id: fixed_to_string(&info.user_id),
            account_holder_name: fixed_to_string(&info.account_holder_name),
            bank_name: fixed_to_string(&info.bank_name),
            swift_code: fixed_to_string(&info.swift_code),
            account_number: fixed_to_string(&info.account_number),
            bank_country: fixed_to_string(&info.bank_country),
            bank_branch: None,
            created_at: info.created_at,
            updated_at: info.updated_at,
        }
    }
}