<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';
</script>

<!--
	Approximates the HeyForm builder's three-pane chrome (content list / question canvas / settings)
	so the Setup tab reads as "the editor is loading" instead of a blank frame while the cross-origin
	iframe logs in and renders. The side panes drop away on narrow widths, matching the real builder.
-->
<div class="bg-muted flex h-full w-full gap-px overflow-hidden">
	<!-- Left: content list -->
	<div class="bg-background hidden w-56 shrink-0 flex-col gap-4 p-4 md:flex">
		<div class="flex items-center justify-between">
			<Skeleton class="h-5 w-20" />
			<Skeleton class="size-5 rounded-md" />
		</div>
		<Skeleton class="h-12 w-full rounded-md" />
		<Skeleton class="h-12 w-full rounded-md" />
	</div>

	<!-- Center: question canvas -->
	<div class="flex flex-1 items-center justify-center p-6">
		<div
			class="bg-card flex h-full w-full max-w-3xl flex-col justify-center gap-4 rounded-lg p-10 shadow-sm"
		>
			<Skeleton class="h-4 w-24" />
			<Skeleton class="h-8 w-2/3" />
			<Skeleton class="h-5 w-1/2" />
			<Skeleton class="mt-4 h-10 w-full max-w-md" />
			<Skeleton class="mt-2 h-10 w-28 rounded-md" />
		</div>
	</div>

	<!-- Right: settings -->
	<div class="bg-background hidden w-72 shrink-0 flex-col gap-6 p-5 lg:flex">
		<div class="flex gap-6">
			<Skeleton class="h-5 w-16" />
			<Skeleton class="h-5 w-16" />
			<Skeleton class="h-5 w-12" />
		</div>
		<div class="flex flex-col gap-2">
			<Skeleton class="h-4 w-12" />
			<Skeleton class="h-10 w-full rounded-md" />
		</div>
		<div class="flex flex-col gap-3">
			<Skeleton class="h-4 w-20" />
			<Skeleton class="h-8 w-full rounded-md" />
			<Skeleton class="h-8 w-full rounded-md" />
		</div>
	</div>
</div>
