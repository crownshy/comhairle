<script lang="ts">
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { GUIDE_NAV } from '$lib/tool_guides';

	let { children }: LayoutProps = $props();

	function isActive(key: string): boolean {
		return page.url.pathname === `/admin/info/tools/${key}`;
	}
</script>

<div class="bg-background flex min-h-full flex-col">
	<!-- Header -->
	<div class="bg-card border-border border-b px-5 py-2">
		<h1 class="text-primary text-2xl font-semibold">Comhairle Tools Guide</h1>
	</div>

	<!-- Body: left nav + article -->
	<div class="mx-auto w-full max-w-[1280px] p-5">
		<div class="flex items-start gap-10 pb-10">
			<nav class="flex w-32 shrink-0 flex-col gap-2" aria-label="Tools">
				{#each GUIDE_NAV as tool (tool.key)}
					<a
						href={`/admin/info/tools/${tool.key}`}
						class="text-foreground inline-flex h-8 items-center rounded-full px-3 text-sm font-medium {isActive(
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
