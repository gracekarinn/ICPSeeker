use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum StorageError {
    NotFound(String),
    AlreadyExists(String),
    InvalidReference(String),
    ValidationError(String),
    OrphanedRecord(String),
    SystemError(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum ValidationError {
    InvalidEmail(String),
    InvalidPhone(String),
    InvalidDate(String),
    MissingRequiredField(String),
    InvalidFormat(String),
    RelationshipError(String),
}