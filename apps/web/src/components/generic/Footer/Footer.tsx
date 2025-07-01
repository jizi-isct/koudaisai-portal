"use client";

import styles from "./Footer.module.css";
import Link from "next/link";
import Image from "next/image";
import {useIsLoggedInMembers} from "@/lib";

export const Footer = () => {
  const isLoggedIn = useIsLoggedInMembers()

  return (
    <footer className={styles.footer}>
      <div className={styles.logoWrapper}>
        <Image
          src="/components/generic/Header/members_logo.png"
          alt="Koudaisai Portal Admin Site Logo"
          width={40}
          height={40}
        />
      </div>
      <div className={styles.navWrapper}>
        <Link href="/" className={styles.navItem}>ホーム</Link>
        <Link href="/forms/" className={styles.navItem}>フォーム</Link>
        <Link href="/documents/" className={styles.navItem}>資料</Link>
        <Link href="/questions/" className={styles.navItem}>よくある質問</Link>
      </div>
      <p className={styles.contacts}>
        公式LINEアカウント：{
        isLoggedIn === undefined ? <>...</> : isLoggedIn
          ? <a href={"https://lin.ee/9Sud7lK"}>https://lin.ee/9Sud7lK</a>
          : <a href={"https://lin.ee/43ugikz"}>https://lin.ee/43ugikz</a>
      } <br/>
        メールアドレス([at]を@に置き換えてください)：sanka[at]koudaisai.jp
      </p>
      <p className={styles.copyrightText}>©︎ 2025 JIZI All Rights Reserved.</p>
    </footer>
  );
};