<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import * as m from '$lib/paraglide/messages';
</script>

<div class="ml-5 flex flex-row space-x-2 md:w-40">
	<Button href="/auth/signup" class="text-nav-text" variant="link">{m.signup()}</Button>
	<Button href="/auth/login" class="text-nav-text" variant="link">{m.login()}</Button>
</div>
