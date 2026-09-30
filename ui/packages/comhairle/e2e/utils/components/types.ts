import { Page } from '../types';

export type Cleanup = (callback: () => Promise<void>) => void;

export type Locators<T> = [key: T, identifier: string][];

export type Refs = {
	page: Page;
	cleanup: Cleanup;
};
