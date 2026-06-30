<script lang="ts">
	import { enhance } from '$app/forms';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Upload } from 'lucide-svelte';

	let fileInput: HTMLInputElement | undefined;
	let uploadForm: HTMLFormElement | undefined;
</script>

<form
	bind:this={uploadForm}
	method="POST"
	action="/admin/media-library?/upload"
	enctype="multipart/form-data"
	use:enhance
>
	<input
		bind:this={fileInput}
		type="file"
		name="images"
		accept="image/*"
		multiple
		class="hidden"
		aria-hidden="true"
		oninput={() => {
			uploadForm?.submit();
		}}
	/>
	<Button
		onclick={() => {
			fileInput?.click();
		}}
	>
		<Upload class="h-4 w-4" />Upload
	</Button>
</form>
