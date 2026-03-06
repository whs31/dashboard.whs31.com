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

function getDirectory(report: ReportMetadata): string {
    return `${report.timestamp}_${report.report_id}`;
}

async function fetchReports(): Promise<ReportMetadata[]> {
    const response = await fetch('/api/reports');
    if (!response.ok) {
        throw new Error('Failed to fetch reports');
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
    const link = document.createElement('a');
    link.href = url;
    link.download = filename; // Suggests a filename to the browser
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
}

export type { ReportMetadata }
export { getDirectory, fetchReports, toggleReportResolved, downloadFile }