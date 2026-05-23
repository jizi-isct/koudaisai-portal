import {getAuthMiddleware} from "@koudaisai/shared-auth-members";
import {getApiFetchClient, getApiQueryClient} from "@koudaisai/shared-api";
import {API_URL, AUTH_URL} from "astro:env/client";
import {getAuthFetchClient} from "@koudaisai/shared-auth";

export const authFetchClient = getAuthFetchClient(AUTH_URL);
const authMiddleware = getAuthMiddleware(authFetchClient);
export const api = getApiFetchClient(API_URL, authMiddleware);
export const $api = getApiQueryClient(api);
