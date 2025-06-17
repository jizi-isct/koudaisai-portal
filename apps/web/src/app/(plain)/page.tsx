'use client'; // クライアントサイドコンポーネントとして実行するために追加

import {useEffect, useState} from 'react';
import {getTokensMembers} from "@/lib";
import "../globals.css";
import styles from "./page.module.css";

import {TopPageUnauthenticated} from "@/components/TopPageUnauthenticated/TopPageUnauthenticated";
import {TopPageAuthenticated} from "@/components/TopPageAuthenticated/TopPageAuthenticated";

export default function Page() {
  const [authenticated, setAuthenticated] = useState(false);

  useEffect(() => {
    // まずローカルストレージに access_token があるか確認
    const token = localStorage.getItem("exhibitor_access_token");
    if (token) {
      setAuthenticated(true); // 仮認証（存在ベース）

      // 非同期でトークンの有効性確認
      (async () => {
        const valid = await getTokensMembers();
        if (!valid) {
          setAuthenticated(false); // 無効なら認証状態をリセット
        }
      })();
    }
  }, []);

  return (
    <>
      {authenticated ? (
        <>
          <TopPageAuthenticated />
        </>
      ) : (
        <>
          <TopPageUnauthenticated />
        </>
      )}
    </>
  );
}