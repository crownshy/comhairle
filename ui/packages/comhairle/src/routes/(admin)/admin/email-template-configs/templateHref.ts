import { resolve } from '$app/paths';
import type { EmailType } from '@crownshy/api-client/api';
import type { EmailTemplateRow } from '$lib/utils/emailTemplateGroups';

/** Edit a saved version, or start customising an email that has none. */
export function templateHref(row: EmailTemplateRow, emailType: EmailType): string {
	if (row.configId) {
		return resolve('/(admin)/admin/email-template-configs/[email_config_id]', {
			email_config_id: row.configId
		});
	}
	return `${resolve('/(admin)/admin/email-template-configs/new')}?type=${emailType}`;
}
