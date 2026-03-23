<script lang="ts">
	import { page } from '$app/state';
	import Button from '$lib/components/ui/button/button.svelte';
	import { ArrowLeft } from 'lucide-svelte';

	let status = $derived(page.status);
	let message = $derived(page.error?.message ?? 'Something went wrong');
</script>

<div class="relative flex min-h-[60vh] flex-col items-center justify-center overflow-hidden py-24">
	{#if status === 404}
		<span
			class="text-primary/10 pointer-events-none absolute hidden text-[500px] leading-none font-bold select-none md:block"
		>
			404
		</span>
		<span
			class="text-primary/15 relative mb-4 block text-8xl leading-none font-bold select-none md:hidden"
		>
			404
		</span>
	{/if}

	<div class="relative z-10 flex max-w-[640px] flex-col items-center gap-8 text-center">
		<div class="flex flex-col gap-2">
			<h1 class="text-foreground text-3xl font-semibold">
				{#if status === 404}
					Page not found
				{:else}
					Something went wrong
				{/if}
			</h1>
			<p class="text-muted-foreground text-lg font-semibold">
				{#if status === 404}
					Oops! The page you are looking for does not exist. It might have been moved or
					deleted.
				{:else}
					{message}
				{/if}
			</p>
		</div>

		<Button href="/" variant="primaryDark" size="lg" class="h-auto gap-2 px-6 py-4 text-lg">
			<ArrowLeft class="size-5" />
			Back to homepage
		</Button>
	</div>
</div>
