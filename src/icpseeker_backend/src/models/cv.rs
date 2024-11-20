use candid::{CandidType, Decode, Encode};
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use std::borrow::Cow;
use ic_stable_structures::{Storable, BoundedStorable};

pub type FixedString = [u8; 32];
pub type FixedContent = [u8; 1024]; 

#[derive(Clone, Debug)]
pub struct StableCV {
    pub id: FixedString,
    pub user_id: FixedString,
    pub title: FixedString,
    pub content: FixedContent,
    pub version: u32,
    pub created_at: u64,
    pub updated_at: u64,
    pub ai_analysis_status: u8, // 0: Not analyzed, 1: In Progress, 2: Completed
    pub ai_feedback: FixedContent,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CV {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub version: u32,
    pub created_at: u64,
    pub updated_at: u64,
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
        bytes.extend_from_slice(&self.created_at.to_be_bytes());
        bytes.extend_from_slice(&self.updated_at.to_be_bytes());
        bytes.push(self.ai_analysis_status);
        bytes.extend_from_slice(&self.ai_feedback);
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

        let mut next_fixed_content = || {
            let mut arr = [0u8; 1024];
            arr.copy_from_slice(&bytes[pos..pos + 1024]);
            pos += 1024;
            arr
        };

        let id = next_fixed_str();
        let user_id = next_fixed_str();
        let title = next_fixed_str();
        let content = next_fixed_content();
        
        let version = u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap());
        pos += 4;
        
        let created_at = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        
        let updated_at = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        
        let ai_analysis_status = bytes[pos];
        pos += 1;
        
        let ai_feedback = next_fixed_content();

        Self {
            id,
            user_id,
            title,
            content,
            version,
            created_at,
            updated_at,
            ai_analysis_status,
            ai_feedback,
        }
    }
}

impl BoundedStorable for StableCV {
    const MAX_SIZE: u32 = 32 * 3 + 1024 * 2 + 4 + 8 + 8 + 1;
    const IS_FIXED_SIZE: bool = true;
}

impl From<CV> for StableCV {
    fn from(cv: CV) -> Self {
        Self {
            id: string_to_fixed(&cv.id),
            user_id: string_to_fixed(&cv.user_id),
            title: string_to_fixed(&cv.title),
            content: string_to_fixed_content(&cv.content),
            version: cv.version,
            created_at: cv.created_at,
            updated_at: cv.updated_at,
            ai_analysis_status: match cv.ai_analysis_status {
                CVAnalysisStatus::NotAnalyzed => 0,
                CVAnalysisStatus::InProgress => 1,
                CVAnalysisStatus::Completed => 2,
            },
            ai_feedback: string_to_fixed_content(&cv.ai_feedback.unwrap_or_default()),
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
            created_at: timestamp,
            updated_at: timestamp,
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
            content: fixed_content_to_string(&cv.content),
            version: cv.version,
            created_at: cv.created_at,
            updated_at: cv.updated_at,
            ai_analysis_status: match cv.ai_analysis_status {
                0 => CVAnalysisStatus::NotAnalyzed,
                1 => CVAnalysisStatus::InProgress,
                _ => CVAnalysisStatus::Completed,
            },
            ai_feedback: Some(fixed_content_to_string(&cv.ai_feedback)),
        }
    }
}

fn string_to_fixed_content(s: &str) -> FixedContent {
    let mut fixed = [0u8; 1024];
    let bytes = s.as_bytes();
    let len = bytes.len().min(1024);
    fixed[..len].copy_from_slice(&bytes[..len]);
    fixed
}

fn fixed_content_to_string(fixed: &FixedContent) -> String {
    String::from_utf8(
        fixed.iter()
            .take_while(|&&x| x != 0)
            .copied()
            .collect()
    ).unwrap_or_default()
}