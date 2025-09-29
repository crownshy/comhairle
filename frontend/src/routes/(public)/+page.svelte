<script lang="ts">
	import * as m from '$lib/paraglide/messages';
	import Button from '$lib/components/ui/button/button.svelte';
</script>

<div class="flex h-full flex-col items-center justify-center gap-y-10 p-10">
	<h1
		class="scroll-m-20 text-center text-4xl font-extrabold tracking-tight md:text-left lg:text-5xl"
	>
		{m.welcome_to_comhairle()}
	</h1>
	<p class="text-center text-2xl md:text-left">{m.a_platfrom_about_what_people_want()}</p>
	<Button size="lg" href="/conversations">{m.explore_conversations()}</Button>
</div>
