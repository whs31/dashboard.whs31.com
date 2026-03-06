<script lang="ts">
    import {TooltipProvider} from '$lib/components/ui/tooltip';
    import * as NavigationMenu from '$lib/components/ui/navigation-menu';
    import * as Accordion from '$lib/components/ui/accordion';
    import * as Card from '$lib/components/ui/card';
    import {Button} from '$lib/components/ui/button';
    import {Badge} from '$lib/components/ui/badge';
    import type {ReportListItem} from '$lib/api/Report';

    let activeTab: string = $state('crashes');

    let mockReports: ReportListItem[] = $state([
        {
            report_id: '1',
            timestamp: '2024-01-15T10:30:00Z',
            app_name: 'MyApp',
            directory: 'crash-1'
        },
        {
            report_id: '2',
            timestamp: '2024-01-15T11:45:00Z',
            app_name: null,
            directory: 'crash-2'
        },
        {
            report_id: '3',
            timestamp: '2024-01-15T14:20:00Z',
            app_name: 'TestApp',
            directory: 'crash-3'
        }
    ]);

    let resolvedReports: Set<string> = $state(new Set());

    function toggleResolve(reportId: string): void {
    }

    function downloadMinidump(reportId: string): void {
    }

    function downloadLog(reportId: string, logFile: string): void {
    }

    $effect(() => {
        const totalCrashes = mockReports.length;
        const resolved = resolvedReports.size;
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
            {#if activeTab === 'crashes'}
                <div class="space-y-6">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <Card.Root>
                            <Card.Header>
                                <Card.Title>Total Crashes</Card.Title>
                            </Card.Header>
                            <Card.Content>
                                <div class="text-3xl font-bold">{mockReports.length}</div>
                            </Card.Content>
                        </Card.Root>

                        <Card.Root>
                            <Card.Header>
                                <Card.Title>Resolved</Card.Title>
                            </Card.Header>
                            <Card.Content>
                                <div class="text-3xl font-bold text-green-600">{resolvedReports.size}</div>
                            </Card.Content>
                        </Card.Root>

                        <Card.Root>
                            <Card.Header>
                                <Card.Title>Unresolved</Card.Title>
                            </Card.Header>
                            <Card.Content>
                                <div class="text-3xl font-bold text-red-600">{mockReports.length - resolvedReports.size}</div>
                            </Card.Content>
                        </Card.Root>
                    </div>

                    <Card.Root>
                        <Card.Header>
                            <Card.Title>Crash Reports</Card.Title>
                        </Card.Header>
                        <Card.Content>
                            <Accordion.Root class="w-full">
                                {#each mockReports as report (report.report_id)}
                                    <Accordion.Item value={report.report_id}>
                                        <Accordion.Trigger>
                                            <div class="flex items-center justify-between w-full pr-4">
                                                <div class="flex items-center gap-3">
                                                    <span class="font-medium">
                                                        {report.app_name || 'Unknown'}
                                                    </span>
                                                    {#if resolvedReports.has(report.report_id)}
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
                                                </div>

                                                <div class="flex flex-wrap gap-2">
                                                    <Button
                                                            variant="outline"
                                                            size="sm"
                                                            onclick={() => downloadMinidump(report.report_id)}
                                                    >
                                                        Download Minidump
                                                    </Button>
                                                    <Button
                                                            variant="outline"
                                                            size="sm"
                                                            onclick={() => downloadLog(report.report_id, 'log.txt')}
                                                    >
                                                        Download Logs
                                                    </Button>
                                                    <Button
                                                            variant={resolvedReports.has(report.report_id) ? 'secondary' : 'default'}
                                                            size="sm"
                                                            onclick={() => toggleResolve(report.report_id)}
                                                    >
                                                        {resolvedReports.has(report.report_id) ? 'Mark as Unresolved' : 'Mark as Resolved'}
                                                    </Button>
                                                </div>
                                            </div>
                                        </Accordion.Content>
                                    </Accordion.Item>
                                {/each}
                            </Accordion.Root>
                        </Card.Content>
                    </Card.Root>
                </div>
            {/if}
        </main>
    </TooltipProvider>
</div>