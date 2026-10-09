<script lang="ts">
	import { ArrowRight, Clock, Play, Timer, Users, X } from 'lucide-svelte';
	import { GUIDE_NAV, TOOL_GUIDES } from '$lib/tool_guides';
	import { TEMPLATE_STEP_TOOLS, TOOL_OVERVIEW } from '$lib/tool_overview';
	import { conversationTemplates } from '$lib/conversation_templates';
	import ToolPlayground from '$lib/components/AdminGuide/ToolPlayground.svelte';
	import { cn } from '$lib/utils';
	import { toolIcon } from '$lib/components/AdminGuide/toolIcons';

	const tools = GUIDE_NAV.map((tool) => ({ ...tool, overview: TOOL_OVERVIEW[tool.key] }));

	let playground: ToolPlayground;
	let playgroundSection: HTMLElement;

	function tryTemplate(key: string) {
		playground.loadTemplate(key);
		playgroundSection.scrollIntoView({ behavior: 'smooth', block: 'start' });
	}

	// "What do you want participants to do?" Picking a goal highlights its tool.
	let picked = $state<string | null>(null);
</script>

<svelte:head>
	<title>Engagement tools - Comhairle Handbook</title>
</svelte:head>

<main class="w-full max-w-6xl min-w-0 rounded-xl p-6 md:p-10">
	<article class="max-w-4xl">
		<p class="text-muted-foreground text-sm font-medium">Reference</p>
		<h1 class="text-foreground mt-1 text-2xl font-bold">Engagement tools</h1>
		<p class="text-muted-foreground mt-3 max-w-3xl text-lg leading-7">
			Every step in a conversation uses one engagement tool. Mix them in any order, so what
			people say in one step can shape the next.
		</p>

		<!-- Pick by goal -->
		<section class="mt-8" aria-labelledby="goal-heading">
			<h2 id="goal-heading" class="text-foreground font-semibold">
				What do you want participants to do?
			</h2>
			<div class="mt-3 flex flex-wrap gap-2">
				{#each tools as tool (tool.key)}
					{#if tool.overview}
						{@const selected = picked === tool.key}
						<button
							type="button"
							aria-pressed={selected}
							onclick={() => (picked = selected ? null : tool.key)}
							class="rounded-full border px-4 py-2 text-sm font-medium transition-colors {selected
								? 'border-primary bg-primary text-primary-foreground'
								: 'border-border text-foreground hover:border-primary/50 bg-card'}"
						>
							{tool.overview.goal}
						</button>
					{/if}
				{/each}
				{#if picked}
					<button
						type="button"
						onclick={() => (picked = null)}
						class="text-muted-foreground hover:text-foreground inline-flex items-center gap-1 px-2 text-sm"
					>
						<X class="size-4" aria-hidden="true" /> Show all
					</button>
				{/if}
			</div>
		</section>

		<!-- The tools -->
		<ul class="mt-6 grid gap-4 sm:grid-cols-2" aria-live="polite">
			{#each tools as tool (tool.key)}
				{@const Icon = toolIcon(tool.key)}
				{@const dimmed = picked !== null && picked !== tool.key}
				{@const highlighted = picked === tool.key}
				<li
					id={`tool-${tool.key}`}
					class="transition-all duration-300 {dimmed ? 'opacity-40 grayscale' : ''}"
				>
					<a
						href={`/admin/info/tools/${tool.key}`}
						class="group border-border bg-card flex h-full flex-col rounded-2xl border p-5 transition-all hover:-translate-y-0.5 hover:shadow-md {highlighted
							? 'border-primary ring-primary/20 ring-2'
							: 'hover:border-primary/40'}"
					>
						<div class="flex items-start gap-4">
							<span
								class="bg-accent text-accent-foreground group-hover:bg-primary group-hover:text-primary-foreground flex size-12 shrink-0 items-center justify-center rounded-xl transition-colors"
							>
								<Icon class="size-6" aria-hidden="true" />
							</span>
							<span>
								<span class="text-foreground block text-lg font-semibold"
									>{tool.navLabel}</span
								>
								{#if tool.atAGlance}
									<span class="text-primary block text-sm font-medium"
										>Best for: {tool.atAGlance.bestFor.toLowerCase()}</span
									>
								{/if}
							</span>
						</div>
						{#if tool.atAGlance}
							<dl class="text-muted-foreground mt-4 grid gap-1.5 text-sm">
								<div class="flex items-center gap-2">
									<Clock class="size-4 shrink-0" aria-hidden="true" />
									<dt class="sr-only">Time for participants</dt>
									<dd>{tool.atAGlance.participantTime} for participants</dd>
								</div>
								<div class="flex items-center gap-2">
									<Timer class="size-4 shrink-0" aria-hidden="true" />
									<dt class="sr-only">Set-up time</dt>
									<dd>Set-up: {tool.atAGlance.setupTime}</dd>
								</div>
							</dl>
						{/if}
						<span
							class="text-primary mt-auto inline-flex items-center gap-1 pt-4 text-sm font-semibold"
						>
							Read the guide
							<ArrowRight
								class="size-4 transition-transform group-hover:translate-x-1"
								aria-hidden="true"
							/>
						</span>
					</a>
				</li>
			{/each}
		</ul>

		<!-- The templates admins see under "Choose from templates", read from conversation_templates.ts -->
		<section class="mt-12" aria-labelledby="templates-heading">
			<h2 id="templates-heading" class="text-foreground text-xl font-semibold">
				Start from a template
			</h2>
			<p class="text-muted-foreground mt-1">
				These are the templates you’ll find under <strong class="font-semibold"
					>Choose from templates</strong
				> in Process design. Each one combines tools in a tried and tested order.
			</p>
			<div class="mt-5 flex flex-col gap-4">
				{#each conversationTemplates.filter((t) => t.available) as template (template.key)}
					<section
						id={`template-${template.key}`}
						class="bg-nav-background target:ring-primary scroll-mt-6 rounded-2xl p-6 target:ring-2"
					>
						<div class="flex flex-wrap items-center gap-x-3 gap-y-2">
							<h3 class="text-foreground text-lg font-semibold">{template.name}</h3>
							<ul class="flex flex-wrap gap-1.5" aria-label="Good for">
								{#each template.badges as badge (badge.label)}
									{@const BadgeIcon = badge.icon}
									<li
										class={cn(
											'text-foreground inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-medium',
											badge.class
										)}
									>
										<BadgeIcon class="size-3.5" aria-hidden="true" />
										{badge.label}
									</li>
								{/each}
							</ul>
						</div>
						<p class="text-muted-foreground mt-1 text-sm">{template.description}</p>

						<ol class="mt-4 flex flex-wrap items-center gap-2">
							{#each template.displaySteps as step, i (i)}
								{@const tool = TEMPLATE_STEP_TOOLS[step.label]}
								<li class="flex items-center gap-2">
									{#if tool && TOOL_GUIDES[tool]}
										{@const Icon = toolIcon(tool)}
										<a
											href={`#tool-${tool}`}
											onclick={() => (picked = tool)}
											title={step.description}
											class="bg-card border-border hover:border-primary inline-flex items-center gap-2 rounded-full border py-1.5 pr-4 pl-1.5 text-sm font-medium transition-colors"
										>
											<span
												class="bg-primary text-primary-foreground flex size-7 items-center justify-center rounded-full"
											>
												<Icon class="size-4" aria-hidden="true" />
											</span>
											{TOOL_GUIDES[tool].navLabel}
										</a>
									{:else}
										<span
											title={step.description}
											class="bg-card/60 border-border text-foreground inline-flex items-center gap-2 rounded-full border border-dashed py-1.5 pr-4 pl-1.5 text-sm font-medium"
										>
											<span
												class="bg-accent text-accent-foreground flex size-7 items-center justify-center rounded-full"
											>
												<Users class="size-4" aria-hidden="true" />
											</span>
											{step.label}
										</span>
									{/if}
									{#if i < template.displaySteps.length - 1}
										<ArrowRight
											class="text-muted-foreground size-4"
											aria-hidden="true"
										/>
									{/if}
								</li>
							{/each}
						</ol>

						<button
							type="button"
							onclick={() => tryTemplate(template.key)}
							class="text-primary mt-4 inline-flex items-center gap-1.5 text-sm font-semibold hover:underline"
						>
							<Play class="size-4" aria-hidden="true" /> Try it in the playground
						</button>
					</section>
				{/each}
			</div>
		</section>

		<!-- Playground: try putting tools in different orders -->
		<section
			class="mt-12 scroll-mt-6"
			aria-labelledby="playground-heading"
			bind:this={playgroundSection}
		>
			<h2 id="playground-heading" class="text-foreground text-xl font-semibold">
				Try it yourself
			</h2>
			<p class="text-muted-foreground mt-1">
				Build a practice conversation: drag tools into steps, change their order, and see
				how long it takes.
			</p>
			<div class="mt-5">
				<ToolPlayground bind:this={playground} />
			</div>
		</section>
	</article>
</main>
