<script lang="ts">
	import * as HoverCard from '$lib/components/ui/hover-card';
	import { Info, Image } from '@lucide/svelte';
	import ExampleDialog from './ExampleDialog.svelte';

	interface Props {
		info: string;
		example?: {
			title: string;
			src: string;
		};
	}

	let { info, example }: Props = $props();

	let open = $state<boolean>(false);
</script>

{#if example}
	<ExampleDialog bind:open {...example} />
{/if}
<HoverCard.Root openDelay={150} closeDelay={100}>
	<HoverCard.Trigger
		class="text-muted-foreground hover:text-foreground inline-flex cursor-help"
		aria-label="More information"
	>
		<Info class="size-4" />
	</HoverCard.Trigger>
	<HoverCard.Content class="w-72 text-sm" side="top" sideOffset={6}>
		<p>{info}</p>
		{#if example}
			<button
				type="button"
				onclick={() => void (open = true)}
				class="text-primary mt-3 inline-flex items-center gap-1 text-sm font-medium hover:underline"
			>
				<Image class="size-3.5" />
				See example
			</button>
		{/if}
	</HoverCard.Content>
</HoverCard.Root>
