"use client";

import {getPlansInfoFetchClient, getPlansInfoQueryClient} from "@koudaisai/shared-api";
import {membersMiddleware} from "./api";

const plansInfoBaseUrl = process.env.NEXT_PUBLIC_PLANS_INFO_API_BASE_URL;
if (!plansInfoBaseUrl) throw new Error("NEXT_PUBLIC_PLANS_INFO_API_BASE_URL が設定されていません");

export const fetchClientPlansInfoMembers = getPlansInfoFetchClient(plansInfoBaseUrl, membersMiddleware);
export const $plansInfoApiMembers = getPlansInfoQueryClient(fetchClientPlansInfoMembers);

export const fetchClientPlansInfoNoAuth = getPlansInfoFetchClient(plansInfoBaseUrl);
export const $plansInfoApiNoLogin = getPlansInfoQueryClient(fetchClientPlansInfoNoAuth);