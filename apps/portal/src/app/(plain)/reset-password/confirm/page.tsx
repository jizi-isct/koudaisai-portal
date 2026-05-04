"use client";

import React, {Suspense, useCallback} from "react";
import {useRouter, useSearchParams} from "next/navigation";
import "../../../globals.css";
import "../../../members.css"
import styles from "./page.module.css"
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Result} from "antd";
import Logo from "@/components/Logo/Logo";
import {Input} from "@/components/common/Input";
import {NextPhaseButton} from "@/components/activation/NextPhaseButton";
import {authFetchClient} from "@/lib/api";
import { validatePassword } from "@koudaisai/shared-utils";

export default function Page() {
  return (
    <Suspense>
      <Inner1/>
    </Suspense>
  )
}

function Inner1() {
  const params = useSearchParams()
  const token = params.get("token")
  if (token === null) {
    return (
      <main>
        <Result
          status="error"
          title="クエリパラメータに不足があります"
          subTitle="tokenが指定されていません。URLに?token=xxxxのように指定してください。"
        />
      </main>
    )
  }
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner2 token={token}/>
    </QueryClientProvider>
  )
}

export function Inner2({token}: { token: string }) {
  const [error, setError] = React.useState<string | null>(null);
  const router = useRouter()

  const handleSubmit = useCallback(async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    const formData = new FormData(document.querySelector("form") as HTMLFormElement);
    const password = formData.get("password") as string;
    const passwordConfirm = formData.get("passwordConfirm") as string;

    if (password !== passwordConfirm) {
      setError("パスワードが一致しません。");
      return
    }

    if (!validatePassword(password)) {
      setError("パスワードは8文字以上、英大文字、英小文字、数字、記号を含む必要があります。");
      return;
    }

    try {
      const response = await authFetchClient.POST("/password/reset/confirm", {
        body: {
          reset_token: token,
          new_password: password
        }
      })

      if (response.error) {
        setError(`エラー：${response.error}`);
        return;
      }
    } catch (e) {
      setError(`エラー：${e}`)
      return
    }

    router.push("./success");
  }, [router, token])

  return (
    <main>
      <form className={styles.root} onSubmit={handleSubmit}>
        <div className={styles.logo}>
          <Logo/>
        </div>
        <h1 className={styles.title}>パスワードのリセット</h1>
        <p className={styles.p}>新しいパスワードを入力してください</p>
        <Input
          placeholder={"パスワードを入力してください"}
          required
          name={"password"}
          type={"password"}
        />
        <Input
          placeholder={"パスワードをもう一度入力してください"}
          required
          name={"passwordConfirm"}
          type={"password"}
        />
        {error && <p className={styles.error}>{error}</p>}
        <NextPhaseButton type={"submit"} label={"パスワードをリセット"}/>
      </form>
    </main>
  )
}