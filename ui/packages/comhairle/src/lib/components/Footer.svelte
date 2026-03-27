<script lang="ts">
	import * as m from '$lib/paraglide/messages';
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import { Separator } from '$lib/components/ui/separator';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';

	let emailValue = $state('');

	const navLinks = [
		{ href: '/', label: () => m.home() },
		{ href: '/about', label: () => m.about_us() },
		{ href: '/contact', label: () => m.contact() },
		{ href: '/support', label: () => m.support() },
		{ href: '/faq', label: () => m.faq() }
	];

	const legalLinks = [
		{ href: '/rights/privacy', label: () => m.privacy_policy() },
		{ href: '/rights/tos', label: () => m.terms_of_service() },
		{ href: '/rights/cookies', label: () => m.cookies_settings() }
	];
</script>

<footer class="bg-sidebar text-sidebar-foreground w-full">
	<div class="mx-auto flex w-full max-w-[1440px] flex-col items-center py-24">
		<div class="flex w-full max-w-[1280px] flex-col gap-12 px-6">
			<!-- Top row: Logo + Nav + Subscribe -->
			<div
				class="flex flex-col items-center gap-8 md:h-[84px] md:flex-row md:justify-between"
			>
				<!-- Logo + Nav -->
				<div
					class="flex flex-col items-center gap-6 md:flex-1 md:flex-row md:justify-between"
				>
					<ComhairleLogo color="text-sidebar-foreground" />
					<nav class="flex flex-wrap items-center justify-center gap-8">
						{#each navLinks as link (link.href)}
							<a
								href={link.href}
								class="text-sidebar-foreground/70 hover:text-sidebar-foreground text-lg leading-tight font-medium transition-colors"
							>
								{link.label()}
							</a>
						{/each}
					</nav>
				</div>
			</div>

			<!-- Separator -->
			<Separator class="bg-sidebar-foreground/20" />

			<!-- Bottom row: Copyright + Legal links -->
			<div class="flex flex-col items-center gap-4 md:flex-row md:justify-between">
				<p class="text-sidebar-foreground/50 order-2 text-base font-normal md:order-1">
					Copyright {new Date().getFullYear()} &copy; Crown Shy
				</p>
				<div class="order-1 flex flex-wrap items-center justify-center gap-8 md:order-2">
					{#each legalLinks as link (link.href)}
						<a
							href={link.href}
							class="text-sidebar-foreground/50 hover:text-sidebar-foreground/80 text-base font-normal transition-colors"
						>
							{link.label()}
						</a>
					{/each}
				</div>
			</div>
		</div>
	</div>
</footer>
