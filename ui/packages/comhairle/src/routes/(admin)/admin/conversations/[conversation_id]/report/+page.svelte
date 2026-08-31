<script lang="ts">
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Select from '$lib/components/ui/select';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte.js';
	import * as m from '$lib/paraglide/messages';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { invalidate } from '$app/navigation';
	import { Switch } from '$lib/components/ui/switch/index.js';
	import Edit from 'lucide-svelte/icons/edit';
	import Delete from 'lucide-svelte/icons/delete';
	import { Separator } from '$lib/components/ui/separator';
	import { report_url } from '$lib/urls.js';
	import 'carta-md/default.css';
	import '@cartamd/plugin-slash/default.css';
	import 'carta-plugin-video/default.css';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte.js';
	import type { Locale } from '$lib/paraglide/runtime.js';
	import { key } from '$lib/utils/invalidationKey.js';
	import TabContent from '../TabContent.svelte';

	let { data } = $props();
	let report = $derived(data.report);
	let conversation = $derived(data.conversation);

	let newImpact = $state({
		title: '',
		details: '',
		kind: 'policy'
	});

	let newFeedback = $state({
		title: '',
		details: ''
	});

	let impactOpen = $state(false);
	let feedbackOpen = $state(false);

	const summaryTranslationSource = createTextContentSource({
		getTranslation: () => report.translations.summary,
		getPrimaryLocale: () => conversation.primaryLocale as Locale,
		getSupportedLanguages: () => conversation.supportedLanguages as Locale[]
	});

	async function createFeedback() {}

	async function createImpact() {
		try {
			await apiClient.CreateImpact(newImpact, {
				params: { report_id: report.id, conversation_id: report.conversationId }
			});
			invalidate(key('conversation/report'));
			impactOpen = false;
			notifications.send({ message: 'Impact Saved', priority: 'INFO' });
		} catch (e) {
			notifications.send({ message: 'Failed to save impact', priority: 'ERROR' });
		}
	}
</script>

<svelte:head>
	<title>Conversation Report - Comhairle Admin</title>
</svelte:head>

<TabContent>
	<PageHeader
		title="Report"
		description="Use this space to edit the report for this conversation"
	/>

	<div class="flex flex-col gap-4">
		<div class="flex w-full flex-row items-center justify-end gap-2">
			<Button variant="ghost" href={report_url(conversation.id, '')}>View Report</Button>
			<Label for="published">Publish Report</Label>
			<Switch name="published" value={report.isPublic} />
		</div>

		<Card.Root>
			<Card.Header>
				<Card.Title>Summary</Card.Title>
				<Card.Description>Overall summary of the conversation</Card.Description>
			</Card.Header>
			<Card.Content>
				<TranslatableField
					source={summaryTranslationSource}
					primaryLocale={conversation.primaryLocale as Locale}
					supportedLanguages={conversation.supportedLanguages as Locale[]}
					inputType="textarea"
					placeholder="Summary to be filled out by the facilitator"
					editorType="rich"
					minHeight="100px"
					reportEmbedSteps={data.reportEmbedSteps}
				/>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header>
				<Card.Title>Impacts</Card.Title>
				<Card.Description>What impacts has this conversation had?</Card.Description>
			</Card.Header>
			<Card.Content>
				{#each report.impacts as impact (impact.id)}
					<div class="flex w-full flex-row items-center justify-between">
						<p class="">{impact.title}</p>
						<div class="flex flex-row">
							<Tooltip.Provider>
								<Tooltip.Root>
									<Tooltip.Trigger>
										<Button aria-label="Edit" variant="ghost"><Edit /></Button>
									</Tooltip.Trigger>
									<Tooltip.Content>
										<p>Edit Impact</p>
									</Tooltip.Content>
								</Tooltip.Root>
							</Tooltip.Provider>

							<Tooltip.Provider>
								<Tooltip.Root>
									<Tooltip.Trigger>
										<Button aria-label="Delete" variant="ghost"
											><Delete /></Button
										>
									</Tooltip.Trigger>
									<Tooltip.Content>
										<p>Delete Impact</p>
									</Tooltip.Content>
								</Tooltip.Root></Tooltip.Provider
							>
						</div>
					</div>
					<Separator class="my-4" />
				{/each}
				<Card.Footer class="flex w-full justify-end">
					<Dialog.Root bind:open={impactOpen}>
						<Dialog.Trigger class={buttonVariants({ variant: 'default' })}
							>Add Impact</Dialog.Trigger
						>

						<Dialog.Content class="sm:max-w-106">
							<Dialog.Header>
								<Dialog.Title>Add an impact</Dialog.Title>
								<Dialog.Description
									>Record an impact that this report has had</Dialog.Description
								>
							</Dialog.Header>
							<div class="grid gap-4 py-4">
								<div class="flex flex-col gap-4">
									<Label for="title">Title</Label>
									<Input bind:value={newImpact.title} id="title" />
									<Label for="title">Details</Label>
									<Textarea
										id="details"
										placeholder="Describe in detail the impact."
										bind:value={newImpact.details}
										class="col-span-3"
									/>
									<Label for="title">Impact Type</Label>
									<Select.Root
										required
										onSelectedChange={(v) => {
											if (v?.value) {
												newImpact.kind = v.value;
											}
										}}
									>
										<Select.Trigger>
											<Select.Value placeholder="Select an impact type" />
										</Select.Trigger>
										<Select.Content class="w-56">
											<Select.Item value="policy" label="Policy" />
											<Select.Item value="debate" label="Debate" />
											<Select.Item
												value="followup_conversation"
												label="Followup Conversation"
											/>
										</Select.Content>
									</Select.Root>
								</div>
							</div>
							<Dialog.Footer>
								<Button onclick={createImpact} type="submit">{m.submit()}</Button>
							</Dialog.Footer>
						</Dialog.Content>
					</Dialog.Root>
				</Card.Footer>
			</Card.Content>
		</Card.Root>

		<Card.Root>
			<Card.Header>
				<Card.Title>Facilitator Notes</Card.Title>
				<Card.Description>Notes gathered by facilitators</Card.Description>
			</Card.Header>
			<Card.Content>
				{#each report.facilitatorFeedback as impact (impact.id)}
					<div class="flex w-full flex-row items-center justify-between">
						<p class="">{impact.title}</p>
						<div class="flex flex-row">
							<Tooltip.Provider>
								<Tooltip.Root>
									<Tooltip.Trigger>
										<Button aria-label="Edit" variant="ghost"><Edit /></Button>
									</Tooltip.Trigger>
									<Tooltip.Content>
										<p>Edit Feedback</p>
									</Tooltip.Content>
								</Tooltip.Root></Tooltip.Provider
							>

							<Tooltip.Provider>
								<Tooltip.Root>
									<Tooltip.Trigger>
										<Button aria-label="Delete" variant="ghost"
											><Delete /></Button
										>
									</Tooltip.Trigger>
									<Tooltip.Content>
										<p>Delete Feedback</p>
									</Tooltip.Content>
								</Tooltip.Root></Tooltip.Provider
							>
						</div>
					</div>
					<Separator class="my-4" />
				{/each}
				<Card.Footer class="flex w-full justify-end">
					<Dialog.Root bind:open={feedbackOpen}>
						<Dialog.Trigger class={buttonVariants({ variant: 'default' })}
							>Add Feedback</Dialog.Trigger
						>

						<Dialog.Content class="sm:max-w-[425px]">
							<Dialog.Header>
								<Dialog.Title>Add feedback</Dialog.Title>
								<Dialog.Description
									>Record some feedback or notes from facilitators</Dialog.Description
								>
							</Dialog.Header>
							<div class="grid gap-4 py-4">
								<div class="flex flex-col gap-4">
									<Label for="title">Title</Label>
									<Input bind:value={newFeedback.title} id="title" />
									<Label for="title">Details</Label>
									<Textarea
										id="details"
										placeholder="Describe in detail the impact."
										bind:value={newFeedback.details}
										class="col-span-3"
									/>
								</div>
							</div>
							<Dialog.Footer>
								<Button onclick={createFeedback} type="submit">{m.submit()}</Button>
							</Dialog.Footer>
						</Dialog.Content>
					</Dialog.Root>
				</Card.Footer>
			</Card.Content>
		</Card.Root>
	</div>
</TabContent>
