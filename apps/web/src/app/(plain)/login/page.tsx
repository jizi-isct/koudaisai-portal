"use client";

import React, {Suspense, useCallback} from "react";
import {useRouter} from "next/navigation";
import "../../globals.css";
import "../../members.css"
import styles from "./page.module.css"
import Logo from "@/components/Logo/Logo";
import {Input} from "@/components/common/Input";
import {NextPhaseButton} from "@/components/activation/NextPhaseButton";
import {login} from "@/lib";
import Link from "next/link";


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
    const password = formData.get("password") as string;

    try {
      await login(mAddress, password);
    } catch (e) {
      setError(`${e}`)
      return
    }

    router.push("/");
  }, [router])

  return (
    <main>
      <form className={styles.root} onSubmit={handleSubmit}>
        <div className={styles.logo}>
          <Logo/>
        </div>
        <h1 className={styles.title}>ログイン</h1>
        <p className={styles.p}>ログインに必要な情報を入力してください</p>
        <label className={styles.label}>
          mアドレス
          <Input
            placeholder={"メールアドレスを入力してください"}
            required
            name={"mAddress"}
            type={"email"}
          />
        </label>
        <label className={styles.label}>
          パスワード
          <Input
            placeholder={"パスワードを入力してください"}
            required
            name={"password"}
            type={"password"}
            width={"100%"}
          />
        </label>
        <div className={styles.links}>
          <Link href={"/"}>トップページに戻る</Link>
          <Link href={"/reset-password"}>パスワードを忘れた場合</Link>
        </div>
        {error && <p className={styles.error}>{error}</p>}
        <NextPhaseButton type={"submit"} label={"工大祭ポータルに進む"}/>
      </form>
    </main>
  )
}