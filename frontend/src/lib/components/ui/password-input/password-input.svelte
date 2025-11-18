<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { EyeOpen, EyeNone } from 'svelte-radix';

	let props = $props();

	let { formData } = props;

	let visible = $state(false);
</script>

<span class="relative flex">
	<Input type={visible ? 'text' : 'password'} bind:value={$formData.password} {...props} />
	<button
		class="absolute top-1/2 right-1 z-10 -translate-1/2"
		type="button"
		onclick={() => (visible = !visible)}
	>
		{#if visible}
			<EyeNone class="h-4 w-4" />
		{:else}
			<EyeOpen class="h-4 w-4" />
		{/if}
	</button>
</span>
