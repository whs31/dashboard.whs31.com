use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
  pub success: bool,
  pub message: String,
  pub report_id: Option<String>,
}