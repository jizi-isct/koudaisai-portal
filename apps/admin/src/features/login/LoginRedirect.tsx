import {setAdminTokens} from "@koudaisai/shared-auth-admin";
import {LoadingScreen} from "@koudaisai/shared-ui";
import {AUTH_URL} from "astro:env/client";
import {useEffect, useState} from "react";
import {authFetchClient} from "@/features/api/api";
import styles from "./LoginRedirect.module.css";

export function LoginRedirect() {
  const [error, setError] = useState<unknown>();

  useEffect(() => {
    const search = new URLSearchParams(window.location.search);
    const code = search.get("code");
    const state = search.get("state");

    if (!code || !state) {
      window.location.assign(`${AUTH_URL}/admin/login`);
      return;
    }

    (async () => {
      const {data, error: redirectError} = await authFetchClient.POST("/admin/redirect", {
        body: {code, state},
      });

      if (data) {
        setAdminTokens(data);
        window.location.assign("/");
        return;
      }

      setError(redirectError ?? "ログインに失敗しました");
    })().catch(setError);
  }, []);

  return (
    <div className={styles.root}>
      {error ? <p className={styles.error}>{String(error)}</p> : <LoadingScreen />}
    </div>
  );
}
