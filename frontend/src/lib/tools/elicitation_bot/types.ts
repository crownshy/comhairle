export interface ExtractedClaim {
	id: string;
	content: string;
	status: 'pending' | 'approved' | 'editing';
}

export interface ElicitationMessage {
	id: string;
	content: string;
	isBot: boolean;
	timestamp: Date | null;
}

export interface ElicitationBotProps {
	chatId?: string;
	conversationId?: string;
	userId?: string;
	title?: string;
	subtitle?: string;
	botName?: string;
	botSubtitle?: string;
	messages?: ElicitationMessage[];
	claims?: ExtractedClaim[];
	placeholder?: string;
	onSendMessage?: (message: string) => void;
	onClaimApprove?: (claimId: string) => void;
	onClaimEdit?: (claimId: string, newContent: string) => void;
	onClaimRemove?: (claimId: string) => void;
	onAddClaim?: () => void;
}
