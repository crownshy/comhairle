<script lang="ts">
	import { Check } from 'lucide-svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { statusToBadgeVariant } from './translationUtils';

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
	class="bg-background outline-border outline-base-border dark:hover:text-accent-foreground dark:hover:bg-accent/50 flex h-7 cursor-pointer items-center justify-center gap-1 rounded-full py-0.5 pr-1.5 pl-2 outline-1 -outline-offset-1 transition-colors hover:bg-gray-50"
	onclick={() => onclick(language)}
>
	{#if status === 'approved'}
		<Check class="text-foreground h-3 w-3" />
	{/if}
	<span class="text-foreground text-xs leading-4 font-normal">{languageName}</span>
	<Badge variant={statusToBadgeVariant[status]} class="rounded-full capitalize">{status}</Badge>
</button>
