<script lang="ts">
	let { data } = $props();
	let { conversation } = data;
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import Button from '$lib/components/ui/button/button.svelte';
</script>

{#if conversation}
	<Breadcrumb.Root class="mb-16">
		<Breadcrumb.List>
			<Breadcrumb.Item>
				<Breadcrumb.Link href="/">Home</Breadcrumb.Link>
			</Breadcrumb.Item>
			<Breadcrumb.Separator />
			<Breadcrumb.Item>
				<Breadcrumb.Link href="/conversations">Conversations</Breadcrumb.Link>
			</Breadcrumb.Item>
			<Breadcrumb.Separator />
			<Breadcrumb.Item>
				<Breadcrumb.Page>{conversation.name}</Breadcrumb.Page>
			</Breadcrumb.Item>
		</Breadcrumb.List>
	</Breadcrumb.Root>

	<div class="h-fill grid grid-cols-2 gap-8 overflow-y-auto">
		<header
			class="relative col-span-2 flex h-[40vh] w-full items-center bg-cover bg-center"
			style={`background-image: url(${conversation.banner_image})`}
		>
			<!-- Gradient Overlay -->
			<div class="absolute inset-0 bg-gradient-to-b from-white/30 to-black/70"></div>

			<!-- Content -->
			<div class="relative z-10 ml-12 max-w-2xl text-white">
				<h1 class="text-5xl font-bold">{conversation.name}</h1>
				<h2 class="mt-4 text-2xl">{conversation.short_text}</h2>
				<Button href={`/conversations/${conversation.id}/s/0`} class="mt-5">Share Your View</Button>
			</div>
		</header>

		<article>
			<h3 class="text-xl font-bold">Intro</h3>
			<p>{conversation.introduction_text}</p>
		</article>

		<aside>
			<h3 class="text-xl font-bold">Other ways to learn about this conversation</h3>
			{#if conversation.video_url}
				<h4 class="text-l font-bold">Watch</h4>
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
				<h4 class="text-l font-bold">Listen</h4>
				<video controls class="h-[45px] w-full">
					<source
						src="https://crownshy.s3.eu-west-2.amazonaws.com/alpha_resources/Fairer+Council+Tax+in+Scotland_+A+Consultation.wav"
						type="audio/wav"
					/>
				</video>
			{/if}
			<aside></aside>
		</aside>
	</div>
{:else}
	<h1>Conversation not found</h1>
{/if}
