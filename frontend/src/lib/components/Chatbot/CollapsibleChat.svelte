<script lang="ts">
	import { MessageCircle, X, ChevronDown } from 'lucide-svelte';
	import ChatBot from './ChatBot.svelte';

	interface Props {
		chatId?: string;
		botName?: string;
		botSubtitle?: string;
	}

	let { chatId, botName = "Tutor bot", botSubtitle = "Ask questions" }: Props = $props();
	
	let isOpen = $state(false);

	function toggle() {
		isOpen = !isOpen;
	}
</script>

<div class=" fixed bottom-6 right-6 z-50 flex flex-col items-end">
	{#if isOpen}
		<!-- Expanded state -->
		<div class="mb-2 w-[420px] h-[600px] flex flex-col rounded-2xl shadow-xl overflow-hidden border border-chat-primary-light">
			<button
				onclick={toggle}
				class="w-full p-4 bg-white rounded-t-2xl flex justify-start items-center gap-4 hover:bg-chat-bg transition-colors cursor-pointer"
			>
				<div class="relative">
					<div class="w-12 h-12 bg-chat-primary rounded-full ring-4 ring-chat-primary-light flex items-center justify-center">
						<MessageCircle class="w-6 h-6 text-white" />
					</div>
					<div class="w-3 h-3 absolute bottom-0 right-0 bg-green-400 rounded-full border-2 border-white"></div>
				</div>
				
				<div class="flex-1 flex flex-col items-start">
					<span class="text-chat-text text-lg font-semibold leading-6 line-clamp-1">{botName}</span>
					<span class="text-chat-primary text-sm font-normal leading-5 line-clamp-1">{botSubtitle}</span>
				</div>

				<ChevronDown class="w-5 h-5 text-chat-text-muted" />
			</button>

			<div class="flex-1 min-h-0 overflow-hidden">
				<ChatBot {chatId} {botName} {botSubtitle} />
			</div>
		</div>
	{:else}
		<!-- Collapsed state - floating button -->
		<button
			onclick={toggle}
			class="w-14 h-14 bg-chat-primary shadow-md rounded-full shadow-lg flex items-center justify-center hover:bg-chat-primary-dark transition-colors ring-4 ring-chat-primary-light"
			aria-label="Open chat"
		>
			<MessageCircle class="w-7 h-7 text-white" />
		</button>
	{/if}
</div>
