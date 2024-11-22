use crate::storage::memory::{ChatStorage, ChatSessionStorage, APIUsageStorage};
use crate::CVStorage;
use crate::services::ai::AIService; 
pub use crate::models::chat::{ChatSession, ChatResponse, ChatHistoryResponse};
use ic_cdk::api::time;
use crate::models::ChatMessage;

pub struct ChatService;

impl ChatService {
    pub async fn start_chat(user_id: &str, cv_id: &str) -> ChatResponse {
        if let Err(_) = APIUsageStorage::check_and_update_limit(user_id) {
            return ChatResponse {
                message: None,
                error: Some("Rate limit exceeded".to_string()),
            };
        }

        match CVStorage::get_cv(cv_id) {
            Ok(cv) if cv.user_id != user_id => {
                return ChatResponse {
                    message: None,
                    error: Some("Access denied to this CV".to_string()),
                };
            }
            Err(_) => {
                return ChatResponse {
                    message: None,
                    error: Some("CV not found".to_string()),
                };
            }
            Ok(_) => {}
        }

        let session = match ChatSessionStorage::create_session(user_id, cv_id) {
            Ok(session) => session,
            Err(e) => {
                return ChatResponse {
                    message: None,
                    error: Some(format!("Failed to create chat session: {:?}", e)),
                };
            }
        };

        let welcome_message = ChatMessage {
            id: format!("msg_{}_welcome", session.id),
            content: "Hello! I'm your CV assistant. I've analyzed your CV and I'm here to help. What would you like to know?".to_string(),
            is_ai: true,
            timestamp: time(),
        };

        if let Err(e) = ChatStorage::store_message(welcome_message.clone()) {
            return ChatResponse {
                message: None,
                error: Some(format!("Failed to store welcome message: {}", e)),
            };
        }

        ChatResponse {
            message: Some(welcome_message),
            error: None,
        }
    }

    pub async fn send_message(
        session_id: &str,
        user_id: &str,
        content: String,
    ) -> ChatResponse {
        if let Err(_) = APIUsageStorage::check_and_update_limit(user_id) {
            return ChatResponse {
                message: None,
                error: Some("Rate limit exceeded".to_string()),
            };
        }

        let mut session = match ChatSessionStorage::get_session(session_id) {
            Ok(session) => {
                if session.user_id != user_id {
                    return ChatResponse {
                        message: None,
                        error: Some("Access denied to this chat session".to_string()),
                    };
                }
                session
            }
            Err(_) => {
                return ChatResponse {
                    message: None,
                    error: Some("Chat session not found".to_string()),
                };
            }
        };

        let user_message = ChatMessage {
            id: format!("msg_{}_{}", session_id, time()),
            content: content.clone(),
            is_ai: false,
            timestamp: time(),
        };

        if let Err(e) = ChatStorage::store_message(user_message) {
            return ChatResponse {
                message: None,
                error: Some(format!("Failed to store message: {}", e)),
            };
        }

        session.touch();
        if let Err(e) = ChatSessionStorage::update_session(session.clone()) {
            return ChatResponse {
                message: None,
                error: Some(format!("Failed to update session: {:?}", e)),
            };
        }

        let history = match ChatStorage::get_session_messages(session_id) {
            Ok(messages) => messages
                .iter()
                .map(|msg| (msg.content.clone(), msg.is_ai))
                .collect(),
            Err(_) => Vec::new(),
        };

        let ai_content = match AIService::generate_response(&session.cv_id, &content, history).await {
            Ok(response) => response,
            Err(e) => {
                return ChatResponse {
                    message: None,
                    error: Some(format!("Failed to generate AI response: {}", e)),
                };
            }
        };

        let ai_response = ChatMessage {
            id: format!("msg_{}_{}_ai", session_id, time()),
            content: ai_content,
            is_ai: true,
            timestamp: time(),
        };

        if let Err(e) = ChatStorage::store_message(ai_response.clone()) {
            return ChatResponse {
                message: None,
                error: Some(format!("Failed to store AI response: {}", e)),
            };
        }

        ChatResponse {
            message: Some(ai_response),
            error: None,
        }
    }

    pub fn get_chat_history(session_id: &str, user_id: &str) -> ChatHistoryResponse {
        match ChatSessionStorage::get_session(session_id) {
            Ok(session) if session.user_id != user_id => {
                return ChatHistoryResponse {
                    messages: vec![],
                    error: Some("Access denied to this chat session".to_string()),
                };
            }
            Err(_) => {
                return ChatHistoryResponse {
                    messages: vec![],
                    error: Some("Chat session not found".to_string()),
                };
            }
            Ok(_) => {}
        }

        match ChatStorage::get_session_messages(session_id) {
            Ok(messages) => ChatHistoryResponse {
                messages,
                error: None,
            },
            Err(e) => ChatHistoryResponse {
                messages: vec![],
                error: Some(format!("Failed to retrieve chat history: {}", e)),
            },
        }
    }
}