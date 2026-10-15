<script lang="ts">
	import { resolve } from '$app/paths';
	import { ArrowRight, BookOpenText } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import { handbookGuides } from './handbookGuides';

	const [featured, ...others] = handbookGuides;
	const extras = [
		{ label: 'Tools', href: resolve('/admin/info/tools') },
		{ label: 'Glossary', href: resolve('/admin/info/glossary') }
	];
</script>

<!-- Guide hrefs come from handbookGuides, which builds /admin/info/how-to paths. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<Card.Root class="bg-primary text-primary-foreground h-full gap-4 border-transparent py-6">
	<Card.Header>
		<Card.Description class="text-primary-foreground/90 flex items-center gap-2 text-base">
			<BookOpenText class="size-5" /> Handbook
		</Card.Description>
		<Card.Action>
			<a
				href={resolve('/admin/info/welcome')}
				class="group flex items-center gap-1 text-sm font-medium"
			>
				Open handbook
				<ArrowRight class="size-3.5 transition-transform group-hover:translate-x-0.5" />
			</a>
		</Card.Action>
	</Card.Header>
	<Card.Content class="flex flex-col gap-4">
		{#if featured}
			<a href={featured.href} class="group flex flex-col gap-1">
				<span class="text-2xl font-semibold underline-offset-4 group-hover:underline">
					{featured.title}
				</span>
				<span class="text-base opacity-90"
					>{featured.topicTitles.slice(0, 3).join(' · ')}</span
				>
			</a>
		{/if}
		<div class="grid grid-cols-2 gap-2">
			{#each others as guide (guide.key)}
				<a
					href={guide.href}
					class="bg-primary-foreground/10 hover:bg-primary-foreground/20 rounded-lg px-3 py-2 text-base transition-colors"
				>
					{guide.title}
				</a>
			{/each}
		</div>
	</Card.Content>
	<Card.Footer class="mt-auto gap-4 text-base">
		{#each extras as extra (extra.href)}
			<a href={extra.href} class="underline-offset-4 hover:underline">{extra.label}</a>
		{/each}
	</Card.Footer>
</Card.Root>
