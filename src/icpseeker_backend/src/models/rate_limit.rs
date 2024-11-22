use candid::{CandidType, Decode, Encode};
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use std::borrow::Cow;
use ic_stable_structures::{Storable, BoundedStorable};
use crate::models::FixedString;
 

#[derive(Clone, Debug)]
pub struct StableUserAPIUsage {
    pub user_id: FixedString,
    pub daily_requests: u32,
    pub last_reset: u64,
    pub total_requests: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserAPIUsage {
    pub user_id: String,
    pub daily_requests: u32,
    pub last_reset: u64,
    pub total_requests: u64,
}

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub daily_limit: u32,
    pub reset_interval_nanos: u64,
}


impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            daily_limit: 50,  
            reset_interval_nanos: 24 * 60 * 60 * 1_000_000_000, 
        }
    }
}