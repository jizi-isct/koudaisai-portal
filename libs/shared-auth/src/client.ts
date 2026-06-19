import createClient, { OpenapiQueryClient } from 'openapi-react-query';
import { type authPaths } from '@koudaisai/shared-types';
import createFetchClient, { Client } from 'openapi-fetch';

export type AuthFetchClient = Client<authPaths, `${string}/${string}`>;
export type AuthQueryClient = OpenapiQueryClient<
  authPaths,
  `${string}/${string}`
>;

/**
 * fetch clientを生成する関数。
 *
 * リフレッシュトークンは HttpOnly Cookie(`__Host-refresh_token`)で配送されるため，
 * login / refresh / logout で Cookie を送受信できるよう `credentials: 'include'` を付ける。
 * CORS は credentials 付き(allow_credentials + origin 許可リスト)で構成されている。
 *
 * @param baseUrl APIのbase url
 */
export function getAuthFetchClient(baseUrl: string): AuthFetchClient {
  return createFetchClient<authPaths>({ baseUrl, credentials: 'include' });
}

/**
 * fetch clientからReact QueryのClientを生成します。
 * @param fetchClient fetch client
 */
export function getAuthQueryClient(
  fetchClient: AuthFetchClient,
): AuthQueryClient {
  return createClient(fetchClient);
}
