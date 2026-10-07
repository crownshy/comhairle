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
		Volume2,
		Workflow
	} from 'lucide-svelte';
	import { ADMIN_WELCOME as W, type WelcomeIcon } from '$lib/admin_welcome';
	import { GUIDE_NAV } from '$lib/tool_guides';
	import GuideRichText from '$lib/components/AdminGuide/GuideRichText.svelte';
	import ToolPlayground from '$lib/components/AdminGuide/ToolPlayground.svelte';

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
	<title>{W.title}</title>
</svelte:head>

<main class="w-full max-w-6xl min-w-0 p-6 md:p-10">
	<!-- Hero, with the curved background shape from the report design -->
	<header
		class="bg-primary text-primary-foreground relative overflow-hidden rounded-2xl px-6 py-10 md:px-10 md:py-14"
	>
		<!-- Layered curves from the report design's background, stretched to fit the hero.
		     Drawn in the hero's text colour at low opacity so they follow every theme. -->
		<svg
			class="pointer-events-none absolute inset-x-0 bottom-0 h-3/4 w-full"
			viewBox="0 0 1200 300"
			preserveAspectRatio="none"
			aria-hidden="true"
		>
			<path d="M0 210C260 110 620 60 1200 70V300H0Z" fill="currentColor" opacity="0.08" />
			<path d="M0 225C420 215 900 235 1200 290V300H0Z" fill="currentColor" opacity="0.12" />
		</svg>
		<!-- relative so the text sits above the absolutely positioned curves -->
		<div class="relative">
			<p class="text-sm font-semibold tracking-wide uppercase opacity-80">Welcome</p>
			<h1 class="mt-2 max-w-xl text-3xl font-bold md:text-5xl">{W.title}</h1>
			<p
				class="mt-3 inline-flex items-center gap-2 rounded-full bg-white/15 px-3 py-1 text-sm"
				title="How to say Comhairle"
			>
				<Volume2 class="size-4" aria-hidden="true" />
				Say it like “{W.comhairle.pronunciation}”
			</p>
			<p class="mt-4 max-w-2xl text-lg leading-7 opacity-90">{W.summary}</p>
		</div>
	</header>

	<!-- What the platform does: a simple, playful version of the tools playground -->
	<section class="mt-12" aria-labelledby="platform-heading">
		<h2 id="platform-heading" class="text-foreground text-xl font-semibold">
			What the platform does
		</h2>
		<p class="text-muted-foreground mt-1 max-w-3xl">
			<GuideRichText
				text="In Comhairle, you build a [conversation](/admin/info/glossary#term-conversation) from [steps](/admin/info/glossary#term-step). Each step uses an [engagement tool](/admin/info/glossary#term-engagement-tool), and what people say in one step can shape the next. Have a go: put a few tools in order."
			/>
		</p>
		<ToolPlayground simple class="mt-5" />
	</section>

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
							<span class="text-muted-foreground text-sm font-semibold">{i + 1}</span>
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
		<!-- Who / what -->
		<div class="mt-10 grid gap-4 md:grid-cols-2">
			<section class="border-border rounded-2xl border p-6">
				<h2 class="text-foreground text-lg font-semibold">{W.crownshy.heading}</h2>
				<p class="text-muted-foreground mt-2 leading-7">
					<GuideRichText text={W.crownshy.text} />
				</p>
			</section>
			<section class="border-border rounded-2xl border p-6">
				<h2 class="text-foreground text-lg font-semibold">{W.comhairle.heading}</h2>
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
	</section>
</main>
