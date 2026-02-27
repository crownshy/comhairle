<script lang="ts">
	import { MessageCircle, X, ChevronDown } from 'lucide-svelte';
	import ChatBot from './ChatBot.svelte';

	interface Props {
		chatId?: string;
		conversationId?: string;
		userId?: string;
		botName?: string;
		botSubtitle?: string;
	}

	let {
		chatId,
		conversationId,
		userId,
		botName = 'Tutor bot',
		botSubtitle = 'Ask questions'
	}: Props = $props();

	let isOpen = $state(true);
	let isMobile = $state(false);

	$effect(() => {
		const mediaQuery = window.matchMedia('(max-width: 768px)');
		isMobile = mediaQuery.matches;

		const handler = (e: MediaQueryListEvent) => {
			isMobile = e.matches;
		};

		mediaQuery.addEventListener('change', handler);
		return () => mediaQuery.removeEventListener('change', handler);
	});

	function toggle() {
		isOpen = !isOpen;
	}
</script>

{#if isMobile}
	<!-- Mobile: Modal overlay -->
	{#if isOpen}
		<div class="fixed inset-0 z-50 flex flex-col">
			<!-- Dark overlay background -->
			<button class="absolute inset-0 bg-black/40" onclick={toggle} aria-label="Close chat"
			></button>

			<!-- Modal panel -->
			<div class="relative mt-auto flex max-h-[85vh] flex-col shadow-xl">
				<!-- Modal header -->
				<div
					class="bg-chat-bubble border-chat-primary-light flex items-center gap-5 rounded-t-2xl border-t border-r border-l p-6"
				>
					<div
						class="bg-chat-primary ring-chat-primary-lighter relative flex h-14 w-14 items-center justify-center rounded-full ring-[6px]"
					>
						<span class="absolute right-0 bottom-0 h-2 w-2 rounded-full bg-green-400"
						></span>
						<MessageCircle class="h-7 w-7 text-white" />
					</div>
					<div class="flex flex-1 flex-col gap-1">
						<span class="text-base-foreground text-xl leading-6 font-semibold"
							>{botName}</span
						>
						<span class="text-chat-primary text-base leading-6 font-normal"
							>{botSubtitle}</span
						>
					</div>
					<button
						onclick={toggle}
						class="hover:bg-chat-bg rounded-lg p-2 transition-colors"
						aria-label="Close chat"
					>
						<X class="text-base-foreground h-6 w-6" />
					</button>
				</div>

				<!-- Chat content -->
				<div
					class="bg-chat-primary-lighter border-chat-primary-light flex max-h-[60vh] min-h-[50vh] flex-1 flex-col overflow-hidden border-r border-b border-l"
				>
					<ChatBot {chatId} {conversationId} {userId} {botName} {botSubtitle} />
				</div>
			</div>
		</div>
	{:else}
		<!-- Mobile: Floating button -->
		<button
			onclick={toggle}
			class="bg-chat-primary hover:bg-chat-primary-dark ring-chat-primary-light fixed right-6 bottom-6 z-50 flex h-14 w-14 items-center justify-center rounded-full shadow-lg ring-4 transition-colors"
			aria-label="Open chat"
		>
			<MessageCircle class="h-7 w-7 text-white" />
		</button>
	{/if}
{:else}
	<!-- Desktop: Fixed bottom-right panel -->
	<div class="fixed right-6 bottom-6 z-50 flex flex-col items-end">
		{#if isOpen}
			<!-- Expanded state -->
			<div
				class="border-chat-primary-light mb-2 flex h-[600px] w-[420px] flex-col overflow-hidden rounded-2xl border shadow-xl"
			>
				<button
					onclick={toggle}
					class="bg-chat-bubble hover:bg-chat-bg flex w-full cursor-pointer items-center justify-start gap-4 rounded-t-2xl p-4 transition-colors"
				>
					<div class="relative">
						<div
							class="bg-chat-primary ring-chat-primary-light flex h-12 w-12 items-center justify-center rounded-full ring-4"
						>
							<MessageCircle class="h-6 w-6 text-white" />
						</div>
						<div
							class="border-chat-bubble absolute right-0 bottom-0 h-3 w-3 rounded-full border-2 bg-green-400"
						></div>
					</div>

					<div class="flex flex-1 flex-col items-start">
						<span class="text-chat-text line-clamp-1 text-lg leading-6 font-semibold"
							>{botName}</span
						>
						<span class="text-chat-primary line-clamp-1 text-sm leading-5 font-normal"
							>{botSubtitle}</span
						>
					</div>

					<ChevronDown class="text-chat-text-muted h-5 w-5" />
				</button>

				<div class="min-h-0 flex-1 overflow-hidden">
					<ChatBot {chatId} {conversationId} {userId} {botName} {botSubtitle} />
				</div>
			</div>
		{:else}
			<!-- Collapsed state - floating button -->
			<button
				onclick={toggle}
				class="bg-chat-primary hover:bg-chat-primary-dark ring-chat-primary-light flex h-14 w-14 items-center justify-center rounded-full shadow-lg shadow-md ring-4 transition-colors"
				aria-label="Open chat"
			>
				<MessageCircle class="h-7 w-7 text-white" />
			</button>
		{/if}
	</div>
{/if}
