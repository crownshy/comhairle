<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { EyeOpen, EyeNone } from 'svelte-radix';
	import { m } from '$lib/paraglide/messages';

	let { value = $bindable(), ...restProps } = $props();

	let visible = $state(false);
</script>

<span class="relative flex">
	<Input bind:value {...restProps} type={visible ? 'text' : 'password'} />
	<button
		class="focus-visible:ring-ring absolute top-1/2 right-2 z-10 flex size-6 -translate-y-1/2 items-center justify-center rounded-sm focus-visible:ring-2 focus-visible:outline-none"
		type="button"
		aria-label={m.show_password()}
		aria-pressed={visible}
		onclick={() => (visible = !visible)}
	>
		{#if visible}
			<EyeNone class="h-4 w-4" aria-hidden="true" />
		{:else}
			<EyeOpen class="h-4 w-4" aria-hidden="true" />
		{/if}
	</button>
</span>
