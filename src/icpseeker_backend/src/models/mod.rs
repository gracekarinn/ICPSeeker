pub mod types;
pub mod user;
pub mod bank;
pub mod education;
pub mod cv;

pub use types::{StorageKey, FixedString, FixedContent};
pub use types::{string_to_storage_key, string_to_fixed};
pub use user::UserProfile;
pub use bank::BankInformation;
pub use education::EducationRecord;
pub use cv::CV;