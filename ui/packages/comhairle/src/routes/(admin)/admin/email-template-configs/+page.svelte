<script lang="ts">
	import { EMAIL_TYPE_COPY } from './emailTypeCopy';
	import EmailTypeTab from './EmailTypeTab.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { buildEmailTypeView } from '$lib/utils/emailTemplateGroups';
	import { emailTemplateConfigsTour } from './tour';
	import { page } from '$app/state';
	import SubTabStrip from '$lib/components/SubTabStrip.svelte';

	useTour(() => emailTemplateConfigsTour);

	let { data } = $props();

	const DEFAULT_TYPE = 'conversation_invite';

	let tabs = $derived(
		data.schemas.map((schema) => ({
			value: schema.email_type,
			label: EMAIL_TYPE_COPY[schema.email_type].name
		}))
	);
	let selectedType = $derived(page.url.searchParams.get('type') ?? DEFAULT_TYPE);
	let schema = $derived(
		data.schemas.find((candidate) => candidate.email_type === selectedType) ?? data.schemas[0]
	);
	let view = $derived(
		buildEmailTypeView(
			schema,
			data.emailConfigs,
			data.conversations.map(({ id, title }) => ({ id, title }))
		)
	);
</script>

<svelte:head>
	<title>Email templates - Comhairle Admin</title>
</svelte:head>

<SectionHeader
	hasTabs
	content={{
		eyebrow: 'Library',
		title: 'Email templates',
		description:
			'The emails Comhairle sends to participants. Change the wording for all of your conversations, or give one conversation its own version.'
	}}
/>
<SubTabStrip tone="primary" paramName="type" items={tabs} defaultValue={DEFAULT_TYPE} />
<TabContent>
	<EmailTypeTab {view} />
</TabContent>
