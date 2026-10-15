<script lang="ts">
	import { ArrowRight } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import type { WorkspaceLink } from './workspaceLinks';
	import { TILE_HOVER } from './tileHover';

	let { link }: { link: WorkspaceLink } = $props();
</script>

<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
<a href={link.href} class="group">
	<Card.Root class="h-full justify-between gap-3 py-5 {TILE_HOVER}">
		<Card.Header class="px-5">
			<link.icon class="text-primary size-6" />
			<Card.Action>
				<ArrowRight
					class="text-muted-foreground group-hover:text-primary size-4 transition-transform group-hover:translate-x-0.5"
				/>
			</Card.Action>
		</Card.Header>
		<Card.Content class="px-5">
			<span class="text-base font-semibold">{link.label}</span>
		</Card.Content>
	</Card.Root>
</a>
