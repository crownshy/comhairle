<script lang="ts">
	import Lock from 'lucide-svelte/icons/lock';
	import { Button } from '$lib/components/ui/button';
	import * as Popover from '$lib/components/ui/popover';

	type Props = { title: string; conversationId: string };
	let { title, conversationId }: Props = $props();
</script>

<!-- The title is the conversation's name, not report text, so clicking it explains where to
	change it instead of making it editable here. -->
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
		<Popover.Content class="flex w-80 flex-col gap-3">
			<p class="flex gap-2 text-base">
				<Lock class="text-muted-foreground mt-1 size-4 shrink-0" />
				This is the conversation's name, so it's set with the rest of the conversation's details.
			</p>
			<Button variant="outline" href="/admin/conversations/{conversationId}/configure">
				Change it in Configure
			</Button>
		</Popover.Content>
	</Popover.Root>
</div>
