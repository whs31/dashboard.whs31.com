# Crash Report Viewer - Web Frontend

A Svelte 5 + TypeScript web interface for viewing and managing crash reports.

## Features

- 📊 **Report List View** - Browse all crash reports in a grid layout
- 🔍 **Detailed View** - View full report details including metadata and user feedback
- ⬇️ **File Downloads** - Download minidumps and log files directly from the browser
- 🏷️ **App Name Display** - Shows application name in badges and detail titles
- 🔄 **Auto-refresh** - Automatically refreshes report list every 30 seconds
- 📱 **Responsive Design** - Works on desktop and mobile devices
- 🎨 **Modern UI** - Clean, professional interface with smooth animations

## Development

### Prerequisites

- Node.js 18+ and npm

### Setup

```bash
# Install dependencies
npm install

# Start development server
npm run dev
```

The dev server will start at `http://localhost:5173` with API proxy to the backend.

## Building for Production

```bash
# Build the frontend
npm run build
```

This creates an optimized production build in the `dist/` directory.

### Quick Build Script

From the project root:

```bash
./build_web.sh
```

## API Endpoints Used

- `GET /api/reports` - List all crash reports
- `GET /api/reports/:report_dir` - Get report details
- `GET /api/reports/:report_dir/download/:filename` - Download files

## Project Structure

```
web/
├── src/
│   ├── App.svelte       # Main application component
│   ├── main.js          # Application entry point
│   └── app.css          # Global styles
├── index.html           # HTML template
├── package.json         # Dependencies
├── vite.config.js       # Vite configuration
└── svelte.config.js     # Svelte configuration
```

## User Feedback Buttons

The detail view includes feedback buttons for crash report management:
- ✓ Acknowledged
- 🔄 In Progress
- ✓ Resolved
- ✗ Won't Fix

These buttons are currently UI-only. To make them functional, implement backend endpoints to store feedback status.

## Customization

### Styling

Edit `src/app.css` to customize colors, fonts, and layout.

### Auto-refresh Interval

In `App.svelte`, change the interval (default: 30000ms = 30s):

```javascript
const interval = setInterval(fetchReports, 30000); // Change this value
```

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
