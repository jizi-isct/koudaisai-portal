import { getAuthMiddleware } from '@koudaisai/shared-auth-members';
import {
  getApiFetchClient,
  getApiQueryClient,
  getEvents26FetchClient,
  getEvents26QueryClient,
} from '@koudaisai/shared-api';
import { API_URL, AUTH_URL, EVENTS26_API_URL } from 'astro:env/client';
import { getAuthFetchClient } from '@koudaisai/shared-auth';

export const authFetchClient = getAuthFetchClient(AUTH_URL);
const authMiddleware = getAuthMiddleware(authFetchClient);
export const api = getApiFetchClient(API_URL, authMiddleware);
export const $api = getApiQueryClient(api);

/** events26 の公開エンドポイントを読むための認証不要クライアント。 */
export const events26Api = getEvents26FetchClient(EVENTS26_API_URL);
export const $events26Api = getEvents26QueryClient(events26Api);
