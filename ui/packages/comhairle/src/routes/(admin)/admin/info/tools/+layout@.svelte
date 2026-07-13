<script lang="ts">
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { GUIDE_NAV } from '$lib/tool_guides';
	import * as Select from '$lib/components/ui/select';

	// The `@` suffix resets this layout to the app-root layout (src/routes/+layout.svelte),
	// bypassing (admin)/+layout.svelte so the guide renders without the AdminNav app sidebar.
	// Auth still holds: it is enforced in (admin)/+layout.ts's load, which runs regardless of
	// this component reset. The guide's own tool sub-nav below is then the only left navigation.
	let { children }: LayoutProps = $props();

	// The active tool's key is the last path segment (matches the hrefs below).
	let currentKey = $derived(page.url.pathname.split('/').filter(Boolean).pop() ?? '');
	let currentLabel = $derived(
		GUIDE_NAV.find((tool) => tool.key === currentKey)?.navLabel ?? 'Select a tool'
	);

	function isActive(key: string): boolean {
		return page.url.pathname === `/admin/info/tools/${key}`;
	}
</script>

<div class="bg-background flex min-h-svh flex-col">
	<!-- Header: full-bleed bar. Logo hugs the left edge (shared px with the body so it
	     lines up with the nav rail below). -->
	<div class="bg-card border-border border-b px-4 py-2 md:px-8">
		<h1 class="text-primary text-3xl font-semibold">Comhairle Tools Guide</h1>
	</div>

	<!-- Body: left nav + article, left-aligned to the window (no centered max-width).
	     The reading measure lives on the article itself (GuideArticle caps its own width),
	     so the nav and logo stay far left while the copy stays readable. Below md the nav
	     collapses into a Select dropdown above the article; from md up it is the left rail. -->
	<div class="px-4 py-6 md:px-8">
		<div class="flex flex-col gap-6 md:flex-row md:items-start md:gap-10 md:pb-10">
			<!-- Mobile: dropdown nav (the wrapping-pill rail is too tall on a phone) -->
			<div class="md:hidden">
				<Select.Root
					type="single"
					value={currentKey}
					onValueChange={(value) => goto(`/admin/info/tools/${value}`)}
				>
					<Select.Trigger class="w-full" aria-label="Tools">{currentLabel}</Select.Trigger
					>
					<Select.Content>
						{#each GUIDE_NAV as tool (tool.key)}
							<Select.Item value={tool.key}>{tool.navLabel}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>

			<!-- Desktop: left rail -->
			<nav class="hidden shrink-0 flex-col gap-2 md:flex md:w-48" aria-label="Tools">
				{#each GUIDE_NAV as tool (tool.key)}
					<a
						href={`/admin/info/tools/${tool.key}`}
						class="text-foreground inline-flex h-8 shrink-0 items-center rounded-full px-3 text-base font-medium whitespace-nowrap {isActive(
							tool.key
						)
							? 'bg-muted'
							: 'hover:bg-muted/60'}"
						aria-current={isActive(tool.key) ? 'page' : undefined}
					>
						{tool.navLabel}
					</a>
				{/each}
			</nav>

			{@render children()}
		</div>
	</div>
</div>
