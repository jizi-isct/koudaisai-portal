import {getAuthMiddleware} from "@koudaisai/shared-auth-admin";
import {
  getApiFetchClient,
  getApiQueryClient,
  getPlansInfoFetchClient,
  getPlansInfoQueryClient
} from "@koudaisai/shared-api";
import {API_URL, AUTH_URL, PLANS_INFO_API_URL} from "astro:env/client";
import {getAuthFetchClient} from "@koudaisai/shared-auth";

export const authFetchClient = getAuthFetchClient(AUTH_URL);
const authMiddleware = getAuthMiddleware(authFetchClient, "/login");
export const api = getApiFetchClient(API_URL, authMiddleware);
export const $api = getApiQueryClient(api);

export const plansInfoApi = getPlansInfoFetchClient(PLANS_INFO_API_URL, authMiddleware)
export const $plansInfoApi = getPlansInfoQueryClient(plansInfoApi);

export const plansInfoApiNoAuth = getPlansInfoFetchClient(PLANS_INFO_API_URL);
export const $plansInfoApiNoAuth = getPlansInfoQueryClient(plansInfoApiNoAuth);
