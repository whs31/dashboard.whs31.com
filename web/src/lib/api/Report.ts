interface ReportListItem {
    report_id: string;
    timestamp: string;
    app_name: string | null;
    directory: string;
}

interface ReportMetadata {
    report_id: string;
    timestamp: string;
    app_name: string | null;
    details: string | null;
    steps_to_reproduce: string | null;
    minidump_filename: string | null;
    log_files: string[];
}

export type { ReportListItem, ReportMetadata }