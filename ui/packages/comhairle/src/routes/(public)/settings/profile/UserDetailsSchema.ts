import type { InputAttr } from '$lib/components/EasyForm';
import { typed } from '$lib/utils/types';

const UserDetailsSchema = {
	username: typed<InputAttr>({
		name: 'username',
		required: true
	}),
	email: typed<InputAttr>({
		name: 'email',
		required: true,
		// eslint-disable-next-line no-useless-escape
		pattern: '.*@(.*\.)+.+'
	})
};

export default UserDetailsSchema;
