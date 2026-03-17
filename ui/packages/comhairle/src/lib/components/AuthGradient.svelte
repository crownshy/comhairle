<script lang="ts">
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';

	type Props = {
		showLogo?: boolean;
		children?: import('svelte').Snippet;
		mode?: 'side' | 'full' | 'fill';
	};

	let { showLogo = true, children, mode = 'side' }: Props = $props();
</script>

<div
	class="auth-gradient relative flex items-center justify-center overflow-hidden {mode === 'side'
		? 'h-[20vh] w-full shrink-0 px-6 py-8 lg:h-auto lg:min-h-0 lg:w-1/2 lg:flex-1 lg:px-8 lg:py-0'
		: mode === 'full'
			? 'min-h-screen flex-1 px-6 py-12 lg:px-8 lg:py-0'
			: 'h-full w-full'}"
>
	<div class="absolute inset-0" style="background-color: var(--auth-gradient-base);">
		<div
			class="absolute inset-0"
			style="
				background:
					radial-gradient(ellipse 70% 60% at 55% 65%, color-mix(in srgb, var(--auth-gradient-1) 45%, transparent) 0%, transparent 60%),
					radial-gradient(ellipse 60% 50% at 75% 70%, color-mix(in srgb, var(--auth-gradient-2) 35%, transparent) 0%, transparent 55%),
					radial-gradient(ellipse 65% 55% at 45% 45%, color-mix(in srgb, var(--auth-gradient-2) 8%, transparent) 0%, transparent 50%),
					radial-gradient(ellipse 70% 70% at 25% 55%, color-mix(in srgb, var(--auth-gradient-3) 35%, transparent) 0%, transparent 55%),
					radial-gradient(ellipse 80% 80% at 50% 50%, color-mix(in srgb, var(--auth-gradient-4) 25%, transparent) 0%, transparent 70%);
			"
		></div>
		<!-- Noise texture overlay -->
		<div class="absolute inset-0 opacity-[0.30] mix-blend-overlay">
			<svg class="h-full w-full" xmlns="http://www.w3.org/2000/svg">
				<filter id="auth-noise">
					<feTurbulence
						type="fractalNoise"
						baseFrequency="0.65"
						numOctaves="3"
						stitchTiles="stitch"
					/>
					<feColorMatrix type="saturate" values="0" />
				</filter>
				<rect width="100%" height="100%" filter="url(#auth-noise)" />
			</svg>
		</div>
	</div>

	<!-- Shadow separator on mobile for side mode -->
	{#if mode === 'side'}
		<div
			class="absolute inset-x-0 bottom-0 h-4 bg-linear-to-b from-transparent to-black/10 lg:hidden"
		></div>
	{/if}

	<!-- Content -->
	<div class="relative z-10 flex flex-col items-center gap-4">
		{#if children}
			{@render children()}
		{:else if showLogo}
			<ComhairleLogo href="/" logoSize={mode === 'side' ? 'md' : 'lg'} color="text-white" />
			<p
				class="text-center text-base leading-6 font-semibold text-white lg:text-2xl lg:leading-7"
			>
				Understand. Contribute. Influence
			</p>
		{/if}
	</div>
</div>
