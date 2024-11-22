use crate::validation::ValidationService;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use crate::models::{
    UserProfile,
    user::{StableUserProfile},
    education::{EducationRecord, StableEducationRecord},
    bank::{BankInformation, StableBankInformation},
    cv::{CV, StableCV, CVAnalysisStatus},
    chat::{ChatMessage, StableChatMessage, ChatSession, StableChatSession},
    FixedString, StorageKey
};
use crate::models::rate_limit::{UserAPIUsage, StableUserAPIUsage};
use crate::types::errors::{StorageError, ChatStorageError};
use crate::models::types::{string_to_storage_key, storage_key_to_string, string_to_content, string_to_fixed};
use ic_cdk::api::time;
use crate::models::RateLimitConfig;

const MEMORY_ID_USERS: MemoryId = MemoryId::new(0);
const MEMORY_ID_EDUCATION: MemoryId = MemoryId::new(1);
const MEMORY_ID_BANK: MemoryId = MemoryId::new(2);
const CV_MEM_ID: MemoryId = MemoryId::new(4);
type CVMemory = VirtualMemory<DefaultMemoryImpl>;


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static USERS: RefCell<StableBTreeMap<StorageKey, StableUserProfile, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_USERS))
        )
    );

    static BANK_INFO: RefCell<StableBTreeMap<StorageKey, StableBankInformation, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_BANK))
        )
    );
    static EDUCATION_RECORDS: RefCell<StableBTreeMap<StorageKey, StableEducationRecord, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_EDUCATION))
        )
    );

    static CV_STORAGE: RefCell<StableBTreeMap<StorageKey, StableCV, CVMemory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(CV_MEM_ID))
        )
    );

    static API_USAGE_STORAGE: RefCell<StableBTreeMap<FixedString, StableUserAPIUsage, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::new(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    static CHAT_STORAGE: RefCell<StableBTreeMap<FixedString, StableChatMessage, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::new(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    static CHAT_SESSION_STORAGE: RefCell<StableBTreeMap<FixedString, StableChatSession, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::new(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );

}

pub struct UserStorage;

impl UserStorage {
    pub fn exists(id: &str) -> bool {
        let key = string_to_storage_key(id);
        USERS.with(|users| users.borrow().contains_key(&key))
    }

    pub fn get(id: &str) -> Option<UserProfile> {
        let key = string_to_storage_key(id);
        USERS.with(|users| {
            users.borrow().get(&key)
                .map(|stable_user| stable_user.into())
        })
    }

     pub fn save_with_validation(user: UserProfile) -> Result<(), StorageError> {
        // Input validation
        if user.name.trim().is_empty() {
            return Err(StorageError::ValidationError("Name cannot be empty".to_string()));
        }
        if user.email.trim().is_empty() {
            return Err(StorageError::ValidationError("Email cannot be empty".to_string()));
        }
        if user.phone_number.trim().is_empty() {
            return Err(StorageError::ValidationError("Phone number cannot be empty".to_string()));
        }
        if user.city.trim().is_empty() {
            return Err(StorageError::ValidationError("City cannot be empty".to_string()));
        }
        if user.country.trim().is_empty() {
            return Err(StorageError::ValidationError("Country cannot be empty".to_string()));
        }

        let key = string_to_storage_key(&user.id);
        
        // Check if user already exists
        if USERS.with(|users| users.borrow().contains_key(&key)) {
            return Err(StorageError::AlreadyExists("User already exists".to_string()));
        }
        
        let stable_user = StableUserProfile::from(user);
        
        USERS.with(|users| {
            users.borrow_mut().insert(key, stable_user);
            Ok(())
        })
    }

    pub fn update_with_validation(user: UserProfile) -> Result<(), StorageError> {
        let key = string_to_storage_key(&user.id);
        let stable_user = StableUserProfile::from(user.clone());
        
        USERS.with(|users| {
            if !users.borrow().contains_key(&key) {
                return Err(StorageError::NotFound("User not found".to_string()));
            }
            users.borrow_mut().insert(key, stable_user);
            Ok(())
        })
    }
}

pub struct EducationStorage;

impl EducationStorage {
    pub fn exists(id: &str) -> bool {
        let key = string_to_storage_key(id);
        BANK_INFO.with(|info| info.borrow().contains_key(&key))
    }

    pub fn save(record: EducationRecord) -> Result<(), String> {
        let stable_record: StableEducationRecord = record.into();
        EDUCATION_RECORDS.with(|records| {
            records.borrow_mut().insert(stable_record.id, stable_record);
            Ok(())
        })
    }

    pub fn get(id: &str) -> Option<EducationRecord> {
        let key = string_to_storage_key(id);
        EDUCATION_RECORDS.with(|records| {
            records.borrow().get(&key)
                .map(|stable_record| stable_record.into())
        })
    }

    pub fn get_by_user(user_id: &str) -> Option<EducationRecord> { // Changed return type
        let user_key = string_to_storage_key(user_id);
        EDUCATION_RECORDS.with(|storage| {
            storage.borrow().iter()
                .find(|(_, record)| record.user_id == user_key)
                .map(|(_, record)| record.into())
        })
    }

    pub fn update_with_validation(record: EducationRecord) -> Result<(), StorageError> {
        let key = string_to_storage_key(&record.id);
        if !EDUCATION_RECORDS.with(|records| records.borrow().contains_key(&key)) {
            return Err(StorageError::NotFound("Education record not found".to_string()));
        }
    
        let stable_record = StableEducationRecord::from(record);
        EDUCATION_RECORDS.with(|records| {
            records.borrow_mut().insert(key, stable_record);
            Ok(())
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
        let key = string_to_storage_key(id);
        BANK_INFO.with(|storage| {
            storage.borrow().get(&key)
                .map(|stable_info| stable_info.into())
        })
    }


    pub fn get_by_user(user_id: &str) -> Option<BankInformation> {
        let user_key = string_to_storage_key(user_id);
        BANK_INFO.with(|storage| {
            storage.borrow().iter()
                .find(|(_, info)| info.user_id == user_key)
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
        println!("Starting save_with_validation");  // Debug log

        // Validate user exists
        if UserStorage::get(&info.user_id).is_none() {
            println!("User not found in save_with_validation");  // Debug log
            return Err(StorageError::InvalidReference(
                "User does not exist".to_string()
            ));
        }

        // Validate SWIFT code
        if !Self::is_valid_swift(&info.swift_code) {
            println!("Invalid SWIFT code");  // Debug log
            return Err(StorageError::ValidationError(
                "Invalid SWIFT code format".to_string()
            ));
        }

        // Check for existing bank info
        if let Some(_) = Self::get_by_user(&info.user_id) {
            println!("Bank info already exists");  // Debug log
            return Err(StorageError::AlreadyExists(
                "Bank information already exists for this user".to_string()
            ));
        }

        println!("Converting to stable storage format");  // Debug log
        let stable_info: StableBankInformation = info.clone().into();
        let key = string_to_storage_key(&info.id);
        
        BANK_INFO.with(|storage| {
            storage.borrow_mut().insert(key, stable_info);
            Ok(())
        })
    }

    pub fn is_valid_swift(code: &str) -> bool {
        let code = code.trim();
        if code.len() != 8 && code.len() != 11 {
            println!("Invalid SWIFT code length: {}", code.len());  // Debug log
            return false;
        }
        // Basic SWIFT code format validation
        code.chars().all(|c| c.is_ascii_alphanumeric())
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

}

pub struct CVStorage;

impl CVStorage {
    pub fn store_cv(cv: CV) -> Result<(), StorageError> {
        let stable_cv = StableCV::from(cv.clone());
        let key = string_to_storage_key(&cv.id);
        
        CV_STORAGE.with(|storage| {
            storage.borrow_mut().insert(key, stable_cv);
            Ok(())
        })
    }

    pub fn get_cv(id: &str) -> Result<CV, StorageError> {
        let key = string_to_storage_key(id);
        CV_STORAGE.with(|storage| {
            storage.borrow().get(&key)
                .map(|cv| CV::from(cv))
                .ok_or_else(|| StorageError::NotFound("CV not found".to_string()))
        })
    }

    pub fn get_user_cvs(user_id: &str) -> Result<Vec<CV>, StorageError> {
        let fixed_user_id = string_to_fixed(user_id);
        CV_STORAGE.with(|storage| {
            let cvs: Vec<CV> = storage
                .borrow()
                .iter()
                .filter(|(_, cv)| cv.user_id == fixed_user_id)
                .map(|(_, cv)| CV::from(cv))
                .collect();

            if cvs.is_empty() {
                Err(StorageError::NotFound("No CVs found for user".to_string()))
            } else {
                Ok(cvs)
            }
        })
    }

    pub fn update_cv(cv: CV) -> Result<(), StorageError> {
        let stable_cv = StableCV::from(cv);
        CV_STORAGE.with(|storage| {
            if !storage.borrow().contains_key(&stable_cv.id) {
                return Err(StorageError::NotFound("CV not found".to_string()));
            }
            storage.borrow_mut().insert(stable_cv.id, stable_cv);
            Ok(())
        })
    }

    pub fn delete_cv(id: &str) -> Result<(), StorageError> {
        let fixed_id = string_to_fixed(id);
        CV_STORAGE.with(|storage| {
            if storage.borrow_mut().remove(&fixed_id).is_none() {
                return Err(StorageError::NotFound("CV not found".to_string()));
            }
            Ok(())
        })
    }

    pub fn get_latest_version(user_id: &str) -> u32 {
        let fixed_user_id = string_to_fixed(user_id);
        CV_STORAGE.with(|storage| {
            storage
                .borrow()
                .iter()
                .filter(|(_, cv)| cv.user_id == fixed_user_id)
                .map(|(_, cv)| cv.version)
                .max()
                .unwrap_or(0)
        })
    }

    pub fn update_ai_analysis(
        id: &str, 
        status: CVAnalysisStatus, 
        feedback: Option<String>
    ) -> Result<(), StorageError> {
        let fixed_id = string_to_fixed(id);
        CV_STORAGE.with(|storage| {
            let mut storage = storage.borrow_mut();
            
            if let Some(cv) = storage.get(&fixed_id) {
                let mut regular_cv = CV::from(cv);
                regular_cv.ai_analysis_status = status;
                regular_cv.ai_feedback = feedback;
                regular_cv.updated_at = ic_cdk::api::time();
                
                let stable_cv = StableCV::from(regular_cv);
                storage.insert(fixed_id, stable_cv);
                Ok(())
            } else {
                Err(StorageError::NotFound("CV not found".to_string()))
            }
        })
    }

    pub fn save_with_validation(cv: CV) -> Result<(), StorageError> {
        if cv.title.len() > 64 {
            return Err(StorageError::ValidationError(
                "Title is too long (max 64 bytes)".to_string()
            ));
        }

        if cv.content.len() > 1024 {
            return Err(StorageError::ValidationError(
                "Content is too long (max 1024 bytes)".to_string()
            ));
        }

        if let Some(feedback) = &cv.ai_feedback {
            if feedback.len() > 1024 {
                return Err(StorageError::ValidationError(
                    "AI feedback is too long (max 1024 bytes)".to_string()
                ));
            }
        }

        let stable_cv: StableCV = cv.into();
        CV_STORAGE.with(|storage| {
            storage.borrow_mut().insert(stable_cv.id, stable_cv);
            Ok(()) 
        })
    }
}

#[cfg(test)]
pub fn clear_cv_storage() {
    CV_STORAGE.with(|storage| {
        storage.borrow_mut().clear();
    });
}

pub struct APIUsageStorage;

impl APIUsageStorage {
    fn get_config() -> RateLimitConfig {
        RateLimitConfig::default()
    }

    pub fn check_and_update_limit(user_id: &str) -> Result<bool, String> {
        let config = Self::get_config();
        let mut usage = Self::get_usage(user_id).unwrap_or(UserAPIUsage {
            user_id: user_id.to_string(),
            daily_requests: 0,
            last_reset: time(),
            total_requests: 0,
        });

        let current_time = time();
        
        if current_time - usage.last_reset >= config.reset_interval_nanos {
            usage.daily_requests = 0;
            usage.last_reset = current_time;
        }

        if usage.daily_requests >= config.daily_limit {
            return Ok(false);
        }

        usage.daily_requests += 1;
        usage.total_requests += 1;

        Self::update_usage(usage)?;

        Ok(true)
    }

    pub fn get_usage(user_id: &str) -> Result<UserAPIUsage, String> {
        let fixed_id = string_to_fixed(user_id);
        API_USAGE_STORAGE.with(|storage| {
            storage
                .borrow()
                .get(&fixed_id)
                .map(|usage| usage.into())
                .ok_or_else(|| "API usage not found".to_string())
        })
    }

    pub fn update_usage(usage: UserAPIUsage) -> Result<(), String> {
        let stable_usage: StableUserAPIUsage = usage.into();
        let fixed_id = stable_usage.user_id;
        API_USAGE_STORAGE.with(|storage| {
            storage.borrow_mut().insert(fixed_id, stable_usage);
            Ok(())
        })
    }

}

pub struct ChatStorage;

impl ChatStorage {
    pub fn store_message(message: ChatMessage) -> Result<(), String> {
        let msg_id = message.id.clone();
        let stable_message: StableChatMessage = message.into();
        let fixed_id = string_to_fixed(&msg_id);

        CHAT_STORAGE.with(|storage| {
            match storage.borrow_mut().insert(fixed_id, stable_message) {
                Some(_) | None => Ok(()),
            }
        })
    }

    pub fn get_message(id: &str) -> Result<ChatMessage, String> {
        let fixed_id = string_to_fixed(id);

        CHAT_STORAGE.with(|storage| {
            storage
                .borrow()
                .get(&fixed_id)
                .map(|msg| msg.into())
                .ok_or_else(|| "Chat message not found".to_string())
        })
    }

    pub fn delete_message(id: &str) -> Result<(), String> {
        let fixed_id = string_to_fixed(id);

        CHAT_STORAGE.with(|storage| {
            storage
                .borrow_mut()
                .remove(&fixed_id)
                .map(|_| ())
                .ok_or_else(|| "Chat message not found".to_string())
        })
    }

    pub fn get_session_messages(session_id: &str) -> Result<Vec<ChatMessage>, String> {
        let prefix = string_to_fixed(session_id);
        
        CHAT_STORAGE.with(|storage| {
            let messages: Vec<ChatMessage> = storage
                .borrow()
                .iter()
                .filter(|(key, _)| key.starts_with(&prefix[..session_id.len().min(32)]))
                .map(|(_, value)| value.into())
                .collect();

            if messages.is_empty() {
                Err("No messages found for session".to_string())
            } else {
                Ok(messages)
            }
        })
    }

    pub fn cleanup_old_messages(older_than_nanos: u64) -> Result<u32, String> {
        let current_time = ic_cdk::api::time();
        let mut deleted_count = 0;

        CHAT_STORAGE.with(|storage| {
            let to_delete: Vec<FixedString> = storage
                .borrow()
                .iter()
                .filter(|(_, msg)| (current_time - msg.timestamp) > older_than_nanos)
                .map(|(key, _)| key)
                .collect();

            for key in to_delete {
                if storage.borrow_mut().remove(&key).is_some() {
                    deleted_count += 1;
                }
            }
        });

        Ok(deleted_count)
    }
}

pub struct ChatSessionStorage;

impl ChatSessionStorage {
    pub fn create_session(user_id: &str, cv_id: &str) -> Result<ChatSession, ChatStorageError> {
        let session = ChatSession::new(user_id.to_string(), cv_id.to_string());
        let stable_session: StableChatSession = session.clone().into();
        let fixed_id = string_to_fixed(&session.id);
    
        CHAT_SESSION_STORAGE.with(|storage| {
            storage.borrow_mut().insert(fixed_id, stable_session);
            Ok(session)
        })
    }
    
    pub fn update_session(session: ChatSession) -> Result<(), ChatStorageError> {
        let stable_session: StableChatSession = session.into();
        let fixed_id = stable_session.id;
    
        CHAT_SESSION_STORAGE.with(|storage| {
            storage.borrow_mut().insert(fixed_id, stable_session);
            Ok(())
        })
    }

    pub fn get_session(session_id: &str) -> Result<ChatSession, ChatStorageError> {
        let fixed_id = string_to_fixed(session_id);

        CHAT_SESSION_STORAGE.with(|storage| {
            storage
                .borrow()
                .get(&fixed_id)
                .map(|session| session.into())
                .ok_or(ChatStorageError::NotFound)
        })
    }

    pub fn get_user_sessions(user_id: &str) -> Vec<ChatSession> {
        let fixed_user_id = string_to_fixed(user_id);

        CHAT_SESSION_STORAGE.with(|storage| {
            storage
                .borrow()
                .iter()
                .filter(|(_, session)| session.user_id == fixed_user_id)
                .map(|(_, session)| session.into())
                .collect()
        })
    }

    pub fn delete_old_sessions(older_than_nanos: u64) -> u32 {
        let current_time = time();
        let mut deleted_count = 0;

        CHAT_SESSION_STORAGE.with(|storage| {
            let to_delete: Vec<FixedString> = storage
                .borrow()
                .iter()
                .filter(|(_, session)| (current_time - session.last_interaction) > older_than_nanos)
                .map(|(key, _)| key)
                .collect();

            for key in to_delete {
                if storage.borrow_mut().remove(&key).is_some() {
                    deleted_count += 1;
                }
            }
        });

        deleted_count
    }
}