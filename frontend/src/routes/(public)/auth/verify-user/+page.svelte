<script lang="ts">
	import { AuthPage } from '$lib/profile';
	import type { PageProps } from './$types';
	import * as m from '$lib/paraglide/messages';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';

	let { data }: PageProps = $props();
	let errorMessage = $derived(data.errorMessage);
	let status = $derived(data.status);
</script>

<AuthPage>
	<section>
		{#if status === 'success'}
			<h1 class="mb-2 text-xl font-bold">{m.verify_user_success_heading()}</h1>
			<p class="mb-4">{@html m.verify_user_success_body()}</p>
		{:else if status === 'error' && errorMessage !== null}
			<h1 class="mb-2 text-xl font-bold">{m.verify_user_error_heading()}</h1>
			<p class="mb-4">
				{@html errorMessage}
			</p>
		{:else}
			<h1 class="mb-2 text-xl font-bold">{m.verifying()}</h1>
			<div class="flex justify-center">
				<Spinner />
			</div>
		{/if}
	</section>
</AuthPage>
