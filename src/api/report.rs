use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportMetadata {
  pub report_id: String,
  pub timestamp: String,
  pub app_name: Option<String>,
  pub details: Option<String>,
  pub steps_to_reproduce: Option<String>,
  pub minidump_filename: Option<String>,
  pub log_files: Vec<String>,

  pub cpu: Option<super::Cpu>,
  pub os: Option<super::Os>,
  pub crash_reason: Option<String>,

  #[serde(default)]
  pub resolved: bool,
}

impl ReportMetadata {
  pub fn save_to_disk(&self, path: &Path) -> Result<()> {
    let metadata_str = serde_json::to_string_pretty(self)?;
    std::fs::write(path, metadata_str)?;
    tracing::info!("Saved metadata: {}", path.display());
    Ok(())
  }
}
