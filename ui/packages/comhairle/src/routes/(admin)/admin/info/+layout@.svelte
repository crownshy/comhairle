<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { ADMIN_GUIDE_NAV } from '$lib/admin_guides';
	import * as Select from '$lib/components/ui/select';
	import { GUIDE_NAV } from '$lib/tool_guides';
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();
	const NAV_GROUPS = [
		{
			label: 'Admin how-to',
			items: ADMIN_GUIDE_NAV.map((guide) => ({
				label: guide.navLabel,
				path: `/admin/info/how-to/${guide.key}` as const
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
			<nav class="hidden shrink-0 flex-col gap-6 md:flex md:w-56" aria-label="Admin guide">
				{#each NAV_GROUPS as group (group.label)}
					<div class="flex flex-col gap-2">
						<p class="text-muted-foreground mb-1 px-3 text-sm font-medium">
							{group.label}
						</p>
						{#each group.items as item (resolve(item.path))}
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
						{/each}
					</div>
				{/each}
			</nav>
			{@render children()}
		</div>
	</div>
</div>
