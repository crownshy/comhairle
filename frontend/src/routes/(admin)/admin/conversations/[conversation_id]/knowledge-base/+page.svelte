<script lang="ts">
	import FileUpload from '$lib/components/FileUpload/FileUpload.svelte';
	import { Database } from 'lucide-svelte';

	let { data } = $props();
	let conversation = $derived(data.conversation);
</script>

<h1 class="mb-10 flex flex-row items-center gap-2 text-4xl"><Database /> Knowledge Base</h1>
<p class="mb-10">Use this space to manage your conversation's knowledge base</p>

<FileUpload conversation_id={conversation.id} />
