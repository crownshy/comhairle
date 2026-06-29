<script lang="ts">
	import { type Snippet } from 'svelte';
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	type Props = {
		open: boolean;
		label: string;
		placeholder?: string;
		buttonText?: string;
		onSubmit: (value: string) => void;
		onOpenChange: (open: boolean) => void;
		validateFn?: (value: string) => string | null;
		children: Snippet;
	};

	let {
		open = $bindable(false),
		label,
		placeholder = 'https://example.com',
		buttonText = 'Insert', // TODO: consider translations
		onSubmit,
		onOpenChange,
		validateFn,
		children
	}: Props = $props();

	let inputValue = $state('');
	let errorMessage = $state<string | null>(null);
	let inputElement: HTMLInputElement | undefined = $state();

	$effect(() => {
		if (!open) {
			inputValue = '';
			errorMessage = null;
		}
	});

	function handleSubmit() {
		const trimmed = inputValue.trim();

		if (!trimmed) {
			errorMessage = 'URL cannot be empty';
			return;
		}

		if (validateFn) {
			const error = validateFn(trimmed);
			if (error) {
				errorMessage = error;
				return;
			}
		}

		onSubmit(trimmed);
		onOpenChange(false);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			handleSubmit();
		}
	}

	function handleCancel() {
		onOpenChange(false);
	}
</script>

<Popover.Root bind:open {onOpenChange}>
	<Popover.Trigger>
		{@render children()}
	</Popover.Trigger>
	<Popover.Content class="w-80" side="bottom" align="start">
		<div class="space-y-4">
			<div class="space-y-2">
				<Label for="url-input">{label}</Label>
				<Input
					id="url-input"
					bind:ref={inputElement}
					bind:value={inputValue}
					type="url"
					{placeholder}
					onkeydown={handleKeydown}
					aria-invalid={!!errorMessage}
					aria-describedby={errorMessage ? 'error-message' : undefined}
				/>
				{#if errorMessage}
					<p id="error-message" class="text-destructive text-sm" role="alert">
						{errorMessage}
					</p>
				{/if}
			</div>
			<div class="flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={handleCancel}>Cancel</Button>
				<Button size="sm" onclick={handleSubmit}>{buttonText}</Button>
			</div>
		</div>
	</Popover.Content>
</Popover.Root>
