<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import type { PageProps } from '../$types';

	let { data }: PageProps = $props();

	async function uploadDocument(e) {
		e.preventDefault();
		const formData = new FormData(e.target);

		try {
			const result = await fetch(
				`http://127.0.0.1:3000/bot/upload_documents/${data.knowledgebaseId}`,
				{
					method: 'POST',
					body: formData
				}
			);
		} catch (e) {
			console.error(e);
		}
	}
</script>

<!-- Remove after testing -->
<h1>Test uploading of documents</h1>

<form onsubmit={uploadDocument}>
	<Label>Upload document</Label>
	<Input type="file" id="documents" name="documents" />

	<Button type="submit">Submit</Button>
</form>
