#[cfg(test)]
mod e2e_tests {
    use crate::{
        create_user, get_user, update_user,
        CreateUserPayload, UpdateUserPayload,
        add_education, get_education,
        EducationPayload, HighSchoolPayload, UniversityPayload,
        add_bank_info, get_bank_info,
        BankInfoPayload,
        models::{
            EducationLevel, EducationStatus,
            UserResponse, EducationResponse, BankResponse
        }
    };
    use ic_cdk::export::Principal;

    fn set_caller(principal_id: &str) {
        ic_cdk::api::testing::set_caller(
            Principal::from_text(principal_id).unwrap()
        );
    }

    #[tokio::test]
    async fn test_complete_user_journey() {
        set_caller("2vxsx-fae");

        let user_payload = CreateUserPayload {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            phone_number: "+1234567890".to_string(),
            city: "New York".to_string(),
            country: "USA".to_string(),
        };

        let user_response = create_user(user_payload).await;
        let user = match user_response {
            UserResponse::Success(u) => {
                assert_eq!(u.name, "John Doe");
                assert_eq!(u.profile_completion, 20); // Basic info only
                u
            },
            UserResponse::Error(e) => panic!("User creation failed: {}", e),
        };

        let education_payload = EducationPayload {
            high_school: Some(HighSchoolPayload {
                school_name: "Central High".to_string(),
                track: "Science".to_string(),
                city: "New York".to_string(),
                country: "USA".to_string(),
                start_year: 2015,
                end_year: Some(2018),
                status: EducationStatus::Graduated,
            }),
            university: Some(vec![
                UniversityPayload {
                    university_name: "State University".to_string(),
                    level: EducationLevel::Bachelor,
                    major: "Computer Science".to_string(),
                    city: "Boston".to_string(),
                    country: "USA".to_string(),
                    start_year: 2018,
                    end_year: Some(2022),
                    gpa: Some(3.8),
                    status: EducationStatus::Graduated,
                },
                UniversityPayload {
                    university_name: "Tech University".to_string(),
                    level: EducationLevel::Master,
                    major: "Data Science".to_string(),
                    city: "San Francisco".to_string(),
                    country: "USA".to_string(),
                    start_year: 2022,
                    end_year: None,
                    gpa: None,
                    status: EducationStatus::Ongoing,
                }
            ]),
        };

        let edu_response = add_education(education_payload).await;
        match edu_response {
            EducationResponse::Success(record) => {
                assert!(record.high_school.is_some());
                assert_eq!(record.university.len(), 2);
                assert_eq!(record.university[0].major, "Computer Science");
                assert_eq!(record.university[1].major, "Data Science");
            },
            EducationResponse::Error(e) => panic!("Education addition failed: {}", e),
        }

        let updated_user = match get_user().await {
            UserResponse::Success(u) => {
                assert!(u.profile_completion >= 60); 
                assert!(u.education_id.is_some());
                u
            },
            UserResponse::Error(e) => panic!("User retrieval failed: {}", e),
        };

        let bank_payload = BankInfoPayload {
            account_holder_name: "John Doe".to_string(),
            bank_name: "International Bank".to_string(),
            swift_code: "INTLBANK123".to_string(),
            account_number: "1234567890123456".to_string(),
            bank_country: "USA".to_string(),
            bank_branch: Some("Manhattan Branch".to_string()),
        };

        let bank_response = add_bank_info(bank_payload).await;
        match bank_response {
            BankResponse::Success(info) => {
                assert_eq!(info.get_swift_code().unwrap(), "INTLBANK123");
                assert_eq!(info.get_account_number().unwrap(), "1234567890123456");
            },
            BankResponse::Error(e) => panic!("Bank info addition failed: {}", e),
        };

        let final_user = match get_user().await {
            UserResponse::Success(u) => {
                assert_eq!(u.profile_completion, 100); 
                assert!(u.education_id.is_some());
                assert!(u.bank_info_id.is_some());
                u
            },
            UserResponse::Error(e) => panic!("Final user retrieval failed: {}", e),
        };

        let final_education = match get_education().await {
            EducationResponse::Success(record) => {
                assert!(record.high_school.is_some());
                assert_eq!(record.university.len(), 2);
                record
            },
            EducationResponse::Error(e) => panic!("Education retrieval failed: {}", e),
        };

        let final_bank_info = match get_bank_info().await {
            BankResponse::Success(info) => {
                assert_eq!(info.bank_name, "International Bank");
                assert_eq!(info.bank_country, "USA");
                info
            },
            BankResponse::Error(e) => panic!("Bank info retrieval failed: {}", e),
        };

        assert_eq!(final_user.education_id.unwrap(), final_education.id);
        assert_eq!(final_user.bank_info_id.unwrap(), final_bank_info.id);
    }
}