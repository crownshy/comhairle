<script lang="ts">
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import TranslatableField, {
		type TFBaseProps,
		type TFInputProps
	} from './TranslatableField.svelte';
	import type { ErrorType, Result } from '$lib/utils/errorHandling';
	import { Skeleton } from '$lib/components/ui/skeleton';

	type Props = Omit<TFBaseProps, 'availableDocuments'> & {
		streamedAvailableDocuments: Promise<Result<'ok', ComhairleDocument[], ErrorType>>;
	} & TFInputProps;

	let { streamedAvailableDocuments, ...props }: Props = $props();
</script>

{#await streamedAvailableDocuments}
	<Skeleton class="h-37.5 w-full" />
{:then availableDocuments}
	{#if availableDocuments.err}
		{console.error(availableDocuments.err)}
	{/if}
	<TranslatableField {...props} availableDocuments={availableDocuments.ok ?? []} />
{/await}
