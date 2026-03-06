<script lang="ts">
    import {onMount} from 'svelte';
    import {TooltipProvider} from '$lib/components/ui/tooltip';
    import * as NavigationMenu from '$lib/components/ui/navigation-menu';
    import * as Accordion from '$lib/components/ui/accordion';
    import * as Card from '$lib/components/ui/card';
    import {Button} from '$lib/components/ui/button';
    import {Badge} from '$lib/components/ui/badge';
    import type {ReportListItem, ReportMetadata} from '$lib/api/Report';
    import {fetchReports, fetchReportDetails, toggleReportResolved, downloadFile} from '$lib/api/Report';

    let activeTab: string = $state('crashes');
    let reports: ReportListItem[] = $state([]);
    let reportDetails: Map<string, ReportMetadata> = $state(new Map());
    let loading: boolean = $state(true);
    let error: string | null = $state(null);

    async function loadReports(): Promise<void> {
        try {
            loading = true;
            error = null;
            reports = await fetchReports();
        } catch (err) {
            error = err instanceof Error ? err.message : 'Unknown error';
            console.error('Failed to load reports:', err);
        } finally {
            loading = false;
        }
    }

    async function loadReportDetails(directory: string): Promise<void> {
        if (reportDetails.has(directory)) {
            return;
        }
        try {
            const details = await fetchReportDetails(directory);
            reportDetails.set(directory, details);
            reportDetails = reportDetails;
        } catch (err) {
            console.error('Failed to load report details:', err);
        }
    }

    async function toggleResolve(report: ReportListItem): Promise<void> {
        const newResolvedState = !report.resolved;
        try {
            await toggleReportResolved(report.directory, newResolvedState);
            report.resolved = newResolvedState;
            reports = reports;
        } catch (err) {
            error = err instanceof Error ? err.message : 'Failed to update report status';
            console.error('Failed to toggle resolve status:', err);
        }
    }

    function downloadMinidump(report: ReportListItem): void {
        const details = reportDetails.get(report.directory);
        if (details?.minidump_filename) {
            downloadFile(report.directory, details.minidump_filename);
        }
    }

    function downloadLogs(report: ReportListItem): void {
        const details = reportDetails.get(report.directory);
        if (details?.log_files && details.log_files.length > 0) {
            details.log_files.forEach(logFile => {
                downloadFile(report.directory, logFile);
            });
        }
    }

    onMount(() => {
        loadReports();
        const interval = setInterval(loadReports, 30000);
        return () => clearInterval(interval);
    });

    $effect(() => {
        const totalCrashes = reports.length;
        const resolved = reports.filter(r => r.resolved).length;
        const unresolved = totalCrashes - resolved;
    });
</script>

<div class="min-h-screen bg-background select-none">
    <TooltipProvider>
        <header class="border-b">
            <div class="container mx-auto px-4 py-4">
                <NavigationMenu.Root bind:value={activeTab}>
                    <NavigationMenu.List>
                        <NavigationMenu.Item value="crashes">
                            <NavigationMenu.Link
                                    href="#crashes"
                                    class="group inline-flex h-10 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[active]:bg-accent/50 data-[state=open]:bg-accent/50"
                            >
                                Crashes
                            </NavigationMenu.Link>
                        </NavigationMenu.Item>
                        <NavigationMenu.Item value="feedback">
                            <NavigationMenu.Link
                                    href="#feedback"
                                    class="group inline-flex h-10 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors pointer-events-none opacity-50"
                            >
                                User Feedback
                            </NavigationMenu.Link>
                        </NavigationMenu.Item>
                    </NavigationMenu.List>
                </NavigationMenu.Root>
            </div>
        </header>

        <main class="container mx-auto px-4 py-8">
            {#if error}
                <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-4">
                    {error}
                </div>
            {/if}

            {#if loading}
                <div class="flex justify-center items-center py-12">
                    <div class="text-muted-foreground">Loading crash reports...</div>
                </div>
            {:else if activeTab === 'crashes'}
                <div class="space-y-6">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <Card.Root>
                            <Card.Header>
                                <Card.Title>Total Crashes</Card.Title>
                            </Card.Header>
                            <Card.Content>
                                <div class="text-3xl font-bold">{reports.length}</div>
                            </Card.Content>
                        </Card.Root>

                        <Card.Root>
                            <Card.Header>
                                <Card.Title>Resolved</Card.Title>
                            </Card.Header>
                            <Card.Content>
                                <div class="text-3xl font-bold text-green-600">{reports.filter(r => r.resolved).length}</div>
                            </Card.Content>
                        </Card.Root>

                        <Card.Root>
                            <Card.Header>
                                <Card.Title>Unresolved</Card.Title>
                            </Card.Header>
                            <Card.Content>
                                <div class="text-3xl font-bold text-red-600">{reports.filter(r => !r.resolved).length}</div>
                            </Card.Content>
                        </Card.Root>
                    </div>

                    {#if reports.length === 0}
                        <Card.Root>
                            <Card.Content class="py-12">
                                <div class="text-center text-muted-foreground">
                                    No crash reports found
                                </div>
                            </Card.Content>
                        </Card.Root>
                    {:else}
                        <Card.Root>
                            <Card.Header>
                                <Card.Title>Crash Reports</Card.Title>
                            </Card.Header>
                            <Card.Content>
                                <Accordion.Root class="w-full">
                                    {#each reports as report (report.report_id)}
                                        <Accordion.Item value={report.report_id}>
                                            <Accordion.Trigger onclick={() => loadReportDetails(report.directory)}>
                                                <div class="flex items-center justify-between w-full pr-4">
                                                    <div class="flex items-center gap-3">
                                                        <span class="font-medium">
                                                            {report.app_name || 'Unknown'}
                                                        </span>
                                                        {#if report.resolved}
                                                            <Badge variant="outline" class="bg-green-50 text-green-700 border-green-200">
                                                                Resolved
                                                            </Badge>
                                                        {:else}
                                                            <Badge variant="destructive">
                                                                Unresolved
                                                            </Badge>
                                                        {/if}
                                                    </div>
                                                    <span class="text-sm text-muted-foreground">
                                                        {new Date(report.timestamp).toLocaleString()}
                                                    </span>
                                                </div>
                                            </Accordion.Trigger>
                                            <Accordion.Content>
                                                <div class="space-y-4 pt-4">
                                                    <div class="flex flex-col gap-2">
                                                        <div class="text-sm">
                                                            <span class="font-semibold">Report ID:</span> {report.report_id}
                                                        </div>
                                                        <div class="text-sm">
                                                            <span class="font-semibold">Directory:</span> {report.directory}
                                                        </div>
                                                        {#if reportDetails.has(report.directory)}
                                                            {@const details = reportDetails.get(report.directory)}
                                                            {#if details?.details}
                                                                <div class="text-sm">
                                                                    <span class="font-semibold">Details:</span> {details.details}
                                                                </div>
                                                            {/if}
                                                            {#if details?.steps_to_reproduce}
                                                                <div class="text-sm">
                                                                    <span class="font-semibold">Steps to Reproduce:</span> {details.steps_to_reproduce}
                                                                </div>
                                                            {/if}
                                                        {/if}
                                                    </div>

                                                    <div class="flex flex-wrap gap-2">
                                                        {#if reportDetails.has(report.directory) && reportDetails.get(report.directory)?.minidump_filename}
                                                            <Button
                                                                    variant="outline"
                                                                    size="sm"
                                                                    onclick={() => downloadMinidump(report)}
                                                            >
                                                                Download Minidump
                                                            </Button>
                                                        {/if}
                                                        {#if reportDetails.has(report.directory) && reportDetails.get(report.directory)?.log_files && reportDetails.get(report.directory)!.log_files.length > 0}
                                                            <Button
                                                                    variant="outline"
                                                                    size="sm"
                                                                    onclick={() => downloadLogs(report)}
                                                            >
                                                                Download Logs
                                                            </Button>
                                                        {/if}
                                                        <Button
                                                                variant={report.resolved ? 'secondary' : 'default'}
                                                                size="sm"
                                                                onclick={() => toggleResolve(report)}
                                                        >
                                                            {report.resolved ? 'Mark as Unresolved' : 'Mark as Resolved'}
                                                        </Button>
                                                    </div>
                                                </div>
                                            </Accordion.Content>
                                        </Accordion.Item>
                                    {/each}
                                </Accordion.Root>
                            </Card.Content>
                        </Card.Root>
                    {/if}
                </div>
            {/if}
        </main>
    </TooltipProvider>
</div>