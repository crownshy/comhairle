<script lang="ts">
	import { page } from '$app/state';
	import Highlight from '$lib/components/Highlight.svelte';

	const sectionTitle = 'text-2xl font-bold text-foreground md:text-3xl';
	const heading = 'text-xl font-semibold text-foreground md:text-2xl';
	const subheading = 'text-lg font-medium text-foreground md:text-xl';
	const body = 'text-base leading-7 text-foreground';
	const link = 'text-primary underline';
</script>

<svelte:head>
	<title>Accessibility statement for Comhairle</title>
</svelte:head>

<section class="flex flex-col gap-8 md:gap-12">
	<h2 class={sectionTitle}>Accessibility statement for Comhairle</h2>

	<div class="flex flex-col gap-10 md:gap-14">
		<div class="flex flex-col gap-5">
			<p class={body}>
				This accessibility statement applies to the Comhairle web platform, available at
				<a href={page.url.origin} class={link}>{page.url.origin}</a>.
			</p>
			<p class={body}>
				This website is run by CrownShy. We want as many people as possible to be able to
				use this website. For example, that means you should be able to:
			</p>
			<Highlight>
				<ul class="flex list-disc flex-col gap-1 pl-6">
					<li>
						Change colours, contrast levels and fonts using browser or device settings
					</li>
					<li>Zoom in up to 400% without the text spilling off the screen</li>
					<li>
						Navigate most of the website using a keyboard or speech recognition software
					</li>
					<li>
						Listen to most of the website using a screen reader (including the most
						recent versions of JAWS, NVDA and VoiceOver)
					</li>
				</ul>
			</Highlight>
			<p class={body}>
				We've also made the website text as simple as possible to understand.
				<a href="https://mcmw.abilitynet.org.uk/" class={link}>AbilityNet</a> has advice on making
				your device easier to use if you have a disability.
			</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>How accessible this website is</h3>
			<p class={body}>We know some parts of this website are not fully accessible:</p>
			<Highlight
				>User provided video and audio may not have automatically generated captions.</Highlight
			>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>Feedback and contact information</h3>
			<p class={body}>
				If you find any problems not listed on this page or think we're not meeting
				accessibility requirements, contact us at
				<a href="mailto:info@crown-shy.com" class={link}>info@crown-shy.com</a>.
			</p>
			<p class={body}>
				If you need information on this website in a different format like accessible PDF,
				large print, easy read, audio recording or braille, email
				<a href="mailto:info@crown-shy.com" class={link}>info@crown-shy.com</a>.
			</p>
			<p class={body}>
				We'll consider your request and get back to you within 5 working days.
			</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>Enforcement procedure</h3>
			<p class={body}>
				The Equality and Human Rights Commission (EHRC) is responsible for enforcing the
				Public Sector Bodies (Websites and Mobile Applications) (No. 2) Accessibility
				Regulations 2018 (the 'accessibility regulations'). If you're not happy with how we
				respond to your complaint,
				<a href="https://www.equalityadvisoryservice.com/" class={link}
					>contact the Equality Advisory and Support Service (EASS)</a
				>.
			</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>Technical information about this website's accessibility</h3>
			<p class={body}>
				CrownShy is committed to making its website accessible, in accordance with the
				Public Sector Bodies (Websites and Mobile Applications) (No. 2) Accessibility
				Regulations 2018.
			</p>
			<h4 class={subheading}>Compliance status</h4>
			<Highlight>
				The website has been tested against the
				<a href="https://www.w3.org/TR/WCAG22/" class={link}
					>Web Content Accessibility Guidelines (WCAG) 2.2 AA standard</a
				>. This website is partially compliant with the
				<a href="https://www.w3.org/TR/WCAG22/" class={link}
					>Web Content Accessibility Guidelines version 2.2 AA standard</a
				>, due to the non-compliances listed below.
			</Highlight>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>Non-accessible content</h3>
			<p class={body}>
				The content listed below is non-accessible for the following reasons.
			</p>
			<h4 class={subheading}>Non-compliance with the accessibility regulations</h4>
			<h4 class={subheading}>
				Content that's not within the scope of the accessibility regulations
			</h4>
			<p class={body}>
				Comhairle integrates a number of third-party open source tools as part of its
				participation platform. The accessibility of interfaces provided directly by those
				tools is outside our direct control. We aim to ensure that all surfaces directly
				owned and maintained by CrownShy meet the required standard.
			</p>
			<Highlight>
				<ul class="flex list-disc flex-col gap-1 pl-6">
					<li>Polis</li>
					<li>HeyForm</li>
					<li>Jitsi</li>
				</ul>
			</Highlight>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>What we're doing to improve accessibility</h3>
			<p class={body}>
				We are actively developing Comhairle and accessibility improvements are part of our
				regular development cycle. Our frontend is built using Svelte and Shadcn, a
				component library that provides accessible defaults, and Tailwind CSS for styling.
				We intend to carry out a more comprehensive accessibility audit as the platform
				matures.
			</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>Preparation of this accessibility statement</h3>
			<Highlight>
				<div class="flex flex-col gap-2">
					<p>
						This statement was prepared on 5th March 2026. It was last reviewed on 5th
						March 2026.
					</p>
					<p>
						This website was last tested on 5th March 2026 against the WCAG 2.2 AA
						standard.
					</p>
					<p>
						The test was carried out by the CrownShy development team. The most viewed
						pages were tested using automated testing tools. A further internal review
						of the website was carried out against the WCAG 2.2 AA standard.
					</p>
				</div>
			</Highlight>
		</div>
	</div>
</section>
