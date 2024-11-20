use candid::{CandidType, Principal};
use ic_cdk_macros::{init, query, update}; 
use serde::{Deserialize, Serialize};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl
};
use std::cell::RefCell;

mod validation;
mod models;
mod storage;
mod types;

use crate::models::user::{UserProfile, Location};
use crate::storage::memory::UserStorage;
use crate::types::errors::StorageError;
use crate::models::education::{EducationRecord, EducationLevel, EducationStatus, HighSchoolEducation, UniversityEducation};
use crate::storage::memory::EducationStorage;
use crate::models::bank::BankInformation;
use crate::storage::memory::BankStorage;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
}

#[init]
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

#[derive(CandidType, Serialize, Deserialize)]
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
    Error(String),
}


#[update]
pub async fn create_user(payload: CreateUserPayload) -> UserResponse {
    let caller = ic_cdk::api::caller();
    let user_id = caller.to_string();
    
    let user = UserProfile::new(
        user_id,
        payload.name,
        payload.email,
        payload.phone_number,
        Location {
            city: payload.city,
            country: payload.country,
        },
    );

    match UserStorage::save_with_validation(user.clone()) {
        Ok(()) => UserResponse::Success(user),
        Err(e) => UserResponse::Error(format!("Failed to create user: {:?}", e)),
    }
}

#[query]
pub async fn get_user() -> UserResponse {
    let user_id = ic_cdk::api::caller().to_string();
    
    match UserStorage::get(&user_id) {
        Some(user) => UserResponse::Success(user),
        None => UserResponse::Error("User not found".to_string()),
    }
}


#[query]
pub async fn get_user_by_id(user_id: String) -> UserResponse {
    let caller = ic_cdk::api::caller().to_string();

    
    match UserStorage::get(&user_id) {
        Some(user) => UserResponse::Success(user),
        None => UserResponse::Error("User not found".to_string()),
    }
}

#[update]
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
        user.location.city = city;
    }
    if let Some(country) = payload.country {
        user.location.country = country;
    }
    
    user.updated_at = ic_cdk::api::time();

    match UserStorage::update_with_validation(user.clone()) {
        Ok(()) => UserResponse::Success(user),
        Err(e) => UserResponse::Error(format!("Failed to update user: {:?}", e)),
    }
}

#[update]
pub async fn add_education(payload: EducationPayload) -> EducationResponse {
    let user_id = ic_cdk::api::caller().to_string();
    let education_id = format!("EDU_{}", user_id);

    let mut education_record = EducationRecord::new(education_id.clone(), user_id.clone());

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

#[query]
pub async fn get_education() -> EducationResponse {
    let user_id = ic_cdk::api::caller().to_string();
    
    match EducationStorage::get_by_user(&user_id) {
        Some(record) => EducationResponse::Success(record),
        None => EducationResponse::Error("Education record not found".to_string()),
    }
}

#[update]
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
        education_record.university.clear(); // Clear existing records
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

#[update]
pub async fn add_bank_info(payload: BankInfoPayload) -> BankResponse {
    let user_id = ic_cdk::api::caller().to_string();
    let bank_id = format!("BANK_{}", user_id);

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

    match BankStorage::save_with_validation(bank_info.clone()) {
        Ok(()) => BankResponse::Success(bank_info),
        Err(e) => BankResponse::Error(format!("Failed to update bank information: {:?}", e)),
    }
}

#[query]
pub async fn get_bank_info() -> BankResponse {
    let user_id = ic_cdk::api::caller().to_string();
    
    match BankStorage::get_by_user(&user_id) {
        Some(info) => BankResponse::Success(info),
        None => BankResponse::Error("Bank information not found".to_string()),
    }
}

#[update]
pub async fn update_bank_info(payload: BankInfoPayload) -> BankResponse {
    let user_id = ic_cdk::api::caller().to_string();
    
    let mut bank_info = match BankStorage::get_by_user(&user_id) {
        Some(info) => info,
        None => return BankResponse::Error("Bank information not found".to_string()),
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
        Err(e) => BankResponse::Error(format!("Failed to update bank information: {:?}", e)),
    }
}

#[query]
pub async fn get_bank_info_by_user_id(user_id: String) -> BankResponse {
    match BankStorage::get_by_user(&user_id) {
        Some(info) => BankResponse::Success(info),
        None => BankResponse::Error("Bank information not found".to_string()),
    }
}

ic_cdk::export_candid!();