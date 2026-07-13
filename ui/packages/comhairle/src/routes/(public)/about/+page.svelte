<script lang="ts">
	import * as m from '$lib/paraglide/messages';
</script>

<svelte:head>
	<title>About - Comhairle</title>
</svelte:head>

<h1 class="mt-10 mb-10 text-4xl font-bold md:mt-20">About</h1>

<article class="prose">
	<section>
		<h2>{m.about_page_what_heading()}</h2>
		{@html m.about_page_what_content()}
	</section>
	<section>
		<h2>{m.about_page_who_heading()}</h2>
		{@html m.about_page_who_content()}
	</section>
</article>
