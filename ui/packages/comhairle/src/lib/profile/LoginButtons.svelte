<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { m } from '$lib/paraglide/messages';
</script>

<!-- Sign up is the filled pill. Its text colour comes from --nav-pill-text, which the NavBar
	sets when it sits on a light hero and the pill turns dark. -->
<div class="flex items-center gap-1 sm:gap-2">
	<Button
		href="/auth/login"
		variant="nav"
		class="hidden h-10 px-3 text-base font-semibold shadow-none sm:flex">{m.login()}</Button
	>
	<Button
		href="/auth/signup"
		class="bg-primary-foreground hover:bg-primary-foreground/90 h-10 px-5 text-base font-semibold text-[var(--nav-pill-text,var(--primary))]"
		>{m.sign_up()}</Button
	>
</div>
