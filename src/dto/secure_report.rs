use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureReportDto {
    pub link: String,
    pub note: String,
}
