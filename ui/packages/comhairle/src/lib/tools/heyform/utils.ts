import type { HeyFormFieldKind } from '$lib/tools/heyform/types';

export const isHeyFormFieldKind = (str: string | null | undefined): str is HeyFormFieldKind =>
	str === 'group' ||
	str === 'welcome' ||
	str === 'thank_you' ||
	str === 'statement' ||
	str === 'short_text' ||
	str === 'long_text' ||
	str === 'number' ||
	str === 'yes_no' ||
	str === 'multiple_choice' ||
	str === 'picture_choice' ||
	str === 'file_upload' ||
	str === 'opinion_scale' ||
	str === 'rating' ||
	str === 'date' ||
	str === 'date_range' ||
	str === 'time' ||
	str === 'input_table' ||
	str === 'payment' ||
	str === 'full_name' ||
	str === 'address' ||
	str === 'email' ||
	str === 'url' ||
	str === 'phone_number' ||
	str === 'country_selector' ||
	str === 'signature' ||
	str === 'legal_terms' ||
	str === 'submit_date' ||
	str === 'hidden_fields' ||
	str === 'variable' ||
	str === 'hidden_checkbox' ||
	str === 'custom_text' ||
	str === 'custom_single' ||
	str === 'custom_multiple' ||
	str === 'custom_date' ||
	str === 'custom_number' ||
	str === 'custom_checkbox';
