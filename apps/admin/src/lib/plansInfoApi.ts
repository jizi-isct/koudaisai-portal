"use client";

import {getPlansInfoFetchClient, getPlansInfoQueryClient} from "@koudaisai/shared-api";
import {getAuthMiddleware} from "@koudaisai/shared-auth-admin";
import {authFetchClient} from "./api";

const plansInfoBaseUrl = process.env.NEXT_PUBLIC_PLANS_INFO_API_BASE_URL;
if (!plansInfoBaseUrl) throw new Error("NEXT_PUBLIC_PLANS_INFO_API_BASE_URL が設定されていません");

const adminMiddleware = getAuthMiddleware(authFetchClient, "/login");

export const fetchClientPlansInfoAdmin = getPlansInfoFetchClient(plansInfoBaseUrl, adminMiddleware);
export const $plansInfoApiAdmin = getPlansInfoQueryClient(fetchClientPlansInfoAdmin);

export const fetchClientPlansInfoNoAuth = getPlansInfoFetchClient(plansInfoBaseUrl);
export const $plansInfoApiNoLogin = getPlansInfoQueryClient(fetchClientPlansInfoNoAuth);
