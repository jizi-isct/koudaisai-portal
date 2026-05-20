import {AuthFetchClient, decodeAccessToken, decodeJwtPayload} from "@koudaisai/shared-auth";
import type {Middleware} from "openapi-fetch";

const ACCESS_TOKEN_KEY = 'exhibitor_access_token';
const REFRESH_TOKEN_KEY = 'exhibitor_refresh_token';

export function getAuthMiddleware(fetchClient: AuthFetchClient): Middleware {
  return {
    async onRequest({request}) {
      const tokens = await getTokensMembers(fetchClient);

      //ログインされてない->ログイン画面へ
      if (!tokens) {
        window.location.assign("/login")
        return;
      }

      request.headers.set("Authorization", `Bearer ${tokens.access_token}`);
      return request;
    }
  }

}

//団体向けトークンの取得
export const getTokensMembers = async (fetchClient: AuthFetchClient) => {
  const refresh_token = localStorage.getItem(REFRESH_TOKEN_KEY)
  const access_token = localStorage.getItem(ACCESS_TOKEN_KEY)

  //nullだったらundefinedに
  if (refresh_token === null || access_token === null) {
    return undefined
  }

  //アクセストークンのexp確認
  const access_token_exp = decodeAccessToken(access_token).exp as number;
  if (access_token_exp * 1000 >= Date.now()) {
    //有効期限OK
    return {
      refresh_token: refresh_token, access_token: access_token
    }
  }

  //リフレッシュトークンのexp確認
  const refresh_token_payload = decodeJwtPayload(refresh_token)
  const refresh_token_exp = refresh_token_payload.exp as number;
  if (refresh_token_exp * 1000 < Date.now()) {
    //有効期限ダメ
    localStorage.removeItem("exhibitor_refresh_token")
    localStorage.removeItem("exhibitor_access_token")
    return undefined
  }

  //トークンのリフレッシュを試みる
  const {data} = await fetchClient.POST(
    "/refresh",
    {
      body: {
        refresh_token: refresh_token
      }
    }
  )

  //リフレッシュに成功した場合トークンを保存しreturn
  if (data) {
    localStorage.setItem("exhibitor_access_token", data.access_token)
    return data
  } else {
    // refresh tokenが無効
    return undefined
  }
};

//団体向けログイン
export const login = async (fetchClient: AuthFetchClient, email: string, password: string) => {
  const {data, response} = await fetchClient.POST(
    "/login",
    {
      body: {
        m_address: email,
        password: password
      }
    }
  )

  if (data) {
    localStorage.setItem(REFRESH_TOKEN_KEY, data.refresh_token)
    localStorage.setItem(ACCESS_TOKEN_KEY, data.access_token)
    return data
  } else {
    switch (response.status) {
      case 401:
        throw new Error("mアドレスまたはパスワードが間違えています。");
      case 429:
        throw new Error("ログイン試行回数が多すぎます。しばらく時間を置いてから再度お試しください。");
      default:
        throw new Error("内部エラー。開発者に問い合わせてください。");
    }
  }
};

//団体向けログアウト
export const logout = () => {
  // localStorageから団体トークンを削除
  if (typeof window !== 'undefined') {
    localStorage.removeItem(ACCESS_TOKEN_KEY);
    localStorage.removeItem(REFRESH_TOKEN_KEY);
  }
};

//アクセストークンのLocalStorageへの保存
export const setAccessToken = (token: string) => {
  if (typeof window !== 'undefined') {
    localStorage.setItem(ACCESS_TOKEN_KEY, token);
  }
};

//リフレッシュトークンのLocalStorageへの保存
export const setRefreshToken = (token: string) => {
  if (typeof window !== 'undefined') {
    localStorage.setItem(REFRESH_TOKEN_KEY, token);
  }
};
