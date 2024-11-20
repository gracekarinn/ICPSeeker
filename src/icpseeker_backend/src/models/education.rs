use candid::{CandidType, Decode, Encode};
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use ic_stable_structures::{Storable, BoundedStorable};
use std::borrow::Cow;

use super::user::FixedString;
use super::user::{string_to_fixed, fixed_to_string};

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum EducationLevel {
    HighSchool,
    Bachelor,
    Master,
    PhD,
    Other
}

impl From<EducationLevel> for u8 {
    fn from(level: EducationLevel) -> Self {
        match level {
            EducationLevel::HighSchool => 0,
            EducationLevel::Bachelor => 1,
            EducationLevel::Master => 2,
            EducationLevel::PhD => 3,
            EducationLevel::Other => 4,
        }
    }
}

impl From<u8> for EducationLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => EducationLevel::HighSchool,
            1 => EducationLevel::Bachelor,
            2 => EducationLevel::Master,
            3 => EducationLevel::PhD,
            _ => EducationLevel::Other,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum EducationStatus {
    InProgress,
    Completed,
    Discontinued,
    OnHold
}

impl From<EducationStatus> for u8 {
    fn from(status: EducationStatus) -> Self {
        match status {
            EducationStatus::InProgress => 0,
            EducationStatus::Completed => 1,
            EducationStatus::Discontinued => 2,
            EducationStatus::OnHold => 3,
        }
    }
}

impl From<u8> for EducationStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => EducationStatus::InProgress,
            1 => EducationStatus::Completed,
            2 => EducationStatus::Discontinued,
            _ => EducationStatus::OnHold,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EducationRecord {
    pub id: String,
    pub user_id: String,
    pub school_name: String,
    pub track: String,
    pub university_name: String,
    pub major: String,
    pub city: String,
    pub country: String,
    pub education_level: EducationLevel,
    pub status: EducationStatus,
    pub start_year: u32,
    pub end_year: Option<u32>,
    pub gpa: Option<u32>,
    pub created_at: u64,
    pub updated_at: u64,
    pub universities: Vec<UniversityEducation>,
}

#[derive(Clone, Debug)]
pub struct StableEducationRecord {
    pub id: FixedString,
    pub user_id: FixedString,
    pub school_name: FixedString,
    pub track: FixedString,
    pub university_name: FixedString,
    pub major: FixedString,
    pub city: FixedString,
    pub country: FixedString,
    pub education_level: u8,
    pub status: u8,
    pub start_year: u32,
    pub end_year: Option<u32>,
    pub gpa: Option<u32>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Storable for StableEducationRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id);
        bytes.extend_from_slice(&self.user_id);
        bytes.extend_from_slice(&self.school_name);
        bytes.extend_from_slice(&self.track);
        bytes.extend_from_slice(&self.university_name);
        bytes.extend_from_slice(&self.major);
        bytes.extend_from_slice(&self.city);
        bytes.extend_from_slice(&self.country);
        bytes.push(self.education_level);
        bytes.push(self.status);
        bytes.extend_from_slice(&self.start_year.to_be_bytes());
        
        if let Some(end_year) = self.end_year {
            bytes.push(1);
            bytes.extend_from_slice(&end_year.to_be_bytes());
        } else {
            bytes.push(0);
            bytes.extend_from_slice(&[0u8; 4]);
        }
        
        if let Some(gpa) = self.gpa {
            bytes.push(1);
            bytes.extend_from_slice(&gpa.to_be_bytes());
        } else {
            bytes.push(0);
            bytes.extend_from_slice(&[0u8; 4]);
        }

        bytes.extend_from_slice(&self.created_at.to_be_bytes());
        bytes.extend_from_slice(&self.updated_at.to_be_bytes());
        
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let mut pos = 0;
        let mut next_fixed_str = || {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&bytes[pos..pos + 32]);
            pos += 32;
            arr
        };

        let id = next_fixed_str();
        let user_id = next_fixed_str();
        let school_name = next_fixed_str();
        let track = next_fixed_str();
        let university_name = next_fixed_str();
        let major = next_fixed_str();
        let city = next_fixed_str();
        let country = next_fixed_str();
        
        let education_level = bytes[pos];
        pos += 1;
        let status = bytes[pos];
        pos += 1;
        
        let start_year = u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap());
        pos += 4;

        let end_year = if bytes[pos] == 1 {
            pos += 1;
            Some(u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap()))
        } else {
            pos += 5;
            None
        };
        
        let gpa = if bytes[pos] == 1 {
            pos += 1;
            Some(u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap()))
        } else {
            pos += 5;
            None
        };

        let created_at = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let updated_at = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());

        Self {
            id,
            user_id,
            school_name,
            track,
            university_name,
            major,
            city,
            country,
            education_level,
            status,
            start_year,
            end_year,
            gpa,
            created_at,
            updated_at,
        }
    }
}

impl BoundedStorable for StableEducationRecord {
    const MAX_SIZE: u32 = (32 * 8) + 2 + 4 + 5 + 5 + 16; 
    const IS_FIXED_SIZE: bool = true;
}

impl EducationRecord {  
    pub fn new(id: String, user_id: String) -> Self {
        let timestamp = time();
        Self {
            id,
            user_id,
            school_name: String::new(),
            track: String::new(),
            university_name: String::new(),
            major: String::new(),
            city: String::new(),
            country: String::new(),
            education_level: EducationLevel::HighSchool,
            status: EducationStatus::InProgress,
            start_year: 0,
            end_year: None,
            gpa: None,
            created_at: timestamp,
            updated_at: timestamp,
            universities: Vec::new(),
        }
    }

    pub fn add_high_school(&mut self, high_school: HighSchoolEducation) {
        self.school_name = high_school.school_name;
        self.track = high_school.track;
        self.city = high_school.city;
        self.country = high_school.country;
        self.education_level = EducationLevel::HighSchool;
        self.status = high_school.status;
        self.start_year = high_school.start_year;
        self.end_year = high_school.end_year;
        self.gpa = None;
        self.updated_at = time();
    }

    pub fn add_university(&mut self, university: UniversityEducation) {
        self.university_name = university.university_name.clone();
        self.major = university.major.clone();
        self.city = university.city.clone();
        self.country = university.country.clone();
        self.education_level = university.level.clone();
        self.status = university.status.clone();
        self.start_year = university.start_year;
        self.end_year = university.end_year;
        self.gpa = university.gpa.map(|gpa| (gpa * 100.0) as u32);
        self.universities.push(university);
        self.updated_at = time();
    }

    pub fn clear_universities(&mut self) {
        self.universities.clear();
    }
}

impl From<StableEducationRecord> for EducationRecord {
    fn from(record: StableEducationRecord) -> Self {
        Self {
            id: fixed_to_string(&record.id),
            user_id: fixed_to_string(&record.user_id),
            school_name: fixed_to_string(&record.school_name),
            track: fixed_to_string(&record.track),
            university_name: fixed_to_string(&record.university_name),
            major: fixed_to_string(&record.major),
            city: fixed_to_string(&record.city),
            country: fixed_to_string(&record.country),
            education_level: record.education_level.into(),
            status: record.status.into(),
            start_year: record.start_year,
            end_year: record.end_year,
            gpa: record.gpa,
            created_at: record.created_at,
            updated_at: record.updated_at,
            universities: Vec::new(), // Add this field
        }
    }
}

impl From<EducationRecord> for StableEducationRecord {
    fn from(record: EducationRecord) -> Self {
        Self {
            id: string_to_fixed(&record.id),
            user_id: string_to_fixed(&record.user_id),
            school_name: string_to_fixed(&record.school_name),
            track: string_to_fixed(&record.track),
            university_name: string_to_fixed(&record.university_name),
            major: string_to_fixed(&record.major),
            city: string_to_fixed(&record.city),
            country: string_to_fixed(&record.country),
            education_level: record.education_level.into(),
            status: record.status.into(),
            start_year: record.start_year,
            end_year: record.end_year,
            gpa: record.gpa,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct HighSchoolEducation {
    pub school_name: String,
    pub track: String,
    pub city: String,
    pub country: String,
    pub start_year: u32,
    pub end_year: Option<u32>,
    pub status: EducationStatus,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UniversityEducation {
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

impl HighSchoolEducation {
    pub fn new(
        school_name: String,
        track: String,
        city: String,
        country: String,
        start_year: u32,
        end_year: Option<u32>,
        status: EducationStatus,
    ) -> Self {
        Self {
            school_name,
            track,
            city,
            country,
            start_year,
            end_year,
            status,
        }
    }
}

impl UniversityEducation {
    pub fn new(
        university_name: String,
        level: EducationLevel,
        major: String,
        city: String,
        country: String,
        start_year: u32,
        end_year: Option<u32>,
        gpa: Option<f32>,
        status: EducationStatus,
    ) -> Self {
        Self {
            university_name,
            level,
            major,
            city,
            country,
            start_year,
            end_year,
            gpa,
            status,
        }
    }
}

