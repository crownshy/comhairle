<script lang="ts">
	import * as m from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import { STEP_COLUMN_CLASS } from './styles';
</script>

<div
	class={cn(STEP_COLUMN_CLASS, 'flex flex-col items-center pt-3 pb-2')}
	aria-busy="true"
	aria-live="polite"
>
	<div class="flex w-full flex-col items-center gap-2">
		<div class="bg-muted/60 h-3 w-20 animate-pulse rounded"></div>
		<div class="bg-muted/60 h-6 w-3/5 animate-pulse rounded md:h-7"></div>
	</div>

	<div class="mx-auto mt-4 flex w-full max-w-3xl flex-col items-center gap-2">
		<div class="bg-muted/50 h-3 w-4/5 animate-pulse rounded"></div>
		<div class="bg-muted/50 h-3 w-3/5 animate-pulse rounded"></div>
	</div>
	<span class="sr-only">{m.step_loading()}</span>
</div>
