import createClient, {OpenapiQueryClient} from "openapi-react-query";
import {type apiPaths} from "@koudaisai/shared-types";
import createFetchClient, {Client, type Middleware} from "openapi-fetch";

export type ApiFetchClient = Client<apiPaths, `${string}/${string}`>
export type ApiQueryClient = OpenapiQueryClient<apiPaths, `${string}/${string}`>

/**
 * fetch clientを生成する関数。必要に応じてミドルウェアを適用できます。
 *
 * @param baseUrl APIのbase url
 * @param middleware ミドルウェア。認証などに使用できます。
 */
export function getApiFetchClient(baseUrl: string, middleware?: Middleware): ApiFetchClient {
  const fetchClient = createFetchClient<apiPaths>({baseUrl})
  if (middleware) {
    fetchClient.use(middleware)
  }
  return fetchClient
}

/**
 * fetch clientからReact QueryのClientを生成します。
 * @param fetchClient fetch client
 */
export function getApiQueryClient(fetchClient: ApiFetchClient): ApiQueryClient {
  return createClient(fetchClient)
}