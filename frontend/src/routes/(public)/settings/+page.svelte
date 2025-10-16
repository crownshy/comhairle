<script lang="ts">
	import { Settings } from 'lucide-svelte';
	import type { PageProps } from './$types';

	let props: PageProps = $props();
</script>

<div class="mt-10 flex flex-row items-center gap-4">
	<Settings size={42} />
	<h1 class="text-4xl">Settings</h1>

	<h2 class="text-2xl">Coming soon</h2>
</div>
