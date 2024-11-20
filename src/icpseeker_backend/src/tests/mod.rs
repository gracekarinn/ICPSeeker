#[cfg(test)]
mod tests {
    use candid::{Decode, Encode};
    use ic_cdk::export::Principal;
    use crate::{
        create_user, get_user, update_user,
        add_education, get_education,
        CreateUserPayload, UpdateUserPayload, EducationPayload,
        models::{
            EducationLevel,
            EducationStatus,
            Location
        }
    };

    fn set_caller(principal_id: &str) {
        ic_cdk::api::testing::set_caller(
            Principal::from_text(principal_id).unwrap()
        );
    }

    #[tokio::test]
    async fn test_user_creation() {
        set_caller("2vxsx-fae");

        let payload = CreateUserPayload {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone_number: "+1234567890".to_string(),
            city: "New York".to_string(),
            country: "USA".to_string(),
        };

        let response = create_user(payload).await;
        match response {
            UserResponse::Success(user) => {
                assert_eq!(user.name, "John Doe");
                assert_eq!(user.email, "john@example.com");
                assert_eq!(user.location.city, "New York");
            },
            UserResponse::Error(e) => panic!("User creation failed: {}", e),
        }

        let get_response = get_user().await;
        match get_response {
            UserResponse::Success(user) => {
                assert_eq!(user.name, "John Doe");
                assert_eq!(user.profile_completion, 20); // Basic info only
            },
            UserResponse::Error(e) => panic!("User retrieval failed: {}", e),
        }
    }

    #[tokio::test]
    async fn test_user_update() {
        set_caller("2vxsx-fae");

        let create_payload = CreateUserPayload {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone_number: "+1234567890".to_string(),
            city: "New York".to_string(),
            country: "USA".to_string(),
        };
        let _ = create_user(create_payload).await;

        let update_payload = UpdateUserPayload {
            name: Some("John Smith".to_string()),
            email: Some("john.smith@example.com".to_string()),
            phone_number: None,
            city: Some("Los Angeles".to_string()),
            country: None,
        };

        let update_response = update_user(update_payload).await;
        match update_response {
            UserResponse::Success(user) => {
                assert_eq!(user.name, "John Smith");
                assert_eq!(user.email, "john.smith@example.com");
                assert_eq!(user.location.city, "Los Angeles");
                assert_eq!(user.location.country, "USA"); // Unchanged
                assert_eq!(user.phone_number, "+1234567890"); // Unchanged
            },
            UserResponse::Error(e) => panic!("User update failed: {}", e),
        }
    }

    #[tokio::test]
    async fn test_education_management() {
        set_caller("2vxsx-fae");

        let create_payload = CreateUserPayload {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone_number: "+1234567890".to_string(),
            city: "New York".to_string(),
            country: "USA".to_string(),
        };
        let _ = create_user(create_payload).await;

        let education_payload = EducationPayload {
            high_school: Some(HighSchoolPayload {
                school_name: "City High School".to_string(),
                track: "Science".to_string(),
                city: "New York".to_string(),
                country: "USA".to_string(),
                start_year: 2015,
                end_year: Some(2018),
                status: EducationStatus::Graduated,
            }),
            university: Some(vec![UniversityPayload {
                university_name: "State University".to_string(),
                level: EducationLevel::Bachelor,
                major: "Computer Science".to_string(),
                city: "Boston".to_string(),
                country: "USA".to_string(),
                start_year: 2018,
                end_year: Some(2022),
                gpa: Some(3.8),
                status: EducationStatus::Graduated,
            }]),
        };

        let edu_response = add_education(education_payload).await;
        match edu_response {
            EducationResponse::Success(record) => {
                assert!(record.high_school.is_some());
                assert_eq!(record.university.len(), 1);
                
                let hs = record.high_school.unwrap();
                assert_eq!(hs.school_name, "City High School");
                assert_eq!(hs.track, "Science");

                let uni = &record.university[0];
                assert_eq!(uni.university_name, "State University");
                assert_eq!(uni.major, "Computer Science");
            },
            EducationResponse::Error(e) => panic!("Education addition failed: {}", e),
        }

        let get_edu_response = get_education().await;
        match get_edu_response {
            EducationResponse::Success(record) => {
                assert!(record.high_school.is_some());
                assert_eq!(record.university.len(), 1);
            },
            EducationResponse::Error(e) => panic!("Education retrieval failed: {}", e),
        }

        let user_response = get_user().await;
        match user_response {
            UserResponse::Success(user) => {
                assert!(user.education_id.is_some());
                assert!(user.profile_completion > 20); 
            },
            UserResponse::Error(e) => panic!("User retrieval failed: {}", e),
        }
    }
}