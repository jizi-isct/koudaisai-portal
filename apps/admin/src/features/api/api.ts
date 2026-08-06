import { getAuthMiddleware } from '@koudaisai/shared-auth-admin';
import {
  getApiFetchClient,
  getApiQueryClient,
  getEvents26FetchClient,
  getEvents26QueryClient,
} from '@koudaisai/shared-api';
import { API_URL, AUTH_URL, EVENTS26_API_URL } from 'astro:env/client';
import { getAuthFetchClient } from '@koudaisai/shared-auth';

export const authFetchClient = getAuthFetchClient(AUTH_URL);
const authMiddleware = getAuthMiddleware('/login');
export const api = getApiFetchClient(API_URL, authMiddleware);
export const $api = getApiQueryClient(api);

/**
 * 企画情報API(events26)の読み取り用クライアント。
 *
 * events26 の公開エンドポイント(`/v1/...`)を直接叩くため認証は付けない。
 * 企画の作成・置換・削除は backend 中継の `$api` の `/events26/projects` を使うこと。
 */
export const events26Api = getEvents26FetchClient(EVENTS26_API_URL);
export const $events26Api = getEvents26QueryClient(events26Api);
