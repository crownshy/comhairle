<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	let { children }: LayoutProps = $props();
</script>

<div class="mt-10 flex h-full w-full flex-col gap-10 md:flex-row">
	<div class="flex flex-col">
		<Button variant="ghost" href="/rights/tos">Terms of service</Button>
		<Button variant="ghost" href="/rights/privacy">Privacy</Button>
		<Button variant="ghost" href="/rights/cookies">Cookie Policy</Button>
	</div>
	<div class="border-l-grey h-full overflow-y-auto">
		{@render children()}
	</div>
</div>
