<script lang="ts">
	import type { WorkflowStep } from '$lib/api/api.js';
	import { Header } from '$lib/components/ui/alert-dialog/index.js';
	let { data } = $props();
	import { draggable, droppable, type DragDropState } from '@thisux/sveltednd';
	import { Plus, Share2, Pencil } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import Button from '$lib/components/ui/button/button.svelte';
	let conversation = $derived(data.conversation);
	let workflow_steps = $derived(data.workflow_steps);

	console.log('workflow steps ', workflow_steps);
	function handleDrop(state: DragDropState<WorkflowStep>) {
		console.log('drop ', state);
	}
</script>

<h1 class="mb-10 flex flex-row items-center gap-2 text-4xl"><Share2 /> Design</h1>
<h2 class="text-2xl">Process steps</h2>

{#each workflow_steps as step}
	<!-- Make items draggable -->
	<Card.Root class="border-">
		<Card.Header>
			{step.name}
		</Card.Header>
		<Card.Footer>
			<Button
				href={`/admin/conversations/${conversation.id}/design/step/${step.id}`}
				class="primray">Configure step</Button
			>
		</Card.Footer>
	</Card.Root>
{/each}

<Button><Plus /> Add Step</Button>

<style>
	.svelte-dnd-dragging {
		opacity: 0.5;
		cursor: grabbing;
	}
	.svelte-dnd-drop-target {
		outline: 2px dashed #4caf50;
	}
</style>
