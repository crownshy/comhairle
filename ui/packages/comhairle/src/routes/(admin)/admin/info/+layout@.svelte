<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { ADMIN_GUIDE_NAV } from '$lib/admin_guides';
	import * as Select from '$lib/components/ui/select';
	import { GUIDE_NAV } from '$lib/tool_guides';
	import type { LayoutProps } from './$types';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import { ChevronDown } from 'lucide-svelte';

	let { children }: LayoutProps = $props();
	const NAV_GROUPS = [
		{
			label: 'Admin how-to',
			items: ADMIN_GUIDE_NAV.map((guide) => ({
				label: guide.navLabel,
				path: `/admin/info/how-to/${guide.key}` as const,
				topics: guide.topics.map((topic) => ({
					label: topic.navLabel,
					path: `/admin/info/how-to/${guide.key}/${topic.key}` as const
				}))
			}))
		},
		{
			label: 'Engagement tools',
			items: GUIDE_NAV.map((guide) => ({
				label: guide.navLabel,
				path: `/admin/info/tools/${guide.key}` as const
			}))
		}
	];
	const NAV_ITEMS = NAV_GROUPS.flatMap<(typeof NAV_GROUPS)[number]['items'][number]>(
		(group) => group.items
	);
	let currentLabel = $derived(
		NAV_ITEMS.find((item) => resolve(item.path) === page.url.pathname)?.label ??
			'Select a guide'
	);

	function navigateToGuide(path: string) {
		const item = NAV_ITEMS.find((item) => resolve(item.path) === path);
		if (item) void goto(resolve(item.path));
	}
</script>

<div class="bg-background flex min-h-svh flex-col">
	<div class="bg-card border-border border-b px-4 py-2 md:px-8">
		<h1 class="text-primary text-3xl font-semibold">Comhairle Admin Guide</h1>
	</div>
	<div class="px-4 py-6 md:px-8">
		<div class="flex flex-col gap-6 md:flex-row md:items-start md:gap-10 md:pb-10">
			<div class="md:hidden">
				<p class="text-muted-foreground mb-2 text-sm font-medium">Admin guide</p>
				<Select.Root
					type="single"
					value={page.url.pathname}
					onValueChange={navigateToGuide}
				>
					<Select.Trigger class="w-full" aria-label="Admin guide"
						>{currentLabel}</Select.Trigger
					>
					<Select.Content>
						{#each NAV_GROUPS as group (group.label)}
							<Select.Group>
								<Select.Label>{group.label}</Select.Label>
								{#each group.items as item (resolve(item.path))}<Select.Item
										value={resolve(item.path)}>{item.label}</Select.Item
									>{/each}
							</Select.Group>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>
			<nav class="hidden shrink-0 flex-col gap-4 md:flex md:w-56" aria-label="Admin guide">
				{#each NAV_GROUPS as group (group.label)}
					<Collapsible.Root open={true} class="group">
						<Collapsible.Trigger
							class="text-foreground hover:bg-muted flex w-full items-center justify-between rounded-lg px-3 py-2 text-base font-semibold"
						>
							{group.label}
							<ChevronDown
								class="size-4 transition-transform group-data-[state=closed]:-rotate-90"
								aria-hidden="true"
							/>
						</Collapsible.Trigger>
						<Collapsible.Content>
							<div class="mt-2 flex flex-col gap-2">
								{#each group.items as item (resolve(item.path))}
									{#if 'topics' in item}
										<Collapsible.Root open={true}>
											<div class="flex items-center gap-1">
												<Collapsible.Trigger
													class="text-foreground hover:bg-muted flex w-full items-center justify-between gap-2 rounded-lg px-3 py-2 text-left text-base font-medium [&[data-state=closed]>svg]:-rotate-90"
												>
													{item.label}
													<ChevronDown
														class="size-4 shrink-0 transition-transform"
														aria-hidden="true"
													/>
												</Collapsible.Trigger>

												<!-- <Collapsible.Trigger
													class="hover:bg-muted rounded-lg p-2 [&[data-state=closed]>svg]:-rotate-90"
													aria-label={`Toggle topics for ${item.label}`}
												>
													<ChevronDown
														class="size-4 transition-transform"
														aria-hidden="true"
													/>
												</Collapsible.Trigger> -->
											</div>

											<Collapsible.Content>
												<ul
													class="border-border ml-4 space-y-1 border-l pl-3"
												>
													{#each item.topics as topic (topic.path)}
														<li>
															<a
																href={resolve(topic.path)}
																class="text-foreground block rounded-lg px-2 py-2 text-base {page
																	.url.pathname ===
																resolve(topic.path)
																	? 'bg-muted'
																	: 'hover:bg-muted/60'}"
																aria-current={page.url.pathname ===
																resolve(topic.path)
																	? 'page'
																	: undefined}
															>
																{topic.label}
															</a>
														</li>
													{/each}
												</ul>
											</Collapsible.Content>
										</Collapsible.Root>
									{:else}
										<a
											href={resolve(item.path)}
											class="text-foreground inline-flex min-h-8 items-center rounded-xl px-3 py-1 text-base font-medium {page
												.url.pathname === resolve(item.path)
												? 'bg-muted'
												: 'hover:bg-muted/60'}"
											aria-current={page.url.pathname === resolve(item.path)
												? 'page'
												: undefined}>{item.label}</a
										>
									{/if}
								{/each}
							</div>
						</Collapsible.Content>
					</Collapsible.Root>
				{/each}
			</nav>
			{@render children()}
		</div>
	</div>
</div>
