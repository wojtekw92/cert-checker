use chrono::Local;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum CertyficateStatus {
    Valid,
    SoonInvalid,
    Invalid,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CertyficateData {
    pub domain: String,
    pub status: CertyficateStatus,
    pub time_stamp: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub expire_in: Option<i32>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub expired_for: Option<i32>,
}
impl CertyficateData {
    pub fn new(domain: &String, expire:i32, status: CertyficateStatus) -> CertyficateData {
        CertyficateData {
            domain: domain.to_string(),
            status: status,
            time_stamp: Local::now().to_rfc3339(),
            expire_in: match expire {
                x if x > 0 => Some(x),
                _ => None
            },
            expired_for: match expire {
                x if x <= 0 => Some(-x), 
                _ => None
            }
        }
    } 
}