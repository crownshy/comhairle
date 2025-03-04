<script lang="ts">
	import { buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
</script>

<div class="flex flex-row space-x-2 md:w-40">
	<a class={cn('flex-1', buttonVariants({ variant: 'outline' }))} href="/auth/signup">Signup</a>
	<a class={cn('flex-1', buttonVariants({ variant: 'default' }))} href="/auth/login">Login</a>
</div>
