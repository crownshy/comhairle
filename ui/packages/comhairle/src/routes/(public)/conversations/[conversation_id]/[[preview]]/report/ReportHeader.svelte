<script lang="ts">
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import type { Attachment } from 'svelte/attachments';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import { REPORT_COLUMN_CLASS, REPORT_PILL_CLASS } from '$lib/reports/styles';

	type Props = {
		title: string;
		/** Back to the report's first screen. */
		startHref: string;
		/** One pill per page. Left out on screens that sit outside the pages. */
		pages?: { label: string; href: string }[];
		activePageIndex?: number;
	};

	let { title, startHref, pages = [], activePageIndex }: Props = $props();

	// The pill row scrolls sideways on a phone, so keep the current page's pill in view.
	const revealActivePill: Attachment<HTMLElement> = (pill) => {
		const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		pill.scrollIntoView({
			inline: 'center',
			block: 'nearest',
			behavior: reduceMotion ? 'auto' : 'smooth'
		});
	};
</script>

<!-- The hrefs arrive as props, built by cardHref from the current URL, so they are already
	resolved paths. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<header class={cn(REPORT_COLUMN_CLASS, 'flex flex-col gap-5 pt-5')}>
	<p class="flex min-w-0 items-center gap-2 text-base">
		<a
			href={startHref}
			class="bg-accent text-primary flex shrink-0 items-center gap-1 rounded-full py-0.5 ps-1 pe-3 font-medium"
		>
			<ArrowLeft class="size-5 rtl:-scale-x-100" aria-hidden="true" />
			{m.report_final_report()}
		</a>
		<span class="text-foreground truncate">{title}</span>
	</p>

	{#if pages.length > 0}
		<nav
			aria-label={m.report_sections()}
			class="-mx-5 overflow-x-auto px-5 [scrollbar-width:none]"
		>
			<ul class="flex w-max gap-2">
				{#each pages as reportPage, index (index)}
					{@const active = index === activePageIndex}
					<li>
						<a
							href={reportPage.href}
							aria-current={active ? 'location' : undefined}
							class={cn(
								REPORT_PILL_CLASS,
								'block truncate',
								active
									? 'bg-secondary-foreground text-primary-foreground'
									: 'bg-accent text-accent-foreground'
							)}
							{@attach active && revealActivePill}
						>
							{reportPage.label}
						</a>
					</li>
				{/each}
			</ul>
		</nav>
	{/if}
</header>
