import { AuthFetchClient, decodeAccessToken } from '@koudaisai/shared-auth';
import type { Middleware } from 'openapi-fetch';

const ACCESS_TOKEN_KEY = 'exhibitor_access_token';
let refreshPromise: Promise<{ access_token: string } | undefined> | undefined;
// リフレッシュトークンは HttpOnly Cookie(dev: `refresh_token` / prod: `__Host-refresh_token`)で
// 配送される。JS からは読めないため localStorage には一切保持しない。

export function getAuthMiddleware(fetchClient: AuthFetchClient): Middleware {
  return {
    async onRequest({ request }) {
      const tokens = await getTokensMembers(fetchClient);

      //ログインされてない->ログイン画面へ
      if (!tokens) {
        window.location.assign('/login');
        return;
      }

      request.headers.set('Authorization', `Bearer ${tokens.access_token}`);
      return request;
    },
  };
}

//団体向けアクセストークンの取得。期限切れ/不在ならリフレッシュ Cookie で再発行する。
export const getTokensMembers = async (fetchClient: AuthFetchClient) => {
  const access_token = localStorage.getItem(ACCESS_TOKEN_KEY);

  //アクセストークンが有効ならそのまま使う(時計ずれ対策に30秒の余裕を持たせる)
  if (access_token !== null) {
    const access_token_exp = decodeAccessToken(access_token).exp as number;
    if (access_token_exp * 1000 > Date.now() + 30_000) {
      return { access_token };
    }
  }

  //期限切れ or 不在 -> リフレッシュトークン Cookie で再発行を試みる。
  //Cookie は HttpOnly のため明示送信できない。client の credentials:'include' で自動送出される。
  //サーバはリクエストボディを取らず、Cookie のみから回転する。
  //リフレッシュトークンは回転式で再利用を検知するとセッションファミリが失効するため、
  //同時に複数の API リクエストが期限切れを検知しても 1 回だけ /refresh する。
  refreshPromise ??= refreshTokensMembers(fetchClient);
  try {
    return await refreshPromise;
  } finally {
    refreshPromise = undefined;
  }
};

const refreshTokensMembers = async (fetchClient: AuthFetchClient) => {
  const { data } = await fetchClient.POST('/refresh', {});

  if (data) {
    localStorage.setItem(ACCESS_TOKEN_KEY, data.access_token);
    return { access_token: data.access_token };
  }

  //リフレッシュ失敗(Cookie 無効/期限切れ/reuse 検知) -> 未ログイン扱い。
  localStorage.removeItem(ACCESS_TOKEN_KEY);
  return undefined;
};

//団体向けログイン。リフレッシュトークンは Set-Cookie で配送され、本文には含まれない。
export const login = async (
  fetchClient: AuthFetchClient,
  email: string,
  password: string,
) => {
  const { data, response } = await fetchClient.POST('/login', {
    body: {
      m_address: email,
      password: password,
    },
  });

  if (data) {
    localStorage.setItem(ACCESS_TOKEN_KEY, data.access_token);
    return data;
  } else {
    switch (response.status) {
      case 401:
        throw new Error('mアドレスまたはパスワードが間違えています。');
      case 429:
        throw new Error(
          'ログイン試行回数が多すぎます。しばらく時間を置いてから再度お試しください。',
        );
      default:
        throw new Error('内部エラー。開発者に問い合わせてください。');
    }
  }
};

//団体向けログアウト。サーバ側で回転セッションを失効し、Cookie をクリアする(冪等)。
export const logout = async (fetchClient: AuthFetchClient) => {
  try {
    await fetchClient.POST('/logout', {});
  } catch {
    //ネットワーク失敗時もローカルのアクセストークンは破棄して未ログイン状態にする。
  }
  if (typeof window !== 'undefined') {
    localStorage.removeItem(ACCESS_TOKEN_KEY);
  }
};

//アクセストークンの localStorage への保存
export const setAccessToken = (token: string) => {
  if (typeof window !== 'undefined') {
    localStorage.setItem(ACCESS_TOKEN_KEY, token);
  }
};
