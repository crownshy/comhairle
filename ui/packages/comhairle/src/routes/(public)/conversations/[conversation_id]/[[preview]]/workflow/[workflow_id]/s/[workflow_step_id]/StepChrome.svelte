<script lang="ts">
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import { useSupportDrawer } from '$lib/components/supportDrawerContext.svelte';
	import { CircleHelp } from '@lucide/svelte';
	import * as m from '$lib/paraglide/messages';
	import StepMenu from './StepMenu.svelte';
	import StepProgressBar from './StepProgressBar.svelte';
	import type { StepItem } from './stepItems';
	import { HEADER_PILL_CLASS, STEP_COLUMN_CLASS } from './styles';
	import { cn } from '$lib/utils';

	type Props = {
		steps: StepItem[];
		currentIndex: number;
		fill: number;
		/** Where the mark goes: this conversation's landing page. */
		introUrl: string;
		/** Marks an admin's preview here, in place of the full-width banner other pages get. */
		preview?: boolean;
	};

	let { steps, currentIndex, fill, introUrl, preview = false }: Props = $props();

	const supportDrawer = useSupportDrawer();
</script>

<header class="bg-background pb-2">
	<div class="mx-auto flex h-[72px] w-full max-w-5xl items-center gap-5 px-5 md:h-20 md:px-6">
		<ComhairleLogo
			href={introUrl}
			ariaLabel={m.step_chrome_conversation_home()}
			logoSize="sm"
			color="text-primary"
			class="-m-2 shrink-0 p-2"
		/>
		{#if preview}
			<span
				class="bg-sidebar text-sidebar-foreground shrink-0 rounded-full px-2.5 py-1 text-sm font-medium tracking-wide uppercase"
			>
				{m.conversation_preview_badge()}
			</span>
		{/if}
		<div class="flex min-w-0 flex-1 items-center justify-end gap-3">
			<!-- Below lg the drawer's sideways tab would cover the step, so it opens from here. -->
			<button
				type="button"
				class={cn(HEADER_PILL_CLASS, 'shrink-0 lg:hidden')}
				aria-label={m.support_find_out_more()}
				onclick={() => (supportDrawer.open = true)}
			>
				<CircleHelp class="size-5" aria-hidden="true" />
				<span class="max-sm:hidden" aria-hidden="true">{m.support_find_out_more()}</span>
			</button>
			<StepMenu {steps} {currentIndex} />
		</div>
	</div>
	<div class={STEP_COLUMN_CLASS}>
		<StepProgressBar {steps} {currentIndex} {fill} />
	</div>
</header>
