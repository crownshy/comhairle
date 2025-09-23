<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
</script>

<div class="ml-5 flex flex-row space-x-2 md:w-40">
	<Button href="/auth/signup" class="text-nav-text" variant="link">Signup</Button>
	<Button href="/auth/login" class="text-nav-text" variant="link">Login</Button>
</div>
