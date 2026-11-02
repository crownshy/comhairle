<script lang="ts">
	import CommonStepConfig from '$lib/components/CommonStepConfig/CommonStepConfig.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { stepConfigureTour } from './tour';

	useTour(() => stepConfigureTour);

	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step = $derived(data.step);
</script>

{#if step}
	<CommonStepConfig conversation_id={conversation.id} {conversation} {step} inline />
{/if}
