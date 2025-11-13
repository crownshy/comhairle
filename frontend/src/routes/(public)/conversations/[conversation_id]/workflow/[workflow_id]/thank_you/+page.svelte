<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { workflow_step_url } from '$lib/urls';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
	let user = $derived(data.user);
	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflow);
</script>

<div class="prose mx-auto mt-20">
	<h1>Thank You For Participating</h1>

	<p>Thank you for taking part in the conversation "{conversation.title}"</p>
	<p>We value your time and effort</p>

	{#if user.auth_type === 'annon'}
		<p>You currently signed in as an anonymous user. Use your anonymous id</p>
		<h2 class="text-center">{user.username}</h2>
		<p>to log back in at a later time to see the results of the conversations.</p>
	{:else}
		<p>
			The conversation is ongoing. When it's done we will email you with a link to the results and
			will follow up with notifications on how those results are used
		</p>
	{/if}
</div>
<div class="mx-auto mt-10 text-center">
	<Button variant="secondary" href={workflow_step_url(conversation.id, workflow.id, 1)}
		>Continue to contribute</Button
	>
</div>
<p></p>
