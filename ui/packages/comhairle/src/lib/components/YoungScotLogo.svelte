<script lang="ts">
	type Props = {
		showText?: boolean;
		logoSize?: 'sm' | 'md' | 'lg';
		/** Wordmark lines. `subtitle` is the Gaelic line; empty drops it. */
		title?: string;
		subtitle?: string;
		domain?: string;
		class?: string;
	};

	let {
		showText = true,
		logoSize = 'md',
		title = 'Young Scot',
		subtitle = '',
		domain = 'young.scot',
		class: className = ''
	}: Props = $props();

	// Flag height and wordmark size are paired so the two blocks read as one lockup.
	const flagSizeMap = {
		sm: 'h-8',
		md: 'h-10',
		lg: 'h-12'
	};

	const textSizeMap = {
		sm: 'text-xs',
		md: 'text-sm',
		lg: 'text-base'
	};
</script>

<span
	role="img"
	aria-label={[title, subtitle, domain].filter(Boolean).join(', ')}
	class="inline-flex shrink-0 items-center gap-2 {className}"
>
	<svg
		viewBox="0 0 100 60"
		class="{flagSizeMap[logoSize]} w-auto shrink-0"
		xmlns="http://www.w3.org/2000/svg"
	>
		<rect width="100" height="60" fill="#0065bd" />
		<!-- Saltire as one polygon rather than two clipped strokes, so the component carries no
			clip-path id to collide when two logos share a page. Band width is a fifth of the
			hoist, measured perpendicular to the diagonal. -->
		<path
			d="M0 0 11.66 0 50 23 88.34 0 100 0 100 7 61.66 30 100 53 100 60 88.34 60 50 37 11.66 60 0 60 0 53 38.34 30 0 7Z"
			fill="#ffffff"
		/>
	</svg>

	{#if showText}
		<!-- Below sm the flag stands alone: three lines of small type eat width a phone header
			does not have. -->
		<span class="hidden items-stretch gap-2 sm:flex">
			<span class="w-px bg-current opacity-25"></span>
			<span class="flex flex-col leading-[1.15] whitespace-nowrap {textSizeMap[logoSize]}">
				<span class="font-semibold">{title}</span>
				{#if subtitle}
					<span class="opacity-60">{subtitle}</span>
				{/if}
				<span class="font-semibold">{domain}</span>
			</span>
		</span>
	{/if}
</span>
