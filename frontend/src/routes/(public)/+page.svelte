<script lang="ts">
	import * as m from '$lib/paraglide/messages';
	import Button from '$lib/components/ui/button/button.svelte';
	import ChatbotWrapper from '$lib/components/Chatbot/ChatBotWrapper.svelte';
</script>

<svelte:head>
	<script src="https://www.unpkg.com/@heyform-inc/embed@latest/dist/index.umd.js"></script>
</svelte:head>

<div class="flex h-full flex-col items-center justify-around pt-50">
	<div class="my-auto flex flex-col items-center justify-center gap-y-10 p-10">
		<h1
			class="scroll-m-20 text-center text-4xl font-extrabold tracking-tight md:text-left lg:text-5xl"
		>
			{#if isCommunity}
				{m.welcome_to_comhairle_community_server()}
			{:else}
				{m.welcome_to_comhairle()}
			{/if}
		</h1>
		{#if isCommunity}
			<p class="w-full text-center text-2xl md:text-left">
				{m.community_server_description_1()}
			</p>
			<p class="w-full text-center text-2xl md:text-left">
				{m.community_server_description_2()}
			</p>
		{:else}
			<p class="text-center text-2xl md:text-left">{m.a_platfrom_about_what_people_want()}</p>
		{/if}
		<div class="flex flex-row gap-2">
			{#if isCommunity}
				<div
					data-heyform-id="PmIqc0hc"
					data-heyform-type="modal"
					data-heyform-custom-url="https://forms.crown-shy.com/form/"
					data-heyform-size="large"
					data-heyform-open-trigger="click"
					data-heyform-open-delay="5"
					data-heyform-open-scroll-percent="30"
					data-heyform-trigger-background="#1d4ed8"
					data-heyform-trigger-text="Open Form"
					data-heyform-hide-after-submit="false"
					data-heyform-auto-close="5"
				>
					<Button size="lg" type="button" onclick={openCommunityModal}
						>Apply for a community account</Button
					>
				</div>
			{/if}
			<Button size="lg" href="/conversations">{m.explore_conversations()}</Button>
		</div>
	</div>
</div>
