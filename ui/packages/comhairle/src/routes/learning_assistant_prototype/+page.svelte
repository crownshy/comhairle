<!--
	PROTOTYPE - throwaway. Answers: how should the Learning Assistant present itself so
	it looks better AND clearly tells participants what it does, what they can ask, and
	what happens to their data? (Today it's a two-line intro + bare input.)

	Three structurally different treatments on one route. Switch with ?variant=A|B|C or
	the floating bottom bar; ?width=sidebar|page previews the two real surfaces (the
	"Find out more" drawer vs. the inline learn-page embed). Delete once a direction is
	chosen; fold the winner into
	src/lib/components/LearningAssistant/LearningAssistant.svelte.

	NB: all data-handling / privacy copy here is PLACEHOLDER - confirm exact wording
	(retention, admin visibility) with the team before folding in.
-->
<script lang="ts">
	import { page } from '$app/state';
	import { X } from 'lucide-svelte';
	import PrototypeSwitcher from './PrototypeSwitcher.svelte';
	import VariantA from './VariantA.svelte';
	import VariantB from './VariantB.svelte';
	import VariantC from './VariantC.svelte';
	import { SAMPLE_QAS, STARTER_PROMPTS } from './sample';

	const variants = [
		{ key: 'A', name: 'Guided intro + data note' },
		{ key: 'B', name: 'About panel + chat thread' },
		{ key: 'C', name: 'Example prompts + trust bar' }
	];

	let current = $derived(page.url.searchParams.get('variant') ?? 'A');
	let width = $derived(
		page.url.searchParams.get('width') === 'page' ? ('page' as const) : ('sidebar' as const)
	);
</script>

<!-- dimmed backdrop so the panel reads like the real right-side drawer -->
<div class="bg-muted/60 fixed inset-0"></div>

{#if width === 'sidebar'}
	<!-- Drawer frame, matching the "Find out more" sidebar -->
	<div class="fixed inset-0 bg-black/40"></div>
	<div
		class="bg-background fixed inset-y-0 right-0 flex w-screen max-w-[100vw] flex-col px-8 py-10 lg:max-w-[480px]"
	>
		<div class="mb-4 flex shrink-0 items-center justify-between">
			<span class="text-muted-foreground text-xs font-semibold tracking-wide uppercase"
				>Find out more</span
			>
			<button
				type="button"
				aria-label="Close"
				class="hover:bg-muted rounded-lg p-1.5 transition-colors"
			>
				<X class="text-foreground size-5" />
			</button>
		</div>
		{#if current === 'A'}
			<VariantA qas={SAMPLE_QAS} />
		{:else if current === 'B'}
			<VariantB qas={SAMPLE_QAS} />
		{:else}
			<VariantC qas={SAMPLE_QAS} prompts={STARTER_PROMPTS} {width} />
		{/if}
	</div>
{:else}
	<!-- Inline embed frame, matching the learn page (article column width) -->
	<div class="relative mx-auto flex min-h-screen max-w-3xl flex-col px-6 py-12">
		<div class="text-muted-foreground border-border mb-8 border-b pb-8 text-base">
			<div class="bg-muted mb-3 h-6 w-2/3 rounded"></div>
			<div class="bg-muted/70 mb-2 h-3 w-full rounded"></div>
			<div class="bg-muted/70 mb-2 h-3 w-full rounded"></div>
			<div class="bg-muted/70 h-3 w-4/5 rounded"></div>
			<p class="mt-4 text-sm italic">
				↑ (placeholder learn-page article - the assistant sits below it)
			</p>
		</div>
		<div class="flex min-h-0 flex-col">
			{#if current === 'A'}
				<VariantA qas={SAMPLE_QAS} />
			{:else if current === 'B'}
				<VariantB qas={SAMPLE_QAS} />
			{:else}
				<VariantC qas={SAMPLE_QAS} prompts={STARTER_PROMPTS} {width} />
			{/if}
		</div>
	</div>
{/if}

<PrototypeSwitcher {variants} {current} {width} />
