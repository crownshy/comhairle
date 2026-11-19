import { test } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import Events from './utils/navigation/Events';
import { Button, Textbox } from './utils/components';
import { testWithRefresh } from './utils';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Events', undefined);
	await Events.gotoPlaywrightEvent(page);
	await Events.subtab(page, 'Location');
});

test('Events/Location page', async ({ page, cleanup }) => {
	const Venue = await Textbox('Venue', { page, cleanup });
	const AddressLine1 = await Textbox('Address line 1', { page, cleanup });
	const AddressLine2 = await Textbox('Address line 2', { page, cleanup });
	const AddressLine3 = await Textbox('Address line 3', { page, cleanup });
	const City = await Textbox('City', { page, cleanup });
	const StateProvince = await Textbox('State / province', { page, cleanup });
	const PostalCode = await Textbox('Postal code', { page, cleanup });
	const CountryCode = await Textbox('Country code', { page, cleanup });
	const SaveChanges = await Button('Save Changes', page);

	cleanup(async () => {
		await SaveChanges.click();
	});

	await Venue.write();
	await AddressLine1.write();
	await AddressLine2.write();
	await AddressLine3.write();
	await City.write();
	await StateProvince.write();
	await PostalCode.write();
	await CountryCode.write();

	await SaveChanges.click();

	await testWithRefresh(page, async () => {
		await Venue.expect();
		await AddressLine1.expect();
		await AddressLine2.expect();
		await AddressLine3.expect();
		await City.expect();
		await StateProvince.expect();
		await PostalCode.expect();
		await CountryCode.expect();
	});
});
