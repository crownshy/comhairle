<script lang="ts">
	import { page } from '$app/stores';
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';
	import { conversationSteps } from '$lib/config/conversation-steps';
	import type { WorkflowStep } from '$lib/api/api';

	interface Props {
		workflowSteps?: WorkflowStep[];
	}

	let { workflowSteps = [] }: Props = $props();

	let conversationId = $derived($page.params.conversation_id);
	let currentPath = $derived($page.url.pathname);
	let currentStepId = $derived($page.params.step_id);

	let isOnDesignPage = $derived(currentPath.includes('/design') && !currentPath.includes('/design/step/'));
	let isOnDesignStepPage = $derived(currentPath.includes('/design/step/'));

	let designIndex = conversationSteps.findIndex(s => s.path === 'design');
	let recruitIndex = conversationSteps.findIndex(s => s.path === 'invites');

	interface NavItem {
		name: string;
		label: string;
		url: string;
		isDesignStep?: boolean;
	}

	let navItems = $derived.by(() => {
		const items: NavItem[] = [];
		const baseUrl = `/admin/conversations/${conversationId}`;

		for (let i = 0; i < designIndex; i++) {
			const step = conversationSteps[i];
			items.push({
				name: step.name,
				label: getLabel(step.name),
				url: `${baseUrl}/${step.path}`
			});
		}

		items.push({
			name: 'Design',
			label: 'Next: setup the participation steps',
			url: `${baseUrl}/design`
		});

		if (workflowSteps && workflowSteps.length > 0) {
			const sortedSteps = [...workflowSteps].sort((a, b) => a.step_order - b.step_order);
			sortedSteps.forEach((ws, idx) => {
				items.push({
					name: ws.name,
					label: `Next: configure ${ws.name}`,
					url: `${baseUrl}/design/step/${ws.id}`,
					isDesignStep: true
				});
			});
		}

		for (let i = recruitIndex; i < conversationSteps.length; i++) {
			const step = conversationSteps[i];
			items.push({
				name: step.name,
				label: getLabel(step.name),
				url: `${baseUrl}/${step.path}`
			});
		}

		return items;
	});

	function getLabel(stepName: string): string {
		const labels: Record<string, string> = {
			'Configure': 'Next: configure your conversation',
			'Design': 'Next: setup the participation steps',
			'Recruit': 'Next: invite participants',
			'Monitor': 'Next: monitor participation',
			'Moderate': 'Next: moderate content',
			'Knowledge base': 'Next: manage knowledge base',
			'Notify': 'Next: setup notifications',
			'Report': 'Next: generate reports'
		};
		return labels[stepName] || `Next: ${stepName}`;
	}

	let currentIndex = $derived.by(() => {
		if (isOnDesignStepPage && currentStepId) {
			return navItems.findIndex(item => item.url.includes(`/design/step/${currentStepId}`));
		}
		if (isOnDesignPage) {
			return navItems.findIndex(item => item.url.endsWith('/design'));
		}
		const pathParts = currentPath.split('/');
		return navItems.findIndex(item => {
			const itemParts = item.url.split('/');
			const lastPart = itemParts[itemParts.length - 1];
			return pathParts.includes(lastPart) && !item.isDesignStep;
		});
	});

	let prevItem = $derived(currentIndex > 0 ? navItems[currentIndex - 1] : null);
	let nextItem = $derived(currentIndex < navItems.length - 1 ? navItems[currentIndex + 1] : null);
</script>

<div class="flex justify-end items-center gap-4">
	{#if prevItem}
		<span class="text-sm text-muted-foreground hidden sm:inline">
			Previous: {prevItem.name}
		</span>
	{/if}
	<a
		href={prevItem ? prevItem.url : '#'}
		class="h-10 w-10 rounded-full shadow-sm border flex justify-center items-center transition-colors
			{prevItem ? 'hover:bg-muted cursor-pointer' : 'opacity-30 cursor-not-allowed pointer-events-none'}"
		aria-label={prevItem ? `Go to ${prevItem.name}` : 'No previous step'}
		title={prevItem?.name}
	>
		<ChevronLeft class="w-5 h-5" />
	</a>
	<a
		href={nextItem ? nextItem.url : '#'}
		class="h-10 w-10 rounded-full shadow-sm border flex justify-center items-center transition-colors
			{nextItem ? 'hover:bg-muted cursor-pointer' : 'opacity-30 cursor-not-allowed pointer-events-none'}"
		aria-label={nextItem ? `Go to ${nextItem.name}` : 'No next step'}
		title={nextItem?.name}
	>
		<ChevronRight class="w-5 h-5" />
	</a>
	{#if nextItem}
		<span class="text-sm text-muted-foreground hidden sm:inline">
			{nextItem.label}
		</span>
	{/if}
</div>
