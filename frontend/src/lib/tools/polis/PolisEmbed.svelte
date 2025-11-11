<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { fade } from 'svelte/transition';

	type Props = {
		polis_id: string;
		polis_url: string;
		user_id: string;
		onDone: () => void;
	};

	$effect(() => {
		setTimeout(() => {
			showThankYou = true;
		}, 180000);
	});
	let showThankYou = $state(false);

	let { polis_id, polis_url, user_id, onDone }: Props = $props();
	let url = $derived(`${polis_url}/${polis_id}?xid=${user_id}`);
</script>

{#if showThankYou}
	<div transition:fade={{ duration: 1000 }} class="prose mx-auto mb-20">
		<h3>This interactive discussion doesn't have a fixed end</h3>
		<p>
			You can keep voting or adding statements as long as you link. You're welcome to return later
			when others have contributed more. Thank you for taking part.
		</p>
		<Button variant="secondary" onclick={onDone}>I'm Done</Button>
	</div>
{/if}

<iframe
	src={url}
	title="Polis poll"
	scrolling="no"
	class="min-h-[500px] border-none"
	style="width:100%;height:100%"
></iframe>
