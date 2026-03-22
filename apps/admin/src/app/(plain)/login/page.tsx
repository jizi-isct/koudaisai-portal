"use client";

import {useSearchParams} from "next/navigation";
import {$auth} from "@/lib/api";
import {setAdminTokens} from "@koudaisai/shared-auth-admin";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Suspense} from "react";
import {LoadingScreen} from "@koudaisai/shared-ui";
import styles from "./page.module.css";

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
  const search = useSearchParams();
  const code = search.get("code")!;
  const state = search.get("state")!;
  const {data, error} = $auth.useQuery("post", "/admin/redirect", {
    body: {code, state},
  });

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
