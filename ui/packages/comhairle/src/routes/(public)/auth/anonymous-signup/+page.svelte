<script lang="ts">
	import { enhance } from '$app/forms';
	import * as m from '$lib/paraglide/messages';
	import type { PageData } from './$types';
	import { LoadingButton } from '$lib/components/ui/button';
	import AuthGradient from '$lib/components/AuthGradient.svelte';

	let { data }: { data: PageData } = $props();
	let loading = $state(false);
</script>

<svelte:head>
	<title>Anonymous Sign Up - Comhairle</title>
</svelte:head>

<div class="flex min-h-screen w-full flex-col lg:flex-row">
	<div
		class="bg-background order-2 flex w-full flex-1 items-center justify-center px-5 py-10 lg:order-1 lg:w-1/2 lg:px-6 lg:py-0"
	>
		<div class="w-full max-w-md">
			<form
				class="space-y-6 lg:space-y-8"
				method="POST"
				use:enhance={() => {
					loading = true;
					return async ({ update }) => {
						await update();
						loading = false;
					};
				}}
			>
				<div class="flex flex-col items-center gap-3 lg:gap-6">
					<h1
						class="text-foreground text-center text-3xl leading-9 font-bold lg:text-5xl lg:leading-[52px]"
					>
						{m.sign_up_anonymously()}
					</h1>
					<p
						class="text-muted-foreground text-center text-lg leading-6 font-semibold lg:text-2xl lg:leading-7"
					>
						{m.get_started_with_comhairle_right_away()}
					</p>
				</div>

				<div class="flex justify-center">
					<LoadingButton
						type="submit"
						size="lg"
						class="h-12 w-full px-7 lg:w-auto"
						variant="default"
						{loading}
					>
						{m.generate_anonymous_id()}
					</LoadingButton>
				</div>

				<div class="flex flex-col gap-1">
					<p class="text-muted-foreground text-base">
						{m.already_have_an_account_login().split('?')[0]}?
						<a
							href={`/auth/login?backTo=${data.backTo ?? '/'}`}
							class="text-primary underline">{m.login()}</a
						>
					</p>
					<p class="text-muted-foreground text-base">
						{m.agree_to_tos()}
						<a href="/rights/tos" class="text-primary underline">TOS</a>
						{m.agree_to_tos2()}
						<a href="/rights/privacy" class="text-primary underline"
							>{m.agree_to_tos_privacy()}</a
						>
					</p>
				</div>
			</form>
		</div>
	</div>

	<div class="order-1 lg:order-2 lg:contents">
		<AuthGradient />
	</div>
</div>
