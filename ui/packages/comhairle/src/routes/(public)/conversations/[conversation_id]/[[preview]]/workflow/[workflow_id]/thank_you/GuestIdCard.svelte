<script lang="ts">
	import { Check } from 'lucide-svelte';
	import type { UserDto } from '@crownshy/api-client/api';
	import { Button } from '$lib/components/ui/button';
	import UpgradeAccountModal from '$lib/components/UpgradeAccountModal/UpgradeAccountModal.svelte';
	import { m } from '$lib/paraglide/messages';
	import { downloadTextFile } from '$lib/utils/downloadFile';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { notifications } from '$lib/notifications.svelte';

	type Props = {
		user: UserDto;
	};
	let { user }: Props = $props();

	const COPIED_FEEDBACK_MS = 2000;

	let guestCode = $derived(user.guestCode ?? '');
	let copied = $state(false);
	let copiedTimeout: ReturnType<typeof setTimeout>;

	async function copyGuestId() {
		const copyResult = await tryCatchAsync(() => navigator.clipboard.writeText(guestCode));
		if (copyResult.err !== null) {
			notifications.send({ message: m.thank_you_copy_failed(), priority: 'ERROR' });
			return;
		}
		copied = true;
		clearTimeout(copiedTimeout);
		copiedTimeout = setTimeout(() => (copied = false), COPIED_FEEDBACK_MS);
	}

	function downloadGuestId() {
		downloadTextFile('comhairle-guest-id.txt', m.guest_id_file_text({ guestCode }));
	}
</script>

<section
	aria-labelledby="guest-id-heading"
	class="bg-card flex w-full max-w-2xl flex-col gap-4 rounded-md border px-4 py-3.5"
>
	<h2 id="guest-id-heading" class="text-muted-foreground text-sm tracking-wide uppercase">
		{m.thank_you_guest_id_label()}
	</h2>
	<p class="text-base">{m.thank_you_guest_id_body()}</p>

	<p
		class="bg-background rounded-2xl p-5 text-center text-3xl font-bold tracking-wider break-all select-all md:text-4xl"
	>
		{guestCode}
	</p>

	<div class="grid grid-cols-2 gap-2">
		<Button size="xl" class="w-full" onclick={copyGuestId}>
			{#if copied}
				<Check aria-hidden="true" />
				{m.thank_you_id_copied()}
			{:else}
				{m.thank_you_copy_id()}
			{/if}
		</Button>
		<Button size="xl" class="w-full" variant="outline" onclick={downloadGuestId}>
			{m.thank_you_download_id()}
		</Button>
	</div>
	<p class="sr-only" aria-live="polite">{copied ? m.thank_you_id_copied() : ''}</p>

	<UpgradeAccountModal currentUser={user}>
		{#snippet trigger({ props })}
			<button
				{...props}
				class="text-primary self-start text-base underline underline-offset-4 hover:no-underline"
			>
				{m.thank_you_create_account_instead()}
			</button>
		{/snippet}
	</UpgradeAccountModal>
</section>
