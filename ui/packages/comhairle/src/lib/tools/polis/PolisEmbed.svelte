<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { fade } from 'svelte/transition';
	import PolisVoteCard from './components/PolisVoteCard.svelte';
	import  PolisApi from "./polisApi.svelte"
	import PolisNewStatementModal from './components/PolisNewStatementModal.svelte';

	type Props = {
		polis_id: string;
		polis_url: string;
		user_id: string;
		onDone: () => void;
	};

	$effect(() => {
		setTimeout(() => {
			showThankYou = true;
		}, 180000);
	});

	let {polis_id, polis_url, user_id, onDone} : Props = $props()
	let showThankYou = $state(false);
	let polis = new PolisApi(user_id, polis_id, "en");

</script>

{#if showThankYou}
	<div transition:fade={{ duration: 1000 }} class="prose mx-auto mb-20">
		<h3>This interactive discussion doesn't have a fixed end</h3>
		<p>
			You can keep voting or adding statements as long as you link. You're welcome to return later
			when others have contributed more. Thank you for taking part.
		</p>
		<Button variant="secondary" onclick={onDone}>I'm Done</Button>
	</div>
{/if}
<div class='flex flex-col gap-6'>
	<div class='flex flex-row justify-center'>
		{#if polis.currentStatement}
			<p class='justify-self-end'>{polis.remaining} of {polis.total}</p>
			<PolisVoteCard statement={polis.currentStatement.txt} onVote={(vote)=>polis.submitVote(vote)}/>
		{:else}
			<h1>No more statements, come back soon</h1>
		{/if}

	</div>
	<PolisNewStatementModal onSubmit={(statement)=>polis.submitStatement(statement)}/>
</div>
