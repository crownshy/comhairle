<script lang="ts">
	import { page } from '$app/state';
	import { m } from '$lib/paraglide/messages';
	import { THEMES, type ThemeName } from '$lib/types/theme';
	import { resolvePageMeta } from '$lib/utils/pageMeta';

	type Props = {
		themeName: ThemeName;
	};

	let { themeName }: Props = $props();

	const SITE_NAME = 'Comhairle';

	let meta = $derived(
		resolvePageMeta(
			page.data.meta,
			{
				title: SITE_NAME,
				description: m.site_description(),
				image: `/${THEMES[themeName].socialImage}`
			},
			page.url
		)
	);
</script>

<!-- Open Graph tags drive the link preview on Facebook, LinkedIn, Slack and most chat apps;
	X reads the twitter: tags and falls back to Open Graph for the rest. -->
<svelte:head>
	<meta name="description" content={meta.description} />
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content={SITE_NAME} />
	<meta property="og:title" content={meta.title} />
	<meta property="og:description" content={meta.description} />
	<meta property="og:url" content={meta.url} />
	<meta property="og:image" content={meta.image} />
	<meta property="og:image:alt" content={meta.title} />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content={meta.title} />
	<meta name="twitter:description" content={meta.description} />
	<meta name="twitter:image" content={meta.image} />
</svelte:head>
