<script lang="ts">
	import MessagesSquare from '@lucide/svelte/icons/messages-square';
	import { m } from '$lib/paraglide/messages';
</script>

<section
	class="bg-card border-border flex flex-col items-center gap-4 rounded-3xl border px-6 py-10 text-center"
>
	<span class="bg-primary/10 text-primary flex size-16 items-center justify-center rounded-full">
		<MessagesSquare class="size-8" aria-hidden="true" />
	</span>
	<h2 class="text-2xl font-bold">{m.home_empty_title()}</h2>
	<p class="text-muted-foreground max-w-md text-base">{m.home_empty_description()}</p>
</section>
