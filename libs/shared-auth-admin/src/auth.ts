import type { Middleware } from 'openapi-fetch';
import { decodeJwtPayload } from '@koudaisai/shared-auth';
import type { AuthFetchClient, Tokens } from '@koudaisai/shared-auth';

export function getAuthMiddleware(
  fetchClient: AuthFetchClient,
  loginPath = '/admin/login',
): Middleware {
  return {
    async onRequest({ request }) {
      const tokens = await getTokensAdmin(fetchClient);

      //ログインされてない->ログイン画面へ
      if (!tokens) {
        window.location.assign(loginPath);
        return;
      }

      request.headers.set('Authorization', `Bearer ${tokens.access_token}`);
      return request;
    },
  };
}
/**
 * admin_refresh_tokenとadmin_access_tokenの取得を試みる．
 * @returns トークンがlocalStorageに存在する場合はTokensを返します．トークンが期限切れだった場合はrefreshを試み，成功した場合はTokensを返します．
 */
export async function getTokensAdmin(
  fetchClient: AuthFetchClient,
): Promise<Tokens | undefined> {
  const refresh_token = localStorage.getItem('admin_refresh_token');
  const access_token = localStorage.getItem('admin_access_token');

  //nullだったらundefinedに
  if (refresh_token === null || access_token === null) {
    return undefined;
  }

  // アクセストークン有効期限チェック
  // TODO: decodeAccessToken() と重複している
  const access_token_payload = decodeJwtPayload(access_token);
  const access_token_exp = access_token_payload.exp as number;
  if (access_token_exp * 1000 >= Date.now()) {
    //有効期限OK
    return { refresh_token, access_token };
  }

  //リフレッシュトークンのexp確認
  const refresh_token_payload = decodeJwtPayload(refresh_token);
  const refresh_token_exp = refresh_token_payload.exp as number;
  if (refresh_token_exp * 1000 < Date.now()) {
    //有効期限ダメ
    return undefined;
  }

  //TODO: トークンのリフレッシュを試みる

  const { data } = await fetchClient.POST('/refresh', {
    body: {
      refresh_token: refresh_token,
    },
  });

  // トークンが有効だった場合，トークンを保存する
  if (data) {
    setAdminTokens(data);
    return data;
  } else {
    // refresh tokenが無効
    return undefined;
  }
}

/**
 * 管理者のログアウト
 */
export function logoutAdmin() {
  localStorage.removeItem('admin_refresh_token');
  localStorage.removeItem('admin_access_token');
}

export function setAdminTokens(tokens: Tokens) {
  localStorage.setItem('admin_refresh_token', tokens.refresh_token);
  localStorage.setItem('admin_access_token', tokens.access_token);
}
