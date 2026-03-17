<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import * as m from '$lib/paraglide/messages';
	import { page } from '$app/state';
	import AuthGradient from '$lib/components/AuthGradient.svelte';
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';

	let { data } = $props();
	let backTo = page.url.searchParams.get('backTo') ?? '/';

	function downloadId() {
		const text = `Your Comhairle Anonymous ID: ${data?.user?.username}\n\nSave this — you'll need it to log in.`;
		const blob = new Blob([text], { type: 'text/plain' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = 'comhairle-anonymous-id.txt';
		a.click();
		URL.revokeObjectURL(url);
	}
</script>

<svelte:head>
	<title>Your Anonymous ID - Comhairle</title>
</svelte:head>

<div class="flex min-h-screen w-full flex-col">
	<AuthGradient showLogo={false} mode="full">
		<div class="flex flex-col items-center gap-5">
			<p
				class="text-center text-lg leading-6 font-semibold text-white lg:text-2xl lg:leading-7"
			>
				This is your anonymous ID<br />Save it — you'll need it to log in.
			</p>

			<div class="flex flex-col items-center gap-2.5">
				<div
					class="inline-flex items-center justify-center gap-2.5 bg-white px-5 py-2 lg:px-8 lg:py-3"
				>
					<span
						class="text-center text-2xl leading-8 font-bold text-black sm:text-3xl lg:text-5xl lg:leading-[52px]"
					>
						{data?.user?.username}
					</span>
				</div>
			</div>

			<div class="flex flex-col items-center gap-3 sm:flex-row sm:gap-2.5">
				<CopyButton copyText={data?.user?.username ?? ''}>
					<span class="text-base font-medium text-white">COPY</span>
				</CopyButton>
				<Button
					variant="default"
					size="lg"
					class="h-12 w-full rounded-full px-7 sm:w-auto"
					onclick={downloadId}
				>
					DOWNLOAD
				</Button>
			</div>

			<Button
				href={backTo}
				variant="outline"
				size="lg"
				class="mt-4 h-12 w-full rounded-full border-white px-10 text-white hover:bg-white/10 sm:w-auto"
			>
				{m.continue_()}
			</Button>

			<div class="flex flex-col items-center gap-2.5 pt-8 lg:pt-12">
				<ComhairleLogo href="/" logoSize="lg" color="text-white" />
				<p class="text-center text-2xl leading-7 font-semibold text-white">
					Understand. Contribute. Influence.
				</p>
			</div>
		</div>
	</AuthGradient>
</div>
