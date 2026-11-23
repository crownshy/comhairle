<script lang="ts">
	import Check from '@lucide/svelte/icons/check';
	import Share2 from '@lucide/svelte/icons/share-2';
	import { Button } from '$lib/components/ui/button';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import { copyToClipboard } from '$lib/utils/copyToClipboard.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { REPORT_COLUMN_CLASS } from '$lib/reports/styles';

	type Props = {
		title: string;
		/** The report's own address, without a card in it, so a shared link opens at the start. */
		shareUrl: string;
	};

	let { title, shareUrl }: Props = $props();

	const clipboard = copyToClipboard();

	// Phones get the native share sheet. Desktop browsers mostly lack it, so they copy the link.
	async function share() {
		if (!navigator.share) {
			await clipboard.copy(shareUrl);
			return;
		}
		// Closing the share sheet rejects too, and that is not an error worth showing.
		await tryCatchAsync(() => navigator.share({ title, url: shareUrl }));
	}
</script>

<div class={cn(REPORT_COLUMN_CLASS, 'flex flex-col items-center gap-5 py-8')}>
	<div class="bg-accent text-primary flex size-40 items-center justify-center rounded-full">
		<Check class="size-20" strokeWidth={3} aria-hidden="true" />
	</div>
	<h1 class="text-foreground text-center text-4xl font-bold">{m.report_thank_you_title()}</h1>
	<div class="text-foreground flex flex-col gap-4 text-lg leading-7">
		<p>{m.report_thank_you_body()}</p>
		<p>{m.report_thank_you_hope()}</p>
	</div>

	<h2 class="text-foreground mt-5 text-center text-4xl font-bold">{m.report_next_steps()}</h2>
	<Button onclick={share} size="lg" class="h-12 w-full text-lg font-semibold">
		{#if clipboard.copied}
			<Check class="size-5" aria-hidden="true" />
			{m.report_link_copied()}
		{:else}
			<Share2 class="size-5" aria-hidden="true" />
			{m.report_share()}
		{/if}
	</Button>
</div>
