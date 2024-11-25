use candid::{CandidType, Deserialize, Principal}; 
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::api::time;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Session {
    pub principal_id: Principal,
    pub expires_at: u64,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct AuthenticatedUser {
    pub principal: Principal,
    pub user_id: String,
}

thread_local! {
    static SESSIONS: RefCell<HashMap<Principal, Session>> = RefCell::new(HashMap::new());
    
    static USER_PRINCIPALS: RefCell<HashMap<Principal, String>> = RefCell::new(HashMap::new());
}

const SESSION_DURATION_NANOS: u64 = 24 * 60 * 60 * 1_000_000_000; 

pub struct AuthService;

impl AuthService {
    pub fn create_session(principal: Principal) -> Session {
        let session = Session {
            principal_id: principal,  
            expires_at: time() + SESSION_DURATION_NANOS,
        };

        SESSIONS.with(|sessions| {
            sessions.borrow_mut().insert(principal, session.clone());
        });

        session
    }
    
    pub fn validate_session(principal: &Principal) -> bool {
        SESSIONS.with(|sessions| {
            if let Some(session) = sessions.borrow().get(principal) {
                if session.expires_at > time() {
                    true
                } else {
                    sessions.borrow_mut().remove(principal);
                    false
                }
            } else {
                false
            }
        })
    }

    pub fn get_user_id(principal: &Principal) -> Option<String> {
        USER_PRINCIPALS.with(|principals| {
            principals.borrow().get(principal).cloned()
        })
    }

    pub fn associate_user_principal(principal: Principal, user_id: String) {
        USER_PRINCIPALS.with(|principals| {
            principals.borrow_mut().insert(principal, user_id);
        });
    }

    pub fn is_authenticated(principal: &Principal) -> bool {
        if principal == &Principal::anonymous() {
            return false;
        }
        Self::validate_session(principal)
    }
}
