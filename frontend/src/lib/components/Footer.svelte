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

<footer class="w-full bg-sidebar text-sidebar-foreground">
	<div class="mx-auto flex w-full max-w-[1440px] flex-col items-center py-24">
		<div class="flex w-full max-w-[1280px] flex-col gap-12 px-6">
			<!-- Top row: Logo + Nav + Subscribe -->
			<div class="flex flex-col items-center gap-8 lg:h-[84px] lg:flex-row lg:justify-between">
				<!-- Logo + Nav -->
				<div class="flex flex-col items-center gap-6 lg:flex-1 lg:flex-row">
					<ComhairleLogo logoSize="lg" />
					<nav class="flex flex-wrap items-center justify-center gap-8">
						{#each navLinks as link (link.href)}
							<a
								href={link.href}
								class="text-lg font-medium leading-tight text-sidebar-foreground/70 transition-colors hover:text-sidebar-foreground"
							>
								{link.label()}
							</a>
						{/each}
					</nav>
				</div>

				<!-- Subscribe form -->
				<div class="hidden lg:block">
					<form class="flex items-center gap-2" onsubmit={(e) => e.preventDefault()}>
						<Input
							type="email"
							bind:value={emailValue}
							placeholder={m.your_email()}
							class="w-[250px]"
						/>
						<Button type="submit" variant="default">
							{m.subscribe()}
						</Button>
					</form>
				</div>
			</div>

			<!-- Separator -->
			<Separator class="bg-sidebar-foreground/20" />

			<!-- Bottom row: Copyright + Legal links -->
			<div class="flex flex-col items-center gap-4 md:flex-row md:justify-between">
				<p class="text-base font-normal text-sidebar-foreground/50">
					Copyright {new Date().getFullYear()} &copy; Crown Shy
				</p>
				<div class="flex flex-wrap items-center justify-center gap-8">
					{#each legalLinks as link (link.href)}
						<a
							href={link.href}
							class="text-base font-normal text-sidebar-foreground/50 transition-colors hover:text-sidebar-foreground/80"
						>
							{link.label()}
						</a>
					{/each}
				</div>
			</div>
		</div>
	</div>
</footer>
