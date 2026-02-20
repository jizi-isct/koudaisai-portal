'use client';

import { useEffect, useState } from 'react';
import { getTokensAdmin } from './auth.js';

/**
 * 管理者のログイン状態を非同期で確認
 */
export async function isLoggedInAdmin(): Promise<boolean> {
  const tokens = await getTokensAdmin();
  return !!tokens;
}

/**
 * 管理者がログインしているかチェックするフック
 */
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