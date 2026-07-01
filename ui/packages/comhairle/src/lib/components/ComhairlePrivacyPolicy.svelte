<script lang="ts">
	import Highlight from '$lib/components/Highlight.svelte';

	let { class: className }: { class?: string } = $props();
</script>

<!--
	Default (fallback) privacy policy. Rendered on /rights/privacy, in the
	pre-participation consent dialog, in the conversation support sidebar, and on
	the per-conversation privacy page. Headings are left colour-neutral so each
	consumer can drive their own styling via the `class` prop.
-->
<div class={className}>
	<h2>Privacy</h2>

	<p>
		To execute the functionality of the website and provide accountability and moderation, we
		collect and associate responses on the platform with one of the following account types:
	</p>

	<Highlight>
		<div class="flex flex-col gap-1">
			<p class="font-bold">An user provided email and username</p>
			<p>
				Where collected and permission is given, email addresses will be used to contact you
				with notifications relevant to your participation on the platform and be used to
				contact you in relation to your participation.
			</p>
		</div>
	</Highlight>

	<Highlight>
		<div class="flex flex-col gap-1">
			<p class="font-bold">A pseudo-anonymous login id</p>
			<p>
				Where permission is given we will publish responses potentially with attribution. We
				will never publish email addresses or other PII information.
			</p>
		</div>
	</Highlight>

	<h3>Accessing your Personal Information</h3>
	<p>
		You can access your personal information and perform actions like downloading a copy of it,
		or correcting it if it is wrong, in your user preferences. If you have further questions or
		requests about the information you have submitted please contact team@crown-shy.com.
	</p>
	<p>
		To maintain transparency and accountability, we may retain a record of any modified
		responses. These records will be used to ensure the consultation reports once published do
		not change or that any changes can be transparently observed.
	</p>

	<h3>Collection of Browser Information</h3>
	<p>
		When you use this website, your computer provides information that is collected by CrownShy.
		Your browser type, IP address, language preference, referring site and the date and time may
		all be recorded to maintain the security of the website and to help improve our software.
	</p>
</div>
