import nextBase64 from 'next-base64';
import {paths} from "./auth_v1";
import createFetchClient from "openapi-fetch";
import createClient from "openapi-react-query";

export type Tokens = {
  refresh_token: string,
  access_token: string,
}

export const fetchClientAuth = createFetchClient<paths>({baseUrl: process.env.NEXT_PUBLIC_AUTH_BASE_URL})

export const $auth = createClient(fetchClientAuth)

/**
 * members_refresh_tokenとmembers_access_tokenの取得を試みる．
 * @returns トークンがlocalStorageに存在する場合はTokensを返します．トークンが期限切れだった場合はrefreshを試み，成功した場合はTokensを返します．
 */
export async function getTokensMembers(): Promise<Tokens | undefined> {
  const refresh_token = localStorage.getItem("exhibitor_refresh_token")
  const access_token = localStorage.getItem("exhibitor_access_token")

  //nullだったらundefinedに
  if (refresh_token === null || access_token === null) {
    return undefined
  }

  //アクセストークンのexp確認
  const access_token_payload_base64 = access_token!.split(".")[1]
  const access_token_payload = JSON.parse(nextBase64.decode(access_token_payload_base64
    .replace(/-/g, "+")
    .replace(/_/g, "/")))
  const access_token_exp = access_token_payload.exp as number;
  if (access_token_exp * 1000 >= Date.now()) {
    //有効期限OK
    return {
      refresh_token: refresh_token, access_token: access_token
    }
  }

  //リフレッシュトークンのexp確認
  const refresh_token_payload_base64 = refresh_token!.split(".")[1]
  const refresh_token_payload = JSON.parse(nextBase64.decode(refresh_token_payload_base64))
  const refresh_token_exp = refresh_token_payload.exp as number;
  if (refresh_token_exp * 1000 < Date.now()) {
    //有効期限ダメ
    return undefined
  }

  //トークンのリフレッシュを試みる
  const {data} = await fetchClientAuth.POST(
    "/refresh",
    {
      body: {
        refresh_token: refresh_token
      }
    }
  )

  //リフレッシュに成功した場合トークンを保存しreturn
  if (data) {
    localStorage.setItem("exhibitor_refresh_token", data.refresh_token)
    localStorage.setItem("exhibitor_access_token", data.access_token)
    return data
  } else {
    // refresh tokenが無効
    return undefined
  }
}

/**
 * admin_refresh_tokenとadmin_access_tokenの取得を試みる．
 * @returns トークンがlocalStorageに存在する場合はTokensを返します．トークンが期限切れだった場合はrefreshを試み，成功した場合はTokensを返します．
 */
export async function getTokensAdmin(): Promise<Tokens | undefined> {
  const refresh_token = localStorage.getItem("admin_refresh_token")
  const access_token = localStorage.getItem("admin_access_token")

    console.log(refresh_token)
    console.log(access_token)
  //nullだったらundefinedに
  if (refresh_token === null || access_token === null) {
    return undefined
  }

  //アクセストークンのexp確認
  const access_token_payload_base64 = access_token!.split(".")[1]
  const access_token_payload = JSON.parse(nextBase64.decode(access_token_payload_base64
    .replace(/-/g, "+")
    .replace(/_/g, "/")))
  const access_token_exp = access_token_payload.exp as number;
  if (access_token_exp * 1000 >= Date.now()) {
    //有効期限OK
    return {
      refresh_token: refresh_token, access_token: access_token
    }
  }

  //リフレッシュトークンのexp確認
  const refresh_token_payload_base64 = refresh_token!.split(".")[1]
  const refresh_token_payload = JSON.parse(nextBase64.decode(refresh_token_payload_base64))
  const refresh_token_exp = refresh_token_payload.exp as number;
  if (refresh_token_exp * 1000 < Date.now()) {
    //有効期限ダメ
    return undefined
  }


  //TODO: トークンのリフレッシュを試みる

  const {data} = await fetchClientAuth.POST(
    "/refresh",
    {
      body: {
        refresh_token: refresh_token
      }
    }
  )

  // トークンが有効だった場合，トークンを保存する
  if (data) {
    localStorage.setItem("admin_refresh_token", data.refresh_token)
    localStorage.setItem("admin_access_token", data.access_token)
    return data
  } else {
    // refresh tokenが無効
    return undefined
  }
}

