<script lang="ts">
	import { AuthPage } from '$lib/profile';
	import type { PageProps } from './$types';
	import * as m from '$lib/paraglide/messages';

	let { data }: PageProps = $props();
	let errorMessage = $derived(data.errorMessage);
</script>

<AuthPage>
	<section>
		{#if errorMessage}
			<h1 class="mb-2 text-xl font-bold">{m.verify_user_error_heading()}</h1>
			<p class="mb-4">
				{@html errorMessage}
			</p>
		{:else}
			<h1 class="mb-2 text-xl font-bold">{m.verify_user_success_heading()}</h1>
			<p class="mb-4">{@html m.verify_user_success_body()}</p>
		{/if}
	</section>
</AuthPage>
