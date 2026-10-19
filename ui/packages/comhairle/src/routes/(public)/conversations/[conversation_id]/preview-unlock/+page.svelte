<script lang="ts">
	import { goto } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import type { ApiError } from '@crownshy/api-client/api';
	import AuthLayout from '$lib/components/AuthLayout.svelte';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { LoadingButton } from '$lib/components/ui/button';
	import { useLoading } from '$lib/hooks/use-loading.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { HttpStatus } from '$lib/utils/constants';
	import { key } from '$lib/utils/invalidationKey';

	let { data } = $props();

	let password = $state('');
	let error = $state<string | null>(null);
	const loader = useLoading();

	async function unlock() {
		if (!password) return;
		error = null;
		await loader.run(async () => {
			const result = await tryCatchAsync<unknown, ApiError>(() =>
				apiClient.UnlockConversationPreview(
					{ password },
					{ params: { conversation_id: data.conversationId } }
				)
			);
			if (result.err !== null) {
				error =
					result.err.status === HttpStatus.TooManyRequests
						? 'Too many attempts, please wait a moment and try again'
						: 'That password is not right';
				return;
			}
			// eslint-disable-next-line svelte/no-navigation-without-resolve -- backTo is checked to be a same-site path in the load
			await goto(data.backTo, { invalidate: [key('public/conversation')] });
		});
	}
</script>

<svelte:head>
	<title>Preview - Comhairle</title>
</svelte:head>

<AuthLayout>
	<form
		class="flex flex-col gap-6"
		onsubmit={(e) => {
			e.preventDefault();
			unlock();
		}}
	>
		<div class="flex flex-col gap-2 text-center">
			<h1 class="text-foreground text-3xl leading-9 font-bold">Preview this conversation</h1>
			<p class="text-muted-foreground text-base">
				This conversation hasn't launched yet. Enter the password you were given to look
				around.
			</p>
		</div>
		<div class="flex flex-col gap-2">
			<Label for="preview-password">Password</Label>
			<Input
				id="preview-password"
				type="password"
				autocomplete="off"
				bind:value={password}
				aria-invalid={error !== null}
			/>
			{#if error}
				<p class="text-destructive text-base" role="alert">{error}</p>
			{/if}
		</div>
		<LoadingButton type="submit" loading={loader.loading} disabled={!password}>
			Open preview
		</LoadingButton>
	</form>
</AuthLayout>
