<script lang="ts">
	import Highlight from '$lib/components/Highlight.svelte';
	import { resolve } from '$app/paths';
	import * as m from '$lib/paraglide/messages';

	let {
		onPrivacyClick
	}: {
		/** Intercepts the in-body privacy link so an embedded reader can swap docs in place. */
		onPrivacyClick?: (event: MouseEvent) => void;
	} = $props();

	// Shared type scale for the Your Rights pages (matches the Jazz Up designs).
	const sectionTitle = 'text-2xl font-bold text-foreground md:text-3xl';
	const heading = 'text-xl font-semibold text-foreground md:text-2xl';
	const body = 'text-base leading-7 text-foreground';
	const list = 'flex list-disc flex-col gap-1 pl-6 text-base leading-7 text-foreground';
</script>

<section class="flex flex-col gap-8 md:gap-12">
	<h2 class={sectionTitle}>{m.tos_title()}</h2>

	<div class="flex flex-col gap-10 md:gap-14">
		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.tos_s1_heading()}</h3>
			<p class={body}>{m.tos_s1_body()}</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.tos_s2_heading()}</h3>
			<ul class={list}>
				<li>{m.tos_s2_item1()}</li>
				<li>{m.tos_s2_item2()}</li>
				<li>{m.tos_s2_item3()}</li>
			</ul>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.tos_s3_heading()}</h3>
			<p class={body}>{m.tos_s3_intro()}</p>
			<ul class={list}>
				<li>{m.tos_s3_item1()}</li>
				<li>{m.tos_s3_item2()}</li>
				<li>{m.tos_s3_item3()}</li>
				<li>{m.tos_s3_item4()}</li>
			</ul>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.tos_s4_heading()}</h3>
			<ul class={list}>
				<li>{m.tos_s4_item1()}</li>
				<li>{m.tos_s4_item2()}</li>
			</ul>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.tos_s5_heading()}</h3>
			<p class={body}>{m.tos_s5_intro()}</p>
			<ul class={list}>
				<li>{m.tos_s5_item1()}</li>
				<li>{m.tos_s5_item2()}</li>
				<li>{m.tos_s5_item3()}</li>
				<li>{m.tos_s5_item4()}</li>
				<li>{m.tos_s5_item5()}</li>
			</ul>
			<p class={body}>{m.tos_s5_body1()}</p>
			<p class={body}>{m.tos_s5_body2()}</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.tos_s6_heading()}</h3>
			<p class={body}>
				{m.tos_s6_body_pre()}<a
					href={resolve('/rights/privacy')}
					onclick={onPrivacyClick}
					class="text-primary underline">{m.privacy_policy()}</a
				>{m.tos_s6_body_post()}
			</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.tos_s7_heading()}</h3>
			<ul class={list}>
				<li>{m.tos_s7_item1()}</li>
				<li>{m.tos_s7_item2()}</li>
				<li>{m.tos_s7_item3()}</li>
			</ul>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.tos_s8_heading()}</h3>
			<ul class={list}>
				<li>{m.tos_s8_item1()}</li>
				<li>{m.tos_s8_item2()}</li>
			</ul>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.tos_s9_heading()}</h3>
			<p class={body}>{m.tos_s9_body()}</p>
		</div>

		<div class="flex flex-col gap-5">
			<h3 class={heading}>{m.tos_s10_heading()}</h3>
			<p class={body}>{m.tos_s10_body()}</p>
			<Highlight>
				<div class="flex flex-col">
					<span>Crown Shy Ltd</span>
					<span>
						{m.tos_s10_email_label()}
						<a href="mailto:team@crown-shy.com" class="text-primary underline"
							>team@crown-shy.com</a
						>
					</span>
				</div>
			</Highlight>
			<p class={body}>{m.tos_outro()}</p>
		</div>
	</div>
</section>
