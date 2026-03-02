import { createApiClient as createApi } from './api';
export declare const createApiClient: (baseUrl: string, authToken: string | undefined, source: string, locale?: string) => ReturnType<typeof createApi>;
export declare const apiClient: ReturnType<typeof createApiClient>;
