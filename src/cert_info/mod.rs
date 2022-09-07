use chrono::Local;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
pub enum CertificateStatus {
    Valid,
    SoonInvalid,
    Invalid,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CertificateData {
    pub domain: String,
    pub status: CertificateStatus,
    pub time_stamp: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub expire_in: Option<i32>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub expired_for: Option<i32>,
}

impl CertificateData {
    pub fn new(domain: &String, expire:i32, status: CertificateStatus) -> CertificateData {
        CertificateData {
            domain: domain.to_string(),
            status: status.clone(),
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