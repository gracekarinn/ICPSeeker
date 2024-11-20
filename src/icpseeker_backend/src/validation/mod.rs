use crate::models::user::{StableUserProfile, fixed_to_string};
use crate::storage::memory::{UserStorage, EducationStorage, BankStorage};
use crate::types::errors::StorageError;

pub struct ValidationService;

impl ValidationService {
    pub fn validate_user(user: &StableUserProfile) -> Result<(), StorageError> {
        if !fixed_to_string(&user.email).contains('@') {
            return Err(StorageError::ValidationError("Invalid email address".to_string()));
        }

        if user.phone_number.len() < 10 {
            return Err(StorageError::ValidationError("Invalid phone number".to_string()));
        }

        if user.name.is_empty() {
            return Err(StorageError::ValidationError("Name is required".to_string()));
        }

        if user.city.is_empty() || user.country.is_empty() {
            return Err(StorageError::ValidationError("Location fields are required".to_string()));
        }

        Ok(())
    }

    pub fn validate_relationships(user_id: &str) -> Result<(), StorageError> {

        let user = match UserStorage::get(user_id) {
            Some(user) => user,
            None => return Err(StorageError::NotFound("User not found".to_string())),
        };

        if let Some(edu_id) = &user.education_id {
            if EducationStorage::get(edu_id).is_none() {
                return Err(StorageError::InvalidReference(
                    "Referenced education record not found".to_string(),
                ));
            }
        }

        if let Some(bank_id) = &user.bank_info_id {
            if BankStorage::get(bank_id).is_none() {
                return Err(StorageError::InvalidReference(
                    "Referenced bank information not found".to_string(),
                ));
            }
        }

        Ok(())
    }

    pub fn validate_swift_code(code: &str) -> Result<(), StorageError> {
        if code.len() != 8 && code.len() != 11 {
            return Err(StorageError::ValidationError(
                "SWIFT code must be 8 or 11 characters".to_string(),
            ));
        }
        Ok(())
    }
}