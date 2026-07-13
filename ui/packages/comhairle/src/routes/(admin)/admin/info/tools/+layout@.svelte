<script lang="ts">
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { GUIDE_NAV } from '$lib/tool_guides';

	// The `@` suffix resets this layout to the app-root layout (src/routes/+layout.svelte),
	// bypassing (admin)/+layout.svelte so the guide renders without the AdminNav app sidebar.
	// Auth still holds: it is enforced in (admin)/+layout.ts's load, which runs regardless of
	// this component reset. The guide's own tool sub-nav below is then the only left navigation.
	let { children }: LayoutProps = $props();

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
	     so the nav and logo stay far left while the copy stays readable. On mobile the nav
	     wraps to pills above the article; from md up it becomes the left rail. -->
	<div class="px-4 py-6 md:px-8">
		<div class="flex flex-col gap-6 md:flex-row md:items-start md:gap-10 md:pb-10">
			<nav
				class="flex w-full shrink-0 flex-row flex-wrap gap-2 md:w-48 md:flex-col md:flex-nowrap"
				aria-label="Tools"
			>
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
