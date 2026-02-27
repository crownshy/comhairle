<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ConversationCard from '$lib/components/ConversationCard.svelte';

	const { Story } = defineMeta({
		title: 'Patterns/ConversationCard',
		component: ConversationCard,
		tags: ['autodocs']
	});

	const baseMock = {
		chatBotId: null,
		description: 'desc-uuid',
		enableQaChatBot: false,
		isComplete: false,
		isInviteOnly: false,
		isPublic: true,
		knowledgeBaseId: null,
		primaryLocale: 'en',
		slug: null,
		supportedLanguages: ['en'],
		tags: [],
		videoUrl: null
	};

	const mockLiveConversation = {
		...baseMock,
		id: '00000000-0000-0000-0000-000000000001',
		title: 'Community Budget Priorities',
		shortDescription: 'Help us decide how to allocate next year\'s community development budget. Share your priorities and ideas for local improvements.',
		isLive: true,
		imageUrl: 'https://images.unsplash.com/photo-1559136555-9303baea8ebd?w=800&auto=format&fit=crop&q=60'
	};

	const mockDraftConversation = {
		...baseMock,
		id: '00000000-0000-0000-0000-000000000002',
		title: 'Transport Infrastructure Consultation',
		shortDescription: 'We\'re planning improvements to local transport links and want to hear your views on proposed changes to bus routes and cycle paths.',
		isLive: false,
		imageUrl: 'https://images.unsplash.com/photo-1449824913935-59a10b8d2000?w=800&auto=format&fit=crop&q=60'
	};

	const mockConversationNoImage = {
		...baseMock,
		id: '00000000-0000-0000-0000-000000000003',
		title: 'Parks and Green Spaces',
		shortDescription: 'Share your ideas for improving our local parks and green spaces.',
		isLive: true,
		imageUrl: ''
	};

	const mockPublicConversation = {
		...baseMock,
		id: '00000000-0000-0000-0000-000000000004',
		title: 'How is Scotland doing on the things that matter most?',
		shortDescription: 'Your responses will help shape the Scottish Government\'s National Performance Framework',
		isLive: true,
		imageUrl: 'https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=800&auto=format&fit=crop&q=60'
	};
</script>

<!-- Admin variant stories -->
<Story name="Admin / Live">
	<div class="p-8 bg-gray-50">
		<ConversationCard conversation={mockLiveConversation} />
	</div>
</Story>

<Story name="Admin / Draft">
	<div class="p-8 bg-gray-50">
		<ConversationCard conversation={mockDraftConversation} />
	</div>
</Story>

<Story name="Admin / No Image">
	<div class="p-8 bg-gray-50">
		<ConversationCard conversation={mockConversationNoImage} />
	</div>
</Story>

<Story name="Admin / All States">
	<div class="flex flex-col gap-16 p-8 bg-gray-50">
		<ConversationCard conversation={mockLiveConversation} />
		<ConversationCard conversation={mockDraftConversation} />
		<ConversationCard conversation={mockConversationNoImage} />
	</div>
</Story>

<!-- Public variant stories -->
<Story name="Public / Live">
	<div class="p-8 bg-gray-50">
		<ConversationCard conversation={mockPublicConversation} variant="public" />
	</div>
</Story>

<Story name="Public / With Organization">
	<div class="p-8 bg-gray-50">
		<ConversationCard conversation={mockPublicConversation} variant="public" organizationName="Scottish Government" />
	</div>
</Story>

<Story name="Public / No Image">
	<div class="p-8 bg-gray-50">
		<ConversationCard conversation={mockConversationNoImage} variant="public" organizationName="City Council" />
	</div>
</Story>

<Story name="Public / All States">
	<div class="flex flex-col gap-20 p-8 bg-gray-50">
		<ConversationCard conversation={mockPublicConversation} variant="public" organizationName="Scottish Government" />
		<ConversationCard conversation={mockLiveConversation} variant="public" />
		<ConversationCard conversation={mockConversationNoImage} variant="public" organizationName="City Council" />
	</div>
</Story>
