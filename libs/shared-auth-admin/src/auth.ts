import type { Middleware } from 'openapi-fetch';
import { decodeJwtPayload } from '@koudaisai/shared-auth';
import type { Tokens } from '@koudaisai/shared-auth';

export function getAuthMiddleware(loginPath = '/admin/login'): Middleware {
  return {
    async onRequest({ request }) {
      const tokens = getTokensAdmin();

      //ログインされてない/期限切れ -> ログイン画面(Keycloak)へ
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
 * admin_refresh_token と admin_access_token を localStorage から取得する。
 *
 * 管理者(JIZI)は Keycloak OIDC のトークンを保持する。新バックエンドには団体向けの
 * 回転セッション `/refresh`(Cookie ベース)しか無く、Keycloak トークンを更新する
 * エンドポイントは存在しない。さらに団体向け `/refresh` を admin から叩くと、別タブ等で
 * 団体セッション Cookie が存在した場合に団体トークンを取り込んでしまう恐れがあるため呼ばない。
 *
 * よってアクセストークンが期限切れの場合は undefined を返し、Keycloak へ再ログイン
 * させる(Keycloak の SSO セッションが生きていればリダイレクトは透過的に完了する)。
 *
 * @returns 有効なトークンがあれば Tokens、無ければ undefined。
 */
export function getTokensAdmin(): Tokens | undefined {
  const refresh_token = localStorage.getItem('admin_refresh_token');
  const access_token = localStorage.getItem('admin_access_token');

  //nullだったらundefinedに
  if (refresh_token === null || access_token === null) {
    return undefined;
  }

  // アクセストークン有効期限チェック
  const access_token_exp = decodeJwtPayload(access_token).exp as number;
  if (access_token_exp * 1000 >= Date.now()) {
    //有効期限OK
    return { refresh_token, access_token };
  }

  //期限切れ -> Keycloak へ再ログイン(サーバ側に admin 用 refresh が無いため)。
  return undefined;
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
