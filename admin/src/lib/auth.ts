import nextBase64 from 'next-base64';
import {paths} from "@/lib/auth_v1";
import createFetchClient from "openapi-fetch";
import createClient from "openapi-react-query";

export type Tokens = {
  refresh_token: string,
  access_token: string,
}

export const fetchClient = createFetchClient<paths>({baseUrl: process.env.NEXT_PUBLIC_API_BASE_URL})

export const $auth = createClient(fetchClient)

/**
 * refresh_tokenとaccess_tokenを取得する．
 * @returns トークンがlocalStorageに存在する場合はTokensを返します．トークンが期限切れだった場合はrefreshを試み，成功した場合はTokensを返します．
 */
export async function getTokens(): Promise<Tokens | undefined> {
  const refresh_token = localStorage.getItem("exhibitor_refresh_token")
  const access_token = localStorage.getItem("exhibitor_access_token")

  //nullだったらundefinedに
  if (refresh_token === null || access_token === null) {
    return undefined
  }

  //アクセストークンのexp確認
  const access_token_payload_base64 = access_token!.split(".")[1]
  const access_token_payload = JSON.parse(nextBase64.decode(access_token_payload_base64))
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
  const {data} = await fetchClient.POST(
    "/refresh",
    {
      body: {
        refresh_token: refresh_token
      }
    }
  )

  if (data) {
    localStorage.setItem("exhibitor_refresh_token", data.refresh_token)
    localStorage.setItem("exhibitor_access_token", data.access_token)
    return data
  } else {
    // refresh tokenが無効
    return undefined
  }
}

