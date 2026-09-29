<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as m from '$lib/paraglide/messages';
</script>

<div class="ml-5 flex flex-row space-x-2">
	<Button href="/auth/signup" class="text-primary-foreground text-base" variant="link"
		>{m.signup()}</Button
	>
	<Button href="/api/auth/login" class="text-primary-foreground text-base" variant="link"
		>{m.login()}</Button
	>
	<Button
		href="/api/auth/login?is_admin=true"
		class="text-primary-foreground text-base"
		variant="link">{m.login_admin()}</Button
	>
</div>
