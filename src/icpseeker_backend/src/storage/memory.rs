use crate::validation::ValidationService;
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

use crate::models::{UserProfile, EducationRecord, BankInformation};
use crate::StorageError;

const MEMORY_ID_USERS: MemoryId = MemoryId::new(0);
const MEMORY_ID_EDUCATION: MemoryId = MemoryId::new(1);
const MEMORY_ID_BANK: MemoryId = MemoryId::new(2);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static USERS: RefCell<StableBTreeMap<FixedString, StableUserProfile, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_USERS))
        )
    );

    static EDUCATION_RECORDS: RefCell<StableBTreeMap<FixedString, StableEducationRecord, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_EDUCATION))
        )
    );

    static BANK_INFO: RefCell<StableBTreeMap<FixedString, StableBankInformation, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_BANK))
        )
    );
}

pub struct UserStorage;

impl UserStorage {
    pub fn save(user: UserProfile) -> Result<(), String> {
        USERS.with(|users| {
            users.borrow_mut().insert(user.id.clone(), user);
            Ok(())
        })
    }

    pub fn get(id: &str) -> Option<UserProfile> {
        USERS.with(|users| users.borrow().get(id))
    }

    pub fn update(user: UserProfile) -> Result<(), String> {
        USERS.with(|users| {
            if users.borrow().contains_key(&user.id) {
                users.borrow_mut().insert(user.id.clone(), user);
                Ok(())
            } else {
                Err("User not found".to_string())
            }
        })
    }

    pub fn delete(id: &str) -> Result<(), String> {
        USERS.with(|users| {
            if users.borrow_mut().remove(id).is_some() {
                Ok(())
            } else {
                Err("User not found".to_string())
            }
        })
    }

    pub fn save_with_validation(user: UserProfile) -> Result<(), StorageError> {
        ValidationService::validate_user(&user)
            .map_err(|e| StorageError::ValidationError(format!("{:?}", e)))?;

        Self::save(user).map_err(|e| StorageError::SystemError(e))
    }

    pub fn update_with_validation(user: UserProfile) -> Result<(), StorageError> {
        ValidationService::validate_user(&user)
            .map_err(|e| StorageError::ValidationError(format!("{:?}", e)))?;

        ValidationService::validate_relationships(&user.id)?;

        Self::update(user).map_err(|e| StorageError::SystemError(e))
    }
}

pub struct EducationStorage;

impl EducationStorage {
    pub fn save(record: EducationRecord) -> Result<(), String> {
        EDUCATION_RECORDS.with(|records| {
            records.borrow_mut().insert(record.id.clone(), record);
            Ok(())
        })
    }

    pub fn get(id: &str) -> Option<EducationRecord> {
        EDUCATION_RECORDS.with(|records| records.borrow().get(id))
    }

    pub fn get_by_user(user_id: &str) -> Option<EducationRecord> {
        EDUCATION_RECORDS.with(|records| {
            records.borrow().iter().find(|(_, record)| record.user_id == user_id)
                .map(|(_, record)| record)
        })
    }

    pub fn update(record: EducationRecord) -> Result<(), String> {
        EDUCATION_RECORDS.with(|records| {
            if records.borrow().contains_key(&record.id) {
                records.borrow_mut().insert(record.id.clone(), record);
                Ok(())
            } else {
                Err("Education record not found".to_string())
            }
        })
    }

    pub fn update_with_validation(record: EducationRecord) -> Result<(), StorageError> {
        if !EDUCATION_RECORDS.with(|records| records.borrow().contains_key(&record.id)) {
            return Err(StorageError::NotFound("Education record not found".to_string()));
        }

        EDUCATION_RECORDS.with(|records| {
            records.borrow_mut().insert(record.id.clone(), record);
            Ok(())
        })
    }

    pub fn save_with_validation(record: EducationRecord) -> Result<(), StorageError> {
        if UserStorage::get(&record.user_id).is_none() {
            return Err(StorageError::InvalidReference(
                "User does not exist".to_string()
            ));
        }

        if let Some(existing) = Self::get_by_user(&record.user_id) {
            if existing.id != record.id {
                return Err(StorageError::AlreadyExists(
                    "User already has an education record".to_string()
                ));
            }
        }

        Self::save(record).map_err(|e| StorageError::SystemError(e))
    }

}

pub struct BankStorage;

impl BankStorage {
    pub fn save(info: BankInformation) -> Result<(), String> {
        BANK_INFO.with(|bank_info| {
            bank_info.borrow_mut().insert(info.id.clone(), info);
            Ok(())
        })
    }

    pub fn get(id: &str) -> Option<BankInformation> {
        BANK_INFO.with(|bank_info| bank_info.borrow().get(id))
    }

    pub fn get_by_user(user_id: &str) -> Option<BankInformation> {
        BANK_INFO.with(|bank_info| {
            bank_info.borrow().iter().find(|(_, info)| info.user_id == user_id)
                .map(|(_, info)| info)
        })
    }

    pub fn update(info: BankInformation) -> Result<(), String> {
        BANK_INFO.with(|bank_info| {
            if bank_info.borrow().contains_key(&info.id) {
                bank_info.borrow_mut().insert(info.id.clone(), info);
                Ok(())
            } else {
                Err("Bank information not found".to_string())
            }
        })
    }

    pub fn update_with_validation(info: BankInformation) -> Result<(), StorageError> {
        if !BANK_INFO.with(|bank_info| bank_info.borrow().contains_key(&info.id)) {
            return Err(StorageError::NotFound("Bank information not found".to_string()));
        }

        if !Self::is_valid_swift(&info.swift_code) {
            return Err(StorageError::ValidationError(
                "Invalid SWIFT code format".to_string()
            ));
        }

        BANK_INFO.with(|bank_info| {
            bank_info.borrow_mut().insert(info.id.clone(), info);
            Ok(())
        })
    }

    pub fn save_with_validation(info: BankInformation) -> Result<(), StorageError> {
        if UserStorage::get(&info.user_id).is_none() {
            return Err(StorageError::InvalidReference(
                "User does not exist".to_string()
            ));
        }

        if let Some(existing) = Self::get_by_user(&info.user_id) {
            if existing.id != info.id {
                return Err(StorageError::AlreadyExists(
                    "User already has bank information".to_string()
                ));
            }
        }

        if !Self::is_valid_swift(&info.swift_code) {
            return Err(StorageError::ValidationError(
                "Invalid SWIFT code format".to_string()
            ));
        }

        Self::save(info).map_err(|e| StorageError::SystemError(e))
    }

    pub fn is_valid_swift(code: &str) -> bool {
        let code_len = code.len();
        code_len == 8 || code_len == 11
    }
}

