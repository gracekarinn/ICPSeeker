use candid::{CandidType, Decode, Encode};
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use ic_stable_structures::{Storable, BoundedStorable};
use std::borrow::Cow;
use super::user::{string_to_fixed, fixed_to_string};
use std::collections::VecDeque;

pub type FixedString = [u8; 32];
pub type FixedContent = [u8; 512]; 

#[derive(Clone, Debug)]
pub struct StableChatMessage {
    pub id: FixedString,
    pub content: FixedContent,
    pub is_ai: bool,
    pub timestamp: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub id: String,
    pub content: String,
    pub is_ai: bool,
    pub timestamp: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ChatSession {
    pub id: String,
    pub user_id: String,
    pub cv_id: String,
    pub created_at: u64,
    pub last_interaction: u64,
}

#[derive(Clone, Debug)]
pub struct StableChatSession {
    pub id: FixedString,           // 32 bytes
    pub user_id: FixedString,      // 32 bytes
    pub cv_id: FixedString,        // 32 bytes
    pub created_at: u64,           // 8 bytes
    pub last_interaction: u64,     // 8 bytes
}

impl BoundedStorable for StableChatSession {
    const MAX_SIZE: u32 = (32 * 3) + (8 * 2);  
    const IS_FIXED_SIZE: bool = true;
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    pub message: Option<ChatMessage>,
    pub error: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ChatHistoryResponse {
    pub messages: Vec<ChatMessage>,
    pub error: Option<String>,
}

impl Storable for StableChatMessage {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id);
        bytes.extend_from_slice(&self.content);
        bytes.push(if self.is_ai { 1 } else { 0 });
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let mut pos = 0;
        
        let mut id = [0u8; 32];
        id.copy_from_slice(&bytes[pos..pos + 32]);
        pos += 32;

        let mut content = [0u8; 512];
        content.copy_from_slice(&bytes[pos..pos + 512]);
        pos += 512;

        let is_ai = bytes[pos] == 1;
        pos += 1;

        let timestamp = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());

        Self {
            id,
            content,
            is_ai,
            timestamp,
        }
    }
}

impl BoundedStorable for StableChatMessage {
    const MAX_SIZE: u32 = 32 + 512 + 1 + 8;
    const IS_FIXED_SIZE: bool = true;
}

fn string_to_fixed_content(s: &str) -> FixedContent {
    let mut fixed = [0u8; 512];
    let bytes = s.as_bytes();
    let len = bytes.len().min(512);
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

impl From<ChatMessage> for StableChatMessage {
    fn from(msg: ChatMessage) -> Self {
        Self {
            id: string_to_fixed(&msg.id),
            content: string_to_fixed_content(&msg.content),
            is_ai: msg.is_ai,
            timestamp: msg.timestamp,
        }
    }
}

impl From<StableChatMessage> for ChatMessage {
    fn from(msg: StableChatMessage) -> Self {
        Self {
            id: fixed_to_string(&msg.id),
            content: fixed_content_to_string(&msg.content),
            is_ai: msg.is_ai,
            timestamp: msg.timestamp,
        }
    }
}

impl Storable for StableChatSession {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id);
        bytes.extend_from_slice(&self.user_id);
        bytes.extend_from_slice(&self.cv_id);
        bytes.extend_from_slice(&self.created_at.to_be_bytes());
        bytes.extend_from_slice(&self.last_interaction.to_be_bytes());
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

        let mut cv_id = [0u8; 32];
        cv_id.copy_from_slice(&bytes[pos..pos + 32]);
        pos += 32;

        let created_at = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;

        let last_interaction = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());

        Self {
            id,
            user_id,
            cv_id,
            created_at,
            last_interaction,
        }
    }
}


impl ChatSession {
    pub fn new(user_id: String, cv_id: String) -> Self {
        let timestamp = time();
        let id = format!("chat_{}_{}", user_id, cv_id);
        
        Self {
            id,
            user_id,
            cv_id,
            created_at: timestamp,
            last_interaction: timestamp,
        }
    }

    pub fn touch(&mut self) {
        self.last_interaction = time();
    }
}

impl From<StableChatSession> for ChatSession {
    fn from(session: StableChatSession) -> Self {
        Self {
            id: fixed_to_string(&session.id),
            user_id: fixed_to_string(&session.user_id),
            cv_id: fixed_to_string(&session.cv_id),
            created_at: session.created_at,
            last_interaction: session.last_interaction,
        }
    }
}

impl From<ChatSession> for StableChatSession {
    fn from(session: ChatSession) -> Self {
        Self {
            id: string_to_fixed(&session.id),
            user_id: string_to_fixed(&session.user_id),
            cv_id: string_to_fixed(&session.cv_id),
            created_at: session.created_at,
            last_interaction: session.last_interaction,
        }
    }
}
