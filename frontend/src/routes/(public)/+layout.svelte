<script lang="ts">
	import Footer from '$lib/components/Footer.svelte';
	import NavBar from '$lib/components/NavBar.svelte';

	let { children, data }: LayoutProps = $props();
</script>

<div class="flex h-screen w-full flex-col">
	<NavBar user={data.user} />
	<div class="mx-auto w-full max-w-[1300px] grow px-4 md:px-20">
		{@render children()}
	</div>
	<Footer />
</div>
