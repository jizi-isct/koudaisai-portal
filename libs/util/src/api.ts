import createClient from "openapi-react-query";
import {getTokensAdmin, getTokensMembers} from "./auth";
import {paths} from "./api_v1";
import createFetchClient, {type Middleware} from "openapi-fetch";

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

//membersトークンを乗せたリクエストを送るclients
export const fetchClientMembers = createFetchClient<paths>({baseUrl: process.env.NEXT_PUBLIC_API_BASE_URL})
fetchClientMembers.use(authMiddlewareMembers)

export const $apiMembers = createClient(fetchClientMembers)


//adminトークンを乗せたリクエストを送るclients

export const fetchClientAdmin = createFetchClient<paths>({baseUrl: process.env.NEXT_PUBLIC_API_BASE_URL})
fetchClientMembers.use(authMiddlewareAdmin)

export const $apiAdmin = createClient(fetchClientAdmin)