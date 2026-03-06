<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Card,
    Badge,
    Button,
    Alert,
    Spinner,
    Modal,
    Heading,
    P,
    Breadcrumb,
    BreadcrumbItem
  } from 'flowbite-svelte';
  import {
    SearchOutline,
    ClockOutline,
    DownloadOutline,
    CheckOutline,
    CloseOutline,
    RefreshOutline,
    ExclamationCircleOutline,
    FileOutline,
    FolderOutline
  } from 'flowbite-svelte-icons';

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

  let reports: ReportListItem[] = $state([]);
  let selectedReport: ReportListItem | null = $state(null);
  let reportDetails: ReportMetadata | null = $state(null);
  let loading: boolean = $state(true);
  let error: string | null = $state(null);
  let detailsModalOpen: boolean = $state(false);

  async function fetchReports(): Promise<void> {
    try {
      loading = true;
      error = null;
      const response = await fetch('/api/reports');
      if (!response.ok) throw new Error('Failed to fetch reports');
      reports = await response.json();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Unknown error';
    } finally {
      loading = false;
    }
  }

  async function selectReport(report: ReportListItem): Promise<void> {
    selectedReport = report;
    try {
      const response = await fetch(`/api/reports/${report.directory}`);
      if (!response.ok) throw new Error('Failed to fetch report details');
      reportDetails = await response.json();
      detailsModalOpen = true;
    } catch (err) {
      error = err instanceof Error ? err.message : 'Unknown error';
      reportDetails = null;
    }
  }

  function downloadFile(filename: string): void {
    if (!selectedReport) return;
    const url = `/api/reports/${selectedReport.directory}/download/${filename}`;
    window.open(url, '_blank');
  }

  function closeDetails(): void {
    detailsModalOpen = false;
    selectedReport = null;
    reportDetails = null;
  }

  onMount(() => {
    fetchReports();
    const interval = setInterval(fetchReports, 30000);
    return () => clearInterval(interval);
  });

  function getAppTitle(report: ReportListItem): string {
    return report.app_name || 'Unknown App';
  }
</script>

<!-- Header -->
<div class="bg-linear-to-r from-blue-600 to-purple-600 text-white py-8 mb-8 shadow-lg">
  <div class="container mx-auto px-4">
    <div class="flex items-center gap-3 mb-2">
      <SearchOutline size="xl" />
      <Heading tag="h1" class="text-white">Crash Report Viewer</Heading>
    </div>
    <P class="text-blue-100">Monitor and analyze application crash reports</P>
  </div>
</div>

<!-- Main Content -->
<div class="container mx-auto px-4 pb-8">
  {#if error}
    <Alert color="red" class="mb-6">
      <ExclamationCircleOutline slot="icon" class="w-5 h-5" />
      <span class="font-medium">Error:</span> {error}
    </Alert>
  {/if}

  {#if loading}
    <div class="flex flex-col items-center justify-center py-20">
      <Spinner size="12" />
      <P class="mt-4 text-gray-600">Loading crash reports...</P>
    </div>
  {:else if reports.length === 0}
    <div class="flex flex-col items-center justify-center py-20">
      <FolderOutline size="xl" class="w-24 h-24 text-gray-400 mb-4" />
      <Heading tag="h2" class="text-gray-600 mb-2">No crash reports yet</Heading>
      <P class="text-gray-500">Crash reports will appear here when they are uploaded</P>
    </div>
  {:else}
    <!-- Reports Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each reports as report}
        <Card class="cursor-pointer hover:shadow-xl transition-shadow duration-200" padding="lg">
          <div class="flex justify-between items-start mb-4">
            <div class="flex-1">
              <P class="text-xs text-gray-500 font-mono mb-2">
                ID: {report.report_id.substring(0, 8)}...
              </P>
              {#if report.app_name}
                <Badge color="blue" class="mb-2">{report.app_name}</Badge>
              {/if}
            </div>
          </div>

          <div class="flex items-center gap-2 text-gray-600 mb-4">
            <ClockOutline size="sm" />
            <P class="text-sm">{report.timestamp}</P>
          </div>

          <Button color="blue" class="w-full" onclick={() => selectReport(report)}>
            <SearchOutline size="sm" class="mr-2" />
            View Details
          </Button>
        </Card>
      {/each}
    </div>
  {/if}
</div>

<!-- Details Modal -->
{#if reportDetails && selectedReport}
  <Modal bind:open={detailsModalOpen} size="xl" class="w-full">
    <div slot="header" class="flex items-center gap-3">
      <FolderOutline size="lg" />
      <Heading tag="h3">{getAppTitle(selectedReport)}</Heading>
    </div>

    <div class="space-y-6">
      <!-- Report ID -->
      <div>
        <Heading tag="h5" class="mb-2 text-blue-600">Report ID</Heading>
        <P class="font-mono text-sm text-gray-700">{reportDetails.report_id}</P>
      </div>

      <!-- Timestamp -->
      <div>
        <Heading tag="h5" class="mb-2 text-blue-600">Timestamp</Heading>
        <div class="flex items-center gap-2">
          <ClockOutline size="sm" />
          <P>{reportDetails.timestamp}</P>
        </div>
      </div>

      <!-- Application -->
      {#if reportDetails.app_name}
        <div>
          <Heading tag="h5" class="mb-2 text-blue-600">Application</Heading>
          <Badge color="blue" large>{reportDetails.app_name}</Badge>
        </div>
      {/if}

      <!-- Details -->
      {#if reportDetails.details}
        <div>
          <Heading tag="h5" class="mb-2 text-blue-600">Details</Heading>
          <P class="whitespace-pre-wrap text-gray-700">{reportDetails.details}</P>
        </div>
      {/if}

      <!-- Steps to Reproduce -->
      {#if reportDetails.steps_to_reproduce}
        <div>
          <Heading tag="h5" class="mb-2 text-blue-600">Steps to Reproduce</Heading>
          <P class="whitespace-pre-wrap text-gray-700">{reportDetails.steps_to_reproduce}</P>
        </div>
      {/if}

      <!-- Files -->
      <div>
        <Heading tag="h5" class="mb-3 text-blue-600">Files</Heading>
        <div class="space-y-2">
          {#if reportDetails.minidump_filename}
            <Card class="bg-gray-50">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <FolderOutline size="sm" />
                  <P class="font-mono text-sm">{reportDetails.minidump_filename}</P>
                </div>
                <Button size="sm" color="green" onclick={() => downloadFile(reportDetails.minidump_filename!)}>
                  <DownloadOutline size="sm" class="mr-1" />
                  Download
                </Button>
              </div>
            </Card>
          {/if}
          {#each reportDetails.log_files as logFile}
            <Card class="bg-gray-50">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <FileOutline size="sm" />
                  <P class="font-mono text-sm">{logFile}</P>
                </div>
                <Button size="sm" color="green" onclick={() => downloadFile(logFile)}>
                  <DownloadOutline size="sm" class="mr-1" />
                  Download
                </Button>
              </div>
            </Card>
          {/each}
        </div>
      </div>

      <!-- User Feedback -->
      <div class="border-t pt-6">
        <Heading tag="h5" class="mb-3 text-blue-600">User Feedback</Heading>
        <div class="flex flex-wrap gap-2">
          <Button color="blue" size="sm">
            <CheckOutline size="sm" class="mr-1" />
            Acknowledged
          </Button>
          <Button color="yellow" size="sm">
            <RefreshOutline size="sm" class="mr-1" />
            In Progress
          </Button>
          <Button color="green" size="sm">
            <CheckOutline size="sm" class="mr-1" />
            Resolved
          </Button>
          <Button color="red" size="sm">
            <CloseOutline size="sm" class="mr-1" />
            Won't Fix
          </Button>
        </div>
      </div>
    </div>

    <div slot="footer" class="flex justify-end">
      <Button color="alternative" onclick={closeDetails}>Close</Button>
    </div>
  </Modal>
{/if}
