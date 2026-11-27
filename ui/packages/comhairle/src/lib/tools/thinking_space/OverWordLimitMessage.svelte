<script lang="ts">
	import { m } from '$lib/paraglide/messages';

	type Props = {
		id: string;
		max: number;
	};

	let { id, max }: Props = $props();
</script>

<!-- Render this only while over the limit: the alert is read once when it mounts, not per keystroke. -->
<p {id} role="alert" class="text-destructive text-base">
	{m.thinking_space_over_word_limit({ max })}
</p>
