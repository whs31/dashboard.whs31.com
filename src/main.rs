use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::{Context, Result};
use axum::routing::delete;
use axum::{
  extract::{Multipart, Path},
  http::{header, StatusCode},
  response::{IntoResponse, Response},
  routing::{get, head, post},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{error, info, warn};

mod api;
mod crash_report;

pub use self::crash_report::CrashReport;

#[derive(Debug)]
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    error!("Request error: {:?}", self.0);
    let response = api::UploadResponse {
      success: false,
      message: format!("Internal server error: {}", self.0),
      report_id: None,
    };
    (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
  }
}

impl<E> From<E> for AppError
where
  E: Into<anyhow::Error>,
{
  fn from(err: E) -> Self {
    Self(err.into())
  }
}

async fn health_check() -> impl IntoResponse {
  StatusCode::OK
}

async fn upload_crash_report(mut multipart: Multipart) -> Result<Response, AppError> {
  info!("Received crash report upload request");

  let mut report = CrashReport::new();

  while let Some(field) = multipart.next_field().await? {
    let name = field.name().unwrap_or("").to_string();
    let filename = field.file_name().map(|s| s.to_string());

    match name.as_str() {
      "minidump" => {
        let filename = filename.unwrap_or_else(|| "minidump.dmp".to_string());
        let data = field.bytes().await?;
        info!("Received minidump: {} ({} bytes)", filename, data.len());
        report.minidump = Some((filename, data.to_vec()));
      }
      "app_name" => {
        let text = field.text().await?;
        info!("App name: {}", text);
        report.app_name = Some(text);
      }
      "details" => {
        let text = field.text().await?;
        info!("Details: {}", text);
        report.details = Some(text);
      }
      "steps_to_reproduce" => {
        let text = field.text().await?;
        info!("Steps to reproduce: {}", text);
        report.steps_to_reproduce = Some(text);
      }
      field_name if field_name.starts_with("logfile_") => {
        if let Some(fname) = filename {
          let data = field.bytes().await?;
          info!("Received log file: {} ({} bytes)", fname, data.len());
          report.log_files.push((fname, data.to_vec()));
        } else {
          let _ = field.bytes().await?;
        }
      }
      _ => {
        warn!("Unknown field: {}", name);
        let _ = field.bytes().await?;
      }
    }
  }

  if report.minidump.is_none() {
    warn!("No minidump received in crash report");
    let response = api::UploadResponse {
      success: false,
      message: "No minidump file provided".to_string(),
      report_id: None,
    };
    return Ok((StatusCode::BAD_REQUEST, Json(response)).into_response());
  }

  let report_id = report.report_id.clone();
  report.save_to_disk().await?;

  info!("Successfully saved crash report: {}", report_id);

  let response = api::UploadResponse {
    success: true,
    message: "Crash report uploaded successfully".to_string(),
    report_id: Some(report_id.to_string()),
  };

  Ok((StatusCode::OK, Json(response)).into_response())
}

async fn list_reports() -> Result<Json<Vec<api::ReportMetadata>>, AppError> {
  let mut reports = Vec::new();
  let mut entries = fs::read_dir("crash_reports").await?;

  while let Some(entry) = entries.next_entry().await? {
    if entry.file_type().await?.is_dir() {
      let metadata_path = entry.path().join("metadata.json");

      if let Ok(metadata_content) = fs::read_to_string(&metadata_path).await {
        if let Ok(metadata) = serde_json::from_str::<api::ReportMetadata>(&metadata_content) {
          reports.push(metadata);
        }
      }
    }
  }

  reports.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
  Ok(Json(reports))
}

async fn get_report(Path(report_dir): Path<String>) -> Result<Json<api::ReportMetadata>, AppError> {
  let metadata_path = PathBuf::from("crash_reports")
    .join(&report_dir)
    .join("metadata.json");
  let metadata_content = fs::read_to_string(&metadata_path)
    .await
    .context("Failed to read metadata file")?;
  let metadata: api::ReportMetadata =
    serde_json::from_str(&metadata_content).context("Failed to parse metadata")?;
  Ok(Json(metadata))
}

async fn delete_report(Path(report_dir): Path<String>) -> Result<Response, AppError> {
  let report_dir_path = PathBuf::from("crash_reports").join(&report_dir);
  if !report_dir_path.exists() || !report_dir_path.is_dir() {
    return Ok((StatusCode::NOT_FOUND, "Report not found").into_response());
  }
  fs::remove_dir_all(&report_dir_path)
    .await
    .context("Failed to delete report directory")?;
  info!("Deleted report: {}", report_dir);
  Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response())
}

async fn download_file(
  Path((report_dir, filename)): Path<(String, String)>,
) -> Result<Response, AppError> {
  let file_path = PathBuf::from("crash_reports")
    .join(&report_dir)
    .join(&filename);

  if !file_path.exists() {
    return Ok((StatusCode::NOT_FOUND, "File not found").into_response());
  }

  let content = fs::read(&file_path).await.context("Failed to read file")?;

  let content_type = if filename.ends_with(".dmp") {
    "application/octet-stream"
  } else if filename.ends_with(".log") {
    "text/plain"
  } else if filename.ends_with(".json") {
    "application/json"
  } else {
    "application/octet-stream"
  };

  Ok(
    (
      StatusCode::OK,
      [
        (header::CONTENT_TYPE, content_type),
        (
          header::CONTENT_DISPOSITION,
          &format!("attachment; filename=\"{}\"", filename),
        ),
      ],
      content,
    )
      .into_response(),
  )
}

#[derive(Debug, Serialize, Deserialize)]
struct ResolveRequest {
  resolved: bool,
}

async fn toggle_resolve(
  Path(report_dir): Path<String>,
  Json(payload): Json<ResolveRequest>,
) -> Result<Response, AppError> {
  let metadata_path = PathBuf::from("crash_reports")
    .join(&report_dir)
    .join("metadata.json");

  let metadata_content = fs::read_to_string(&metadata_path)
    .await
    .context(format!("Failed to read metadata file: {}", metadata_path.display()))?;

  let mut metadata: api::ReportMetadata =
    serde_json::from_str(&metadata_content).context("Failed to parse metadata")?;

  metadata.resolved = payload.resolved;
  metadata.save_to_disk(&metadata_path)?;

  info!(
    "Updated resolve status for {}: {}",
    report_dir, payload.resolved
  );

  Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response())
}

fn create_app() -> Router {
  let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

  Router::new()
    .route("/api/upload", post(upload_crash_report))
    .route("/api/upload", head(health_check))
    .route("/api/reports", get(list_reports))
    .route("/api/reports/{report_dir}", get(get_report))
    .route("/api/reports/{report_dir}/resolve", post(toggle_resolve))
    .route(
      "/api/reports/{report_dir}/download/{filename}",
      get(download_file),
    )
    .route("/api/reports/{report_dir}/delete", delete(delete_report))
    .fallback_service(ServeDir::new("web/dist"))
    .layer(cors)
    .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt()
    .with_env_filter(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,tower_http=debug".into()),
    )
    .init();

  fs::create_dir_all("crash_reports")
    .await
    .context("Failed to create crash_reports directory")?;

  let app = create_app();

  let addr = std::env::var("BIND_ADDRESS")
    .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
    .parse::<SocketAddr>()
    .context("Failed to parse bind address")?;

  info!("Starting crash report server on {}", addr);
  info!("Upload endpoint: http://{}/api/upload", addr);

  let listener = tokio::net::TcpListener::bind(addr)
    .await
    .context("Failed to bind to address")?;

  info!("Server is ready to accept connections");

  axum::serve(listener, app).await.context("Server error")?;

  Ok(())
}
