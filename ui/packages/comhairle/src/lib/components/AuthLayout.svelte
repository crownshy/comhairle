<script lang="ts">
	import AuthGradient from '$lib/components/AuthGradient.svelte';
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<div class="flex min-h-screen flex-col lg:flex-row">
	<!-- Gradient: header on mobile, side panel on desktop -->
	<div class="relative h-[20vh] shrink-0 lg:h-auto lg:w-1/2">
		<AuthGradient mode="fill" showLogo={false}>
			<div class="flex flex-col items-center gap-2 lg:gap-4">
				<ComhairleLogo href="/" logoSize="md" color="text-white" class="lg:hidden" />
				<ComhairleLogo href="/" logoSize="lg" color="text-white" class="hidden lg:flex" />
				<p class="text-center text-sm font-semibold text-white lg:text-2xl lg:leading-7">
					Understand. Contribute. Influence.
				</p>
			</div>
		</AuthGradient>
	</div>

	<!-- Card on mobile (overlaps gradient), form panel on desktop -->
	<div
		class="bg-card lg:bg-background relative z-10 -mt-6 mb-10 flex flex-1 flex-col rounded-t-3xl sm:mb-0 lg:mt-0 lg:w-1/2 lg:rounded-none"
	>
		<!-- Form content -->
		<div
			class="flex flex-1 items-start justify-center px-5 pt-8 lg:items-center lg:px-6 lg:py-12"
		>
			<div class="w-full max-w-md">
				{@render children()}
			</div>
		</div>
	</div>
</div>
