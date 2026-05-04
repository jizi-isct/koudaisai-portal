"use client";

import styles from "@/app/(plain)/reset-password/page.module.css";
import Logo from "@/components/Logo/Logo";
import {NextPhaseButton} from "@/components/activation/NextPhaseButton";
import React from "react";
import {useRouter} from "next/navigation";

export default function Page() {
  const router = useRouter()

  return (
    <main className={styles.root}>
      <div className={styles.logo}>
        <Logo/>
      </div>
      <h1 className={styles.title}>パスワードのリセット</h1>
      <p className={styles.p}>パスワードをリセットしました</p>
      <NextPhaseButton onClick={() => router.push("/login")} label={"工大祭ポータルに進む"}/>
    </main>
  )
}