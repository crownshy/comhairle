<script lang="ts">
	import * as m from '$lib/paraglide/messages';
	import AuthGradient from '$lib/components/AuthGradient.svelte';
	import { Button } from '$lib/components/ui/button';
</script>

<svelte:head>
	<title>Password Reset Email Sent - Comhairle</title>
</svelte:head>

<div class="flex min-h-screen w-full flex-col">
	<AuthGradient showLogo={false} mode="full">
		<div
			class="dark:bg-card mx-4 w-full max-w-lg rounded-xl bg-white p-6 shadow-sm lg:mx-0 lg:p-10"
		>
			<div class="flex flex-col items-center gap-6 lg:gap-8">
				<div class="flex flex-col items-center gap-3 lg:gap-6">
					<h1
						class="text-foreground text-center text-3xl leading-9 font-bold lg:text-5xl lg:leading-[52px]"
					>
						{m.password_reset_instructions_heading()}
					</h1>
					<p
						class="text-muted-foreground text-center text-lg leading-6 font-semibold lg:text-2xl lg:leading-7"
					>
						{m.password_reset_instructions_body()}
					</p>
				</div>

				<Button href="/auth/login" variant="default" size="lg">
					{m.back_to_login()}
				</Button>
			</div>
		</div>
	</AuthGradient>
</div>
