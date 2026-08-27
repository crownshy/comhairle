<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { Hand } from 'lucide-svelte';

	interface Props {
		roomName: string;
		question?: string;
		description?: string;
		timeLeftFormatted: string;
		isModerator: boolean;
		onCallForSupport?: () => void;
		onLeaveBreakoutRoom?: () => void;
	}

	let {
		roomName,
		question,
		description,
		timeLeftFormatted,
		isModerator,
		onCallForSupport,
		onLeaveBreakoutRoom
	}: Props = $props();
</script>

<div
	class="bg-muted flex h-full flex-col overflow-hidden rounded-3xl shadow-[0px_2px_4px_0px_rgba(0,0,0,0.12)]"
>
	<!-- Header -->
	<div class="flex shrink-0 flex-col items-start gap-6 px-5">
		<div class="flex w-full max-w-[1304px] items-center justify-center">
			<h2 class="text-muted-foreground text-center text-xl leading-7 font-semibold">
				Breakout session
			</h2>
		</div>
	</div>

	<!-- Content -->
	<div class="flex flex-1 flex-col items-center gap-6 overflow-y-auto px-3 py-5">
		{#if question}
			<div
				class="bg-background flex w-full shrink-0 flex-col items-center gap-4 rounded-xl p-6"
			>
				<div class="flex flex-col items-center gap-1">
					<span class="text-primary text-xs leading-4 font-medium uppercase"
						>Question</span
					>
					<p
						class="text-foreground text-center text-lg leading-7 font-semibold break-words"
					>
						{question}
					</p>
				</div>

				{#if description}
					<div class="h-0 w-full border-t"></div>
					<div class="text-foreground w-full text-base leading-6 font-normal">
						<ContentRenderer content={description} />
					</div>
				{/if}
			</div>
		{/if}
	</div>

	<!-- Footer -->
	<div class="flex shrink-0 flex-col items-center gap-6 border-t p-5">
		{#if isModerator}
			<Button
				variant="destructive"
				class="h-10 px-4 text-sm font-medium"
				onclick={() => onLeaveBreakoutRoom?.()}
			>
				Leave {roomName}
			</Button>
		{:else}
			<Button
				variant="primaryDark"
				class="h-10 px-4 text-sm font-medium"
				onclick={() => onCallForSupport?.()}
			>
				<Hand class="h-4 w-4" />
				Call for support
			</Button>
		{/if}
	</div>
</div>
