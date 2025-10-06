<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	let { children }: LayoutProps = $props();
</script>

<h1 class="my-10 text-4xl font-bold">Your Rights</h1>
<div class="mt-10 flex h-full w-full flex-col gap-10 md:flex-row">
	<div class="flex flex-col">
		<Button variant="ghost" href="/rights/tos">Terms of service</Button>
		<Button variant="ghost" href="/rights/privacy">Privacy</Button>
		<Button variant="ghost" href="/rights/cookies">Cookie Policy</Button>
	</div>
	<div class="border-l-grey h-full grow overflow-y-auto">
		{@render children()}
	</div>
</div>
