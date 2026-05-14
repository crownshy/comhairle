<script lang="ts">
	import PrioritisationUser from './PrioritisationUser.svelte';
	import type { PrioritisationStore } from './store.svelte';

	let { store }: { store: PrioritisationStore } = $props();
</script>

<div class="rounded-md border p-2">
	<div class="bg-muted text-muted-foreground -m-2 mb-2 rounded-t-md p-1 text-center text-xs">
		Preview — your changes are not saved here
	</div>
	<PrioritisationUser
		storeOverride={store}
		participantId={`preview:${store.stepId}`}
		readonly={false}
	/>
</div>
