<script lang="ts">
    import {onMount} from 'svelte';

    import {TooltipProvider} from '$lib/components/ui/tooltip';

    // let reports: ReportListItem[] = $state([]);
    // let selectedReport: ReportListItem | null = $state(null);
    // let reportDetails: ReportMetadata | null = $state(null);
    // let loading: boolean = $state(true);
    // let error: string | null = $state(null);
    // let detailsModalOpen: boolean = $state(false);
    //
    // async function fetchReports(): Promise<void> {
    //   try {
    //     loading = true;
    //     error = null;
    //     const response = await fetch('/api/reports');
    //     if (!response.ok) throw new Error('Failed to fetch reports');
    //     reports = await response.json();
    //   } catch (err) {
    //     error = err instanceof Error ? err.message : 'Unknown error';
    //   } finally {
    //     loading = false;
    //   }
    // }
    //
    // async function selectReport(report: ReportListItem): Promise<void> {
    //   selectedReport = report;
    //   try {
    //     const response = await fetch(`/api/reports/${report.directory}`);
    //     if (!response.ok) throw new Error('Failed to fetch report details');
    //     reportDetails = await response.json();
    //     detailsModalOpen = true;
    //   } catch (err) {
    //     error = err instanceof Error ? err.message : 'Unknown error';
    //     reportDetails = null;
    //   }
    // }
    //
    // function downloadFile(filename: string): void {
    //   if (!selectedReport) return;
    //   const url = `/api/reports/${selectedReport.directory}/download/${filename}`;
    //   window.open(url, '_blank');
    // }
    //
    // function closeDetails(): void {
    //   detailsModalOpen = false;
    //   selectedReport = null;
    //   reportDetails = null;
    // }
    //
    // onMount(() => {
    //   fetchReports();
    //   const interval = setInterval(fetchReports, 30000);
    //   return () => clearInterval(interval);
    // });
    //
    // function getAppTitle(report: ReportListItem): string {
    //   return report.app_name || 'Unknown App';
    // }
</script>

<div
        class="min-h-screen bg-background select-none"
>
    <TooltipProvider>
<!--        header? -->
    </TooltipProvider>
</div>