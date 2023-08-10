import { writable } from 'svelte/store';
import { v4 as uuid } from 'uuid';

type Toast = {
	type?: 'success' | 'error' | 'info';
	message: string;
	timeout?: number;
};

export const toasts = writable<{ [K: string]: Toast }>({});

export const toast = (toast: Toast) => {
	if (typeof toast.type === 'undefined') toast.type = 'info';
	if (typeof toast.timeout === 'undefined') toast.timeout = 5000;

	const id = uuid();
	toasts.update((v) => ({ ...v, [id]: toast }));
	setTimeout(() => {
		deleteToast(id);
	}, toast.timeout);
};

export const deleteToast = (id: string) => {
	toasts.update((v) => {
		delete v[id];
		return v;
	});
};
