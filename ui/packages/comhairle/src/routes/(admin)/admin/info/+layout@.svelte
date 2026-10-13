<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { ADMIN_GUIDE_NAV } from '$lib/admin_guides';
	import * as Select from '$lib/components/ui/select';
	import { GUIDE_NAV } from '$lib/tool_guides';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import { BookA, BookOpen, Blocks, ChevronDown, Folder, FolderOpen } from 'lucide-svelte';
	import Logo from '$lib/components/Logo.svelte';
	import { toolIcon } from '$lib/components/AdminGuide/toolIcons';
	import { IsMobile } from '$lib/hooks/is-mobile.svelte';

	type MobileGroup = {
		label: string;
		options: {
			value: string;
			label: string;
		}[];
	};

	let { children } = $props();

	const isMobile = new IsMobile();

	const NAV_GROUPS = [
		{
			label: 'How-to Comhairle',
			// Small primary-colour header shown above this group in the rail.
			section: 'Admin guide',
			icon: BookOpen,
			overview: null,
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
			section: 'Reference',
			icon: Blocks,
			// The group heading links to an overview of all the tools.
			overview: '/admin/info/tools' as const,
			items: GUIDE_NAV.map((guide) => ({
				key: guide.key,
				label: guide.navLabel,
				path: `/admin/info/tools/${guide.key}` as const
			}))
		}
	];
	const MOBILE_GROUPS: MobileGroup[] = [
		{
			label: 'Start here',
			options: [{ value: 'welcome', label: 'Welcome' }]
		},
		...ADMIN_GUIDE_NAV.map((guide) => ({
			label: guide.navLabel,
			options: guide.topics.map((topic) => ({
				value: `how-to/${guide.key}/${topic.key}`,
				label: topic.navLabel
			}))
		})),
		{
			label: 'Engagement tools',
			options: [
				{ value: 'tools', label: 'All engagement tools' },
				...GUIDE_NAV.map((tool) => ({ value: `tools/${tool.key}`, label: tool.navLabel }))
			]
		},
		{
			label: 'Reference',
			options: [{ value: 'glossary', label: 'Glossary' }]
		}
	];

	let isGlossary = $derived(page.route.id === '/(admin)/admin/info/glossary');
	let isWelcome = $derived(page.route.id === '/(admin)/admin/info/welcome');
	let isToolsOverview = $derived(page.route.id === '/(admin)/admin/info/tools');

	let currentValue = $derived(
		page.params.topic_id
			? `how-to/${page.params.guide_id}/${page.params.topic_id}`
			: page.params.tool_id
				? `tools/${page.params.tool_id}`
				: isGlossary
					? 'glossary'
					: isWelcome
						? 'welcome'
						: isToolsOverview
							? 'tools'
							: undefined
	);

	let currentLabel = $derived(
		MOBILE_GROUPS.flatMap((group) => group.options).find((o) => o.value === currentValue)
			?.label ?? 'Select a topic'
	);

	type NavItem = (typeof NAV_GROUPS)[number]['items'][number];

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
		if (section === 'tools' && !first) {
			void goto(resolve('/admin/info/tools'));
		} else if (section === 'welcome') {
			void goto(resolve('/admin/info/welcome'));
		} else if (section === 'glossary') {
			void goto(resolve('/admin/info/glossary'));
		} else if (section === 'how-to') {
			void goto(
				resolve('/(admin)/admin/info/how-to/[guide_id]/[topic_id]', {
					guide_id: first,
					topic_id: second
				})
			);
		} else {
			void goto(resolve('/(admin)/admin/info/tools/[tool_id]', { tool_id: first }));
		}
	}
</script>

<div class="bg-nav-background flex min-h-svh flex-col">
	<!-- <div class="bg-card border-border border-b px-4 py-2 md:px-8">
		<h1 class="text-primary my-2 text-2xl font-semibold">Comhairle Admin Guide</h1>
	</div> -->
	<div class="px-4 py-6 md:px-8">
		<div class="flex flex-col gap-6 md:flex-row md:items-start md:gap-10 md:pb-10">
			{#if isMobile.current}
				<!-- Same "The Comhairle Handbook" heading as the top of the desktop rail -->
				<a
					href={resolve('/admin/info/welcome')}
					class="text-primary hover:bg-muted -mx-3 mb-2 flex items-center gap-2 rounded-lg px-3 py-2 text-base font-semibold"
					aria-current={isWelcome ? 'page' : undefined}
				>
					<span class="fill-primary size-5 shrink-0 [&_svg]:size-full" aria-hidden="true"
						><Logo /></span
					>
					The Comhairle Handbook
				</a>
				<Select.Root type="single" value={currentValue} onValueChange={navigateTo}>
					<Select.Trigger class="w-full" aria-label="Comhairle Handbook"
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
			{:else}
				<nav
					class="flex shrink-0 flex-col gap-4 rounded-lg md:w-56"
					aria-label="Comhairle Handbook"
				>
					<a
						href={resolve('/admin/info/welcome')}
						class="flex items-center gap-2 rounded-lg px-3 py-2 text-base font-semibold {isWelcome
							? 'bg-accent text-primary'
							: 'text-primary hover:bg-muted'}"
						aria-current={isWelcome ? 'page' : undefined}
					>
						<span
							class="fill-primary size-5 shrink-0 [&_svg]:size-full"
							aria-hidden="true"><Logo /></span
						>
						The Comhairle Handbook
					</a>

					{#each NAV_GROUPS as group, i (group.label)}
						{#if group.section}
							<p
								class="text-primary px-3 text-xs font-semibold tracking-wide uppercase {i >
								0
									? 'mt-4'
									: 'mt-2'} -mb-2"
							>
								{group.section}
							</p>
						{/if}
						<Collapsible.Root
							open={group.items.some(isActiveItem) ||
								(!!group.overview && isToolsOverview)}
							class="group"
						>
							{#if group.overview}
								<!-- Heading links to the overview page; the chevron still opens and closes. -->
								<div
									class="flex items-center rounded-lg {isToolsOverview
										? 'bg-accent text-accent-foreground'
										: 'text-foreground hover:bg-muted'}"
								>
									<a
										href={resolve(group.overview)}
										class="flex flex-1 items-center gap-2 py-2 pl-3 text-base font-semibold"
										aria-current={isToolsOverview ? 'page' : undefined}
									>
										<group.icon class="size-4 shrink-0" aria-hidden="true" />
										{group.label}
									</a>
									<Collapsible.Trigger
										class="rounded-lg px-3 py-2"
										aria-label={`Show or hide ${group.label}`}
									>
										<ChevronDown
											class="size-4 transition-transform group-data-[state=closed]:-rotate-90"
											aria-hidden="true"
										/>
									</Collapsible.Trigger>
								</div>
							{:else}
								<Collapsible.Trigger
									class="text-foreground hover:bg-muted flex w-full items-center justify-between rounded-lg px-3 py-2 text-base font-semibold"
								>
									<span class="flex items-center gap-2">
										<group.icon class="size-4 shrink-0" aria-hidden="true" />
										{group.label}
									</span>
									<ChevronDown
										class="size-4 transition-transform group-data-[state=closed]:-rotate-90"
										aria-hidden="true"
									/>
								</Collapsible.Trigger>
							{/if}
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
															<!-- Same treatment as the engagement tool icons: accent circle,
															     primary with white lines for the guide you're in. -->
															<span
																class="text-primary flex size-6 shrink-0 items-center justify-center rounded-4xl {isActiveItem(
																	item
																)
																	? 'bg-primary text-primary-foreground'
																	: 'bg-accent text-accent-foreground'}"
															>
																<Folder
																	class="size-3.5 group-data-[state=open]/guide:hidden"
																	aria-hidden="true"
																/>
																<FolderOpen
																	class="hidden size-3.5 group-data-[state=open]/guide:block"
																	aria-hidden="true"
																/>
															</span>
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
											{@const Icon = toolIcon(item.key)}
											{@const active = isActiveItem(item)}
											<a
												href={resolve(item.path)}
												class="primary text-accent-foreground flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium {active
													? 'bg-accent text-accent-foreground'
													: 'text-foreground hover:bg-muted/60'}"
												aria-current={active ? 'page' : undefined}
											>
												<span
													class="text-primary flex size-6 shrink-0 items-center justify-center rounded-4xl {active
														? 'bg-primary text-primary-foreground'
														: 'bg-accent text-accent-foreground'}"
												>
													<Icon class="size-3.5" aria-hidden="true" />
												</span>
												{item.label}
											</a>
										{/if}
									{/each}
								</div>
							</Collapsible.Content>
						</Collapsible.Root>
					{/each}

					<!-- Reference, not a reading step, so it sits on its own below the guides. -->
					<a
						href={resolve('/admin/info/glossary')}
						class="flex items-center gap-2 rounded-lg px-3 py-2 text-base font-semibold {isGlossary
							? 'bg-accent text-accent-foreground'
							: 'text-foreground hover:bg-muted'}"
						aria-current={isGlossary ? 'page' : undefined}
					>
						<BookA class="size-4 shrink-0" aria-hidden="true" />
						Glossary
					</a>
				</nav>
			{/if}
			<div class="bg-card min-w-0 flex-1 rounded-xl">
				{@render children()}
			</div>
		</div>
	</div>
</div>
