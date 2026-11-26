<script lang="ts">
	import { m } from '$lib/paraglide/messages';
	import PageHero from '$lib/components/PageHero.svelte';
</script>

<svelte:head>
	<title>About - Comhairle</title>
</svelte:head>

<PageHero compact>
	<h1 class="text-[1.75rem] leading-tight font-bold sm:text-4xl lg:text-[2.625rem]">
		{m.about()}
	</h1>
</PageHero>

<div class="mx-auto w-full max-w-[1300px] px-4 pt-12 pb-24 md:px-8 md:pt-16">
	<article class="prose mx-auto w-full max-w-6xl">
		<section>
			<h2>{m.about_page_what_heading()}</h2>
			<!-- eslint-disable-next-line svelte/no-at-html-tags -- our own translation strings, which carry links -->
			{@html m.about_page_what_content()}
		</section>
		<section>
			<h2>{m.about_page_who_heading()}</h2>
			<!-- eslint-disable-next-line svelte/no-at-html-tags -- our own translation strings, which carry links -->
			{@html m.about_page_who_content()}
		</section>
	</article>
</div>
