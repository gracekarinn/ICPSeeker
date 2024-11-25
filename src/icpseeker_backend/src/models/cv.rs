use candid::{CandidType, Decode, Encode};
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use std::borrow::Cow;
use ic_stable_structures::{Storable, BoundedStorable};
use super::user::{string_to_fixed, fixed_to_string};

pub type StorageKey = [u8; 32];  
pub type FixedString = [u8; 32];
pub type FixedContent400 = [u8; 400]; 
pub type FixedContent32 = [u8; 32];

#[derive(Clone, Debug)]
pub struct StableCV {
    pub id: StorageKey,
    pub user_id: StorageKey,
    pub title: FixedString,
    pub content: FixedContent400,
    pub version: u32,
    pub ai_analysis_status: u8,
    pub ai_feedback: FixedContent32,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CV {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub version: u32,
    pub ai_analysis_status: CVAnalysisStatus,
    pub ai_feedback: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CVAnalysisStatus {
    NotAnalyzed,
    InProgress,
    Completed,
}

impl Storable for StableCV {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id);
        bytes.extend_from_slice(&self.user_id);
        bytes.extend_from_slice(&self.title);
        bytes.extend_from_slice(&self.content);
        bytes.extend_from_slice(&self.version.to_be_bytes());
        bytes.push(self.ai_analysis_status);
        bytes.extend_from_slice(&self.ai_feedback);
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let mut pos = 0;
        
        let mut id = [0u8; 32];
        id.copy_from_slice(&bytes[pos..pos + 32]);
        pos += 32;

        let mut user_id = [0u8; 32];
        user_id.copy_from_slice(&bytes[pos..pos + 32]);
        pos += 32;

        let mut title = [0u8; 32];
        title.copy_from_slice(&bytes[pos..pos + 32]);
        pos += 32;

        let mut content = [0u8; 400];
        content.copy_from_slice(&bytes[pos..pos + 400]);
        pos += 400;
        
        let version = u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap());
        pos += 4;

        let ai_analysis_status = bytes[pos];
        pos += 1;

        let mut ai_feedback = [0u8; 32];
        ai_feedback.copy_from_slice(&bytes[pos..pos + 32]);

        Self {
            id,
            user_id,
            title,
            content,
            version,
            ai_analysis_status,
            ai_feedback,
        }
    }
}

impl BoundedStorable for StableCV {
    const MAX_SIZE: u32 = 32 * 3 + 400 + 4 + 1 + 32;
    const IS_FIXED_SIZE: bool = true;
}

impl From<CV> for StableCV {
    fn from(cv: CV) -> Self {
        Self {
            id: string_to_fixed(&cv.id),
            user_id: string_to_fixed(&cv.user_id),
            title: string_to_fixed(&cv.title),
            content: string_to_fixed_content_400(&cv.content),
            version: cv.version,
            ai_analysis_status: match cv.ai_analysis_status {
                CVAnalysisStatus::NotAnalyzed => 0,
                CVAnalysisStatus::InProgress => 1,
                CVAnalysisStatus::Completed => 2,
            },
            ai_feedback: string_to_fixed_content_32(&cv.ai_feedback.unwrap_or_default()),
        }
    }
}


impl CV {
    pub fn new(
        id: String,
        user_id: String,
        title: String,
        content: String,
    ) -> Self {
        let timestamp = time();
        Self {
            id,
            user_id,
            title,
            content,
            version: 1,
            ai_analysis_status: CVAnalysisStatus::NotAnalyzed,
            ai_feedback: None,
        }
    }
}

impl From<StableCV> for CV {
    fn from(cv: StableCV) -> Self {
        Self {
            id: fixed_to_string(&cv.id),
            user_id: fixed_to_string(&cv.user_id),
            title: fixed_to_string(&cv.title),
            content: fixed_content_to_string_400(&cv.content),
            version: cv.version,
            ai_analysis_status: match cv.ai_analysis_status {
                0 => CVAnalysisStatus::NotAnalyzed,
                1 => CVAnalysisStatus::InProgress,
                _ => CVAnalysisStatus::Completed,
            },
            ai_feedback: Some(fixed_content_to_string_32(&cv.ai_feedback)),
        }
    }
}

fn string_to_fixed_content_400(s: &str) -> FixedContent400 {
    let mut fixed = [0u8; 400];
    let bytes = s.as_bytes();
    let len = bytes.len().min(400);
    fixed[..len].copy_from_slice(&bytes[..len]);
    fixed
}

fn string_to_fixed_content_32(s: &str) -> FixedContent32 {
    let mut fixed = [0u8; 32];
    let bytes = s.as_bytes();
    let len = bytes.len().min(32);
    fixed[..len].copy_from_slice(&bytes[..len]);
    fixed
}

fn fixed_content_to_string_400(fixed: &FixedContent400) -> String {
    String::from_utf8(
        fixed.iter()
            .take_while(|&&x| x != 0)
            .copied()
            .collect()
    ).unwrap_or_default()
}

fn fixed_content_to_string_32(fixed: &FixedContent32) -> String {
    String::from_utf8(
        fixed.iter()
            .take_while(|&&x| x != 0)
            .copied()
            .collect()
    ).unwrap_or_default()
}
