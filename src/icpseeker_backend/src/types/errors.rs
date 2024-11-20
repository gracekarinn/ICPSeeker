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

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::NotFound(msg) => write!(f, "Not found: {}", msg),
            StorageError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            StorageError::InvalidReference(msg) => write!(f, "Invalid reference: {}", msg),
            StorageError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            StorageError::OrphanedRecord(msg) => write!(f, "Orphaned record: {}", msg),
            StorageError::SystemError(msg) => write!(f, "System error: {}", msg),
        }
    }
}