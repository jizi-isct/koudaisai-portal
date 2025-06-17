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
    (async () => {
      const tokens = await getTokensMembers()
      if (tokens) {
        setAuthenticated(true);
      }
    })();
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