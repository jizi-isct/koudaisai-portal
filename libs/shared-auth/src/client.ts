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
 * @param baseUrl APIのbase url
 */
export function getAuthFetchClient(baseUrl: string): AuthFetchClient {
  return createFetchClient<authPaths>({ baseUrl });
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
