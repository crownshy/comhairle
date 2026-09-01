import type { InviteDto } from '@crownshy/api-client/api';

export type PendingInvite = Pick<InviteDto, 'id' | 'status'> & { email: string };
