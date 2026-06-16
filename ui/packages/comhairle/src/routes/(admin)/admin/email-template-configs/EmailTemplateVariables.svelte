<script lang="ts">
	type Props = { templateVariables: string[] };
	let { templateVariables }: Props = $props();
</script>

{#snippet quote(message: string)}
	<blockquote class="w-fit border-l-12 border-gray-600 bg-gray-300 p-4 text-gray-900 italic">
		{message}
	</blockquote>
{/snippet}

<div class="flex max-w-4xl flex-col gap-8">
	<div class="flex flex-col gap-2">
		<h2 class="text-lg font-bold">Personalise your email content</h2>
		<p>
			The fields below support dynamic <span class="font-semibold italic">variables</span>,
			which are automatically replaced with real values when the email is sent. To use a
			variable, include it in your content wrapped in double curly braces.
		</p>
	</div>

	<div class="flex flex-col gap-2">
		<p>For example, if <code>conversation_title</code> is available, writing:</p>
		{@render quote('You are invited to take part in {{conversation_title}}.')}
		<p>will be sent to recipients as:</p>
		{@render quote('You are invited to take part in Town Centre Regeneration Consultation.')}
	</div>

	<div class="flex flex-col gap-2">
		<h3 class="font-bold">Available variables</h3>
		<p>The following variables can be used in any content field for this email:</p>
		<ul class="list-inside list-disc">
			{#each templateVariables as templateVar (templateVar)}
				<li><code>{templateVar}</code></li>
			{/each}
		</ul>
	</div>
</div>
