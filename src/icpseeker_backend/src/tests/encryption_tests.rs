#[cfg(test)]
mod encryption_tests {
    use crate::{
        encryption::{Encryption, EncryptedData, EncryptedBankInformation},
        add_bank_info, get_bank_info, update_bank_info,
        BankInfoPayload, BankResponse,
        create_user, CreateUserPayload,
    };
    use ic_cdk::export::Principal;

    fn set_caller(principal_id: &str) {
        ic_cdk::api::testing::set_caller(
            Principal::from_text(principal_id).unwrap()
        );
    }

    async fn create_test_user() -> String {
        let user_payload = CreateUserPayload {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            phone_number: "+1234567890".to_string(),
            city: "Test City".to_string(),
            country: "Test Country".to_string(),
        };
        let _ = create_user(user_payload).await;
        ic_cdk::api::caller().to_string()
    }

    #[tokio::test]
    async fn test_encryption_key_generation() {
        set_caller("2vxsx-fae");
        let user_id = create_test_user().await;

        let key_result = Encryption::generate_key(&user_id).await;
        assert!(key_result.is_ok(), "Key generation should succeed");

        let second_key_result = Encryption::generate_key(&user_id).await;
        assert!(second_key_result.is_ok(), "Second key generation should succeed");
    }

    #[tokio::test]
    async fn test_data_encryption_decryption() {
        set_caller("2vxsx-fae");
        let user_id = create_test_user().await;
        
        let _ = Encryption::generate_key(&user_id).await;

        let sensitive_data = "1234-5678-9012-3456";

        let encrypted = Encryption::encrypt(sensitive_data, &user_id)
            .expect("Encryption should succeed");
        
        assert!(!encrypted.data.is_empty(), "Encrypted data should not be empty");
        assert!(!encrypted.nonce.is_empty(), "Nonce should not be empty");
        
        let decrypted = Encryption::decrypt(&encrypted, &user_id)
            .expect("Decryption should succeed");
        
        assert_eq!(decrypted, sensitive_data, "Decrypted data should match original");
    }

    #[tokio::test]
    async fn test_encrypted_bank_information() {
        set_caller("2vxsx-fae");
        let user_id = create_test_user().await;

        let bank_payload = BankInfoPayload {
            account_holder_name: "Test User".to_string(),
            bank_name: "Test Bank".to_string(),
            swift_code: "TESTSWIFT".to_string(),
            account_number: "1234567890".to_string(),
            bank_country: "Test Country".to_string(),
            bank_branch: Some("Main Branch".to_string()),
        };

        let add_response = add_bank_info(bank_payload).await;
        match add_response {
            BankResponse::Success(bank_info) => {
                assert!(bank_info.get_swift_code().is_ok());
                assert!(bank_info.get_account_number().is_ok());
                
                assert_eq!(bank_info.get_swift_code().unwrap(), "TESTSWIFT");
                assert_eq!(bank_info.get_account_number().unwrap(), "1234567890");
            },
            BankResponse::Error(e) => panic!("Failed to add bank information: {}", e),
        }
    }

    #[tokio::test]
    async fn test_encrypted_bank_info_update() {
        set_caller("2vxsx-fae");
        let user_id = create_test_user().await;

        let initial_payload = BankInfoPayload {
            account_holder_name: "Test User".to_string(),
            bank_name: "Test Bank".to_string(),
            swift_code: "TESTSWIFT".to_string(),
            account_number: "1234567890".to_string(),
            bank_country: "Test Country".to_string(),
            bank_branch: Some("Main Branch".to_string()),
        };

        let _ = add_bank_info(initial_payload).await;

        let update_payload = BankInfoPayload {
            account_holder_name: "Test User Updated".to_string(),
            bank_name: "Test Bank".to_string(),
            swift_code: "NEWSWIFT".to_string(),
            account_number: "0987654321".to_string(),
            bank_country: "Test Country".to_string(),
            bank_branch: Some("New Branch".to_string()),
        };

        let update_response = update_bank_info(update_payload).await;
        match update_response {
            BankResponse::Success(updated_info) => {
                assert_eq!(updated_info.get_swift_code().unwrap(), "NEWSWIFT");
                assert_eq!(updated_info.get_account_number().unwrap(), "0987654321");
            },
            BankResponse::Error(e) => panic!("Failed to update bank information: {}", e),
        }
    }

    #[tokio::test]
    async fn test_encryption_data_integrity() {
        set_caller("2vxsx-fae");
        let user_id = create_test_user().await;

        let _ = Encryption::generate_key(&user_id).await;
        let sensitive_data = "SENSITIVE123";
        let encrypted = Encryption::encrypt(sensitive_data, &user_id).unwrap();

        let mut tampered_data = encrypted.clone();
        if let Some(first_byte) = tampered_data.data.get_mut(0) {
            *first_byte = first_byte.wrapping_add(1);
        }

        let tampered_decrypted = Encryption::decrypt(&tampered_data, &user_id).unwrap();
        assert_ne!(tampered_decrypted, sensitive_data, 
            "Tampered data should not decrypt to original value");

        let original_decrypted = Encryption::decrypt(&encrypted, &user_id).unwrap();
        assert_eq!(original_decrypted, sensitive_data, 
            "Original data should decrypt correctly");
    }
}