use ic_cdk::api::management_canister::main::raw_rand;
use candid::{CandidType, Decode, Encode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EncryptedData {
    pub data: Vec<u8>,
    pub nonce: Vec<u8>,
}

thread_local! {
    static ENCRYPTION_KEYS: RefCell<HashMap<String, Vec<u8>>> = RefCell::new(HashMap::new());
}

pub struct Encryption;

impl Encryption {
    pub async fn generate_key(user_id: &str) -> Result<(), String> {
        let random_bytes = match raw_rand().await {
            Ok((bytes, _)) => bytes,
            Err(e) => return Err(format!("Failed to generate random bytes: {:?}", e)),
        };

        ENCRYPTION_KEYS.with(|keys| {
            keys.borrow_mut().insert(user_id.to_string(), random_bytes);
        });

        Ok(())
    }

    pub fn encrypt(data: &str, user_id: &str) -> Result<EncryptedData, String> {
        ENCRYPTION_KEYS.with(|keys| {
            let keys = keys.borrow();
            let key = keys.get(user_id)
                .ok_or_else(|| "Encryption key not found".to_string())?;

            let nonce = ic_cdk::api::time().to_be_bytes().to_vec();

            let mut encrypted = Vec::new();
            for (i, byte) in data.as_bytes().iter().enumerate() {
                let key_byte = key[i % key.len()];
                let nonce_byte = nonce[i % nonce.len()];
                encrypted.push(byte ^ key_byte ^ nonce_byte);
            }

            Ok(EncryptedData {
                data: encrypted,
                nonce,
            })
        })
    }

    pub fn decrypt(encrypted: &EncryptedData, user_id: &str) -> Result<String, String> {
        ENCRYPTION_KEYS.with(|keys| {
            let keys = keys.borrow();
            let key = keys.get(user_id)
                .ok_or_else(|| "Encryption key not found".to_string())?;

            let mut decrypted = Vec::new();
            for (i, byte) in encrypted.data.iter().enumerate() {
                let key_byte = key[i % key.len()];
                let nonce_byte = encrypted.nonce[i % encrypted.nonce.len()];
                decrypted.push(byte ^ key_byte ^ nonce_byte);
            }

            String::from_utf8(decrypted)
                .map_err(|e| format!("Failed to decode decrypted data: {:?}", e))
        })
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EncryptedBankInformation {
    pub id: String,
    pub user_id: String,
    pub account_holder_name: String,
    pub bank_name: String,
    pub encrypted_swift_code: EncryptedData,
    pub encrypted_account_number: EncryptedData,
    pub bank_country: String,
    pub bank_branch: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl EncryptedBankInformation {
    pub async fn new(
        id: String,
        user_id: String,
        account_holder_name: String,
        bank_name: String,
        swift_code: String,
        account_number: String,
        bank_country: String,
        bank_branch: Option<String>,
    ) -> Result<Self, String> {
        if let Err(e) = Encryption::generate_key(&user_id).await {
            return Err(format!("Failed to generate encryption key: {}", e));
        }

        let encrypted_swift_code = Encryption::encrypt(&swift_code, &user_id)?;
        let encrypted_account_number = Encryption::encrypt(&account_number, &user_id)?;

        let timestamp = ic_cdk::api::time();
        Ok(Self {
            id,
            user_id,
            account_holder_name,
            bank_name,
            encrypted_swift_code,
            encrypted_account_number,
            bank_country,
            bank_branch,
            created_at: timestamp,
            updated_at: timestamp,
        })
    }

    pub fn get_swift_code(&self) -> Result<String, String> {
        Encryption::decrypt(&self.encrypted_swift_code, &self.user_id)
    }

    pub fn get_account_number(&self) -> Result<String, String> {
        Encryption::decrypt(&self.encrypted_account_number, &self.user_id)
    }
}