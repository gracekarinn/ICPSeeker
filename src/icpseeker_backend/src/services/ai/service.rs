use crate::models::cv::CV;
use crate::storage::memory::CVStorage;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
    TransformArgs, TransformContext, TransformFunc,
};
use crate::services::ai::{
    OpenAIMessage, OpenAIResponse, OpenAIRequest, OPENAI_API_URL, OPENAI_MODEL, AIConfig
};
use ic_cdk::api::time;
use serde_json::json;
use candid::Func;
use ic_cdk::api::call::CallResult;
use candid::Principal;
use candid::Nat;

pub struct AIService;

impl AIService {
    pub async fn generate_response(
        cv_id: &str,
        user_message: &str,
        chat_history: Vec<(String, bool)>,
    ) -> Result<String, String> {
        let cv = CVStorage::get_cv(cv_id)
            .map_err(|e| format!("Failed to get CV: {}", e))?;

        let system_message = Self::create_system_message(&cv);
        
        let mut messages = vec![OpenAIMessage {
            role: "system".to_string(),
            content: system_message,
        }];

        for (content, is_ai) in chat_history {
            messages.push(OpenAIMessage {
                role: if is_ai { "assistant" } else { "user" }.to_string(),
                content,
            });
        }

        messages.push(OpenAIMessage {
            role: "user".to_string(),
            content: user_message.to_string(),
        });

        let request = OpenAIRequest {
            model: OPENAI_MODEL.to_string(),
            messages,
            temperature: AIConfig::default().temperature,
            max_tokens: AIConfig::default().max_tokens,
        };

        let response = Self::call_openai(request).await?;

        response.choices.first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| "No response generated".to_string())
    }

    fn create_system_message(cv: &CV) -> String {
        format!(
            "You are a helpful CV assistant. You help users improve their CVs and provide career advice. \
            You have access to the user's CV with the following content:\n\n{}\n\n\
            When providing feedback or answering questions:\n\
            1. Be specific and reference actual content from the CV\n\
            2. Provide constructive criticism when needed\n\
            3. Suggest concrete improvements\n\
            4. Keep responses concise but helpful\n\
            5. Focus on professional development",
            cv.content
        )
    }

    async fn call_openai(request: OpenAIRequest) -> Result<OpenAIResponse, String> {
        let api_key = AIConfig::default().api_key;
        
        let request_headers = vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "Authorization".to_string(),
                value: format!("Bearer {}", api_key),
            },
        ];
    
        let request_body = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;
    
        let request = CanisterHttpRequestArgument {
            url: OPENAI_API_URL.to_string(),
            method: HttpMethod::POST,
            body: Some(request_body.into_bytes()),
            max_response_bytes: Some(2048),
            transform: Some(TransformContext {
                function: TransformFunc(Func {
                    method: "transform_response_query".to_string(),
                    principal: ic_cdk::id(),
                }),
                context: vec![],
            }),
            headers: request_headers,
        };
    
        match http_request(request, 0).await {
            Ok((response,)) => {
                let status = response.status.to_string();
                if status != "200" {
                    return Err(format!(
                        "API error (status {}): {}", 
                        status,
                        String::from_utf8_lossy(&response.body)
                    ));
                }
    
                serde_json::from_slice(&response.body)
                    .map_err(|e| format!("Failed to parse response: {}", e))
            }
            Err((code, msg)) => Err(format!("HTTP request failed: {:?} - {}", code, msg)),
        }
    }
}

#[ic_cdk::query]
fn transform_response_query(args: TransformArgs) -> HttpResponse {
    let mut response = args.response;
    
    response.headers.push(HttpHeader {
        name: "Access-Control-Allow-Origin".to_string(),
        value: "*".to_string(),
    });
    
    if let Ok(body_str) = String::from_utf8(response.body.clone()) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body_str) {
            return response;
        }
    }
    
    HttpResponse {
        status: Nat::from(500u32),
        headers: response.headers,
        body: json!({
            "error": "Invalid response format",
            "timestamp": time(),
        })
        .to_string()
        .into_bytes(),
    }
}

