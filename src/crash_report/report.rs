use anyhow::{Context, Result};
use std::path::PathBuf;
use tokio::{fs, io::AsyncWriteExt};

#[derive(Debug)]
pub struct CrashReport {
  pub report_id: uuid::Uuid,
  pub timestamp: chrono::DateTime<chrono::Local>,
  pub app_name: Option<String>,
  pub details: Option<String>,
  pub steps_to_reproduce: Option<String>,
  pub minidump: Option<(String, Vec<u8>)>,
  pub log_files: Vec<(String, Vec<u8>)>,

  pub cpu: Option<crate::api::Cpu>,
  pub os: Option<crate::api::Os>,
  pub crash_reason: Option<crate::api::CrashReason>,
}

impl CrashReport {
  pub fn new() -> Self {
    let report_id = uuid::Uuid::new_v4();
    let timestamp = chrono::Local::now();

    Self {
      report_id,
      timestamp,
      app_name: None,
      details: None,
      steps_to_reproduce: None,
      minidump: None,
      log_files: Vec::new(),
      cpu: None,
      os: None,
      crash_reason: None,
    }
  }

  #[inline]
  pub fn report_dir(&self) -> PathBuf {
    PathBuf::from("crash_reports").join(format!("{}", self.report_id))
  }

  #[inline]
  pub fn metadata(&self) -> crate::api::ReportMetadata {
    crate::api::ReportMetadata {
      report_id: self.report_id.to_string(),
      timestamp: self.timestamp.to_rfc3339(),
      app_name: self.app_name.clone(),
      details: self.details.clone(),
      steps_to_reproduce: self.steps_to_reproduce.clone(),
      minidump_filename: self.minidump.as_ref().map(|(name, _)| name.clone()),
      log_files: self
        .log_files
        .iter()
        .map(|(name, _)| name.clone())
        .collect(),
      cpu: self.cpu,
      os: self.os,
      crash_reason: self.crash_reason.as_ref().map(|c| c.to_string()),
      resolved: false,
    }
  }

  fn parse_minidump(&mut self) -> Result<()> {
    if self.minidump.is_none() {
      return Ok(());
    }

    let minidump = minidump::Minidump::read(self.minidump.as_ref().unwrap().1.as_slice())?;
    let system_info = minidump.get_stream::<minidump::MinidumpSystemInfo>()?;
    let exception = minidump.get_stream::<minidump::MinidumpException>()?;
    let crash_reason = exception.get_crash_reason(system_info.os, system_info.cpu);

    self.cpu = Some(system_info.cpu.into());
    self.os = Some(system_info.os.into());
    self.crash_reason = Some(crash_reason.into());

    Ok(())
  }

  pub async fn save_to_disk(&mut self) -> Result<()> {
    let report_dir = self.report_dir();
    fs::create_dir_all(&report_dir)
      .await
      .context("Failed to create report directory")?;

    if let Some((filename, data)) = &self.minidump {
      let minidump_path = report_dir.join(filename);
      let mut file = fs::File::create(&minidump_path)
        .await
        .context("Failed to create minidump file")?;
      file
        .write_all(data)
        .await
        .context("Failed to write minidump data")?;
      tracing::info!("Saved minidump: {}", minidump_path.display());
    }
    self.parse_minidump()?;

    for (filename, data) in &self.log_files {
      let log_path = report_dir.join(filename);
      let mut file = fs::File::create(&log_path)
        .await
        .context("Failed to create log file")?;
      file
        .write_all(data)
        .await
        .context("Failed to write log data")?;
      tracing::info!("Saved log file: {}", log_path.display());
    }

    self
      .metadata()
      .save_to_disk(&report_dir.join("metadata.json"))?;

    Ok(())
  }
}
