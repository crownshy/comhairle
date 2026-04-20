export type AgendaItem = {
	id: string;
	title: string;
	isCurrent: boolean;
};

export type BreakoutRoom = {
	id: string;
	name: string;
	participants: string[];
};

export type BreakoutCreateMode = 'manual' | 'automatic' | 'hybrid';

export type DiversityCriterion =
	| 'age'
	| 'gender'
	| 'understanding_of_ai'
	| 'education'
	| 'postcode'
	| 'in_call_activities';

export type HybridRule = {
	id: string;
	participants: string[];
};

export type RoomContext =
	| { type: 'plenary' }
	| { type: 'breakout'; roomId: string; roomName: string };

export type ActivePanel = 'agenda' | 'breakoutRooms' | 'debug' | null;
