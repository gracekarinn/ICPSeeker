use crate::models::cv::{CV, CVAnalysisStatus};
use crate::storage::CVStorage;
use candid::candid_method;
use crate::ai_service::CVAnalyzer;
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

pub mod ai_service;
mod validation;
mod models;
mod storage;
mod types;


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
}

fn init() {
    MEMORY_MANAGER.with(|m| {
        let _ = m.borrow_mut();
    });
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
    let user_id = ic_cdk::api::caller().to_string();
    
    match UserStorage::get(&user_id) {
        Some(user) => UserResponse::Success(user),
        None => UserResponse::Error("User not found".to_string()),
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
    let user_id = ic_cdk::api::caller().to_string();
    
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
    let caller = ic_cdk::caller();
    let user_id = caller.to_string();
    
    if !UserStorage::exists(&user_id) {
        return CVResponse {
            cv: None,
            message: "User not found".to_string(),
        };
    }

    match CVStorage::get_cv(&id) {
        Ok(cv) => {
            if cv.user_id != user_id {
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
            cv.updated_at = ic_cdk::api::time();
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

ic_cdk::export_candid!();