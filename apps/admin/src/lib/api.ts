"use client";

import {getAuthFetchClient, getAuthQueryClient} from "@koudaisai/shared-auth";
import {getApiFetchClient, getApiQueryClient} from "@koudaisai/shared-api";
import {getAuthMiddleware} from "@koudaisai/shared-auth-admin";

const authBaseUrl = process.env.NEXT_PUBLIC_AUTH_BASE_URL;
if (!authBaseUrl) throw new Error("NEXT_PUBLIC_AUTH_BASE_URL が設定されていません");

const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL;
if (!apiBaseUrl) throw new Error("NEXT_PUBLIC_API_BASE_URL が設定されていません");

export const authFetchClient = getAuthFetchClient(authBaseUrl);
export const $auth = getAuthQueryClient(authFetchClient);

export const adminMiddleware = getAuthMiddleware(authFetchClient, `${authBaseUrl}/admin/login`);

export const fetchClientAdmin = getApiFetchClient(apiBaseUrl, adminMiddleware);
export const $apiAdmin = getApiQueryClient(fetchClientAdmin);

export const fetchClientNoAuth = getApiFetchClient(apiBaseUrl);
export const $apiNoAuth = getApiQueryClient(fetchClientNoAuth);
