<script lang="ts">
	import { afterNavigate, beforeNavigate, goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { LoaderCircle } from 'lucide-svelte';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Button } from '$lib/components/ui/button';
	import { mountedAutosaveSources } from './autosaveRegistry.svelte';
	import { hasUnsavedChanges, type AutosaveSource } from './translationUtils';

	let phase = $state<'idle' | 'saving' | 'failed'>('idle');
	let heldNavigation: { url: URL; delta?: number } | null = null;
	let leavingAnyway = false;
	let sourcesToSync: AutosaveSource[] = [];

	const unsavedSources = () => [...mountedAutosaveSources].filter(hasUnsavedChanges);

	onMount(() => {
		function handleBeforeUnload(event: BeforeUnloadEvent) {
			if (unsavedSources().length === 0) return;
			event.preventDefault();
			event.returnValue = '';
		}
		window.addEventListener('beforeunload', handleBeforeUnload);
		return () => window.removeEventListener('beforeunload', handleBeforeUnload);
	});

	beforeNavigate((navigation) => {
		if (navigation.willUnload || !navigation.to) return;
		if (navigation.to.url.pathname === navigation.from?.url.pathname) return;
		if (leavingAnyway) {
			leavingAnyway = false;
			return;
		}
		if (unsavedSources().length === 0) {
			sourcesToSync = [...mountedAutosaveSources].filter((source) => source.stale);
			return;
		}
		navigation.cancel();
		heldNavigation = { url: navigation.to.url, delta: navigation.delta };
		void settleThen(() => Promise.allSettled(unsavedSources().map((s) => s.flush())));
	});

	// Reloading during a navigation would cancel it, so stale data reloads once we've arrived.
	afterNavigate(() => {
		const toSync = sourcesToSync;
		sourcesToSync = [];
		for (const source of toSync) void source.sync();
	});

	async function settleThen(save: () => Promise<unknown>) {
		phase = 'saving';
		await save();
		if (unsavedSources().length > 0) {
			phase = 'failed';
			return;
		}
		continueNavigation();
	}

	function continueNavigation() {
		phase = 'idle';
		const navigation = heldNavigation;
		heldNavigation = null;
		if (!navigation) return;
		if (navigation.delta) history.go(navigation.delta);
		// The URL comes from a SvelteKit navigation, so it is already resolved.
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		else void goto(navigation.url);
	}

	function retry() {
		void settleThen(() => Promise.allSettled(unsavedSources().map((s) => s.retry())));
	}

	function leaveAnyway() {
		leavingAnyway = true;
		continueNavigation();
	}

	function stay() {
		phase = 'idle';
		heldNavigation = null;
	}
</script>

<AlertDialog.Root open={phase !== 'idle'}>
	<AlertDialog.Content>
		{#if phase === 'saving'}
			<AlertDialog.Header>
				<AlertDialog.Title class="flex items-center gap-2">
					<LoaderCircle class="size-5 animate-spin" />
					Saving your changes
				</AlertDialog.Title>
			</AlertDialog.Header>
		{:else}
			<AlertDialog.Header>
				<AlertDialog.Title>Your changes haven't been saved</AlertDialog.Title>
				<AlertDialog.Description class="text-base">
					Some changes couldn't be saved. Retry, or leave and lose them.
				</AlertDialog.Description>
			</AlertDialog.Header>
			<AlertDialog.Footer class="flex-col-reverse gap-2 sm:flex-row">
				<Button variant="ghost" onclick={stay}>Stay on page</Button>
				<Button variant="outline" onclick={leaveAnyway}>Leave anyway</Button>
				<Button onclick={retry}>Retry</Button>
			</AlertDialog.Footer>
		{/if}
	</AlertDialog.Content>
</AlertDialog.Root>
