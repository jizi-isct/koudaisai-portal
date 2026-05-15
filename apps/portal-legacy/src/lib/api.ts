"use client";

import {getApiFetchClient, getApiQueryClient} from "@koudaisai/shared-api";
import {getAuthFetchClient, getAuthQueryClient} from "@koudaisai/shared-auth";
import {getAuthMiddleware} from "@koudaisai/shared-auth-members";
import {decodeAccessToken} from "@koudaisai/shared-auth";
import {useState, useEffect} from "react";

const authBaseUrl = process.env.NEXT_PUBLIC_AUTH_BASE_URL;
if (!authBaseUrl) throw new Error("NEXT_PUBLIC_AUTH_BASE_URL が設定されていません");

const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL;
if (!apiBaseUrl) throw new Error("NEXT_PUBLIC_API_BASE_URL が設定されていません");

export const authFetchClient = getAuthFetchClient(authBaseUrl);
export const $auth = getAuthQueryClient(authFetchClient);

export const membersMiddleware = getAuthMiddleware(authFetchClient);

export const fetchClientMembers = getApiFetchClient(apiBaseUrl, membersMiddleware);
export const $apiMembers = getApiQueryClient(fetchClientMembers);

export const fetchClientNoAuth = getApiFetchClient(apiBaseUrl);
export const $apiNoAuth = getApiQueryClient(fetchClientNoAuth);

export function useUserIdFromAccessToken(): string | undefined {
  const [userId, setUserId] = useState<string | undefined>(); 

  useEffect(() => {
    (async () => {
      const access_token = localStorage.getItem("exhibitor_access_token");
      if (!access_token) return;
      const payload = decodeAccessToken(access_token);
      setUserId(payload.sub);
    })();
  }, []);

  return userId;
}