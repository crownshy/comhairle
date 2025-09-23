<script lang="ts">
	import type { Conversation } from '$lib/api/api';
	import type { Snippet } from 'svelte';
	import * as m from '$lib/paraglide/messages';

	type Props = {
		conversation: Conversation;
		children: Snippet;
	};
	let { conversation, children }: Props = $props();
	let hasAdditionalLearnMethods = $derived(conversation.video_url || conversation.audio_url);
</script>

<div class="h-fill grid grid-cols-1 gap-8 overflow-y-auto md:grid-cols-2">
	<header
		class="relative flex h-[40vh] w-full items-center bg-cover bg-center md:col-span-2"
		style={`background-image: url(${conversation.image_url})`}
	>
		<!-- Gradient Overlay -->
		<div class="absolute inset-0 bg-linear-to-b from-white/30 to-black/70"></div>

		<!-- Content -->
		<div class="relative z-10 ml-12 max-w-2xl text-white">
			<h1 class="text-5xl font-bold">{conversation.title}</h1>
			<h2 class="mt-4 text-2xl">{conversation.description}</h2>
			<div class="hidden md:block">
				{@render children()}
			</div>
		</div>
	</header>

	<div class="col-span-2 block md:hidden">
		{@render children()}
	</div>

	<article class={hasAdditionalLearnMethods ? '' : 'col-span-2'}>
		<h3 class="text-xl font-bold">{m.intro()}</h3>
		<p>{conversation.short_description}</p>
		<p>{conversation.description}</p>
	</article>
	{#if hasAdditionalLearnMethods}
		<aside>
			<h3 class="text-xl font-bold">{m.other_ways_to_learn_about_this_conversation()}</h3>
			{#if conversation.video_url}
				<h4 class="text-l font-bold">{m.watch()}</h4>
				<iframe
					width="560"
					height="315"
					src={conversation.video_url}
					title="YouTube video player"
					frameborder="0"
					allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
					referrerpolicy="strict-origin-when-cross-origin"
					allowfullscreen
				></iframe>
			{/if}

			{#if conversation.audio_url}
				<h4 class="text-l font-bold">{m.listen()}</h4>
				<video controls class="h-[45px] w-full">
					<source
						src="https://crownshy.s3.eu-west-2.amazonaws.com/alpha_resources/Fairer+Council+Tax+in+Scotland_+A+Consultation.wav"
						type="audio/wav"
					/>
				</video>
			{/if}
			<aside></aside>
		</aside>
	{/if}
</div>
