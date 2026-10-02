<script lang="ts">
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import TranslatableField, {
		type TranslatableFieldBaseProps,
		type TranslatableFieldInputProps
	} from './TranslatableField.svelte';
	import type { ErrorType, Result } from '$lib/utils/errorHandling';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { resource } from 'runed';

	type Props = Omit<TranslatableFieldBaseProps, 'availableDocuments'> & {
		streamedAvailableDocuments: Promise<Result<'ok', ComhairleDocument[], ErrorType>>;
	} & TranslatableFieldInputProps;

	let { streamedAvailableDocuments, ...props }: Props = $props();

	// Every autosave reruns the page load and hands us a new promise. Keeping the last resolved
	// list (rather than {#await}) stops the editor remounting, which reset its scroll and cursor.
	const availableDocuments = resource(
		() => streamedAvailableDocuments,
		async (promise, _previous, { signal }) => {
			const result = await promise;
			if (signal.aborted) throw new DOMException('Superseded', 'AbortError');
			if (result.err !== null) console.error(result.err);
			return result.ok ?? [];
		}
	);
</script>

{#if availableDocuments.current === undefined}
	<Skeleton class="h-37.5 w-full" />
{:else}
	<TranslatableField {...props} availableDocuments={availableDocuments.current} />
{/if}
