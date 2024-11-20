use crate::validation::ValidationService;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

use crate::models::{
    UserProfile,
    user::{StableUserProfile, string_to_fixed, FixedString},
    education::{EducationRecord, StableEducationRecord},
    bank::{BankInformation, StableBankInformation},
};
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
        let stable_user: StableUserProfile = user.into();
        USERS.with(|users| {
            users.borrow_mut().insert(stable_user.id, stable_user);
            Ok(())
        })
    }

    pub fn get(id: &str) -> Option<UserProfile> {
        let fixed_id = string_to_fixed(id);
        USERS.with(|users| {
            users.borrow().get(&fixed_id)
                .map(|stable_user| stable_user.into())
        })
    }

    pub fn update(user: UserProfile) -> Result<(), String> {
        let stable_user: StableUserProfile = user.into();
        USERS.with(|users| {
            if users.borrow().contains_key(&stable_user.id) {
                users.borrow_mut().insert(stable_user.id, stable_user);
                Ok(())
            } else {
                Err("User not found".to_string())
            }
        })
    }

    pub fn save_with_validation(user: UserProfile) -> Result<(), StorageError> {
        let stable_user: StableUserProfile = user.clone().into();
        ValidationService::validate_user(&stable_user)
            .map_err(|e| StorageError::ValidationError(format!("{:?}", e)))?;

        Self::save(user).map_err(|e| StorageError::SystemError(e))
    }

    pub fn update_with_validation(user: UserProfile) -> Result<(), StorageError> {
        let stable_user: StableUserProfile = user.clone().into();
        ValidationService::validate_user(&stable_user)
            .map_err(|e| StorageError::ValidationError(format!("{:?}", e)))?;

        ValidationService::validate_relationships(&user.id)?;

        Self::update(user).map_err(|e| StorageError::SystemError(e))
    }
}

pub struct EducationStorage;

impl EducationStorage {
    pub fn save(record: EducationRecord) -> Result<(), String> {
        let stable_record: StableEducationRecord = record.into();
        EDUCATION_RECORDS.with(|records| {
            records.borrow_mut().insert(stable_record.id, stable_record);
            Ok(())
        })
    }

    pub fn get(id: &str) -> Option<EducationRecord> {
        let fixed_id = string_to_fixed(id);
        EDUCATION_RECORDS.with(|records| {
            records.borrow().get(&fixed_id)
                .map(|stable_record| stable_record.into())
        })
    }

    pub fn get_by_user(user_id: &str) -> Option<EducationRecord> {
        let fixed_user_id = string_to_fixed(user_id);
        EDUCATION_RECORDS.with(|records| {
            records.borrow().iter()
                .find(|(_, record)| record.user_id == fixed_user_id)
                .map(|(_, record)| record.into())
        })
    }

    pub fn update(record: EducationRecord) -> Result<(), String> {
        let stable_record: StableEducationRecord = record.into();
        EDUCATION_RECORDS.with(|records| {
            if records.borrow().contains_key(&stable_record.id) {
                records.borrow_mut().insert(stable_record.id, stable_record);
                Ok(())
            } else {
                Err("Education record not found".to_string())
            }
        })
    }

    pub fn update_with_validation(record: EducationRecord) -> Result<(), StorageError> {
        let fixed_id = string_to_fixed(&record.id);
        if !EDUCATION_RECORDS.with(|records| records.borrow().contains_key(&fixed_id)) {
            return Err(StorageError::NotFound("Education record not found".to_string()));
        }
    
        let stable_record: StableEducationRecord = record.into();
        EDUCATION_RECORDS.with(|records| {
            records.borrow_mut().insert(stable_record.id, stable_record);
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
        let stable_info: StableBankInformation = info.into();
        BANK_INFO.with(|bank_info| {
            bank_info.borrow_mut().insert(stable_info.id, stable_info);
            Ok(())
        })
    }

    pub fn get(id: &str) -> Option<BankInformation> {
        let fixed_id = string_to_fixed(id);
        BANK_INFO.with(|bank_info| {
            bank_info.borrow().get(&fixed_id)
                .map(|stable_info| stable_info.into())
        })
    }

    pub fn get_by_user(user_id: &str) -> Option<BankInformation> {
        let fixed_user_id = string_to_fixed(user_id);
        BANK_INFO.with(|bank_info| {
            bank_info.borrow().iter()
                .find(|(_, info)| info.user_id == fixed_user_id)
                .map(|(_, info)| info.into())
        })
    }

    pub fn update(info: BankInformation) -> Result<(), String> {
        let stable_info: StableBankInformation = info.into();
        BANK_INFO.with(|bank_info| {
            if bank_info.borrow().contains_key(&stable_info.id) {
                bank_info.borrow_mut().insert(stable_info.id, stable_info);
                Ok(())
            } else {
                Err("Bank information not found".to_string())
            }
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

    pub fn update_with_validation(info: BankInformation) -> Result<(), StorageError> {
        let fixed_id = string_to_fixed(&info.id); 
        if !BANK_INFO.with(|bank_info| bank_info.borrow().contains_key(&fixed_id)) {
            return Err(StorageError::NotFound("Bank information not found".to_string()));
        }

        if !Self::is_valid_swift(&info.swift_code) {
            return Err(StorageError::ValidationError(
                "Invalid SWIFT code format".to_string()
            ));
        }

        let stable_info: StableBankInformation = info.into();
        BANK_INFO.with(|bank_info| {
            bank_info.borrow_mut().insert(stable_info.id, stable_info);
            Ok(())
        })
    }

    pub fn is_valid_swift(code: &str) -> bool {
        let code_len = code.len();
        code_len == 8 || code_len == 11
    }
}

