<script lang="ts">
	import * as Popover from '$lib/components/ui/popover';
	import { Info } from 'lucide-svelte';

	type Props = { templateVariables: string[] };
	let { templateVariables }: Props = $props();

	let open = $state(false);
</script>

{#snippet quote(message: string)}
	<blockquote class="w-fit border-l-12 border-gray-600 bg-gray-300 p-4 text-gray-900 italic">
		{message}
	</blockquote>
{/snippet}

<div class="flex flex-col gap-8">
	<Popover.Root bind:open>
		<Popover.Trigger class="flex justify-start">
			<span class="flex items-center gap-2 font-bold">
				Available variables <Info class="text-primary inline size-5 self-center" />
			</span>
		</Popover.Trigger>
		<Popover.Content class="max-w-[70vw] overflow-y-auto" side="bottom" align="start">
			<div class="flex flex-col gap-2">
				<h2 class="text-lg font-bold">Personalise your email content</h2>
				<p>
					The fields below support dynamic <span class="font-semibold italic"
						>variables</span
					>, which are automatically replaced with real values when the email is sent. To
					use a variable, include it in your content wrapped in double curly braces.
				</p>
			</div>

			<div class="flex flex-col gap-2">
				<p>For example, if <code>conversation_title</code> is available, writing:</p>
				{@render quote('You are invited to take part in {{conversation_title}}.')}
				<p>will be sent to recipients as:</p>
				{@render quote(
					'You are invited to take part in Town Centre Regeneration Consultation.'
				)}
			</div>
		</Popover.Content>
	</Popover.Root>

	<div class="flex flex-col gap-2">
		<p>The following variables can be used in any content field for this email:</p>
		<ul class="list-inside list-disc">
			{#each templateVariables as templateVar (templateVar)}
				<li><code>{templateVar}</code></li>
			{/each}
		</ul>
	</div>
</div>
