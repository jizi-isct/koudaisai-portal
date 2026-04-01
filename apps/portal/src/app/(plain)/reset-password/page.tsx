"use client";

import React, {Suspense, useCallback} from "react";
import {useRouter} from "next/navigation";
import "../../globals.css";
import "../../members.css"
import styles from "./page.module.css"
import Logo from "@/components/Logo/Logo";
import {Input} from "@/components/common/Input";
import {NextPhaseButton} from "@/components/activation/NextPhaseButton";
import {authFetchClient} from "@/lib/api";


export default function Page() {
  return (
    <Suspense>
      <Inner/>
    </Suspense>
  )
}

export function Inner() {
  const [error, setError] = React.useState<string | null>(null);
  const router = useRouter()

  const handleSubmit = useCallback(async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    const formData = new FormData(document.querySelector("form") as HTMLFormElement);
    const mAddress = formData.get("mAddress") as string;

    try {
      const response = await authFetchClient.POST("/password/reset", {
        body: {
          m_address: mAddress
        }
      })
      if (response.error) {
        setError(response.error)
        return
      }
    } catch (e) {
      setError(`${e}`)
      return
    }

    router.push("./sent");
  }, [router])

  return (
    <main>
      <form className={styles.root} onSubmit={handleSubmit}>
        <div className={styles.logo}>
          <Logo/>
        </div>
        <h1 className={styles.title}>パスワードのリセット</h1>
        <p className={styles.p}>
          あなたのmアドレスを入力してください。
          パスワードリセット用のメールを送信します。
        </p>
        <div className={styles.input}>
          <Input
            placeholder={"メールアドレスを入力してください"}
            required
            name={"mAddress"}
            type={"email"}
          />
        </div>
        {error && <p className={styles.error}>{error}</p>}
        <NextPhaseButton type={"submit"} label={"工大祭ポータルに進む"}/>
      </form>
    </main>
  )
}