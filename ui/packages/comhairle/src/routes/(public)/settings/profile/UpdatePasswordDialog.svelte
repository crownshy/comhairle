<script lang="ts">
	import { resolve } from '$app/paths';
	import { Input } from '$lib/components/EasyForm';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { m } from '$lib/paraglide/messages';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	let open = $state<boolean>(false);
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger>
		{@render children()}
	</Dialog.Trigger>
	<Dialog.Content class="sm:max-w-106.25">
		<Dialog.Header>
			<Dialog.Title>{m.password_updated_successfully()}</Dialog.Title>
			<Dialog.Description>
				{m.password_updated_successfully_body()}
			</Dialog.Description>
		</Dialog.Header>
		<Input
			type="password"
			name="password"
			label="New password"
			placeholder="Enter new password (optional)"
		/>
		<Input
			type="password"
			label="Confirm New Password"
			name="confirmPassword"
			placeholder="Confirm new password"
		/>
		<Dialog.Footer>
			<Button href={resolve('/(public)')}>
				{m.continue_to_home()}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
