<script lang="ts">
	import Lock from 'lucide-svelte/icons/lock';
	import * as Popover from '$lib/components/ui/popover';

	type Props = { title: string };
	let { title }: Props = $props();
</script>

<!-- The title is the conversation's name, not report text, so clicking it explains why it
	can't be edited here. -->
<div class="flex flex-col items-center gap-5">
	<p class="bg-accent text-primary rounded-full px-4 py-1 text-base font-medium">Final report</p>
	<Popover.Root>
		<Popover.Trigger>
			{#snippet child({ props })}
				<h1
					{...props}
					class="text-foreground hover:bg-accent/60 cursor-pointer rounded-xl px-3 text-center text-4xl leading-tight font-bold transition-colors"
				>
					{title}
				</h1>
			{/snippet}
		</Popover.Trigger>
		<Popover.Content class="w-[28rem] max-w-[calc(100vw-2rem)]">
			<p class="flex gap-2 text-base">
				<Lock class="text-muted-foreground mt-1 size-4 shrink-0" aria-hidden="true" />
				This is the conversation's name, so it's set with the rest of the conversation's details.
			</p>
		</Popover.Content>
	</Popover.Root>
</div>
