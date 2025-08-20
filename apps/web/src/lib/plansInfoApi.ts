import createFetchClient from "openapi-fetch";
import {paths} from "@/lib/plans_info_api_v1";
import createClient from "openapi-react-query";
import {authMiddlewareAdmin} from "@/lib/api";

export const fetchClientPlansInfoApi = createFetchClient<paths>({baseUrl: process.env.NEXT_PUBLIC_PLANS_INFO_API_BASE_URL})
fetchClientPlansInfoApi.use(authMiddlewareAdmin)

export const $plansInfoApi = createClient(fetchClientPlansInfoApi)

export type plansInfoApiClientType = typeof $plansInfoApi

export type plansInfoApiFetchClientType = typeof fetchClientPlansInfoApi
