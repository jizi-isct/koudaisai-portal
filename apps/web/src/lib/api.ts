"use client";

import createClient from "openapi-react-query";
import {getTokensAdmin, getTokensMembers, getUserIdFromAccessToken} from "./auth";
import {paths} from "./api_v1";
import createFetchClient, {type Middleware} from "openapi-fetch";
import {Exhibitor} from "@/lib/types";
import {useEffect, useState} from "react";

const authMiddlewareMembers: Middleware = {
  async onRequest({request}) {
    const tokens = await getTokensMembers();

    //ログインされてない->ログイン画面へ
    if (!tokens) {
      window.location.assign("/login")
      return;
    }

    request.headers.set("Authorization", `Bearer ${tokens.access_token}`);
    return request;
  },
}

const authMiddlewareAdmin: Middleware = {
  async onRequest({request}) {
    const tokens = await getTokensAdmin();

    //ログインされてない->ログイン画面へ
    if (!tokens) {
      window.location.assign("/admin/login")
      return;
    }

    request.headers.set("Authorization", `Bearer ${tokens.access_token}`);
    return request;
  }
}

//membersのログイン状態を確認する関数
export async function isLoggedInMembers(): Promise<boolean> {
  const tokens = await getTokensMembers();
  return !!tokens;
}

//adminのログイン状態を確認する関数
export async function isLoggedInAdmin(): Promise<boolean> {
  const tokens = await getTokensAdmin();
  return !!tokens;
}

export function useIsLoggedInMembers(): boolean | undefined {
  const [isLoggedIn, setIsLoggedIn] = useState<boolean | undefined>();

  useEffect(() => {
    (async () => {
      const loggedIn = await isLoggedInMembers();
      setIsLoggedIn(loggedIn);
    })();
  }, []);

  return isLoggedIn;
}

export function useIsLoggedInAdmin(): boolean | undefined {
  const [isLoggedIn, setIsLoggedIn] = useState<boolean | undefined>();

  useEffect(() => {
    (async () => {
      const loggedIn = await isLoggedInAdmin();
      setIsLoggedIn(loggedIn);
    })();
  }, []);

  return isLoggedIn;
}

//membersトークンを乗せたリクエストを送るclients
export const fetchClientMembers = createFetchClient<paths>({baseUrl: process.env.NEXT_PUBLIC_API_BASE_URL})
fetchClientMembers.use(authMiddlewareMembers)

export const $apiMembers = createClient(fetchClientMembers)


//adminトークンを乗せたリクエストを送るclients

export const fetchClientAdmin = createFetchClient<paths>({baseUrl: process.env.NEXT_PUBLIC_API_BASE_URL})
fetchClientAdmin.use(authMiddlewareAdmin)

export const $apiAdmin = createClient(fetchClientAdmin)

export const fetchClientNoAuth = createFetchClient<paths>({baseUrl: process.env.NEXT_PUBLIC_API_BASE_URL})

export const $apiNoAuth = createClient(fetchClientNoAuth)

export type apiQueryClientType = typeof $apiNoAuth

export type apiClientType = typeof fetchClientNoAuth

export type User = paths["/users/{user_id}"]["get"]["responses"]["200"]["content"]["application/json"];

// ユーザ情報を取得する関数
export const getUser = async (): Promise<User> => {
  const userId = await getUserIdFromAccessToken();
  if (!userId) {
    throw new Error("不明なユーザーです");
  }
  const res = await fetchClientMembers.GET("/users/{user_id}", {
    params: {
      path: {
        user_id: userId,
      },
    },
  });
  if (!res.data) {
    throw new Error("ユーザー情報の取得に失敗しました");
  }
  return res.data;
}

// 参加団体情報を取得する関数
export const getExhibitor = async (exhibitorId: string): Promise<Exhibitor> => {
  const res = await fetchClientMembers.GET("/exhibitors/{id}", {
    params: {
      path: {
        id: exhibitorId,
      },
    },
  });
  if (!res.data) {
    throw new Error("参加団体情報の取得に失敗しました");
  }
  return res.data;
}

// 参加団体情報を更新する関数
export const updateExhibitor = async (exhibitor: Exhibitor): Promise<Exhibitor>  => {
  const res = await fetchClientMembers.PUT("/exhibitors/{id}", {
    params: {
      path: {
        id: exhibitor.id,
      },
    },
    headers: {
      "Content-Type": "application/json",
    },
    body: {
      exhibition_name: exhibitor.exhibition_name ?? undefined,
      icon_id: exhibitor.icon_id ?? undefined,
      description: exhibitor.description ?? undefined,
    },
  });

  if (!res.data) {
    throw new Error("Failed to update exhibitor");
  }

  return res.data;
}