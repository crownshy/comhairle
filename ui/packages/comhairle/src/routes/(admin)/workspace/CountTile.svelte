<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import SeeAllLabel from './SeeAllLabel.svelte';
	import { TILE_HOVER } from './tileHover';

	type Props = { label: string; count: number | null; href: string };
	let { label, count, href }: Props = $props();
</script>

<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
<a {href} class="group">
	<Card.Root class="h-full gap-2 py-5 {TILE_HOVER}">
		<Card.Header class="px-5">
			<Card.Description class="text-base">{label}</Card.Description>
			<Card.Action><SeeAllLabel /></Card.Action>
		</Card.Header>
		<Card.Content class="px-5">
			<span class="text-4xl font-semibold tabular-nums">{count ?? '–'}</span>
		</Card.Content>
	</Card.Root>
</a>
