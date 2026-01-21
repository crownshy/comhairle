<script lang="ts">
	import { Check } from 'lucide-svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { statusToBadgeVariant } from './useTranslations.svelte';

	interface Props {
		language: string;
		languageName: string;
		status: 'draft' | 'approved' | 'primary';
		onclick: (language: string) => void;
	}

	let { language, languageName, status, onclick }: Props = $props();
</script>

<button 
	type="button"
	class="h-7 pl-2 pr-1.5 py-0.5 bg-base-background rounded-full outline outline-1 outline-offset-[-1px] outline-base-border flex justify-center items-center gap-1 hover:bg-gray-50 transition-colors cursor-pointer"
	onclick={() => onclick(language)}
>
	{#if status === 'approved'}
		<Check class="w-3 h-3 text-base-foreground" />
	{/if}
	<span class="text-base-foreground text-xs font-normal leading-4">{languageName}</span>
	<Badge variant={statusToBadgeVariant[status]} class="capitalize rounded-full">{status}</Badge>
</button>
