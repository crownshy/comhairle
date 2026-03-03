<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';

	type Props = {
		onSubmit: (statement:string)=>void
	}

	let {onSubmit}: Props= $props()

	let text = $state("")

	function submit(){
		onSubmit(text)
		text=""
	}

</script>

<Dialog.Root>
	<Dialog.Trigger>
			<Button>Share your view</Button>
	</Dialog.Trigger>
	<Dialog.Content>

	<Dialog.Header>
			        <Dialog.Title>Share your view</Dialog.Title>
        <Dialog.Description>
			Share a view about the topic. Good views are standalone and speak to your presonal experince 
        </Dialog.Description>
	</Dialog.Header>
		<Textarea bind:value={text} />
		<Button onclick={submit}>Share</Button>

	</Dialog.Content>

</Dialog.Root>
