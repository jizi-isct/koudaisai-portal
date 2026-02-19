'use client';

import { useEffect, useState } from 'react';
import { getTokensMembers } from './auth';

/**
 * 団体がログインしているかチェックするフック
 */
export const useIsLoggedInMembers = () => {
  const [isLoggedIn, setIsLoggedIn] = useState(false);

  useEffect(() => {
    (async () => {
      const tokens = await getTokensMembers();
      setIsLoggedIn(!!tokens);
    })();
  }, []);

  return isLoggedIn;
};