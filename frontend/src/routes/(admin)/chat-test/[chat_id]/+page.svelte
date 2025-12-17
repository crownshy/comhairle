<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import type { PageProps } from '../$types';

	let { data }: PageProps = $props();

	let responseText = $state('');
	let errorMessage = $state('');

	async function talkToChat(e) {
		e.preventDefault();

		const question = document.querySelector('#question')?.value;

		if (!question) return;

		try {
			await fetch(`http://127.0.0.1:3000/bot/chats/${data.chatId}`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					question,
					stream: true,
					session_id: 'a1a74fccd4d811f085052ee1a04d79f0'
				})
			}).then((response) => {
				const reader = response.body?.getReader();
				if (!reader) return;

				const decoder = new TextDecoder('utf-8');
				reader.read().then(function pump({ done, value }) {
					if (done) {
						return;
					}

					const text = decoder.decode(value, { stream: true });
					console.log();
					console.log('    >>>>    Chunk value', text);
					console.log();

					if (text.startsWith('data:')) {
						const trimmed = text.replace('data:', '');
						const json = JSON.parse(trimmed);
						responseText = json.data.answer;
					} else {
						try {
							const json = JSON.parse(text);
							if (json.code !== 0 && json.message) {
								errorMessage = json.message;
							}
						} catch (e) {
							console.error(e);
						}
					}

					reader.read().then(pump);
				});
			});
		} catch (e) {
			console.error(e);
		}
	}
</script>

<!-- Remove after testing -->
<h1 class="mb-8">Test conversing with AI chat</h1>

<form onsubmit={talkToChat} class="space-y-4">
	<Label>Ask a question</Label>
	<Input id="question" name="question" />

	<Label>AI Answer</Label>
	<p>{responseText}</p>
	<Button type="submit">Submit</Button>
</form>
