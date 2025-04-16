<script lang="ts">
	import NavBar from '$lib/components/NavBar.svelte';

	let { children, data }: LayoutProps = $props();
</script>

<NavBar user={data.user} />
<div class="mx-auto mb-16 w-full max-w-[1300px] pt-40 md:px-20">
	{@render children()}
</div>
