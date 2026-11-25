import type { ApiClient } from '@crownshy/api-client/api';

export enum Tool {
	Polis
}

export type Values<T extends Record<string, unknown>> = T[keyof T];

type _ApiResponse<T extends keyof ApiClient> = ReturnType<ApiClient[T]>;
export type ApiResponse<T extends keyof ApiClient> = Awaited<_ApiResponse<T>>;
