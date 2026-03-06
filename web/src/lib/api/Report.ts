interface ReportListItem {
    report_id: string;
    timestamp: string;
    app_name: string | null;
    directory: string;
    resolved?: boolean;
}

interface ReportMetadata {
    report_id: string;
    timestamp: string;
    app_name: string | null;
    details: string | null;
    steps_to_reproduce: string | null;
    minidump_filename: string | null;
    log_files: string[];
    resolved?: boolean;
}

async function fetchReports(): Promise<ReportListItem[]> {
    const response = await fetch('/api/reports');
    if (!response.ok) {
        throw new Error('Failed to fetch reports');
    }
    return await response.json();
}

async function fetchReportDetails(directory: string): Promise<ReportMetadata> {
    const response = await fetch(`/api/reports/${directory}`);
    if (!response.ok) {
        throw new Error('Failed to fetch report details');
    }
    return await response.json();
}

async function toggleReportResolved(directory: string, resolved: boolean): Promise<void> {
    const response = await fetch(`/api/reports/${directory}/resolve`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ resolved }),
    });
    if (!response.ok) {
        throw new Error('Failed to update report status');
    }
}

function downloadFile(directory: string, filename: string): void {
    const url = `/api/reports/${directory}/download/${filename}`;
    window.open(url, '_blank');
}

export type { ReportListItem, ReportMetadata }
export { fetchReports, fetchReportDetails, toggleReportResolved, downloadFile }