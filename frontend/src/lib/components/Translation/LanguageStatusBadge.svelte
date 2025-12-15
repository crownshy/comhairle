<script lang="ts">
	import { Check } from 'lucide-svelte';

	interface Props {
		language: string;
		languageName: string;
		status: 'draft' | 'approved' | 'primary';
		onclick?: (language: string) => void;
	}

	let { language, languageName, status, onclick }: Props = $props();

	const statusConfig = {
		draft: { color: 'bg-colors-CS_grey-200', label: 'Draft' },
		approved: { color: 'bg-colors-CS_Blue-200', label: 'Approved' },
		primary: { color: 'bg-green-200', label: 'Primary' }
	} as const;
</script>

<button 
	type="button"
	class="h-7 pl-2 pr-1.5 py-0.5 bg-base-background rounded-full outline outline-1 outline-offset-[-1px] outline-base-border flex justify-center items-center gap-1 hover:bg-gray-50 transition-colors cursor-pointer"
	onclick={() => onclick?.(language)}
>
	<Check class="w-3 h-3 text-base-foreground" />
	<span class="text-base-foreground text-xs font-normal leading-4">{languageName}</span>
	<div class="h-5 px-2 {statusConfig[status].color} rounded-full shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] outline outline-1 outline-offset-[-1px] flex justify-center items-center">
		<span class="text-base-foreground text-xs font-normal leading-4">{statusConfig[status].label}</span>
	</div>
</button>
