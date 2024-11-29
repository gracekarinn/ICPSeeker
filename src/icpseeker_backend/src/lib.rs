use crate::models::cv::{CV, CVAnalysisStatus};
use crate::storage::CVStorage;
use candid::{candid_method, Principal};
use ic_cdk_macros::{query, update};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use ic_stable_structures::{
    memory_manager::MemoryManager,
    DefaultMemoryImpl,
};
use std::cell::RefCell;
use ic_cdk::println;
use ic_cdk::api::{self, caller}; 
use crate::models::chat::{ChatResponse, ChatHistoryResponse};
use crate::services::chat::ChatService;
use crate::ai_service::analyzer;
pub use crate::ai_service::analyzer::CVAnalyzer;
use ic_cdk::api::management_canister::http_request::{ http_request, CanisterHttpRequestArgument, HttpMethod, TransformContext, HttpHeader, HttpResponse, TransformArgs };
use ic_stable_structures::memory_manager::{MemoryId, VirtualMemory};
use ic_stable_structures::StableBTreeMap;
use crate::models::{
    StorageKey,
    FixedString,
    user::StableUserProfile,
    bank::StableBankInformation,
    education::StableEducationRecord,
    cv::StableCV,
    chat::{StableChatMessage, StableChatSession, ChatMessage},
    rate_limit::StableUserAPIUsage,
};
use ic_cdk::api::time;
use crate::storage::memory::ChatSessionStorage;  
use crate::storage::ChatStorage;
use crate::auth::{AuthService, Session};

pub mod ai_service;
pub mod services {
    pub mod chat;
    pub mod ai; 
}
mod validation;
mod models;
mod storage;
mod types;
mod auth;

const MEMORY_ID_USERS: MemoryId = MemoryId::new(0);
const MEMORY_ID_EDUCATION: MemoryId = MemoryId::new(1);
const MEMORY_ID_BANK: MemoryId = MemoryId::new(2);
const MEMORY_ID_API_USAGE: MemoryId = MemoryId::new(3);
const MEMORY_ID_CV: MemoryId = MemoryId::new(4);
const MEMORY_ID_CHAT: MemoryId = MemoryId::new(5);
const MEMORY_ID_CHAT_SESSION: MemoryId = MemoryId::new(6);

type CVMemory = VirtualMemory<DefaultMemoryImpl>;

use crate::models::{
    UserProfile,
    education::{
        EducationRecord, EducationLevel, EducationStatus,
        HighSchoolEducation, UniversityEducation
    },
    bank::BankInformation
};
use crate::storage::memory::{UserStorage, EducationStorage, BankStorage};
use crate::types::errors::StorageError;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static CONTROLLER: RefCell<Principal> = RefCell::new(Principal::anonymous());
    
    static OPENAI_API_KEY: RefCell<String> = RefCell::new(String::new());

    static USERS: RefCell<StableBTreeMap<StorageKey, StableUserProfile, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new({
        let memory = MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_USERS));
        StableBTreeMap::init(memory)
    });

    static BANK_INFO: RefCell<StableBTreeMap<StorageKey, StableBankInformation, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new({
        let memory = MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_BANK));
        StableBTreeMap::init(memory)
    });

    static CV_STORAGE: RefCell<StableBTreeMap<StorageKey, StableCV, CVMemory>> = RefCell::new({
        let memory = MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_CV));
        StableBTreeMap::init(memory)
    });

    static EDUCATION_RECORDS: RefCell<StableBTreeMap<StorageKey, StableEducationRecord, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new({
        let memory = MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_EDUCATION));
        StableBTreeMap::init(memory)
    });

    static API_USAGE_STORAGE: RefCell<StableBTreeMap<FixedString, StableUserAPIUsage, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new({
        let memory = MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)));
        StableBTreeMap::init(memory)
    });

    static CHAT_STORAGE: RefCell<StableBTreeMap<FixedString, StableChatMessage, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::new(MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_CHAT)))
    );
    
   
    static CHAT_SESSION_STORAGE: RefCell<StableBTreeMap<FixedString, StableChatSession, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new({
        let memory = MEMORY_MANAGER.with(|m| m.borrow().get(MEMORY_ID_CHAT_SESSION));
        StableBTreeMap::init(memory)
    });
}

#[ic_cdk::init]
fn init() {
    let caller = ic_cdk::caller();
    
    MEMORY_MANAGER.with(|m| {
        let _ = m.borrow_mut();
    });

    CONTROLLER.with(|c| {
        *c.borrow_mut() = caller;
    });
}

#[ic_cdk::query]
#[candid_method(query)]
fn get_principal() -> Principal {
    ic_cdk::api::caller()
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct CreateUserPayload {
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub city: String,
    pub country: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct UpdateUserPayload {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum UserResponse {
    Success(UserProfile),
    Error(String),
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct HighSchoolPayload {
    pub school_name: String,
    pub track: String,
    pub city: String,
    pub country: String,
    pub start_year: u32,
    pub end_year: Option<u32>,
    pub status: EducationStatus,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct UniversityPayload {
    pub university_name: String,
    pub level: EducationLevel,
    pub major: String,
    pub city: String,
    pub country: String,
    pub start_year: u32,
    pub end_year: Option<u32>,
    pub gpa: Option<f32>,
    pub status: EducationStatus,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct EducationPayload {
    pub high_school: Option<HighSchoolPayload>,
    pub university: Option<Vec<UniversityPayload>>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum EducationResponse {
    Success(EducationRecord),
    Error(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct BankInfoPayload {
    pub account_holder_name: String,
    pub bank_name: String,
    pub swift_code: String,
    pub account_number: String,
    pub bank_country: String,
    pub bank_branch: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum BankResponse {
    Success(BankInformation),
    Error(StorageError),
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct CreateCVPayload {
    pub title: String,
    pub content: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct UpdateCVPayload {
    pub id: String,
    pub title: String,
    pub content: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct CVResponse {
    pub cv: Option<CV>,
    pub message: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct CVListResponse {
    pub cvs: Vec<CV>,
    pub message: String,
}

#[ic_cdk::query]
fn transform_response(args: TransformArgs) -> HttpResponse {
    args.response
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn create_user(payload: CreateUserPayload) -> UserResponse {
    let caller = caller();
    let user_id = caller.to_string();
    
    if payload.name.trim().is_empty() || 
       payload.email.trim().is_empty() || 
       payload.phone_number.trim().is_empty() || 
       payload.city.trim().is_empty() || 
       payload.country.trim().is_empty() {
        return UserResponse::Error("All fields are required".to_string());
    }
    
    let user = UserProfile::new(
        user_id,
        payload.name,
        payload.email,
        payload.phone_number,
        payload.city,
        payload.country,
    );

    // Attempt to save the user
    match UserStorage::save_with_validation(user.clone()) {
        Ok(()) => UserResponse::Success(user),
        Err(e) => UserResponse::Error(format!("Failed to create user: {:?}", e)),
    }
}

#[ic_cdk::query]
#[candid_method(query)]
pub async fn get_user() -> UserResponse {
    let principal = ic_cdk::api::caller();
    
    match AuthService::get_user_id(&principal) {
        Some(user_id) => {
            match UserStorage::get(&user_id) {
                Some(user) => UserResponse::Success(user),
                None => UserResponse::Error("User not found".to_string()),
            }
        }
        None => UserResponse::Error("No user associated with this principal".to_string()),
    }
}

#[ic_cdk::query]
#[candid_method(query)]
pub async fn get_user_by_id(user_id: String) -> UserResponse {
    let caller = ic_cdk::api::caller().to_string();

    
    match UserStorage::get(&user_id) {
        Some(user) => UserResponse::Success(user),
        None => UserResponse::Error("User not found".to_string()),
    }
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn update_user(payload: UpdateUserPayload) -> UserResponse {
    let principal = ic_cdk::api::caller();
    

    let user_id = match AuthService::get_user_id(&principal) {
        Some(id) => id,
        None => {
            let new_user_id = generate_unique_user_id();
            let user = UserProfile::new(
                new_user_id.clone(),
                payload.name.unwrap_or_default(),
                payload.email.unwrap_or_default(),
                payload.phone_number.unwrap_or_default(),
                payload.city.unwrap_or_default(),
                payload.country.unwrap_or_default(),
            );
            
            match UserStorage::save_with_validation(user.clone()) {
                Ok(_) => {
                    AuthService::associate_user_principal(principal, new_user_id.clone());
                    return UserResponse::Success(user);
                }
                Err(e) => return UserResponse::Error(format!("Failed to create user: {:?}", e)),
            }
        }
    };

    let mut user = match UserStorage::get(&user_id) {
        Some(user) => user,
        None => return UserResponse::Error("User not found".to_string()),
    };

    if let Some(name) = payload.name {
        user.name = name;
    }
    if let Some(email) = payload.email {
        user.email = email;
    }
    if let Some(phone) = payload.phone_number {
        user.phone_number = phone;
    }
    if let Some(city) = payload.city {
        user.city = city;
    }
    if let Some(country) = payload.country {
        user.country = country;
    }

    match UserStorage::update_with_validation(user.clone()) {
        Ok(()) => UserResponse::Success(user),
        Err(e) => UserResponse::Error(format!("Failed to update user: {:?}", e)),
    }
}

fn generate_unique_user_id() -> String {
    format!("user_{}", ic_cdk::api::time())
}


#[ic_cdk::update]
#[candid_method(update)]
pub async fn add_education(payload: EducationPayload) -> EducationResponse {
    let user_id = ic_cdk::api::caller().to_string();
    let education_id = format!("EDU_{}", user_id);

    let mut education_record = EducationRecord::new(education_id, user_id);

    if let Some(hs_payload) = payload.high_school {
        let high_school = HighSchoolEducation::new(
            hs_payload.school_name,
            hs_payload.track,
            hs_payload.city,
            hs_payload.country,
            hs_payload.start_year,
            hs_payload.end_year,
            hs_payload.status,
        );
        education_record.add_high_school(high_school);
    }

    if let Some(uni_payloads) = payload.university {
        education_record.clear_universities(); 
        for uni_payload in uni_payloads {
            let university = UniversityEducation::new(
                uni_payload.university_name,
                uni_payload.level,
                uni_payload.major,
                uni_payload.city,
                uni_payload.country,
                uni_payload.start_year,
                uni_payload.end_year,
                uni_payload.gpa,
                uni_payload.status,
            );
            education_record.add_university(university);
        }
    }

    match EducationStorage::save_with_validation(education_record.clone()) {
        Ok(()) => EducationResponse::Success(education_record),
        Err(e) => EducationResponse::Error(format!("Failed to update education record: {:?}", e)),
    }
}

#[ic_cdk::query]
#[candid_method(query)]
pub async fn get_education() -> EducationResponse {
    let user_id = ic_cdk::api::caller().to_string();
    
    match EducationStorage::get_by_user(&user_id) {
        Some(record) => EducationResponse::Success(record),
        None => EducationResponse::Error("Education record not found".to_string()),
    }
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn update_education(payload: EducationPayload) -> EducationResponse {
    let user_id = ic_cdk::api::caller().to_string();
    
    let mut education_record = match EducationStorage::get_by_user(&user_id) {
        Some(record) => record,
        None => return EducationResponse::Error("Education record not found".to_string()),
    };

    if let Some(hs_payload) = payload.high_school {
        let high_school = HighSchoolEducation::new(
            hs_payload.school_name,
            hs_payload.track,
            hs_payload.city,
            hs_payload.country,
            hs_payload.start_year,
            hs_payload.end_year,
            hs_payload.status,
        );
        education_record.add_high_school(high_school);
    }

    if let Some(uni_payloads) = payload.university {
        education_record.universities.clear(); 
        for uni_payload in uni_payloads {
            let university = UniversityEducation::new(
                uni_payload.university_name,
                uni_payload.level,
                uni_payload.major,
                uni_payload.city,
                uni_payload.country,
                uni_payload.start_year,
                uni_payload.end_year,
                uni_payload.gpa,
                uni_payload.status,
            );
            education_record.add_university(university);
        }
    }

    match EducationStorage::update_with_validation(education_record.clone()) {
        Ok(()) => EducationResponse::Success(education_record),
        Err(e) => EducationResponse::Error(format!("Failed to update education record: {:?}", e)),
    }
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn add_bank_info(payload: BankInfoPayload) -> BankResponse {
    println!("Starting add_bank_info with payload: {:?}", payload);  // Debug log

    // Validate payload fields
    if payload.account_holder_name.trim().is_empty() {
        return BankResponse::Error(StorageError::ValidationError(
            "Account holder name cannot be empty".to_string()
        ));
    }

    if payload.account_number.trim().is_empty() {
        return BankResponse::Error(StorageError::ValidationError(
            "Account number cannot be empty".to_string()
        ));
    }

    if payload.swift_code.trim().is_empty() {
        return BankResponse::Error(StorageError::ValidationError(
            "SWIFT code cannot be empty".to_string()
        ));
    }

    let user_id = ic_cdk::api::caller().to_string();
    println!("User ID: {}", user_id);  // Debug log

    // Check if user exists
    if !UserStorage::exists(&user_id) {
        println!("User not found: {}", user_id);  // Debug log
        return BankResponse::Error(StorageError::InvalidReference(
            "User does not exist".to_string()
        ));
    }

    let bank_id = format!("BANK_{}", user_id);
    println!("Generated bank_id: {}", bank_id);  // Debug log

    // Check if bank info already exists
    if let Some(_existing) = BankStorage::get_by_user(&user_id) {
        println!("Bank info already exists for user: {}", user_id);  // Debug log
        return BankResponse::Error(StorageError::AlreadyExists(
            "Bank information already exists for this user".to_string()
        ));
    }

    let bank_info = BankInformation::new(
        bank_id.clone(),
        user_id.clone(),
        payload.account_holder_name,
        payload.bank_name,
        payload.swift_code,
        payload.account_number,
        payload.bank_country,
        payload.bank_branch,
    );

    println!("Created bank_info object: {:?}", bank_info);  // Debug log

    match BankStorage::save_with_validation(bank_info.clone()) {
        Ok(()) => {
            println!("Successfully saved bank info");  // Debug log
            BankResponse::Success(bank_info)
        },
        Err(e) => {
            println!("Error saving bank info: {:?}", e);  // Debug log
            BankResponse::Error(e)
        }
    }
}

#[ic_cdk::query]
#[candid_method(query)]
pub async fn get_bank_info() -> BankResponse {
    let user_id = ic_cdk::api::caller().to_string();
    
    match BankStorage::get_by_user(&user_id) {
        Some(info) => BankResponse::Success(info),
        None => BankResponse::Error(StorageError::NotFound(
            "Bank information not found".to_string()
        )),
    }
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn update_bank_info(payload: BankInfoPayload) -> BankResponse {
    let user_id = ic_cdk::api::caller().to_string();
    
    let mut bank_info = match BankStorage::get_by_user(&user_id) {
        Some(info) => info,
        None => return BankResponse::Error(StorageError::NotFound(
            "Bank information not found".to_string()
        )),
    };

    bank_info.account_holder_name = payload.account_holder_name;
    bank_info.bank_name = payload.bank_name;
    bank_info.swift_code = payload.swift_code;
    bank_info.account_number = payload.account_number;
    bank_info.bank_country = payload.bank_country;
    bank_info.bank_branch = payload.bank_branch;
    bank_info.updated_at = ic_cdk::api::time();

    match BankStorage::update_with_validation(bank_info.clone()) {
        Ok(()) => BankResponse::Success(bank_info),
        Err(e) => BankResponse::Error(e), 
    }
}

#[ic_cdk::query]
#[candid_method(query)]
pub async fn get_bank_info_by_user_id(user_id: String) -> BankResponse {
    match BankStorage::get_by_user(&user_id) {
        Some(info) => BankResponse::Success(info),
        None => BankResponse::Error(StorageError::NotFound(
            "Bank information not found".to_string()
        )),
    }
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn upload_cv(payload: CreateCVPayload) -> CVResponse {
    let caller = ic_cdk::caller();
    let user_id = caller.to_string();
    
    if !UserStorage::exists(&user_id) {
        return CVResponse {
            cv: None,
            message: "User not found".to_string(),
        };
    }

    let next_version = CVStorage::get_latest_version(&user_id) + 1;
    
    let cv = CV::new(
        format!("{}_{}", user_id, next_version),
        user_id,
        payload.title,
        payload.content,
    );

    match CVStorage::store_cv(cv.clone()) {
        Ok(_) => {
            let cv_id = cv.id.clone();
            ic_cdk::spawn(async move {
                let _ = CVAnalyzer::analyze_cv(cv_id).await;
            });

            CVResponse {
                cv: Some(cv),
                message: "CV uploaded successfully".to_string(),
            }
        }
        Err(e) => CVResponse {
            cv: None,
            message: format!("Failed to store CV: {}", e),
        },
    }
}

#[ic_cdk::query]
#[candid_method(query)]
pub async fn get_cv(id: String) -> CVResponse {
    let caller = ic_cdk::caller().to_string();
    
    match CVStorage::get_cv(&id) {
        Ok(cv) => {
            if !cv.user_id.starts_with(&caller[..caller.len().min(cv.user_id.len())]) {
                return CVResponse {
                    cv: None,
                    message: "Access denied".to_string(),
                };
            }
            CVResponse {
                cv: Some(cv),
                message: "CV retrieved successfully".to_string(),
            }
        }
        Err(e) => CVResponse {
            cv: None,
            message: format!("Failed to retrieve CV: {}", e),
        },
    }
}

#[ic_cdk::query]
#[candid_method(query)]
pub async fn get_my_cvs() -> CVListResponse {
    let caller = ic_cdk::caller();
    let user_id = caller.to_string();
    
    if !UserStorage::exists(&user_id) {
        return CVListResponse {
            cvs: vec![],
            message: "User not found".to_string(),
        };
    }

    match CVStorage::get_user_cvs(&user_id) {
        Ok(cvs) => CVListResponse {
            cvs,
            message: "CVs retrieved successfully".to_string(),
        },
        Err(e) => CVListResponse {
            cvs: vec![],
            message: format!("Failed to retrieve CVs: {}", e),
        },
    }
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn update_cv(payload: UpdateCVPayload) -> CVResponse {
    let caller = ic_cdk::caller();
    let user_id = caller.to_string();
    
    if !UserStorage::exists(&user_id) {
        return CVResponse {
            cv: None,
            message: "User not found".to_string(),
        };
    }

    match CVStorage::get_cv(&payload.id) {
        Ok(mut cv) => {
            if cv.user_id != user_id {
                return CVResponse {
                    cv: None,
                    message: "Access denied".to_string(),
                };
            }

            cv.title = payload.title;
            cv.content = payload.content;
            cv.ai_analysis_status = CVAnalysisStatus::NotAnalyzed;
            cv.ai_feedback = None;

            match CVStorage::update_cv(cv.clone()) {
                Ok(_) => {
                    let cv_id = cv.id.clone();
                    ic_cdk::spawn(async move {
                        let _ = CVAnalyzer::analyze_cv(cv_id).await;
                    });

                    CVResponse {
                        cv: Some(cv),
                        message: "CV updated successfully".to_string(),
                    }
                }
                Err(e) => CVResponse {
                    cv: None,
                    message: format!("Failed to update CV: {}", e),
                },
            }
        }
        Err(e) => CVResponse {
            cv: None,
            message: format!("Failed to retrieve CV: {}", e),
        },
    }
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn start_cv_chat(cv_id: String) -> ChatResponse {
    let caller = ic_cdk::caller().to_string();
    ChatService::start_chat(&caller, &cv_id).await
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn send_chat_message(
    session_id: String, 
    content: String
) -> ChatResponse {
    let caller = ic_cdk::caller().to_string();
    println!("Caller ID: {}", caller);

    let session = match ChatSessionStorage::get_session(&session_id) {
        Ok(session) => {
            println!("Session user_id: {}", session.user_id);
            if session.user_id != caller {  // Remove .to_string()
                return ChatResponse {
                    message: None,
                    error: Some("Access denied to this chat session".to_string()),
                };
            }
            session
        }
        Err(_) => {
            return ChatResponse {
                message: None,
                error: Some("Chat session not found".to_string()),
            };
        }
    };

    let message = ChatMessage {
        id: format!("msg_{}_{}", session_id, time()),
        content: content.clone(),
        is_ai: false,
        timestamp: time(),
    };

    if let Err(e) = ChatStorage::store_message(
        &session.id,
        message.content.clone(),
        message.is_ai
    ) {
        return ChatResponse {
            message: None,
            error: Some(format!("Failed to store message: {}", e)),
        };
    }
    
    // Let ChatService handle the AI communication
    let result = ChatService::send_message(&session_id, &caller, content).await;
    result
}

#[ic_cdk::query]
#[candid_method(query)]
pub fn get_chat_history(session_id: String) -> ChatHistoryResponse {
    let caller = ic_cdk::caller().to_string();
    ChatService::get_chat_history(&session_id, &caller)
}

#[ic_cdk::update]
#[candid_method(update)]
fn set_openai_key(key: String) -> Result<(), String> {
    let caller = ic_cdk::caller();
    
    let canister_id = ic_cdk::api::id();
    
    if caller.to_string() != "ftbln-b7mfk-fjq6u-dh3u3-7rylz-2vyi3-jqzhy-7phve-hswv4-u4fze-2qe" {
        return Err("Unauthorized: Only controller can set API key".to_string());
    }

    OPENAI_API_KEY.with(|k| {
        *k.borrow_mut() = key;
    });
    
    Ok(())
}

pub fn get_openai_key() -> Result<String, String> {
    OPENAI_API_KEY.with(|k| {
        let key = k.borrow().clone();
        if key.is_empty() {
            Err("OpenAI API key not set".to_string())
        } else {
            Ok(key)
        }
    })
}

#[ic_cdk::update]
#[candid_method(update)]
fn clear_all_storage() -> Result<String, String> {
    let caller = ic_cdk::caller();
    
    if caller.to_string() != "ftbln-b7mfk-fjq6u-dh3u3-7rylz-2vyi3-jqzhy-7phve-hswv4-u4fze-2qe" {
        return Err("Unauthorized: Only controller can clear storage".to_string());
    }

    USERS.with(|users| {
        let mut users = users.borrow_mut();
        let keys: Vec<_> = users.iter().map(|(k, _)| k).collect();
        for key in keys {
            users.remove(&key);
        }
    });

    BANK_INFO.with(|bank_info| {
        let mut bank_info = bank_info.borrow_mut();
        let keys: Vec<_> = bank_info.iter().map(|(k, _)| k).collect();
        for key in keys {
            bank_info.remove(&key);
        }
    });

    CV_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let keys: Vec<_> = storage.iter().map(|(k, _)| k).collect();
        for key in keys {
            storage.remove(&key);
        }
    });

    Ok("All storage cleared successfully".to_string())
}

#[ic_cdk::update]
#[candid_method(update)]
fn clear_cv_storage() -> Result<String, String> {
    let caller = ic_cdk::caller();
    
    if caller.to_string() != "ftbln-b7mfk-fjq6u-dh3u3-7rylz-2vyi3-jqzhy-7phve-hswv4-u4fze-2qe" {
        return Err("Unauthorized: Only controller can clear storage".to_string());
    }

    CV_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let keys: Vec<_> = storage.iter().map(|(k, _)| k).collect();
        for key in keys {
            storage.remove(&key);
        }
    });

    Ok("CV storage cleared successfully".to_string())
}

#[ic_cdk::update]
#[candid_method(update)]
async fn login() -> Result<Session, String> {
    let principal = ic_cdk::api::caller();
    
    if principal == Principal::anonymous() {
        return Err("Anonymous principals cannot log in".to_string());
    }

    let session = AuthService::create_session(principal);
    
    if AuthService::get_user_id(&principal).is_none() {
        let user_id = generate_unique_user_id(); 
        let user = UserProfile::new(
            user_id.clone(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        );
        
        match UserStorage::save_with_validation(user) {
            Ok(_) => {
                AuthService::associate_user_principal(principal, user_id);
                Ok(session)
            }
            Err(e) => Err(format!("Failed to create user profile: {:?}", e))
        }
    } else {
        Ok(session)
    }
}

#[ic_cdk::query]
#[candid_method(query)]
fn is_logged_in() -> bool {
    let principal = ic_cdk::api::caller();
    AuthService::is_authenticated(&principal)
}



ic_cdk::export_candid!();