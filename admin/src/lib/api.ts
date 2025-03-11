import createClient from "openapi-react-query";
import {getTokens} from "@/lib/auth";
import {components, paths} from "@/lib/api_v1";
import createFetchClient, {type Middleware} from "openapi-fetch";

export type Item = components["schemas"]["Item"];
export type Form = components["schemas"]["Form"];

const authMiddleware: Middleware = {
  async onRequest({request}) {
    const tokens = await getTokens();

    //ログインされてない->ログイン画面へ
    if (!tokens) {
      window.location.assign("/login")
      return;
    }

    request.headers.set("Authorization", `Bearer ${tokens.access_token}`);
    return request;
  },
}

export const fetchClient = createFetchClient<paths>({baseUrl: process.env.NEXT_PUBLIC_API_BASE_URL})
fetchClient.use(authMiddleware)

export const $api = createClient(fetchClient)