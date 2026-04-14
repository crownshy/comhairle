<script lang="ts">
	import ArrowDown from '@lucide/svelte/icons/arrow-down';

	const sections = [
		{ id: 'engagement', label: 'Dive In' },
		{ id: 'agreement', label: 'Areas of agreement' },
		{ id: 'groups', label: 'Emerging opinion groups' },
		{ id: 'deep-dive', label: 'Deep Dive' }
	];

	let nextIndex = $state(0);
	let hidden = $state(false);

	const nextSection = $derived(sections[nextIndex] ?? null);

	function scrollToNext() {
		if (!nextSection) return;
		const el = document.getElementById(nextSection.id);
		if (el) {
			el.scrollIntoView({ behavior: 'smooth', block: 'start' });
		}
	}

	function handleScroll() {
		// find how many sections we've passed
		let passed = 0;
		for (let i = 0; i < sections.length; i++) {
			const el = document.getElementById(sections[i].id);
			if (el) {
				const rect = el.getBoundingClientRect();
				if (rect.top <= 150) {
					passed = i + 1;
				}
			}
		}
		nextIndex = passed;
		hidden = passed >= sections.length;
	}

	$effect(() => {
		window.addEventListener('scroll', handleScroll, { passive: true });
		return () => window.removeEventListener('scroll', handleScroll);
	});
</script>

{#if nextSection}
	<button
		onclick={scrollToNext}
		class="bg-background fixed bottom-8 left-1/2 z-50
			inline-flex -translate-x-1/2 items-center gap-2.5 rounded-full px-3.5 py-3
			text-base
			leading-6 font-semibold shadow-[0px_4px_4px_0px_rgba(0,0,0,0.25)]
			transition-all duration-300
			{hidden ? 'pointer-events-none translate-y-4 opacity-0' : 'opacity-100'}"
	>
		<ArrowDown class="size-5" />
		{nextSection.label}
	</button>
{/if}
