<script>
  import { onMount } from 'svelte';

  let reports = [];
  let selectedReport = null;
  let reportDetails = null;
  let loading = true;
  let error = null;

  async function fetchReports() {
    try {
      loading = true;
      error = null;
      const response = await fetch('/api/reports');
      if (!response.ok) throw new Error('Failed to fetch reports');
      reports = await response.json();
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }

  async function selectReport(report) {
    selectedReport = report;
    try {
      const response = await fetch(`/api/reports/${report.directory}`);
      if (!response.ok) throw new Error('Failed to fetch report details');
      reportDetails = await response.json();
    } catch (err) {
      error = err.message;
      reportDetails = null;
    }
  }

  function downloadFile(filename) {
    if (!selectedReport) return;
    const url = `/api/reports/${selectedReport.directory}/download/${filename}`;
    window.open(url, '_blank');
  }

  function closeDetails() {
    selectedReport = null;
    reportDetails = null;
  }

  onMount(() => {
    fetchReports();
    const interval = setInterval(fetchReports, 30000);
    return () => clearInterval(interval);
  });

  function getAppTitle(report) {
    return report.app_name || 'Unknown App';
  }
</script>

<div class="header">
  <div class="container">
    <h1>🔍 Crash Report Viewer</h1>
    <p>Monitor and analyze application crash reports</p>
  </div>
</div>

<div class="container">
  {#if error}
    <div class="error">
      <strong>Error:</strong> {error}
    </div>
  {/if}

  {#if loading}
    <div class="loading">
      <p>Loading crash reports...</p>
    </div>
  {:else if reports.length === 0}
    <div class="empty-state">
      <h2>No crash reports yet</h2>
      <p>Crash reports will appear here when they are uploaded</p>
    </div>
  {:else}
    {#if reportDetails && selectedReport}
      <div class="detail-panel">
        <div class="detail-header">
          <h2>📋 {getAppTitle(selectedReport)}</h2>
          <button class="btn btn-secondary" on:click={closeDetails}>✕ Close</button>
        </div>

        <div class="detail-section">
          <h3>Report ID</h3>
          <p style="font-family: monospace; color: #666;">{reportDetails.report_id}</p>
        </div>

        <div class="detail-section">
          <h3>Timestamp</h3>
          <p>{reportDetails.timestamp}</p>
        </div>

        {#if reportDetails.app_name}
          <div class="detail-section">
            <h3>Application</h3>
            <p>{reportDetails.app_name}</p>
          </div>
        {/if}

        {#if reportDetails.details}
          <div class="detail-section">
            <h3>Details</h3>
            <p>{reportDetails.details}</p>
          </div>
        {/if}

        {#if reportDetails.steps_to_reproduce}
          <div class="detail-section">
            <h3>Steps to Reproduce</h3>
            <p>{reportDetails.steps_to_reproduce}</p>
          </div>
        {/if}

        <div class="detail-section">
          <h3>Files</h3>
          <div class="files-list">
            {#if reportDetails.minidump_filename}
              <div class="file-item">
                <span class="file-name">📦 {reportDetails.minidump_filename}</span>
                <button
                  class="btn btn-download"
                  on:click={() => downloadFile(reportDetails.minidump_filename)}
                >
                  ⬇ Download
                </button>
              </div>
            {/if}
            {#each reportDetails.log_files as logFile}
              <div class="file-item">
                <span class="file-name">📄 {logFile}</span>
                <button
                  class="btn btn-download"
                  on:click={() => downloadFile(logFile)}
                >
                  ⬇ Download
                </button>
              </div>
            {/each}
          </div>
        </div>

        <div class="feedback-section">
          <span class="feedback-label">User Feedback:</span>
          <div class="feedback-buttons">
            <button class="btn btn-primary">✓ Acknowledged</button>
            <button class="btn btn-secondary">🔄 In Progress</button>
            <button class="btn btn-secondary">✓ Resolved</button>
            <button class="btn btn-secondary">✗ Won't Fix</button>
          </div>
        </div>
      </div>
    {/if}

    <div class="reports-grid">
      {#each reports as report}
        <div
          class="report-card"
          class:selected={selectedReport?.report_id === report.report_id}
          on:click={() => selectReport(report)}
          on:keydown={(e) => e.key === 'Enter' && selectReport(report)}
          role="button"
          tabindex="0"
        >
          <div class="report-header">
            <div class="report-id">ID: {report.report_id.substring(0, 8)}...</div>
            {#if report.app_name}
              <div class="app-badge">{report.app_name}</div>
            {/if}
          </div>
          <div class="report-timestamp">📅 {report.timestamp}</div>
          <div class="report-actions">
            <button class="btn btn-primary" on:click|stopPropagation={() => selectReport(report)}>
              View Details
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
