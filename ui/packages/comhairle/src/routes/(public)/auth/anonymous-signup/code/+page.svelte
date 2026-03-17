<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import Icon from '@iconify/svelte';
	import { copy } from 'svelte-copy';
	import { fade } from 'svelte/transition';
	import * as m from '$lib/paraglide/messages';
	import { page } from '$app/state';
	import AuthGradient from '$lib/components/AuthGradient.svelte';
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import { Download } from 'lucide-svelte';

	let { data } = $props();
	let backTo = page.url.searchParams.get('backTo') ?? '/';

	let copied = $state(false);
	let copyTimeout: ReturnType<typeof setTimeout>;

	function onCopied() {
		copied = true;
		clearTimeout(copyTimeout);
		copyTimeout = setTimeout(() => (copied = false), 2000);
	}

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

			<Tooltip.Provider delayDuration={0}>
				<Tooltip.Root open={copied ? true : undefined}>
					<Tooltip.Trigger>
						<button
							use:copy={{ text: data?.user?.username ?? '', onCopy: onCopied }}
							class="group inline-flex cursor-pointer items-center gap-3 rounded-lg bg-white px-5 py-2 transition-all duration-200 hover:bg-white/90 hover:shadow-lg active:scale-[0.98] lg:gap-4 lg:px-8 lg:py-3"
						>
							<span
								class="text-center text-2xl leading-8 font-bold text-black select-all sm:text-3xl lg:text-5xl lg:leading-[52px]"
							>
								{data?.user?.username}
							</span>
							<span
								class="text-black/40 transition-colors duration-200 group-hover:text-black/70"
							>
								{#if copied}
									<span in:fade={{ duration: 150 }}>
										<Icon
											icon="solar:check-circle-bold"
											class="size-5 text-green-600 lg:size-7"
										/>
									</span>
								{:else}
									<span in:fade={{ duration: 150 }}>
										<Icon icon="solar:copy-bold" class="size-5 lg:size-7" />
									</span>
								{/if}
							</span>
						</button>
					</Tooltip.Trigger>
					<Tooltip.Content side="top" sideOffset={8}>
						{copied ? 'Copied!' : 'Click to copy'}
					</Tooltip.Content>
				</Tooltip.Root>
			</Tooltip.Provider>

			<div class="flex flex-row gap-2 sm:gap-3">
				<Button
					variant="default"
					size="lg"
					class="h-12  rounded-full px-7"
					onclick={downloadId}
				>
					<Download class="size-5" />
					Download
				</Button>
				<Button
					href={backTo}
					variant="outline"
					size="lg"
					class="hover:bg-background/70 h-12"
				>
					{m.continue_()}
				</Button>
			</div>

			<div class="flex flex-col items-center gap-2.5 pt-40 lg:pt-12">
				<ComhairleLogo href="/" logoSize="lg" color="text-white" />
				<p class="text-center text-2xl leading-7 font-semibold text-white">
					Understand. Contribute. Influence.
				</p>
			</div>
		</div>
	</AuthGradient>
</div>
