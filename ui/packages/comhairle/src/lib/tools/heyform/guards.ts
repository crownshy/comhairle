import {
	HEYFORM_CHOICE_FIELD_KIND,
	HEYFORM_FIELD_KIND,
	HEYFORM_NON_CHOICE_FIELD_KIND,
	HEYFORM_OTHER_FIELD_KIND,
	type HeyFormChoiceFieldKind,
	type HeyFormEmptyValue,
	type HeyFormFieldKind,
	type HeyFormNonChoiceFieldKind,
	type HeyFormOtherFieldKind
} from './utils';

export const isHeyFormFieldKind = (str: string | HeyFormFieldKind): str is HeyFormFieldKind =>
	HEYFORM_FIELD_KIND.includes(str as HeyFormFieldKind);

export const isHeyFormChoiceFieldKind = (
	str: string | HeyFormChoiceFieldKind
): str is HeyFormChoiceFieldKind =>
	HEYFORM_CHOICE_FIELD_KIND.includes(str as HeyFormChoiceFieldKind);

export const isHeyFormNonChoiceFieldKind = (
	str: string | HeyFormNonChoiceFieldKind
): str is HeyFormNonChoiceFieldKind =>
	HEYFORM_NON_CHOICE_FIELD_KIND.includes(str as HeyFormNonChoiceFieldKind);

export const isHeyFormOtherFieldKind = (
	str: string | HeyFormOtherFieldKind
): str is HeyFormOtherFieldKind => HEYFORM_OTHER_FIELD_KIND.includes(str as HeyFormOtherFieldKind);

export const isHeyFormEmptyValue = (value: unknown): value is HeyFormEmptyValue => {
	if (typeof value === 'string') {
		return value === '';
	}

	if (
		typeof value === 'object' &&
		value !== null &&
		'value' in value &&
		Array.isArray(value.value)
	) {
		return value?.value?.length === 0;
	}

	return false;
};
