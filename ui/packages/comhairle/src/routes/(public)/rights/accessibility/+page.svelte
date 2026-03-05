<script>
	import { page } from '$app/stores';
</script>

<svelte:head>
	<meta charset="UTF-8" />
	<meta name="viewport" content="width=device-width, initial-scale=1.0" />
	<title>Accessibility statement for Comhairle</title>
</svelte:head>

<h1>Accessibility statement for Comhairle</h1>

<p>
	This accessibility statement applies to the Comhairle web platform, available at {$page.url
		.origin}.
</p>

<p>
	This website is run by CrownShy. We want as many people as possible to be able to use this
	website. For example, that means you should be able to:
</p>

<ul>
	<li>change colours, contrast levels and fonts using browser or device settings</li>
	<li>zoom in up to 400% without the text spilling off the screen</li>
	<li>navigate most of the website using a keyboard or speech recognition software</li>
	<li>
		listen to most of the website using a screen reader (including the most recent versions of
		JAWS, NVDA and VoiceOver)
	</li>
</ul>

<p>We've also made the website text as simple as possible to understand.</p>

<p>
	<a href="https://mcmw.abilitynet.org.uk/">AbilityNet</a> has advice on making your device easier to
	use if you have a disability.
</p>

<h2>How accessible this website is</h2>

<p>We know some parts of this website are not fully accessible:</p>

<ul>
	<li>User provided video and audio may not have automatically generated captions.</li>
</ul>

<h2>Feedback and contact information</h2>

<p>
	If you find any problems not listed on this page or think we're not meeting accessibility
	requirements, contact us at <a href="mailto:info@crown-shy.com">info@crown-shy.com</a>.
</p>

<p>
	If you need information on this website in a different format like accessible PDF, large print,
	easy read, audio recording or braille, email <a href="mailto:info@crown-shy.com"
		>info@crown-shy.com</a
	>.
</p>

<p>We'll consider your request and get back to you within 5 working days.</p>

<h2>Enforcement procedure</h2>

<p>
	The Equality and Human Rights Commission (EHRC) is responsible for enforcing the Public Sector
	Bodies (Websites and Mobile Applications) (No. 2) Accessibility Regulations 2018 (the
	'accessibility regulations'). If you're not happy with how we respond to your complaint, <a
		href="https://www.equalityadvisoryservice.com/"
		>contact the Equality Advisory and Support Service (EASS)</a
	>.
</p>

<h2>Technical information about this website's accessibility</h2>

<p>
	CrownShy is committed to making its website accessible, in accordance with the Public Sector
	Bodies (Websites and Mobile Applications) (No. 2) Accessibility Regulations 2018.
</p>

<h3>Compliance status</h3>

<p>
	The website has been tested against the Web Content Accessibility Guidelines (WCAG) 2.2 AA
	standard.
</p>

<p>
	This website is partially compliant with the <a href="https://www.w3.org/TR/WCAG22/"
		>Web Content Accessibility Guidelines version 2.2</a
	> AA standard, due to the non-compliances listed below.
</p>

<h2>Non-accessible content</h2>

<p>The content listed below is non-accessible for the following reasons.</p>

<h3>Non-compliance with the accessibility regulations</h3>

<h3>Content that's not within the scope of the accessibility regulations</h3>

<p>
	Comhairle integrates a number of third-party open source tools as part of its participation
	platform. The accessibility of interfaces provided directly by those tools is outside our direct
	control. We aim to ensure that all surfaces directly owned and maintained by CrownShy meet the
	required standard.
</p>

<ul>
	<li>Polis</li>
	<li>HeyForm</li>
	<li>Jitsi</li>
</ul>

<h2>What we're doing to improve accessibility</h2>

<p>
	We are actively developing Comhairle and accessibility improvements are part of our regular
	development cycle. Our frontend is built using Shadcn, a component library that provides
	accessible defaults, and Tailwind CSS for styling. We intend to carry out a more comprehensive
	accessibility audit as the platform matures.
</p>

<h2>Preparation of this accessibility statement</h2>

<p>This statement was prepared on 5th March 2026. It was last reviewed on 5th March 2026.</p>

<p>This website was last tested on 5th March 2026 against the WCAG 2.2 AA standard.</p>

<p class="mb-10">
	The test was carried out by the CrownShy development team. The most viewed pages were tested
	using automated testing tools. A further internal review of the website was carried out against
	the WCAG 2.2 AA standard.
</p>

<style>
	h1 {
		font-size: 2rem;
		margin-bottom: 0.5rem;
	}
	h2 {
		font-size: 1.5rem;
		border-bottom: 2px solid #1d70b8;
		padding-bottom: 0.25rem;
		margin-top: 2rem;
	}
	h3 {
		font-size: 1.2rem;
		margin-top: 1.5rem;
	}
	a {
		color: #1d70b8;
	}
	.todo {
		background: #fff3cd;
		border-left: 4px solid #f0a500;
		padding: 0.5rem 1rem;
		margin: 0.5rem 0;
		font-style: italic;
		color: #594300;
	}
	ul {
		padding-left: 1.5rem;
	}
	li {
		margin-bottom: 0.4rem;
	}
</style>
