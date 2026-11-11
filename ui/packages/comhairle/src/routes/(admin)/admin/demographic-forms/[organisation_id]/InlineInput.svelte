<script lang="ts">
	import { Textarea, Input } from '$lib/components/EasyForm';
	import { cn } from '$lib/utils';
	import type { ComponentProps } from 'svelte';

	type InputProps = ComponentProps<typeof Input> & {
		multiline: false;
	};
	type TextareaProps = ComponentProps<typeof Textarea> & {
		multiline: true;
	};
	type Props = InputProps | TextareaProps;

	let { class: className, multiline = false, ...props }: Props = $props();
</script>

{#if !multiline}
	<Input
		{...props as ComponentProps<typeof Input>}
		class={cn(
			'border-primary rounded-none border-0 border-b-3 bg-transparent! pl-1',
			className
		)}
	/>
{:else}
	<Textarea
		{...props as ComponentProps<typeof Textarea>}
		class={cn(
			'border-primary min-h-0 resize-none rounded-none border-0 border-b-3 bg-transparent! pl-1',
			className
		)}
	></Textarea>
{/if}
