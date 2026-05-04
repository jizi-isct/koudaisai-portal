"use client";

import React from "react";
import "../../../globals.css";
import "../../../members.css"
import styles from "./page.module.css"
import Logo from "@/components/Logo/Logo";

export default function Page() {
  return (
    <main>
      <form className={styles.root}>
        <div className={styles.logo}>
          <Logo/>
        </div>
        <h1 className={styles.title}>パスワードのリセット</h1>
        <p className={styles.p}>
          入力されたmアドレスで登録された参加団体責任者アカウントが存在する場合、入力されたmアドレス宛にパスワードのご案内を送信されました。ご確認ください。
        </p>
      </form>
    </main>
  )
}