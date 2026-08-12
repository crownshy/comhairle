export const HEYFORM_CHOICE_FIELD_KIND = [
	'yes_no',
	'multiple_choice',
	'picture_choice',
	'ranking',
	'matrix',
	'legal_terms'
] as const;

export const HEYFORM_NON_CHOICE_FIELD_KIND = [
	'number',
	'short_text',
	'long_text',
	'file_upload',
	'opinion_scale',
	'full_name',
	'rating',
	'address',
	'email',
	'url',
	'phone_number',
	'country_selector',
	'date',
	'date_range'
] as const;

export const HEYFORM_OTHER_FIELD_KIND = [
	'group',
	'welcome',
	'thank_you',
	'statement',
	'time',
	'input_table',
	'payment',
	'signature',
	'submit_date',
	'hidden_fields',
	'variable',
	'hidden_checkbox',
	'custom_text',
	'custom_single',
	'custom_multiple',
	'custom_date',
	'custom_number',
	'custom_checkbox'
] as const;

// Taken from: https://github.com/heyform/heyform/blob/main/packages/shared-types-enums/src/form.ts
export interface Choice {
	id: string;
	label: string;

	// Picture choice
	image?: string;
	icon?: {
		name: string;
		color: string;
		background: string;
	};

	// HeySheet custom columns
	color?: string;

	// Quiz
	score?: number;
	isExpected?: boolean;
}

export interface Column {
	id: string;
	label: string;
	type?: string;
}

export interface NumberPrice {
	type: 'number';
	value: number;
}

export interface VariablePrice {
	type: 'variable';
	ref: string;
}

export interface Properties {
	// Statement
	showButton?: boolean;
	buttonText?: string;
	hideMarks?: boolean;

	// Choice
	allowOther?: boolean;
	allowMultiple?: boolean;
	choices?: Choice[];
	randomize?: boolean;
	other?: string;
	badge?: 'letter' | 'number'; // default is ChoiceBadgeEnum.LETTER
	verticalAlignment?: boolean;

	// Only for group
	fields?: FormField[];

	// Rating
	shape?: string;
	total?: number;
	start?: number;

	// Opinion Scale
	leftLabel?: string;
	centerLabel?: string;
	rightLabel?: string;

	// PhoneNumber
	defaultCountryCode?: string;

	// Payment
	currency?: string;
	price?: NumberPrice | VariablePrice;

	// Date
	format?: string;
	// Allow input time
	allowTime?: boolean;
	// Time
	timeFormat?: string;
	use12Hours?: boolean;

	// Data
	tableColumns?: Column[];

	// Score
	score?: number;

	// HeyForm Form Builder v2.0
	// Embed & Image
	sourceUrl?: string;

	// Screen
	enableShareIcon?: boolean;
	enableCompleteTime?: boolean;

	// Thank You
	buttonLinkUrl?: string;
	redirectUrl?: string;
	redirectOnCompletion?: boolean;
	redirectDelay?: number;
}

export interface Validation {
	required?: boolean;
	min?: number;
	max?: number;
	matchExpected?: boolean;
}

export interface FormField {
	id: string;
	title?: string;
	description?: string;
	kind: HeyFormFieldKind;
	validations?: Validation;
	properties?: Properties;

	// Label for short title
	label?: string;

	// layout?: Layout;

	// HeyForm question number
	number?: number;

	index?: number;

	// HeySheet custom columns or embed blocks
	width?: number;
	frozen?: boolean;
	visible?: boolean;

	hide?: boolean;
}

export type HeyFormFullNameValue = { firstName: string; lastName: string };
export type HeyFormAddressValue = {
	address1: string;
	address2: string;
	county: string;
	postcode: string;
	country: string;
};
export type HeyFormDateRangeValue = {
	end: string;
	start: string;
};
export type HeyFormRankedValue = {
	value: string[];
};
export type HeyFormMatrixValue = Record<string, number>;
export type HeyFormLegalTermsValue = boolean;
export type HeyFormFileUploadValue = {
	filename: string;
	size: number;
	url: string;
};

export type HeyFormChoiceFieldKind = (typeof HEYFORM_CHOICE_FIELD_KIND)[number];
export type HeyFormNonChoiceFieldKind = (typeof HEYFORM_NON_CHOICE_FIELD_KIND)[number];
export type HeyFormOtherFieldKind = (typeof HEYFORM_OTHER_FIELD_KIND)[number];
export type HeyFormFieldKind =
	| HeyFormChoiceFieldKind
	| HeyFormNonChoiceFieldKind
	| HeyFormOtherFieldKind;

export const HEYFORM_FIELD_KIND: HeyFormFieldKind[] = Array<HeyFormFieldKind>().concat(
	HEYFORM_CHOICE_FIELD_KIND,
	HEYFORM_NON_CHOICE_FIELD_KIND,
	HEYFORM_OTHER_FIELD_KIND
);
