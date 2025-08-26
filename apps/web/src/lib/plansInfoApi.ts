import createFetchClient from "openapi-fetch";
import {paths} from "@/lib/plans_info_api_v1";
import createClient from "openapi-react-query";
import {authMiddlewareAdmin, authMiddlewareMembers} from "@/lib/api";

export const fetchClientPlansInfoApiAdmin = createFetchClient<paths>({baseUrl: process.env.NEXT_PUBLIC_PLANS_INFO_API_BASE_URL})
fetchClientPlansInfoApiAdmin.use(authMiddlewareAdmin)

export const $plansInfoApiAdmin = createClient(fetchClientPlansInfoApiAdmin)

export const fetchClientPlansInfoApiMembers = createFetchClient<paths>({baseUrl: process.env.NEXT_PUBLIC_PLANS_INFO_API_BASE_URL})
fetchClientPlansInfoApiMembers.use(authMiddlewareMembers)

export const $plansInfoApiMembers = createClient(fetchClientPlansInfoApiMembers)

export const fetchClientPlansInfoApiNoLogin = createFetchClient<paths>({baseUrl: process.env.NEXT_PUBLIC_PLANS_INFO_API_BASE_URL})

export const $plansInfoApiNoLogin = createClient(fetchClientPlansInfoApiNoLogin)

export type plansInfoApiClientType = typeof $plansInfoApiAdmin

export type plansInfoApiFetchClientType = typeof fetchClientPlansInfoApiAdmin
