import {getAuthMiddleware} from "@koudaisai/shared-auth-members";
import {getApiFetchClient, getApiQueryClient} from "@koudaisai/shared-api";
import {API_URL} from "astro:env/client";
import {authFetchClient} from "./authClient";

const authMiddleware = getAuthMiddleware(authFetchClient);
export const api = getApiFetchClient(API_URL, authMiddleware);
export const $api = getApiQueryClient(api);
