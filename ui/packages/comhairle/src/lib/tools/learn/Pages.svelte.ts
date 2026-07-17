import type { LocalizedPage } from '@crownshy/api-client/api';

type Id = string;
type Language = string;
type IPages = Record<Id, Record<Language, ExtendedLocalizedPage>>;

interface ExtendedLocalizedPage extends LocalizedPage {
	lang: string;
	requires_validation: boolean;
}

type From = 'source' | 'target';
type Callback = (options: { invalidate?: boolean }) => Promise<void>;
type Order = { id: string }[]; // Matching DraggableList "items" prop

class Pages {
	items = $state<IPages>({});
	currentId = $state<number>(0);
	#callback: Callback = () => Promise.resolve();
	order = $state<Order>([]);

	get count() {
		return Object.keys(this.items).length;
	}

	onChange(fn: Callback) {
		this.#callback = fn;
	}

	new(primaryLocale: Language) {
		const keys = Object.keys(this.items);
		const latestId = Number(keys[keys.length - 1]);
		const newId = (latestId + 1).toString();
		const newPage: ExtendedLocalizedPage = {
			lang: primaryLocale,
			content: '# New Page',
			type: 'markdown',
			requires_validation: false
		};
		this.items[newId] = { [primaryLocale]: newPage };
		this.order.push({ id: newId });
		return this.#callback({});
	}

	load(source: ExtendedLocalizedPage[][]) {
		const order: Order = [];
		source.forEach((page, i) => {
			const extendedLocalizedPage: Record<Language, ExtendedLocalizedPage> = {};
			page.forEach((p) => {
				extendedLocalizedPage[p.lang] = p;
			});
			this.items[i] = extendedLocalizedPage;
			order.push({ id: i.toString() });
		});
		this.order = order;
	}

	toLocalizedPages(): ExtendedLocalizedPage[][] {
		return this.order.map((p) => Object.values(this.items[p.id]));
	}

	get current() {
		return {
			delete: () => {
				delete this.items[this.currentId];
				const index = this.order.findIndex((p) => Number(p.id) === this.currentId);
				this.order.splice(index, 1);
				this.currentId = Number(Math.max(index - 1, 0));
				return this.#callback({});
			},

			upsertContent: (
				from: From,
				lang: string,
				content: ExtendedLocalizedPage['content'] | undefined
			) => {
				const requires_validation = from === 'target';
				const page = this.items[this.currentId];
				if (!page) return;
				page[lang] = {
					lang,
					type: 'markdown',
					content: content ?? page[lang]?.content ?? '',
					requires_validation
				};
				switch (from) {
					case 'source':
						for (const translation in page) {
							if (page[translation].lang !== lang) {
								page[translation].requires_validation = true;
							}
						}
						break;
					case 'target':
						break;
				}
				this.items[this.currentId] = page;
				return this.#callback({ invalidate: false });
			},

			modifyValidation: async (lang: string, validation: boolean) => {
				const page = this.items[this.currentId];
				if (!page || !page[lang]) return;
				page[lang].requires_validation = validation;
				this.items[this.currentId] = page;
				return this.#callback({ invalidate: false });
			}
		};
	}
}

export default Pages;
