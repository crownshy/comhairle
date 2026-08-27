<script lang="ts">
	import { page } from '$app/state';
	import { Plus, Settings2 } from 'lucide-svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import TabStripShell from '$lib/components/TabStripShell.svelte';
	import type { WorkflowStepWithTranslations } from '@crownshy/api-client/api';

	let {
		conversationId,
		steps,
		onAddStep
	}: {
		conversationId: string;
		steps: WorkflowStepWithTranslations[] | undefined;
		onAddStep: () => void;
	} = $props();

	let basePath = $derived(`/admin/conversations/${conversationId}/design`);
	let orderedSteps = $derived(steps ? steps.toSorted((a, b) => a.stepOrder - b.stepOrder) : []);
	let loading = $derived(steps === undefined);
	let manageActive = $derived(page.url.pathname === basePath);

	function isStepActive(stepId: string, currentPath: string): boolean {
		return currentPath.startsWith(`${basePath}/step/${stepId}`);
	}
</script>

<TabStripShell ariaLabel="Workflow steps">
	<li>
		<a
			href={basePath}
			class="text-foreground inline-flex h-9 items-center gap-1.5 px-3.5 text-sm font-medium whitespace-nowrap transition-opacity"
			class:text-primary={manageActive}
			class:opacity-70={!manageActive}
			class:hover:opacity-100={!manageActive}
			aria-current={manageActive ? 'page' : undefined}
		>
			<Settings2 class="size-4" />
			Design
		</a>
	</li>
	{#if loading}
		{#each [1, 2, 3] as i (i)}
			<li class="px-3.5 py-1.5">
				<Skeleton class="h-5 w-24" />
			</li>
		{/each}
	{:else}
		{#each orderedSteps as step (step.id)}
			{@const active = isStepActive(step.id, page.url.pathname)}
			<li>
				<a
					href={`${basePath}/step/${step.id}`}
					title={step.name || 'Unnamed step'}
					class="text-foreground inline-flex h-9 max-w-[220px] items-center px-3.5 text-sm font-medium transition-opacity"
					class:text-primary={active}
					class:opacity-70={!active}
					class:hover:opacity-100={!active}
					aria-current={active ? 'page' : undefined}
				>
					<span class="truncate">{step.name || 'Unnamed step'}</span>
				</a>
			</li>
		{/each}
		<li>
			<button
				type="button"
				onclick={onAddStep}
				class="text-foreground/40 hover:text-foreground inline-flex h-9 items-center gap-1 px-3.5 text-sm font-medium whitespace-nowrap"
			>
				<Plus class="size-4" />
				Add step
			</button>
		</li>
	{/if}
</TabStripShell>
