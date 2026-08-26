import createClient, { type OpenapiQueryClient } from 'openapi-react-query';
import { type events26Paths } from '@koudaisai/shared-types';
import createFetchClient, { type Client, type Middleware } from 'openapi-fetch';

export type Events26FetchClient = Client<events26Paths, `${string}/${string}`>;
export type Events26QueryClient = OpenapiQueryClient<
  events26Paths,
  `${string}/${string}`
>;

/**
 * 企画情報API(events26)の fetch clientを生成する関数。
 *
 * 読み取り(`/v1/...`)は認証不要で events26 が直接公開しているため、こちらを使います。
 * 書き込み(`/admin/v1/...`)は events26 を直接叩かず、backend の
 * `/api/v3/events26` 経由(`ApiFetchClient`)で行ってください。
 *
 * @param baseUrl APIのbase url
 * @param middleware ミドルウェア。認証などに使用できます。
 */
export function getEvents26FetchClient(
  baseUrl: string,
  middleware?: Middleware,
): Events26FetchClient {
  const fetchClient = createFetchClient<events26Paths>({ baseUrl });
  if (middleware) {
    fetchClient.use(middleware);
  }
  return fetchClient;
}

/**
 * fetch clientからReact QueryのClientを生成します。
 * @param fetchClient fetch client
 */
export function getEvents26QueryClient(
  fetchClient: Events26FetchClient,
): Events26QueryClient {
  return createClient(fetchClient);
}
