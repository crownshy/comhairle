export const HEYFORM_CHOICE_FIELD_KIND = [
	'yes_no',
	'multiple_choice',
	'picture_choice',
	'file_upload',
	'rating'
] as const;

export const HEYFORM_NON_CHOICE_FIELD_KIND = [
	'number',
	'short_text',
	'long_text',
	'opinion_scale',
	'full_name',
	'address',
	'email',
	'url',
	'phone_number',
	'country_selector'
] as const;

export const HEYFORM_OTHER_FIELD_KIND = [
	'group',
	'welcome',
	'thank_you',
	'statement',
	'date',
	'date_range',
	'time',
	'input_table',
	'payment',
	'signature',
	'legal_terms',
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
