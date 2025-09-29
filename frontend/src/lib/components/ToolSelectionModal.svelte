<script lang="ts">
	import { AvailableTools } from '$lib/avaliable_tools';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import type { Snippet } from 'svelte';
	type Props = {
		onSelection: (tool: string) => void;
		children?: Snippet;
		prompt: string;
	};
	let { onSelection, prompt, children }: Props = $props();
	let selected: string | null = $state(null);

	let open = $state(false);

	function submit() {
		open = false;
		if (selected) {
			onSelection(selected);
		}
	}
</script>

<Dialog.Root bind:open>
	<Dialog.DialogTrigger>
		{#if children}
			{@render children()}
		{/if}
	</Dialog.DialogTrigger>
	<Dialog.Content class="min-w-[90vw]">
		<Dialog.DialogHeader>
			<Dialog.DialogTitle>{prompt}</Dialog.DialogTitle>
		</Dialog.DialogHeader>
		<div class="grid grid-cols-3 gap-2">
			{#each AvailableTools as tool}
				<Card.Root class="border">
					<Card.Header>
						<div class="align-center flex flex-row justify-between">
							<h1 class="flex flex-row items-center gap-x-2 text-xl">
								{@render tool.icon({ size: 24, color: 'blue' })}
								{tool.name}
							</h1>
							<a href={`/admin/info/tools/${tool.infoLink}`}>Learn More</a>
						</div>
					</Card.Header>
					<Card.Content class="grow">
						<p>{tool.description}</p>
					</Card.Content>
					<Card.Footer class="flex flex-row justify-end">
						{#if tool.available}
							{#if selected === tool.name}
								<p class="text-primary py-2 font-bold">Selected</p>
							{:else}
								<Button variant="default" onclick={() => (selected = tool.name)}>Select</Button>
							{/if}
						{:else}
							<p class="text-[hsl(80, 52% 91%)] py-2 font-bold">Comming Soon</p>
						{/if}
					</Card.Footer>
				</Card.Root>
			{/each}
		</div>
		<Dialog.Footer>
			<Button class="secondary" onclick={submit} disabled={!selected}>Select</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
