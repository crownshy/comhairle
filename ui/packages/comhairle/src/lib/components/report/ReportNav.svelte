<script lang="ts">
	import ArrowDown from '@lucide/svelte/icons/arrow-down';
	import ArrowUp from '@lucide/svelte/icons/arrow-up';

	interface Props {
		sections: { id: string; label: string }[];
	}

	let { sections }: Props = $props();

	const navSections = $derived([...sections, { id: 'back-to-top', label: 'Back to Top' }]);

	let nextIndex = $state(0);
	let hidden = $state(false);

	const nextSection = $derived(navSections[nextIndex] ?? null);
	const isBackToTop = $derived(nextSection?.id === 'back-to-top');

	function scrollToNext() {
		if (!nextSection) return;
		if (isBackToTop) {
			window.scrollTo({ top: 0, behavior: 'smooth' });
			return;
		}
		const el = document.getElementById(nextSection.id);
		if (el) {
			el.scrollIntoView({ behavior: 'smooth', block: 'start' });
		}
	}

	function handleScroll() {
		let passed = 0;
		for (let i = 0; i < sections.length; i++) {
			const el = document.getElementById(sections[i].id);
			if (el && el.getBoundingClientRect().top <= 150) {
				passed = i + 1;
			}
		}
		nextIndex = passed;
		hidden = false;
	}

	$effect(() => {
		window.addEventListener('scroll', handleScroll, { passive: true });
		handleScroll();
		return () => window.removeEventListener('scroll', handleScroll);
	});
</script>

{#if nextSection}
	<button
		onclick={scrollToNext}
		class="bg-background fixed bottom-4 left-1/2 z-50
			inline-flex -translate-x-1/2 items-center gap-1.5 rounded-full px-3 py-2
			text-sm leading-5 font-semibold shadow-[0px_4px_4px_0px_rgba(0,0,0,0.25)]
			transition-all duration-300
			md:bottom-8 md:gap-2.5 md:px-3.5 md:py-3 md:text-base md:leading-6
			{hidden ? 'pointer-events-none translate-y-4 opacity-0' : 'opacity-100'}"
	>
		{#if isBackToTop}
			<ArrowUp class="size-4 md:size-5" />
		{:else}
			<ArrowDown class="size-4 md:size-5" />
		{/if}
		{nextSection.label}
	</button>
{/if}
