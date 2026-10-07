<script lang="ts">
	import {
		Activity,
		ArrowRight,
		FilePlus2,
		FileText,
		MessageSquareWarning,
		Rocket,
		ShieldCheck,
		UserPen,
		UserRound,
		Workflow
	} from 'lucide-svelte';
	import { ADMIN_WELCOME, type WelcomeIcon } from '$lib/admin_welcome';
	import { GUIDE_NAV } from '$lib/tool_guides';
	import GuideRichText from '$lib/components/AdminGuide/GuideRichText.svelte';

	const W = ADMIN_WELCOME;
	const ICONS: Record<WelcomeIcon, typeof Activity> = {
		create: FilePlus2,
		design: Workflow,
		launch: Rocket,
		run: Activity,
		moderate: ShieldCheck,
		report: FileText
	};
	// One icon per "where to start" card, in the same order as ADMIN_WELCOME.paths.items.
	const PATH_ICONS = [UserRound, UserPen, MessageSquareWarning];
</script>

<svelte:head>
	<title>{W.title} - Comhairle Admin Guide</title>
</svelte:head>

<main class="w-full max-w-6xl min-w-0 rounded-xl p-6 md:p-10">
	<article class="max-w-4xl">
		<p class="text-muted-foreground text-sm font-medium">Start here</p>
		<h1 class="text-foreground mt-1 text-3xl font-bold">{W.title}</h1>
		<p class="text-muted-foreground mt-3 max-w-3xl text-lg leading-7">{W.summary}</p>

		<!-- Who we are / what the platform is -->
		<div class="mt-10 grid gap-8 lg:grid-cols-[1fr_2fr]">
			<section aria-labelledby="crownshy-heading">
				<h2 id="crownshy-heading" class="text-foreground text-lg font-semibold">
					{W.crownshy.heading}
				</h2>
				<p class="text-muted-foreground mt-2 leading-7">
					<GuideRichText text={W.crownshy.text} />
				</p>
			</section>
			<section aria-labelledby="comhairle-heading">
				<h2 id="comhairle-heading" class="text-foreground text-lg font-semibold">
					{W.comhairle.heading}
				</h2>
				{#each W.comhairle.paragraphs as paragraph, i (i)}
					<p class="text-muted-foreground mt-2 leading-7">
						<GuideRichText text={paragraph} />
					</p>
				{/each}
				<h3 class="text-foreground mt-5 text-sm font-semibold">
					{W.comhairle.toolsHeading}
				</h3>
				<ul class="mt-2 flex flex-wrap gap-2">
					{#each GUIDE_NAV as tool (tool.key)}
						<li>
							<a
								href={`/admin/info/tools/${tool.key}`}
								class="border-border hover:border-primary/40 hover:bg-muted/60 flex flex-col rounded-lg border px-3 py-2 text-sm transition-colors"
							>
								<span class="text-foreground font-medium">{tool.navLabel}</span>
								{#if tool.atAGlance?.bestFor}
									<span class="text-muted-foreground text-xs"
										>{tool.atAGlance.bestFor}</span
									>
								{/if}
							</a>
						</li>
					{/each}
				</ul>
			</section>
		</div>

		<!-- What an admin does: the conversation lifecycle, each card leading to its tutorials -->
		<section class="mt-12" aria-labelledby="responsibilities-heading">
			<h2 id="responsibilities-heading" class="text-foreground text-xl font-semibold">
				{W.responsibilities.heading}
			</h2>
			<p class="text-muted-foreground mt-1">{W.responsibilities.intro}</p>
			<ol class="mt-5 grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
				{#each W.responsibilities.items as item, i (item.title)}
					{@const Icon = ICONS[item.icon]}
					<li>
						<a
							href={item.href}
							class="group border-border bg-card hover:border-primary/40 flex h-full flex-col rounded-xl border p-5 transition-colors hover:shadow-sm"
						>
							<div class="flex items-center justify-between">
								<span
									class="bg-accent text-accent-foreground flex size-10 items-center justify-center rounded-lg"
								>
									<Icon class="size-5" aria-hidden="true" />
								</span>
								<span class="text-muted-foreground text-sm font-semibold"
									>{i + 1}</span
								>
							</div>
							<h3 class="text-foreground mt-4 font-semibold">{item.title}</h3>
							<p class="text-muted-foreground mt-1 flex-1 text-sm leading-6">
								{item.text}
							</p>
							<span
								class="text-primary mt-4 inline-flex items-center gap-1 text-sm font-semibold"
							>
								{item.linkLabel}
								<ArrowRight
									class="size-4 transition-transform group-hover:translate-x-1"
									aria-hidden="true"
								/>
							</span>
						</a>
					</li>
				{/each}
			</ol>
		</section>

		<!-- Where to start: pick the description that sounds like you -->
		<section class="mt-12" aria-labelledby="paths-heading">
			<h2 id="paths-heading" class="text-foreground text-xl font-semibold">
				{W.paths.heading}
			</h2>
			<p class="text-muted-foreground mt-1">{W.paths.intro}</p>
			<div class="mt-5 grid gap-3 lg:grid-cols-3">
				{#each W.paths.items as path, i (path.title)}
					{@const Icon = PATH_ICONS[i]}
					<section class="bg-nav-background flex flex-col rounded-xl p-5">
						<Icon class="text-primary size-6" aria-hidden="true" />
						<h3 class="text-foreground mt-3 text-lg leading-snug font-semibold">
							“{path.title}”
						</h3>
						<p class="text-muted-foreground mt-2 text-sm leading-6">
							<GuideRichText text={path.text} />
						</p>
						<ul class="mt-4 flex flex-col gap-1">
							{#each path.links as link (link.href)}
								<li>
									<a
										href={link.href}
										class="group text-foreground hover:bg-card -mx-2 flex items-center justify-between gap-2 rounded-md px-2 py-1.5 text-sm font-medium transition-colors"
									>
										{link.label}
										<ArrowRight
											class="text-primary size-4 shrink-0 transition-transform group-hover:translate-x-1"
											aria-hidden="true"
										/>
									</a>
								</li>
							{/each}
						</ul>
					</section>
				{/each}
			</div>
		</section>
	</article>
</main>
