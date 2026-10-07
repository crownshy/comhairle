<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { ADMIN_GUIDE_NAV } from '$lib/admin_guides';
	import * as Select from '$lib/components/ui/select';
	import { GUIDE_NAV } from '$lib/tool_guides';
	import type { LayoutProps } from './$types';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import { ChevronDown, Folder, FolderOpen } from 'lucide-svelte';

	let { children }: LayoutProps = $props();
	const NAV_GROUPS = [
		{
			label: 'Admin how-to',
			items: ADMIN_GUIDE_NAV.map((guide) => ({
				key: guide.key,
				label: guide.navLabel,
				path: `/admin/info/how-to/${guide.key}` as const,
				topics: guide.topics.map((topic) => ({
					key: topic.key,
					label: topic.navLabel,
					path: `/admin/info/how-to/${guide.key}/${topic.key}` as const
				}))
			}))
		},
		{
			label: 'Engagement tools',
			items: GUIDE_NAV.map((guide) => ({
				key: guide.key,
				label: guide.navLabel,
				path: `/admin/info/tools/${guide.key}` as const
			}))
		}
	];
	const MOBILE_GROUPS = [
		...ADMIN_GUIDE_NAV.map((guide) => ({
			label: guide.navLabel,
			options: guide.topics.map((topic) => ({
				value: `how-to/${guide.key}/${topic.key}`,
				label: topic.navLabel
			}))
		})),
		{
			label: 'Engagement tools',
			options: GUIDE_NAV.map((tool) => ({ value: `tools/${tool.key}`, label: tool.navLabel }))
		}
	];

	let currentValue = $derived(
		page.params.topic_id
			? `how-to/${page.params.guide_id}/${page.params.topic_id}`
			: page.params.tool_id
				? `tools/${page.params.tool_id}`
				: undefined
	);

	let currentLabel = $derived(
		MOBILE_GROUPS.flatMap((group) => group.options).find((o) => o.value === currentValue)
			?.label ?? 'Select a topic'
	);

	type NavItem = (typeof NAV_ITEMS)[number];

	function isActiveItem(item: NavItem) {
		return 'topics' in item
			? page.params.guide_id === item.key
			: page.params.tool_id === item.key;
	}

	function isActiveTopic(guideKey: string, topicKey: string) {
		return page.params.guide_id === guideKey && page.params.topic_id === topicKey;
	}

	function navigateTo(value: string) {
		const [section, first, second] = value.split('/');
		if (section === 'how-to') {
			void goto(
				resolve('/admin/info/how-to/[guide_id]/[topic_id]', {
					guide_id: first,
					topic_id: second
				})
			);
		} else {
			void goto(resolve('/admin/info/tools/[tool_id]', { tool_id: first }));
		}
	}
</script>

<div class="bg-nav-background flex min-h-svh flex-col">
	<div class="bg-card border-border border-b px-4 py-2 md:px-8">
		<h1 class="text-primary my-2 text-2xl font-semibold">Comhairle Admin Guide</h1>
	</div>
	<div class="px-4 py-6 md:px-8">
		<div class="flex flex-col gap-6 md:flex-row md:items-start md:gap-10 md:pb-10">
			<div class="md:hidden">
				<p class="text-muted-foreground mb-2 text-sm font-medium">Admin guide</p>
				<Select.Root type="single" value={currentValue} onValueChange={navigateTo}>
					<Select.Trigger class="w-full" aria-label="Admin guide"
						>{currentLabel}</Select.Trigger
					>
					<Select.Content class="max-h-[70vh]">
						{#each MOBILE_GROUPS as group (group.label)}
							<Select.Group>
								<Select.Label>{group.label}</Select.Label>
								{#each group.options as option (option.value)}
									<Select.Item value={option.value} label={option.label}
										>{option.label}</Select.Item
									>
								{/each}
							</Select.Group>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>
			<nav
				class="hidden shrink-0 flex-col gap-4 rounded-lg md:flex md:w-56"
				aria-label="Admin guide"
			>
				{#each NAV_GROUPS as group (group.label)}
					<Collapsible.Root open={group.items.some(isActiveItem)} class="group">
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
										<Collapsible.Root open={isActiveItem(item)}>
											<div class="flex items-center gap-1">
												<Collapsible.Trigger
													class="group/guide text-foreground hover:bg-muted flex w-full items-center justify-between gap-2 rounded-lg px-3 py-2 text-left text-sm font-medium [&[data-state=closed]>svg]:-rotate-90"
												>
													<span class="flex items-center gap-2">
														<Folder
															class="text-muted-foreground size-4 shrink-0 group-data-[state=open]/guide:hidden"
															aria-hidden="true"
														/>
														<FolderOpen
															class="text-muted-foreground hidden size-4 shrink-0 group-data-[state=open]/guide:block"
															aria-hidden="true"
														/>
														{item.label}
													</span>
													<ChevronDown
														class="size-4 shrink-0 transition-transform"
														aria-hidden="true"
													/>
												</Collapsible.Trigger>
											</div>

											<Collapsible.Content>
												<ul
													class="border-border ml-4 space-y-1 border-l pl-3"
												>
													{#each item.topics as topic (topic.path)}
														<li>
															<a
																href={resolve(topic.path)}
																class="block rounded-lg px-2 py-2 text-sm {isActiveTopic(
																	item.key,
																	topic.key
																)
																	? 'bg-accent text-accent-foreground'
																	: 'text-foreground hover:bg-muted/60'}"
																aria-current={isActiveTopic(
																	item.key,
																	topic.key
																)
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
											class="inline-flex min-h-8 items-center rounded-xl px-3 py-1 text-base font-medium {isActiveItem(
												item
											)
												? 'bg-accent text-accent-foreground'
												: 'text-foreground hover:bg-muted/60'}"
											aria-current={isActiveItem(item) ? 'page' : undefined}
											>{item.label}</a
										>
									{/if}
								{/each}
							</div>
						</Collapsible.Content>
					</Collapsible.Root>
				{/each}
			</nav>
			<div class="bg-card min-w-0 flex-1 rounded-xl">
				{@render children()}
			</div>
		</div>
	</div>
</div>
