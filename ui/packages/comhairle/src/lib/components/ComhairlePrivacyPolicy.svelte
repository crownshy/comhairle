<script lang="ts">
	import Highlight from '$lib/components/Highlight.svelte';
	import * as m from '$lib/paraglide/messages';

	let { class: className }: { class?: string } = $props();
</script>

<!--
	Default (fallback) privacy policy. Rendered on /rights/privacy, in the
	pre-participation consent dialog, in the conversation support sidebar, and on
	the per-conversation privacy page. Headings are left colour-neutral so each
	consumer can drive their own styling via the `class` prop.
-->
<div class={className}>
	<h2>{m.privacy_title()}</h2>

	<p>{m.privacy_intro()}</p>

	<Highlight>
		<div class="flex flex-col gap-1">
			<p class="font-bold">{m.privacy_email_title()}</p>
			<p>{m.privacy_email_body()}</p>
		</div>
	</Highlight>

	<Highlight>
		<div class="flex flex-col gap-1">
			<p class="font-bold">{m.privacy_anon_title()}</p>
			<p>{m.privacy_anon_body()}</p>
		</div>
	</Highlight>

	<h3>{m.privacy_access_heading()}</h3>
	<p>{m.privacy_access_body1()}</p>
	<p>{m.privacy_access_body2()}</p>

	<h3>{m.privacy_browser_heading()}</h3>
	<p>{m.privacy_browser_body()}</p>
</div>
