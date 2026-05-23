import createClient, { OpenapiQueryClient } from 'openapi-react-query';
import { type plansInfoPaths } from '@koudaisai/shared-types';
import createFetchClient, { Client, type Middleware } from 'openapi-fetch';

export type PlansInfoFetchClient = Client<
  plansInfoPaths,
  `${string}/${string}`
>;
export type PlansInfoQueryClient = OpenapiQueryClient<
  plansInfoPaths,
  `${string}/${string}`
>;

/**
 * fetch clientを生成する関数。必要に応じてミドルウェアを適用できます。
 *
 * @param baseUrl APIのbase url
 * @param middleware ミドルウェア。認証などに使用できます。
 */
export function getPlansInfoFetchClient(
  baseUrl: string,
  middleware?: Middleware,
): PlansInfoFetchClient {
  const fetchClient = createFetchClient<plansInfoPaths>({ baseUrl });
  if (middleware) {
    fetchClient.use(middleware);
  }
  return fetchClient;
}

/**
 * fetch clientからReact QueryのClientを生成します。
 * @param fetchClient fetch client
 */
export function getPlansInfoQueryClient(
  fetchClient: PlansInfoFetchClient,
): PlansInfoQueryClient {
  return createClient(fetchClient);
}
