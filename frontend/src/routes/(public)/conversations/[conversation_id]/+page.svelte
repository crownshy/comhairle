<script lang="ts">
	import type { PageProps } from '../$types.js';

	let { data }: PageProps = $props();
	let { conversation, workflows } = data;
	let user = $derived(data.user);
	let hasAdditionalLearnMethods = $derived(conversation.video_url || conversation.audio_url);

	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as m from '$lib/paraglide/messages';
	import { notifications } from '$lib/notifications.svelte.js';
	import { goto } from '$app/navigation';
	import { apiClient } from '$lib/api/client';
	import { page } from '$app/state';

	let firstWorkflowPath = `/conversations/${conversation.id}/workflow/${workflows[0].id}/s/1`;

	async function redirectToLogin() {
		goto(`/auth/login?backTo=${page.url.pathname}`);
	}

	async function registerUser() {
		try {
			await apiClient.RegisterUserForWorkflow(undefined, {
				params: { conversation_id: data.conversation.id, workflow_id: data.workflows[0].id }
			});

			notifications.addFlash({
				message: `You are part of the "${conversation.title}" conversation!`
			});

			goto(firstWorkflowPath);
		} catch (e) {
			let message;

			if (e instanceof Error) message = e.message;
			else message = String(e);

			console.warn(`Failed to register user for workflow ${message}`);

			notifications.send({
				message: 'Failed to sign you up for the conversation, try again later',
				priority: 'ERROR'
			});
		}
	}
</script>

{#snippet joinButtons(participation, user)}
	{#if user}
		{#if participation}
			<Button class="mt-5 w-full md:w-fit" href={firstWorkflowPath}>{m.jump_back_in()}</Button>
		{:else}
			<Button class="mt-5 w-full md:w-fit" onclick={registerUser}
				>{m.join_the_conversation()}</Button
			>
		{/if}
	{:else}
		<Button class="mt-5 w-full md:w-fit" onclick={redirectToLogin}>{m.login_to_take_part()}</Button>
	{/if}
{/snippet}

<div class="pt-5 md:pt-20">
	{#if conversation}
		<div class="hidden md:block">
			<Breadcrumbs {conversation} />
		</div>

		<div class="h-fill grid grid-cols-1 gap-8 overflow-y-auto md:grid-cols-2">
			<header
				class="relative flex h-[40vh] w-full items-center bg-cover bg-center md:col-span-2"
				style={`background-image: url(${conversation.image_url})`}
			>
				<!-- Gradient Overlay -->
				<div class="absolute inset-0 bg-gradient-to-b from-white/30 to-black/70"></div>

				<!-- Content -->
				<div class="relative z-10 ml-12 max-w-2xl text-white">
					<h1 class="text-5xl font-bold">{conversation.title}</h1>
					<h2 class="mt-4 text-2xl">{conversation.description}</h2>
					<div class="hidden md:block">
						{@render joinButtons(data.participation, user)}
					</div>
				</div>
			</header>

			<div class="col-span-2 block md:hidden">
				{@render joinButtons(data.participation, user)}
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
	{:else}
		<h1>Conversation not found</h1>
	{/if}
</div>
