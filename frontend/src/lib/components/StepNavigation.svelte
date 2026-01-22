<script lang="ts">
	import { page } from '$app/stores';
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';
	import { conversationSteps } from '$lib/config/conversation-steps';

	let conversationId = $derived($page.params.conversation_id);
	let currentPath = $derived($page.url.pathname);

	let currentIndex = $derived.by(() => {
		const pathParts = currentPath.split('/');
		return conversationSteps.findIndex(s => pathParts.includes(s.path));
	});

	let prevStep = $derived(currentIndex > 0 ? conversationSteps[currentIndex - 1] : null);
	let nextStep = $derived(currentIndex < conversationSteps.length - 1 ? conversationSteps[currentIndex + 1] : null);

	function getStepUrl(step: { path: string }) {
		return `/admin/conversations/${conversationId}/${step.path}`;
	}
</script>

<div class="flex justify-end items-center gap-2.5">
	<a
		href={prevStep ? getStepUrl(prevStep) : '#'}
		class="h-10 w-10 rounded-full shadow-sm border flex justify-center items-center transition-colors
			{prevStep ? 'hover:bg-muted cursor-pointer' : 'opacity-30 cursor-not-allowed pointer-events-none'}"
		aria-label={prevStep ? `Go to ${prevStep.name}` : 'No previous step'}
		title={prevStep?.name}
	>
		<ChevronLeft class="w-5 h-5" />
	</a>
	<a
		href={nextStep ? getStepUrl(nextStep) : '#'}
		class="h-10 w-10 rounded-full shadow-sm border flex justify-center items-center transition-colors
			{nextStep ? 'hover:bg-muted cursor-pointer' : 'opacity-30 cursor-not-allowed pointer-events-none'}"
		aria-label={nextStep ? `Go to ${nextStep.name}` : 'No next step'}
		title={nextStep?.name}
	>
		<ChevronRight class="w-5 h-5" />
	</a>
</div>
