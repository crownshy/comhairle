<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';

	let { data } = $props();
	let conversation = $derived(data.conversation);
</script>

<div>
	<Label for="title">Title</Label>
	<Input value={conversation.title} name="title" />
</div>

<div>
	<Label for="short_description">Short Description</Label>
	<Textarea value={conversation.short_description} name="title" />
</div>
<div>
	<Label for="description">Description</Label>
	<Textarea value={conversation.description} name="description" />
</div>

<div>
	<Label for="image">Banner Image</Label>
	<Input value={conversation.image_url} name="image_url" />
</div>

<div>
	<Label for="is_public">Show conversation publically</Label>
	<Switch checked={conversation.is_public} name="is_public" />
</div>

<div>
	<Label for="is_invite_only">Only allow participation by invite</Label>
	<Switch checked={conversation.is_invite_only} name="is_invite_only" />
</div>
