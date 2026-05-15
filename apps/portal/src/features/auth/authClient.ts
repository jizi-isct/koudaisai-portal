import {getAuthFetchClient} from "@koudaisai/shared-auth";
import {AUTH_URL} from "astro:env/client";

export const authFetchClient = getAuthFetchClient(AUTH_URL);
