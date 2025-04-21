<script lang="ts">
	import NavBar from '$lib/components/NavBar.svelte';

	let { children, data }: LayoutProps = $props();
</script>

<div class="flex h-screen w-full flex-col">
	<NavBar user={data.user} />
	<div class="mx-auto w-full max-w-[1300px] grow overflow-y-auto px-4 md:px-20">
		{@render children()}
	</div>

	<div class="w-full py-4 text-center text-muted-foreground">© 2025 CrownShy Ltd</div>
</div>
