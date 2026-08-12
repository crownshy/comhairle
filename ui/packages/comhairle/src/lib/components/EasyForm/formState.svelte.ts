import { writable } from 'svelte/store';

export const isSubmitting = writable<boolean>(false);
