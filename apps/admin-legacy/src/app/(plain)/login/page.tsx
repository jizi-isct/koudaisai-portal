"use client";

import {setAdminTokens} from "@koudaisai/shared-auth-admin";
import {LoadingScreen} from "@koudaisai/shared-ui";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Suspense} from "react";
import styles from "./page.module.css";
import {$auth} from "@/lib/api";
import {getSearchParam} from "@/lib/browserNavigation";

export default function Login() {
  return (
    <Suspense>
      <QueryClientProvider client={new QueryClient()}>
        <Inner />
      </QueryClientProvider>
    </Suspense>
  );
}

function Inner() {
  const code = getSearchParam("code");
  const state = getSearchParam("state");
  const {data, error} = $auth.useQuery("post", "/admin/redirect", {
    body: {code: code ?? "", state: state ?? ""},
    enabled: code !== null && state !== null,
  });

  if (!code || !state) {
    window.location.assign(`${process.env.NEXT_PUBLIC_AUTH_BASE_URL}/admin/login`);
    return <div className={styles.root}><LoadingScreen /></div>;
  }

  if (data) {
    setAdminTokens(data);
    window.location.assign("/");
  }

  return (
    <div className={styles.root}>
      {error && <p style={{color: "red"}}>{String(error)}</p>}
      {!data && !error && <LoadingScreen />}
    </div>
  );
}
