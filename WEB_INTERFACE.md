# Web Interface Guide

## Overview

The crash report server now includes a modern Svelte-based web interface for viewing and managing crash reports through your browser.

## Features

### 📊 Report List View
- Grid layout showing all crash reports
- Each card displays:
  - Report ID (first 8 characters)
  - Timestamp
  - Application name badge (if provided)
  - "View Details" button

### 🔍 Detailed Report View
- **App name in title**: The detail panel shows the application name prominently in the header
- Full report metadata:
  - Report ID
  - Timestamp
  - Application name
  - User-provided details
  - Steps to reproduce
- **File downloads**: Download minidumps and log files with one click
- **User feedback buttons**: Mark reports as:
  - ✓ Acknowledged
  - 🔄 In Progress
  - ✓ Resolved
  - ✗ Won't Fix

### 🔄 Auto-refresh
- Automatically refreshes the report list every 30 seconds
- No need to manually reload the page

### 📱 Responsive Design
- Works on desktop, tablet, and mobile devices
- Clean, modern interface with smooth animations

## Setup

### 1. Install Node.js Dependencies

```bash
cd web
npm install
```

### 2. Build the Frontend

```bash
# From the web directory
npm run build

# Or from the project root
./build_web.sh
```

This creates an optimized production build in `web/dist/`.

### 3. Run the Server

```bash
# From the project root
cargo run --release
```

### 4. Access the Interface

Open your browser to: `http://localhost:3000`

## Development Mode

For frontend development with hot-reload:

```bash
# Terminal 1: Run the backend server
cargo run

# Terminal 2: Run the Vite dev server
cd web
npm run dev
```

The dev server runs at `http://localhost:5173` and proxies API requests to the backend.

## Usage

### Viewing Reports

1. The main page shows all crash reports in a grid
2. Reports are sorted by timestamp (newest first)
3. Click any report card to view details

### Viewing Report Details

1. Click "View Details" on any report
2. The detail panel opens showing:
   - **Title**: Shows the app name (e.g., "📋 MyApp")
   - Full metadata
   - User-provided details and reproduction steps
   - List of files (minidump and logs)

### Downloading Files

1. Open a report's detail view
2. Click the "⬇ Download" button next to any file
3. The file downloads to your browser's download folder

### Managing Reports with Feedback

1. Open a report's detail view
2. Scroll to the "User Feedback" section
3. Click one of the feedback buttons:
   - **✓ Acknowledged**: You've seen the report
   - **🔄 In Progress**: Working on a fix
   - **✓ Resolved**: Issue has been fixed
   - **✗ Won't Fix**: Not planning to fix

**Note**: Feedback buttons are currently UI-only. To make them functional, implement backend endpoints to store feedback status.

### Closing Details

Click the "✕ Close" button in the detail panel header to return to the list view.

## API Integration

The web interface uses these API endpoints:

- `GET /api/reports` - Fetch all reports
- `GET /api/reports/{report_dir}` - Fetch report details
- `GET /api/reports/{report_dir}/download/{filename}` - Download files

## Customization

### Change Auto-refresh Interval

Edit `web/src/App.svelte`:

```javascript
// Change from 30 seconds to 60 seconds
const interval = setInterval(fetchReports, 60000);
```

### Customize Styling

Edit `web/src/app.css` to change:
- Colors
- Fonts
- Layout
- Animations

### Add Feedback Backend

To make feedback buttons functional:

1. Add a new field to `ReportMetadata`:
```rust
feedback_status: Option<String>,
```

2. Create a new endpoint:
```rust
async fn update_feedback(
  Path((report_dir, status)): Path<(String, String)>
) -> Result<StatusCode, AppError> {
  // Update metadata.json with feedback status
  Ok(StatusCode::OK)
}
```

3. Add route:
```rust
.route("/api/reports/{report_dir}/feedback/{status}", post(update_feedback))
```

4. Update the Svelte component to call this endpoint when buttons are clicked.

## Troubleshooting

### Web interface shows "No crash reports yet"

- Check that crash reports exist in the `crash_reports/` directory
- Verify each report has a `metadata.json` file
- Check browser console for API errors

### Files won't download

- Verify the file exists in the report directory
- Check server logs for errors
- Ensure the server has read permissions on the files

### Interface not loading

- Verify `web/dist/` directory exists and contains built files
- Run `./build_web.sh` to rebuild the frontend
- Check server logs for errors serving static files

### API errors

- Open browser DevTools (F12) → Network tab
- Check for failed API requests
- Verify the server is running on the expected port

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+

## File Structure

```
web/
├── dist/                # Built files (served by the server)
├── src/
│   ├── App.svelte      # Main component
│   ├── main.js         # Entry point
│   └── app.css         # Styles
├── index.html          # HTML template
├── package.json        # Dependencies
├── vite.config.js      # Build config
└── README.md           # Frontend docs
```

## Production Deployment

The built frontend is automatically served by the Rust server from `web/dist/`.

No separate web server is needed - just build the frontend and run the Rust server:

```bash
./build_web.sh
cargo build --release
./target/release/mcr_server
```

## Security Notes

- The web interface is served over HTTP by default
- For production, use a reverse proxy (nginx/caddy) with HTTPS
- Consider adding authentication for the web interface
- Implement rate limiting on API endpoints
