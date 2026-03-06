./bui# MMS Crash Report Server

A crash report server built with Rust and Axum that receives crash dumps, log files, and metadata from client applications. Includes a modern Svelte-based web interface for viewing and managing reports.

## Features

### Backend
- **Multipart Upload Support**: Handles minidump files, log files, and metadata
- **Automatic Organization**: Saves crash reports in timestamped directories
- **JSON Metadata**: Stores report information in structured JSON format
- **CORS Enabled**: Accepts requests from any origin
- **Health Check**: HEAD endpoint for server availability checks
- **No OpenSSL**: Uses rustls for TLS support (when needed)
- **REST API**: Full API for listing, viewing, and downloading reports

### Web Interface
- **📊 Report List View**: Browse all crash reports in a grid layout
- **🔍 Detailed View**: View full report details with app name in title
- **⬇️ File Downloads**: Download minidumps and log files directly
- **🏷️ App Name Display**: Shows application name in badges and titles
- **🔄 Auto-refresh**: Automatically refreshes every 30 seconds
- **📱 Responsive Design**: Works on desktop and mobile
- **✓ User Feedback Buttons**: Acknowledge, In Progress, Resolved, Won't Fix

## API Endpoints

### Upload API

#### POST /api/upload

Accepts multipart form data with the following fields:

- `minidump` (file, required): The crash minidump file
- `logfile_N` (file, optional): Log files (where N is an index)
- `app_name` (text, optional): Name of the application
- `details` (text, optional): User-provided crash details
- `steps_to_reproduce` (text, optional): Steps to reproduce the crash

**Response:**
```json
{
  "success": true,
  "message": "Crash report uploaded successfully",
  "report_id": "uuid-here"
}
```

#### HEAD /api/upload

Health check endpoint. Returns 200 OK if server is available.

### Web API

#### GET /api/reports

List all crash reports.

**Response:**
```json
[
  {
    "report_id": "uuid",
    "timestamp": "2024-01-15_14-30-45",
    "app_name": "MyApp",
    "directory": "2024-01-15_14-30-45_uuid"
  }
]
```

#### GET /api/reports/:report_dir

Get detailed information about a specific report.

**Response:**
```json
{
  "report_id": "uuid",
  "timestamp": "2024-01-15_14-30-45",
  "app_name": "MyApp",
  "details": "Crash details...",
  "steps_to_reproduce": "Steps...",
  "minidump_filename": "minidump.dmp",
  "log_files": ["app.log", "debug.log"]
}
```

#### GET /api/reports/:report_dir/download/:filename

Download a specific file (minidump or log) from a report.

## Quick Start

### 1. Build the Web Interface

```bash
./build_web.sh
```

Or manually:
```bash
cd web
npm install
npm run build
cd ..
```

### 2. Build the Server

```bash
cargo build --release
```

### 3. Run the Server

```bash
# Run with default settings (binds to 0.0.0.0:3000)
cargo run --release

# Or specify a custom bind address
BIND_ADDRESS=127.0.0.1:8080 cargo run --release

# Enable debug logging
RUST_LOG=debug cargo run --release
```

### 4. Access the Web Interface

Open your browser to `http://localhost:3000`

The web interface will show:
- List of all crash reports
- Click any report to view details
- Download minidumps and logs
- User feedback buttons for report management

## Configuration

The server can be configured using environment variables:

- `BIND_ADDRESS`: Server bind address (default: `0.0.0.0:3000`)
- `RUST_LOG`: Logging level (default: `info,tower_http=debug`)

## Crash Report Storage

Crash reports are stored in the `crash_reports/` directory with the following structure:

```
crash_reports/
└── 2024-01-15_14-30-45_uuid/
    ├── metadata.json
    ├── minidump.dmp
    ├── app.log
    └── debug.log
```

### metadata.json Example

```json
{
  "report_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-01-15_14-30-45",
  "app_name": "MyApp",
  "details": "Application crashed when clicking button",
  "steps_to_reproduce": "1. Open app\n2. Click button\n3. Crash",
  "minidump_filename": "minidump.dmp",
  "log_files": ["app.log", "debug.log"]
}
```

## Client Integration

The server is designed to work with the crash reporter client. Configure the client with:

```rust
let data = CrashReportData {
    minidump_path: PathBuf::from("crash.dmp"),
    log_files: vec![PathBuf::from("app.log")],
    app_name: Some("MyApp".to_string()),
    upload_url: "http://localhost:3000/api/upload".to_string(),
};
```

## Dependencies

- **axum**: Web framework
- **tokio**: Async runtime
- **tower-http**: HTTP middleware (CORS, tracing)
- **serde**: Serialization
- **chrono**: Timestamp generation
- **uuid**: Unique report IDs
- **tracing**: Structured logging

All dependencies use `rustls` for TLS support, avoiding OpenSSL requirements.

## License

See LICENSE file for details.
