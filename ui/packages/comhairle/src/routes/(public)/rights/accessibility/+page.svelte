<script lang="ts">
	import { page } from '$app/state';
	import Highlight from '$lib/components/Highlight.svelte';
	import * as m from '$lib/paraglide/messages';

	const sectionTitle = 'text-2xl font-bold text-foreground md:text-3xl';
	const heading = 'text-xl font-semibold text-foreground md:text-2xl';
	const subheading = 'text-lg font-medium text-foreground md:text-xl';
	const body = 'text-base leading-7 text-foreground';
	const link = 'text-primary underline';
</script>

<svelte:head>
	<title>{m.a11y_title()}</title>
</svelte:head>

<section class="flex flex-col gap-8 md:gap-12">
	<h2 class={sectionTitle}>{m.a11y_title()}</h2>

	<div class="flex flex-col gap-10 md:gap-14">
		<div class="flex flex-col gap-5">
			<p class={body}>
				{m.a11y_intro_pre()}<a href={page.url.origin} class={link}>{page.url.origin}</a>.
			</p>
			<p class={body}>{m.a11y_run_by()}</p>
			<Highlight>
				<ul class="flex list-disc flex-col gap-1 pl-6">
					<li>{m.a11y_can1()}</li>
					<li>{m.a11y_can2()}</li>
					<li>{m.a11y_can3()}</li>
					<li>{m.a11y_can4()}</li>
				</ul>
			</Highlight>
			<p class={body}>
				{m.a11y_abilitynet_pre()}<a href="https://mcmw.abilitynet.org.uk/" class={link}
					>AbilityNet</a
				>{m.a11y_abilitynet_post()}
			</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.a11y_howaccessible_heading()}</h3>
			<p class={body}>{m.a11y_howaccessible_body()}</p>
			<Highlight>{m.a11y_howaccessible_item()}</Highlight>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.a11y_feedback_heading()}</h3>
			<p class={body}>
				{m.a11y_feedback_body1_pre()}<a href="mailto:info@crown-shy.com" class={link}
					>info@crown-shy.com</a
				>.
			</p>
			<p class={body}>{m.a11y_feedback_body2()}</p>
			<p class={body}>{m.a11y_feedback_body3()}</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.a11y_enforcement_heading()}</h3>
			<p class={body}>
				{m.a11y_enforcement_pre()}<a
					href="https://www.equalityadvisoryservice.com/"
					class={link}>{m.a11y_enforcement_link()}</a
				>.
			</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.a11y_technical_heading()}</h3>
			<p class={body}>{m.a11y_technical_body()}</p>
			<h4 class={subheading}>{m.a11y_compliance_heading()}</h4>
			<Highlight>
				{m.a11y_compliance_pre()}<a href="https://www.w3.org/TR/WCAG22/" class={link}
					>{m.a11y_compliance_link1()}</a
				>{m.a11y_compliance_mid()}<a href="https://www.w3.org/TR/WCAG22/" class={link}
					>{m.a11y_compliance_link2()}</a
				>{m.a11y_compliance_post()}
			</Highlight>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.a11y_noncontent_heading()}</h3>
			<p class={body}>{m.a11y_noncontent_body()}</p>
			<h4 class={subheading}>{m.a11y_noncompliance_heading()}</h4>
			<h4 class={subheading}>{m.a11y_outofscope_heading()}</h4>
			<p class={body}>{m.a11y_outofscope_body()}</p>
			<Highlight>
				<ul class="flex list-disc flex-col gap-1 pl-6">
					<li>Polis</li>
					<li>HeyForm</li>
					<li>Jitsi</li>
				</ul>
			</Highlight>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.a11y_improving_heading()}</h3>
			<p class={body}>{m.a11y_improving_body()}</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.a11y_prep_heading()}</h3>
			<Highlight>
				<div class="flex flex-col gap-2">
					<p>{m.a11y_prep_line1()}</p>
					<p>{m.a11y_prep_line2()}</p>
					<p>{m.a11y_prep_line3()}</p>
				</div>
			</Highlight>
		</div>
	</div>
</section>
